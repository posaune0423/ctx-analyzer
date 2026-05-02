//! AppState — pure data model that drives the TUI render and reacts to
//! `KeyAction`s (`docs/development/wireframe/`).

use std::path::PathBuf;

use ratatui::layout::Rect;

use crate::application::usecases::list_session_turns::{self, TurnSummary};
use crate::application::usecases::list_workspace_sessions::{self, SessionListItem};
use crate::application::usecases::preview_segment::SegmentPreview;
use crate::application::usecases::{
    analyze_workspace, build_context_breakdown, estimate_tokens, preview_segment,
};
use crate::application::view_models::context_breakdown::Breakdown;
use crate::constants::preview::PREVIEW_MODAL_VISIBLE_BODY_LINES;
use crate::domain::Session;

use super::keymap::{KeyAction, Mode};
use super::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    SessionList,
    TurnList,
    Breakdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewFilter {
    Cumulative,
    Delta,
}

#[derive(Debug, Clone)]
pub struct LoadedState {
    pub session: Session,
    pub source_path: PathBuf,
    pub active_session_idx: usize,
    pub turn_list_cursor: usize,
    pub active_turn_idx: usize,
    pub turn_summaries: Vec<TurnSummary>,
    pub view_filter: ViewFilter,
    pub breakdown: Breakdown,
    pub expanded_sections: Vec<bool>,
    pub breakdown_cursor: usize,
}

impl LoadedState {
    fn new(session: Session, source_path: PathBuf, active_session_idx: usize) -> Self {
        let turn_summaries = list_session_turns::summarize(&session);
        let turn_list_cursor = 0;
        let breakdown = build_context_breakdown::group(&session);
        let expanded_sections = vec![true; breakdown.sections.len()];
        Self {
            session,
            source_path,
            active_session_idx,
            turn_list_cursor,
            active_turn_idx: turn_list_cursor,
            turn_summaries,
            view_filter: ViewFilter::Delta,
            breakdown,
            expanded_sections,
            breakdown_cursor: 0,
        }
    }

    fn rebuild_breakdown(&mut self) {
        self.breakdown = match self.view_filter {
            ViewFilter::Cumulative => {
                build_context_breakdown::group_until_turn(&self.session, self.active_turn_idx)
            }
            ViewFilter::Delta => {
                build_context_breakdown::group_delta_for_turn(&self.session, self.active_turn_idx)
            }
        };
        self.expanded_sections = vec![true; self.breakdown.sections.len()];
    }
}

#[derive(Debug, Clone)]
pub enum Overlay {
    Preview(SegmentPreview),
    Help,
    Warning(String),
}

#[derive(Debug, Clone)]
pub struct ClickRegion {
    pub rect: Rect,
    pub target: EditorTarget,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub stage: Stage,
    pub sessions: Vec<SessionListItem>,
    pub session_list_cursor: usize,
    pub loaded: Option<LoadedState>,
    /// Working directory when `inspect` started; used with git (or this path) to scope sessions.
    pub launch_dir: PathBuf,
    /// `inspect --file` / `--session`: back from Turn List quits.
    pub launched_with_path: bool,
    pub overlay: Option<Overlay>,
    pub preview_scroll: usize,
    pub footer_message: Option<String>,
    pub detail_mode: bool,
    pub theme: Theme,
    pub should_quit: bool,
    pub click_regions: Vec<ClickRegion>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Section,
    Row,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisibleNode {
    pub kind: NodeKind,
    pub section: usize,
    pub row: Option<usize>,
}

impl AppState {
    pub fn new_session_picker(
        sessions: Vec<SessionListItem>,
        theme: Theme,
        launch_dir: PathBuf,
    ) -> Self {
        let mut state = AppState {
            stage: Stage::SessionList,
            sessions,
            session_list_cursor: 0,
            loaded: None,
            launch_dir,
            launched_with_path: false,
            overlay: None,
            preview_scroll: 0,
            footer_message: None,
            detail_mode: false,
            theme,
            should_quit: false,
            click_regions: Vec::new(),
        };
        if let Some(err) = state.theme.load_error.clone() {
            state.overlay = Some(Overlay::Warning(format!(
                "Theme could not be loaded.\nUsing built-in default theme.\n\n{err}"
            )));
        }
        state
    }

    pub fn new_with_loaded_session(
        session: Session,
        sessions: Vec<SessionListItem>,
        active_session_idx: usize,
        theme: Theme,
        launched_with_path: bool,
        launch_dir: PathBuf,
    ) -> Self {
        let source_path = session.source_path.clone();
        let loaded = LoadedState::new(session, source_path, active_session_idx);
        let mut state = AppState {
            stage: Stage::TurnList,
            sessions,
            session_list_cursor: active_session_idx,
            loaded: Some(loaded),
            launch_dir,
            launched_with_path,
            overlay: None,
            preview_scroll: 0,
            footer_message: None,
            detail_mode: false,
            theme,
            should_quit: false,
            click_regions: Vec::new(),
        };
        if let Some(err) = state.theme.load_error.clone() {
            state.overlay = Some(Overlay::Warning(format!(
                "Theme could not be loaded.\nUsing built-in default theme.\n\n{err}"
            )));
        }
        state
    }

    pub fn clear_click_regions(&mut self) {
        self.click_regions.clear();
    }

    pub fn register_click(&mut self, rect: Rect, target: EditorTarget) {
        self.click_regions.push(ClickRegion { rect, target });
    }

    pub fn handle_mouse_click(&mut self, column: u16, row: u16) -> AppCommand {
        for reg in self.click_regions.iter().rev() {
            let r = reg.rect;
            if column >= r.x
                && column < r.x.saturating_add(r.width)
                && row >= r.y
                && row < r.y.saturating_add(r.height)
            {
                return AppCommand::OpenEditor(Some(reg.target.clone()));
            }
        }
        AppCommand::None
    }

    pub fn mode(&self) -> Mode {
        match &self.overlay {
            Some(Overlay::Preview(_)) => Mode::Preview,
            Some(Overlay::Help) | Some(Overlay::Warning(_)) => Mode::Help,
            None => match self.stage {
                Stage::SessionList => Mode::SessionList,
                Stage::TurnList => Mode::TurnList,
                Stage::Breakdown => Mode::Breakdown,
            },
        }
    }

    pub fn visible_nodes(&self) -> Vec<VisibleNode> {
        if self.stage != Stage::Breakdown {
            return Vec::new();
        }
        let Some(ld) = self.loaded.as_ref() else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for (s_idx, section) in ld.breakdown.sections.iter().enumerate() {
            if section.total_tokens == 0 && section.rows.is_empty() {
                continue;
            }
            out.push(VisibleNode {
                kind: NodeKind::Section,
                section: s_idx,
                row: None,
            });
            if !ld.expanded_sections.get(s_idx).copied().unwrap_or(false) {
                continue;
            }
            for (r_idx, _row) in section.rows.iter().enumerate() {
                out.push(VisibleNode {
                    kind: NodeKind::Row,
                    section: s_idx,
                    row: Some(r_idx),
                });
            }
        }
        out
    }

    pub fn current_node(&self) -> Option<VisibleNode> {
        let ld = self.loaded.as_ref()?;
        if self.stage != Stage::Breakdown {
            return None;
        }
        self.visible_nodes().get(ld.breakdown_cursor).copied()
    }

    pub fn apply(&mut self, action: KeyAction) -> AppCommand {
        self.footer_message = None;
        match action {
            KeyAction::Noop => {}
            KeyAction::Quit => self.should_quit = true,
            KeyAction::PopStage => self.pop_stage(),
            KeyAction::ToggleHelp => {
                self.overlay = match &self.overlay {
                    Some(Overlay::Help) => None,
                    _ => Some(Overlay::Help),
                };
            }
            KeyAction::Refresh => {
                if let Some(ld) = self.loaded.as_mut() {
                    if self.stage == Stage::Breakdown {
                        ld.rebuild_breakdown();
                        ld.breakdown_cursor = 0;
                        self.footer_message = Some("refreshed".into());
                    }
                }
            }
            KeyAction::StartSearch => {
                self.footer_message = Some("search not implemented yet".into());
            }
            KeyAction::MoveUp => self.move_up(),
            KeyAction::MoveDown => self.move_down(),
            KeyAction::MoveLeft => self.collapse_current(),
            KeyAction::MoveRight => self.move_right_or_preview(),
            KeyAction::Confirm => self.confirm_current(),
            KeyAction::OpenSessionList => {
                self.stage = Stage::SessionList;
                self.loaded = None;
                self.overlay = None;
                self.detail_mode = false;
                self.sessions = list_workspace_sessions::list_for_launch_dir(&self.launch_dir)
                    .unwrap_or_default();
                self.session_list_cursor = self
                    .session_list_cursor
                    .min(self.sessions.len().saturating_sub(1));
            }
            KeyAction::OpenTurnList => {
                if self.loaded.is_some() {
                    self.stage = Stage::TurnList;
                    self.overlay = None;
                    self.detail_mode = false;
                }
            }
            KeyAction::SessionGraphPlaceholder => {
                self.footer_message = Some("session graph view: coming soon".into());
            }
            KeyAction::OpenPreview => self.open_preview(),
            KeyAction::OpenInEditor => {
                return AppCommand::OpenEditor(self.editor_target());
            }
            KeyAction::ToggleDetail => {
                if self.stage == Stage::Breakdown {
                    self.detail_mode = !self.detail_mode;
                }
            }
            KeyAction::ToggleViewFilter => {
                if self.stage == Stage::Breakdown {
                    if let Some(ld) = self.loaded.as_mut() {
                        ld.view_filter = match ld.view_filter {
                            ViewFilter::Cumulative => ViewFilter::Delta,
                            ViewFilter::Delta => ViewFilter::Cumulative,
                        };
                        ld.rebuild_breakdown();
                        ld.breakdown_cursor = 0;
                        self.footer_message = Some(match ld.view_filter {
                            ViewFilter::Cumulative => {
                                "view: cumulative (full context history)".into()
                            }
                            ViewFilter::Delta => "view: scoped (this turn only)".into(),
                        });
                    }
                }
            }
            KeyAction::CloseOverlay => {
                self.overlay = None;
                self.preview_scroll = 0;
            }
        }
        AppCommand::None
    }

    fn pop_stage(&mut self) {
        match self.stage {
            Stage::Breakdown => {
                self.stage = Stage::TurnList;
                self.detail_mode = false;
                self.overlay = None;
            }
            Stage::TurnList => {
                if self.launched_with_path {
                    self.should_quit = true;
                } else {
                    self.stage = Stage::SessionList;
                    self.loaded = None;
                    self.overlay = None;
                }
            }
            Stage::SessionList => {}
        }
    }

    fn move_up(&mut self) {
        match self.mode() {
            Mode::Preview => {
                self.preview_scroll = self.preview_scroll.saturating_sub(1);
            }
            _ => match self.stage {
                Stage::SessionList => {
                    step_cursor(&mut self.session_list_cursor, -1, self.sessions.len());
                }
                Stage::TurnList => {
                    if let Some(ld) = self.loaded.as_mut() {
                        step_cursor(&mut ld.turn_list_cursor, -1, ld.turn_summaries.len());
                    }
                }
                Stage::Breakdown => {
                    let len = self.visible_nodes().len();
                    if let Some(ld) = self.loaded.as_mut() {
                        if len > 0 {
                            ld.breakdown_cursor =
                                (ld.breakdown_cursor as i32 - 1).rem_euclid(len as i32) as usize;
                        }
                    }
                }
            },
        }
    }

    fn move_down(&mut self) {
        match self.mode() {
            Mode::Preview => {
                if let Some(Overlay::Preview(p)) = &self.overlay {
                    let n = p.body.lines().count();
                    let max_scroll =
                        n.saturating_sub(PREVIEW_MODAL_VISIBLE_BODY_LINES.min(n.max(1)));
                    self.preview_scroll = (self.preview_scroll + 1).min(max_scroll);
                }
            }
            _ => match self.stage {
                Stage::SessionList => {
                    step_cursor(&mut self.session_list_cursor, 1, self.sessions.len());
                }
                Stage::TurnList => {
                    if let Some(ld) = self.loaded.as_mut() {
                        step_cursor(&mut ld.turn_list_cursor, 1, ld.turn_summaries.len());
                    }
                }
                Stage::Breakdown => {
                    let len = self.visible_nodes().len();
                    if let Some(ld) = self.loaded.as_mut() {
                        if len > 0 {
                            ld.breakdown_cursor =
                                (ld.breakdown_cursor as i32 + 1).rem_euclid(len as i32) as usize;
                        }
                    }
                }
            },
        }
    }

    fn collapse_current(&mut self) {
        if self.overlay.is_some() || self.stage != Stage::Breakdown {
            return;
        }
        let Some(node) = self.current_node() else {
            return;
        };
        let Some(ld) = self.loaded.as_mut() else {
            return;
        };
        match node.kind {
            NodeKind::Section => {
                if let Some(flag) = ld.expanded_sections.get_mut(node.section) {
                    *flag = false;
                }
            }
            NodeKind::Row => {
                if let Some(flag) = ld.expanded_sections.get_mut(node.section) {
                    *flag = false;
                }
            }
        }
        self.clamp_breakdown_cursor();
    }

    fn move_right_or_preview(&mut self) {
        if self.overlay.is_some() || self.stage != Stage::Breakdown {
            return;
        }
        let Some(node) = self.current_node() else {
            return;
        };
        if node.kind == NodeKind::Row {
            self.open_preview();
            return;
        }
        self.expand_current();
    }

    fn expand_current(&mut self) {
        if self.overlay.is_some() || self.stage != Stage::Breakdown {
            return;
        }
        let Some(node) = self.current_node() else {
            return;
        };
        let Some(ld) = self.loaded.as_mut() else {
            return;
        };
        match node.kind {
            NodeKind::Section => {
                if let Some(flag) = ld.expanded_sections.get_mut(node.section) {
                    *flag = true;
                }
            }
            NodeKind::Row => {}
        }
    }

    fn confirm_current(&mut self) {
        match &self.overlay {
            Some(Overlay::Preview(_)) | Some(Overlay::Help) | Some(Overlay::Warning(_)) => {
                self.overlay = None;
                self.preview_scroll = 0;
            }
            None => match self.stage {
                Stage::SessionList => self.confirm_session_pick(),
                Stage::TurnList => self.enter_breakdown(),
                Stage::Breakdown => {
                    let Some(node) = self.current_node() else {
                        return;
                    };
                    let Some(ld) = self.loaded.as_mut() else {
                        return;
                    };
                    match node.kind {
                        NodeKind::Section => {
                            if let Some(flag) = ld.expanded_sections.get_mut(node.section) {
                                *flag = !*flag;
                            }
                        }
                        NodeKind::Row => {
                            self.open_preview();
                        }
                    }
                }
            },
        }
        self.clamp_breakdown_cursor();
    }

    fn confirm_session_pick(&mut self) {
        let idx = self.session_list_cursor;
        let Some(item) = self.sessions.get(idx).cloned() else {
            return;
        };
        let estimator = estimate_tokens::default_estimator();
        match analyze_workspace::run(&item.path, &estimator) {
            Ok(session) => {
                self.loaded = Some(LoadedState::new(session, item.path, idx));
                self.session_list_cursor = idx;
                self.stage = Stage::TurnList;
            }
            Err(e) => {
                self.footer_message = Some(format!("session load failed: {e}"));
            }
        }
    }

    /// Advance from **Turn List** to **Breakdown** for the cursor turn.
    /// Exposed for integration tests (`tests/integration/cli/inspect.rs`).
    pub fn enter_breakdown(&mut self) {
        let Some(ld) = self.loaded.as_mut() else {
            return;
        };
        let summary = &ld.turn_summaries[ld.turn_list_cursor];
        ld.active_turn_idx = summary.index.saturating_sub(1);
        ld.view_filter = ViewFilter::Delta;
        ld.rebuild_breakdown();
        ld.breakdown_cursor = 0;
        self.stage = Stage::Breakdown;
        self.detail_mode = false;
    }

    fn open_preview(&mut self) {
        if self.stage != Stage::Breakdown {
            return;
        }
        let Some(node) = self.current_node() else {
            return;
        };
        if node.kind != NodeKind::Row {
            self.footer_message = Some("Preview unavailable for this segment.".into());
            return;
        }
        let Some(row) = self.row_at(node) else {
            return;
        };
        let Some(ld) = self.loaded.as_ref() else {
            return;
        };
        let preview = preview_segment::for_row(row, &ld.session);
        self.preview_scroll = 0;
        self.overlay = Some(Overlay::Preview(preview));
    }

    fn clamp_breakdown_cursor(&mut self) {
        let len = self.visible_nodes().len();
        let Some(ld) = self.loaded.as_mut() else {
            return;
        };
        if len == 0 {
            ld.breakdown_cursor = 0;
        } else if ld.breakdown_cursor >= len {
            ld.breakdown_cursor = len - 1;
        }
    }

    fn row_at(
        &self,
        node: VisibleNode,
    ) -> Option<&crate::application::view_models::context_breakdown::SegmentRow> {
        let ld = self.loaded.as_ref()?;
        let section = ld.breakdown.sections.get(node.section)?;
        section.rows.get(node.row?)
    }

    fn editor_target(&self) -> Option<EditorTarget> {
        if let Some(Overlay::Preview(p)) = &self.overlay {
            return editor_target_from_preview(p);
        }
        let node = self.current_node()?;
        let row = self.row_at(node)?;
        let path = row.source_ref.file.clone();
        if path.as_os_str().is_empty() {
            return None;
        }
        Some(EditorTarget {
            path,
            line: row.source_ref.line,
        })
    }
}

fn editor_target_from_preview(p: &SegmentPreview) -> Option<EditorTarget> {
    let path_str = p.source_path.as_ref()?;
    if path_str.is_empty() {
        return None;
    }
    Some(EditorTarget {
        path: PathBuf::from(path_str),
        line: p.source_line,
    })
}

fn step_cursor(cursor: &mut usize, delta: i32, len: usize) {
    if len == 0 {
        return;
    }
    let new = (*cursor as i32 + delta).rem_euclid(len as i32) as usize;
    *cursor = new;
}

#[derive(Debug, Clone)]
pub struct EditorTarget {
    pub path: PathBuf,
    pub line: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum AppCommand {
    None,
    OpenEditor(Option<EditorTarget>),
}
