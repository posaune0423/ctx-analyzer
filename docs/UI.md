# ctx-analyzer UI

## 1. Overview

`ctx-analyzer` の UI は terminal-first の TUI として設計する。

想定利用は、開発中の terminal split pane の右側で `ctx-analyzer .` を実行し、現在の workspace / session / turn の context breakdown を素早く確認すること。

UI は Codex-first で実装するが、表示 model は agent-agnostic にする。

---

## 2. UI Principles

- terminal-first
- keyboard-first
- right-pane friendly
- composable components
- theme configurable
- information dense
- visually rich
- preview-first
- read-only
- partial result tolerant

UI は context の可視化に集中し、agent 固有の parsing logic は持たない。

---

## 3. Primary Command

```txt
ctx-analyzer .
```

Expected behavior:

- current workspace を検出する
- 利用可能な agent を検出する
- MVP では Codex を選択する
- 最新または関連 session を選択する
- Main Context View を開く

---

## 4. UI Composition

TUI は composable な component tree として実装する。

```txt
App
  ├─ Header
  ├─ SummaryBar
  ├─ MainPane
  │   ├─ ContextAccordion
  │   │   ├─ ContextCategorySection
  │   │   ├─ ContextSourceGroup
  │   │   └─ ContextSegmentRow
  │   └─ EmptyState
  ├─ FooterHelp
  └─ Overlay
      ├─ SessionSelector
      ├─ TurnSelector
      ├─ SessionGraphView
      ├─ PreviewModal
      ├─ HelpModal
      └─ WarningModal
```

Rules:

- 各 component は input として view model を受け取り、rendering のみを担当する。
- component 内で Codex rollout や raw artifact を parse しない。
- component は theme token を参照し、直接色を hardcode しない。
- business logic は application layer に置く。
- UI state は `selected row`, `expanded sections`, `active overlay`, `detail mode` などに限定する。

---

## 5. Screen Structure

主要 screen:

- Main Context View
- Session Selector
- Turn Selector
- Session Graph View
- Context Segment Preview
- Help View
- Warning View, later
- Compare View, later

MVP では以下を優先する。

- Main Context View
- Session Selector
- Turn Selector
- Preview Modal
- Open in Editor
- Help View

---

## 6. Main Context View

Main Context View は right-pane でも読める compact layout にする。

```txt
┌ ctx-analyzer ─────────────────────────────────────┐
│ ~/dev/project · Codex · latest · 8/12             │
│ Total ~62.6k · Config ~14.2k · Runtime ~48.4k     │
│ Session graph: 1 root · 2 child · 0 orphan         │
├───────────────────────────────────────────────────┤
│ ▾ Configuration Context                 ~14.2k    │
│   ▸ Instruction Sources                  ~3.8k     │
│   ▸ Capability Sources                   ~8.4k     │
│   ▸ Config Sources                       ~2.0k     │
│                                                   │
│ ▾ Runtime Context                       ~31.1k    │
│   ▸ User Prompts                         ~0.8k    │
│   ▸ Tool Results                        ~12.4k    │
│   ▸ File Reads                          ~10.2k    │
│   ▸ Assistant Text                       ~6.7k     │
│                                                   │
│ ▾ Delegated Context                     ~17.3k    │
│   ▸ Child Session: security-review       ~9.1k    │
│   ▸ Child Session: test-fix              ~8.2k    │
│                                                   │
│ ▸ Unknown Context                         ~?       │
├───────────────────────────────────────────────────┤
│ ↑↓ move · enter expand · p preview · o open · q   │
└───────────────────────────────────────────────────┘
```

---

## 7. Header

Header には現在の解析対象を表示する。

表示項目:

- workspace path
- selected agent
- selected session
- selected turn
- total estimated tokens
- configuration tokens
- runtime tokens
- delegated tokens
- child session count
- unknown context indicator

Display rules:

- workspace path は長い場合 middle truncate する。
- selected agent は badge 表示する。
- unknown context がある場合は `Unknown` indicator を表示する。
- parse error がある場合は warning badge を表示する。

---

## 8. Summary Bar

SummaryBar は context 全体の状態を一目で見るための領域。

表示項目:

- Total tokens
- Configuration tokens
- Runtime tokens
- Delegated tokens
- Unknown tokens
- Warning count, later
- Estimated cost, later

Example:

```txt
Total ~62.6k  Config ~14.2k  Runtime ~48.4k  Delegated ~17.3k  Unknown ?
```

Rules:

- token 数は常に estimate として `~` を付ける。
- 数値が大きい項目は token heat style を適用する。
- warning / unknown は color だけに依存せず、symbol も併用する。

---

## 9. Context Breakdown

context は category ごとに accordion 表示する。

Categories:

- System Context
- Configuration Context
- Runtime Context
- Delegated Context
- Unknown Context

各 category の中に source group を表示する。

Source groups:

- Instruction Sources
- Configuration Sources
- Capability Sources
- Runtime Sources
- Delegation Sources
- Unknown Sources

各 row には最低限以下を表示する。

- name
- estimated tokens
- source type
- loaded turn, if known
- confidence indicator, if useful

---

## 10. Token Heat Styling

token 数が多い context segment は視覚的に強調する。

目的:

- context bloat を一目で見つける
- configuration overhead を見逃さない
- runtime growth を素早く把握する

### Token Severity

```txt
low       < 1k tokens
medium    1k - 5k tokens
high      5k - 15k tokens
critical  > 15k tokens
unknown   unknown / unavailable
```

### Styling Rules

- `low`: normal text
- `medium`: accent color
- `high`: warning color + bold
- `critical`: critical color + bold
- `unknown`: dim text + `?`

Example:

```txt
Instruction Sources                  ~3.8k
Tool Results                        ~12.4k   !
Task Coordination                   ~28.0k   !!
Unknown Context                       ~?      ?
```

Rules:

- 色だけで意味を伝えない。
- `!`, `!!`, `?` などの symbol を併用する。
- selected row は token severity より selection style を優先する。
- threshold は theme / config で変更可能にする。

---

## 11. Rich Text Style

TUI では ratatui の style modifier を活用する。

使用する表現:

- bold
- dim
- italic, if supported
- underline, for source path / active link
- reversed, for selected row
- symbols
- indentation
- box borders
- badge-like labels

Examples:

```txt
[Codex]  [latest]  [Turn 8/12]
```

```txt
▾ Configuration Context        ~14.2k  !!
  ▸ Instruction Sources         ~3.8k
  ▸ Capability Sources          ~8.4k  !
```

---

## 12. Theme System

UI style は theme file で管理する。

Theme は semantic token を持ち、component は直接色名を hardcode しない。

### Theme Resolution

優先順:

1. CLI option: `--theme <name-or-path>`
2. Project theme: `.ctx-analyzer/theme.toml`
3. User theme: `~/.config/ctx-analyzer/theme.toml`
4. Built-in theme: `default`

### Theme Files

Repository に built-in theme を置く。

```txt
themes/
  default.toml
  dark.toml
  light.toml
  high-contrast.toml
```

User override:

```txt
~/.config/ctx-analyzer/theme.toml
```

Project override:

```txt
.ctx-analyzer/theme.toml
```

---

## 13. Theme Tokens

Theme file は semantic style を定義する。

Example:

```toml
[palette]
background = "default"
foreground = "gray"
muted = "dark_gray"
accent = "cyan"
success = "green"
warning = "yellow"
critical = "red"
unknown = "magenta"
border = "dark_gray"
selection_fg = "black"
selection_bg = "cyan"

[text]
normal = "foreground"
muted = "muted"
title = "foreground,bold"
subtitle = "muted"
path = "accent,underline"
badge = "accent,bold"

[token.low]
style = "foreground"

[token.medium]
style = "accent"

[token.high]
style = "warning,bold"
symbol = "!"

[token.critical]
style = "critical,bold"
symbol = "!!"

[token.unknown]
style = "unknown,dim"
symbol = "?"

[context.system]
style = "magenta,bold"

[context.configuration]
style = "cyan,bold"

[context.runtime]
style = "green,bold"

[context.delegated]
style = "yellow,bold"

[context.unknown]
style = "unknown,dim"

[row]
selected = "selection_fg,selection_bg,bold"
expanded = "foreground,bold"
collapsed = "muted"

[border]
normal = "border"
focused = "accent"
warning = "warning"
critical = "critical"
```

Rules:

- theme parser は unknown field を許容する。
- invalid theme の場合は built-in default に fallback する。
- theme は color / modifier / symbol を分離して扱う。
- monochrome fallback を用意する。

---

## 14. Component Theme Usage

各 component は semantic token を使う。

Examples:

- Header: `text.title`, `text.subtitle`, `badge`
- SummaryBar: `token.*`, `context.*`
- ContextAccordion: `context.*`, `row.*`
- SegmentRow: `token.*`, `text.path`
- PreviewModal: `border.focused`, `text.path`
- WarningModal: `warning`, `critical`

Component は `Color::Yellow` のような直接指定を避ける。

---

## 15. Session Graph View

Session Graph View は main session と child session の関係を見るための画面。

```txt
Session Graph

Root Session: codex-abc
  ├─ 4
  │   └─ Child Session: security-review
  ├─ 6
  │   └─ Child Session: test-fix
  └─ Orphan Sessions
      └─ research-helper
```

表示項目:

- root session
- child session
- forked session
- delegated session
- orphan session
- link confidence
- linked turn, if known

Styling:

- root session: bold
- child session: delegated context style
- orphan session: unknown style
- weak link: dim
- explicit link: normal or bold

---

## 16. Session Selector

Session Selector は session を切り替えるための overlay。

```txt
Select Session · ctx-analyzer · Codex

> 2026-05-01 22:31  2026-05-01 22:31  Conversation preview...
  2026-05-01 18:20  2026-05-01 18:20  Conversation preview...
  2026-04-30 23:10  2026-04-30 23:10  Conversation preview...
```

表示項目:

- Created
- Updated
- Conversation (First prompt)

---

## 17. Turn Selector

Turn Selector は interaction を切り替えるための overlay。

```txt
Select Interaction

> 2026-05-01 22:31  Conversation preview...   ~12.4k
  2026-05-01 22:32  Conversation preview...   ~18.7k
  2026-05-01 22:33  Conversation preview...   ~33.2k
```

表示項目:

- When
- Conversation
- Tokens

Token heat styling は turn total に適用する。

---

## 18. Preview Modal

Preview Modal は selected context segment の中身を見るための overlay。

```txt
Preview

Source: ./AGENTS.md
Category: Configuration Context
Type: Instruction Source
Tokens: ~1.8k
Confidence: observed

----------------------------------------------------
# Project Instructions

...
----------------------------------------------------

o open in editor · y copy path · esc close
```

表示項目:

- source path
- source type
- context category
- token estimate
- confidence
- text preview
- truncation notice, if needed

Styling:

- source path は underline
- token が high / critical の場合は heat style
- preview body は normal text
- truncation notice は warning style

---

## 19. Open in Editor

`o` で selected segment の source を editor で開く。

優先順:

- `CTX_ANALYZER_EDITOR`
- `VISUAL`
- `EDITOR`
- `cursor`
- `code`
- `zed`
- `nvim`
- `vim`

source path がない segment は editor open 不可として表示する。

---

## 20. Keybindings

Global:

- `q`: quit
- `?`: help
- `r`: refresh
- `/`: search

Navigation:

- `↑` / `k`: move up
- `↓` / `j`: move down
- `←` / `h`: go back / collapse
- `→` / `l`: expand / enter / select session
- `enter`: expand / select

Views:

- `s`: session selector
- `t`: turn selector
- `g`: session graph view
- `p`: preview selected segment
- `o`: open selected source in editor

Toggles:

- `d`: toggle detail
- `$`: toggle cost display, later
- `w`: warning view, later

---

## 21. Detail Mode

Normal mode は compact に表示する。

```txt
Instruction Sources       ~3.8k
  Project Instructions    ~1.8k
  Directory Rules         ~0.9k
```

Detail mode では selected row の詳細を表示する。

```txt
Project Instructions
  category: configuration
  source: instruction
  tokens: ~1.8k
  path: ./AGENTS.md
  confidence: observed
  loaded: 1
```

Rules:

- detail mode は selected row を中心に表示する。
- 全 row を詳細表示して noise を増やさない。
- detail mode でも right-pane width を考慮する。

---

## 22. Search

Search は context row / source path / segment text preview を対象にする。

MVP では row name と source path の検索を優先する。

Search match は accent + bold で強調する。

---

## 23. Display Rules

- token 数は常に estimate として表示する。
- unknown context は隠さず表示する。
- parse できない artifact があっても partial result を表示する。
- long text は preview で truncate する。
- binary file は preview しない。
- source mutation はしない。
- color だけに依存せず、symbols / typography も併用する。
- terminal width が狭い場合は columns を減らし、重要情報を優先する。

---

## 24. Empty / Error States

No session:

```txt
No matching sessions found for this workspace.
```

Partial parse:

```txt
Some artifacts could not be parsed.
Showing partial context analysis.
```

No preview:

```txt
Preview unavailable for this segment.
```

No source path:

```txt
This segment has no source file.
```

Invalid theme:

```txt
Theme could not be loaded.
Using built-in default theme.
```

---

## 25. MVP Scope

MVP UI で実装する。

- Main Context View
- Session Selector
- Turn Selector
- Context Breakdown
- Token Heat Styling
- Theme loading
- Preview Modal
- Open in Editor
- Help View

MVP 後に追加する。

- Session Graph View の詳細表示
- Warning View
- Compare View
- Cost display
- Live watch mode
