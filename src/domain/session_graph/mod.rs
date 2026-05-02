use serde::Serialize;

/// Parent / child / orphan relationships between sessions (PRD §10.4).
///
/// MVP scope: a single rollout file maps to one `SessionGraph`. If
/// `is_delegated_child` is true, this session was launched as a subagent
/// from a parent; the parent itself is not yet recovered (cross-file
/// linking is post-MVP — see `docs/specs/session-graph.md`).
#[derive(Debug, Clone, Serialize)]
pub struct SessionGraph {
    pub root_id: String,
    pub is_delegated_child: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subagent_label: Option<String>,
    pub children: Vec<String>,
}

impl SessionGraph {
    pub fn root(id: impl Into<String>) -> Self {
        Self {
            root_id: id.into(),
            is_delegated_child: false,
            subagent_label: None,
            children: Vec::new(),
        }
    }
}
