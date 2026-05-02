# Session Graph Spec

## 1. Goal

Coding agent の delegated task / subagent / forked session が、別 rollout
file・別 process として記録されることがある。それらの parent ↔ child の
関係を first-class に扱える graph model を定義する（PRD §10.4）。

## 2. Domain types

`src/domain/session_graph/`:

```rust
pub struct SessionGraph {
    pub root_id: String,         // root session id
    pub is_delegated_child: bool,// この session 自体が parent から見て child か
    pub subagent_label: Option<String>, // child としての分類名 (例 "review")
    pub children: Vec<String>,   // 紐付いた child session id
    // post-MVP: 推定 link の confidence、spawn metadata 等
}
```

## 3. MVP scope

single-rollout-file 内で観測できる関係のみを表現:

- `session_meta.source.subagent` が `"review"` などにセットされていれば、その
  session は delegated child である。`is_delegated_child = true`、
  `subagent_label = Some(...)` を立てる。
- そのことを示す `Delegated/SubagentMarker` segment を 1 件 emit する。
- `children` は MVP では常に空。

## 4. Post-MVP scope

cross-file linking。同一 `~/.codex/sessions/` 配下にある複数 rollout file を
横断して parent ↔ child を復元する:

| 推定 source                                                 | confidence       |
| ----------------------------------------------------------- | ---------------- |
| 明示 link (parent_session_id 等が rollout に書かれた場合)   | Observed         |
| `cwd` + `originator` + 開始時刻が parent の特定 turn と一致 | Estimated        |
| label のみ一致 (link 不明)                                  | Unknown / orphan |

orphan session は `SessionGraph` のどれにも繋がらない子として保持し、PRD §18.3
の Mitigation に従い「link 不明」と表示する。

### 4.1 アルゴリズム（claude-devtools `SubagentResolver` 参照）

`references/claude-devtools/src/main/services/discovery/SubagentResolver.ts`
の方式を Codex / Claude Code に展開する:

1. **Discovery**: 親 session と同じ workspace の subagent file 群を列挙する
   （Codex: 同一 cwd / originator の rollout file 群、Claude Code:
   `<sessionId>/subagents/agent-<id>.jsonl`）。
2. **Linking**: 親側の tool-call event の id を使って subagent に紐付ける
   （Codex は `function_call.call_id` ↔ child の `parent_call_id` 等。
   Claude Code は `sourceToolUseID` field で 1 対 1 結合）。
3. **Parallelism detection**: 各 subagent の最初/最後の event の timestamp
   から `[start, end]` を算出。`PARALLEL_WINDOW_MS = 100ms` を超えて
   重なるものは parallel フラグを立てる。

このアルゴリズムは将来 `src/application/usecases/reconstruct_session_graph/`
に実装する。current MVP 実装は no-op linker のまま。

## 5. UI implications

- `Session Selector` (UI.md §16): child count を列に表示
- `Session Graph View` (UI.md §15): root → child のツリー表示。MVP では実装しない。
- `Main Context View` の `Delegated Context` accordion: child marker を 1 件表示
  （MVP の現状）。

## 6. Tests

- `tests/integration/session_graph/delegated_session.rs`:
  bundled subagent rollout を読み、`is_delegated_child == true` & `subagent_label == Some("review")` を assert。
- `tests/unit/domain/session_graph.rs`:
  `SessionGraph::root("abc")` が `is_delegated_child == false`、`children` 空であることを assert。
