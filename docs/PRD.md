# ctx-analyzer PRD

## 1. Overview

`ctx-analyzer` は、coding agent が session / turn ごとにどのような context を読み込んでいるかを可視化・分析する local-first な context analyzer です。
最初の実装対象は Codex です。
ただし、product と core model は Codex 専用にはせず、将来的に Claude Code、Cursor、Gemini CLI、OpenCode など複数の coding agent を同じ interface で扱える agent-agnostic architecture を前提にします。
`ctx-analyzer` の目的は、provider に送信された hidden prompt を完全復元することではありません。
目的は、local に観測可能な config、instruction sources、capability sources、session artifacts、tool events、file reads などから context construction を再構成し、developer が agent config / skill / MCP / subagent / rules 設計を改善できるようにす
ることです。

---

## 2. Product Statement

`ctx-analyzer` は、coding agent の context construction を session / turn / session graph 単位で可視化し、token 消費量・context load の差異・configuration overhead を分析するための developer tool です。
plugin / connector / アプリ連携 / MCP / skill など接続先が増えたときに、同一内容が複数経路で context に載り **重複ロード** や **見えない肥大化** が起き、ウィンドウやプロダクト制限に早期到達しやすい問題を、観測・帰属・削減の材料として扱う。

---

## 3. Core Positioning

`ctx-analyzer` は単なる token counter ではありません。
以下を統合した context observability tool です。

- coding-agent context inspector
- session / turn analyzer
- instruction source analyzer
- skill / MCP / plugin load viewer
- delegated context / subagent context viewer
- context cost profiler
- agent configuration diagnostics tool
  短く言うと、`ctx-analyzer` は coding agents のための context observability tool です。

---

## 4. Problem

プラグイン、コネクタ、アプリ連携、MCP、スキル、rules など、coding agent に接続する「表面」が増えるほど、同じ説明・ツール定義・メタデータが **複数経路から同時に** context に載る可能性が高まる。
開発者からは経路が見えにくいため、**重複ロード** と **段階的な肥大化** に気づきにくく、すぐに context / token の上限に達して本筋のプロンプトや実行結果を載せる余白が失われる。
`ctx-analyzer` が解くべき中心課題の一つは、この **接続の積み重ねによる context 圧迫** を、session / turn 単位で **何が・どこから・どれだけ** 載っているかに分解して把握できるようにすることである。

coding agent は user prompt 以外にも、多くの implicit context を読み込みます。
Examples:

- system / developer instructions
- global instructions
- project instructions
- directory-scoped instructions
- local override instructions
- rules
- memory files
- config files
- profiles
- CLI overrides
- skills
- plugins
- apps
- MCP server definitions
- MCP tool definitions
- custom subagents
- delegated task sessions
- user prompts
- assistant messages
- tool calls
- tool results
- file reads
- command outputs
- compact summaries
- hidden / unknown context
  しかし developer は通常、以下を把握しづらいです。
- どの instruction source が読み込まれているか
- どの config layer が active になっているか
- global / project / directory-scoped instructions がどう合成されているか
- skill が metadata だけ load されているのか、full instruction まで読まれているのか
- MCP definitions が context をどれだけ圧迫しているか
- load された MCP が実際に使われているか
- delegated task / subagent がどの context を持って動いたか
- delegated task / subagent が同一 session 内にあるのか、別 session として記録されているのか
- main session と child session の繋がりがどうなっているか
- main session から見た delegated task がどの tool call / task delegation に対応するか
- turn ごとに context がどう増減したか
- compact 後に何が残ったか
- agent ごとに context management がどう違うか
- token 消費の主因が configuration なのか runtime なのか
- 不要・過剰・重複した context source がどこにあるか
  その結果、coding agent の設定改善が勘に依存している。

---

## 5. Target Users

### 5.1 Primary Users

- Codex、Claude Code、Cursor などの coding agent を日常的に使う developer
- agent-specific instruction files や rules を運用している developer
- MCP server を複数設定している developer
- skills / plugins / apps を使っている developer
- custom subagents や delegated task workflow を使う、または設計したい developer
- agent ごとの token 消費量や context management の違いを比較したい developer
- agent config を軽量化・最適化したい developer

### 5.2 Secondary Users

- 大きな monorepo で coding agent を運用する engineering team
- agent config を team 標準化したい organization
- reusable skills / plugins / subagents を設計する platform engineer
- AI coding workflow を整備する developer tooling team

---

## 6. Goals

`ctx-analyzer` の goal は、developer が以下を把握できるようにすることです。

1. 何が context に読み込まれているか
2. それぞれ何 token 程度消費しているか
3. どの source file / config / skill / MCP / delegated task に由来するか
4. session / turn ごとに context がどう変化したか
5. main session と child session / delegated session がどう繋がっているか
6. main session から見た delegated task がどの context を生成したか
7. configuration context と runtime context の比率
8. agent ごとの context management の違い
9. 不要・過剰・重複した context source
10. agent config / skill / MCP / delegated task / rules 設計の改善余地

---

## 7. Product Principles

### 7.1 Agent-agnostic core

最初の adapter は Codex だが、core model は Codex 専用にしない。
各 agent 固有の context source は normalized schema に変換する。

Examples:

- Codex の instruction file
- Claude Code の instruction / memory file
- Gemini CLI の instruction file
- Cursor の rules
- agent-specific skill definitions
- agent-specific MCP definitions
- agent-specific session artifacts
  これらはすべて、agent adapter を通じて共通の context model に変換する。

### 7.2 Observable reconstruction

完全な prompt reconstruction は目指さない。
local artifact から確認できるもの、推定できるもの、確認できないものを区別して扱う。

### 7.3 Configuration and runtime separation

agent 設定由来の context と、作業中に生成される context を分けて扱う。

### 7.4 Session graph as first-class model

delegated task や subagent は、同一 session 内の child thread として見える場合も、別 session として見える場合もある。
そのため、`ctx-analyzer` は単一 session の中だけで完結する model ではなく、session 同士の parent-child relationship を表現できる session graph を扱う。

### 7.5 Comparison as a core use case

将来的に、agent 間・session 間・turn 間で context load を比較できるようにする。

---

## 8. Scope

### 8.1 MVP Scope

MVP では Codex adapter のみを実装する。
ただし、product concept と data model は agent-agnostic にする。
MVP で扱う対象:

- Codex configuration sources
- Codex instruction sources
- Codex capability sources
- Codex session artifacts
- turn-level runtime context
- delegated context / child session relationship
- token estimate
- source reference
- context reconstruction confidence
- JSON export

### 8.2 Future Scope

MVP 後に以下を追加する。

- Claude Code adapter
- Cursor adapter
- Gemini CLI adapter
- agent comparison
- session comparison
- turn diff
- model cost estimation
- warnings / diagnostics
- optimization suggestions
- static report export
- live watch mode

---

## 9. Non-Goals

MVP では以下を対象外とする。

- exact provider prompt reconstruction
- hidden system prompt の復元
- 正確な billing token count の保証
- config の自動書き換え
- instruction source の自動 rewrite
- web dashboard
- cloud sync
- team dashboard
- realtime monitoring
- semantic quality scoring
- model output evaluation
- non-Codex adapter

---

## 10. Core Concepts

### 10.1 Workspace

解析対象となる repository / project directory。

### 10.2 Agent

対象 coding agent。

MVP:

- Codex

Future:

- Claude Code
- Cursor
- Gemini CLI
- OpenCode

### 10.3 Session

coding agent の作業 session。

### 10.4 Session Graph

session 同士の関係を表す graph。
delegated task や subagent が別 session として記録される場合、main session と child session の関係を表現する。

Example:

```
    Main Session
      ├─ Turn 4
      │   └─ Child Session: security-review
      ├─ Turn 5
      │   └─ Child Session: test-fix
      └─ Turn 8
          └─ Child Session: refactor-plan
```

### 10.5 Session Link

session 間の関係。

Examples:

- parent session
- child session
- spawned by turn
- spawned by tool call
- spawned by delegated task
- summary returned to parent
- result merged into main session

### 10.6 Turn

user interaction 単位。

### 10.7 Context Source

context に読み込まれる情報源。

Examples:

- instruction source
- configuration source
- capability source
- runtime source
- delegation source
- unknown source

### 10.8 Context Segment

context source から抽出された、token 計測・preview・source tracking の単位。

Examples:

- one instruction file
- one rule block
- one skill definition
- one MCP tool definition
- one user prompt
- one tool result
- one file read
- one child session summary
- one compact summary

---

## 11. Context Source Categories

### 11.1 Instruction Sources

agent の振る舞い、制約、project convention、task policy を定義する text source。

Examples:

- global instructions
- user instructions
- project instructions
- repository instructions
- directory-scoped instructions
- local overrides
- rules
- memory files
- policy files
- task-specific guidance

Agent-specific file names are treated as implementation details of each adapter.

Examples:

- Codex-specific instruction files
- Claude Code-specific instruction / memory files
- Gemini-specific instruction files
- Cursor rules

### 11.2 Configuration Sources

agent の runtime behavior や tool availability を決める structured config。

Examples:

- user config
- project config
- profiles
- CLI overrides
- sandbox settings
- approval settings
- model settings
- reasoning settings

### 11.3 Capability Sources

agent が利用可能な external capability を定義する source。

Examples:

- skills
- plugins
- apps
- MCP servers
- MCP tool definitions
- slash commands
- hooks

### 11.4 Runtime Sources

session 中に生成・取得される context。

Examples:

- user prompts
- assistant messages
- tool calls
- tool results
- file reads
- file edits
- command outputs
- search results
- compact summaries

### 11.5 Delegation Sources

main session から呼び出される delegated execution に関する context。

Examples:

- subagents
- child sessions
- delegated task sessions
- subagent instructions
- child session summaries
- task delegation results

### 11.6 Unknown Sources

local artifact から確認できないが、context に含まれる可能性がある source。

Examples:

- hidden provider prompt
- redacted telemetry
- unavailable session state
- internal summaries

---

## 12. Context Categories

### 12.1 System Context

agent / runtime / provider に由来する context。

### 12.2 Configuration Context

agent 設定に由来する context。

Examples:

- configuration sources
- instruction sources
- capability definitions
- subagent definitions

### 12.3 Runtime Context

session 中に生成・取得される context。

Examples:

- user prompt
- assistant messages
- tool calls
- tool results
- file reads
- file edits
- command outputs
- search results
- compact summaries

### 12.4 Delegated Context

main session から見ると tool call や task delegation のように見えるが、実体としては child session / delegated session に分離される context。

Examples:

- subagent session
- delegated review session
- delegated search session
- child task execution
- child session summary returned to parent

### 12.5 Unknown Context

観測できないが存在する可能性がある context。

Examples:

- hidden provider prompt
- redacted telemetry
- unavailable session state
- internal summary not exposed

---

## 13. MVP Requirements

### 13.1 Functional Requirements

MVP では以下を満たす。

1. current workspace を解析できる
2. Codex configuration sources を検出できる
3. Codex instruction sources を再構成できる
4. Codex capability sources を検出できる
5. Codex session を検出できる
6. session 一覧を取得できる
7. session graph を表現できる
8. main session と child session の関係を表現できる
9. turn 一覧を取得できる
10. turn ごとの context breakdown を表示できる
11. configuration context と runtime context を分けて集計できる
12. delegated context / child session context を main session から辿れる
13. context segment ごとの token 数を概算できる
14. context segment の source file を参照できる
15. context segment の内容を preview できる
16. source file を editor で開ける
17. context reconstruction の信頼度を表示できる
18. agent-agnostic JSON として export できる

### 13.2 Non-functional Requirements

- local-first
- no cloud upload
- no telemetry by default
- fast enough for interactive use
- large file protection
- binary file protection
- parser failure 時も partial result を表示する
- unknown context を明示する
- future adapters を追加しやすい
- agent-specific logic が adapter に閉じている

---

## 14. Comparison Requirements

将来的に、agent 間・session 間・turn 間の比較を可能にする。

比較対象:

- total tokens
- visible tokens
- unknown tokens
- system tokens
- configuration tokens
- runtime tokens
- delegated context tokens
- instruction tokens
- capability tokens
- MCP tokens
- tool result tokens
- file content tokens
- child session tokens
- compact summary tokens
- estimated cost
- warning count

主な比較ユースケース:

- Codex と Claude Code の context load 差分を見る
- Codex と Cursor の rules / instructions の扱いを比較する
- 同じ repository で agent ごとの token 消費量を比較する
- config 変更前後で session の token 消費を比較する
- MCP / skills を disable した前後で比較する
- delegated task / child session を使った場合と使わない場合で token 消費を比較する

---

## 15. Diagnostics Requirements

Diagnostics は MVP 後に追加する。
Initial diagnostics:

- configuration context が大きすぎる
- instruction source が大きすぎる
- capability definitions が大きすぎる
- MCP definitions が大きすぎる
- skill が load されたが使用されていない
- MCP が load されたが tool call がない
- rules / instructions が重複している
- delegated context / child session の context が大きすぎる
- child session の結果 summary が小さすぎる
- unknown context ratio が高すぎる
- compact 後に重要 context が消えた可能性がある

---

## 16. Optimization Suggestions

Optimization suggestions は MVP 後に追加する。
Examples:

- global instruction を削る
- project-specific instruction に移す
- directory-scoped instructions に分割する
- task-specific instruction を skill に移す
- unused MCP を disable する
- MCP を project scope に限定する
- skill trigger を狭める
- child session / delegated task の instruction を短くする
- delegated task の粒度を調整する
- duplicated rules を統合する
- agent-specific config profile を作る

---

## 17. Success Criteria

MVP の成功条件:

- Codex workspace で `ctx-analyzer` が動作する
- Codex configuration sources を検出できる
- Codex instruction sources を再構成できる
- Codex capability sources を検出できる
- session / turn を扱える
- session graph を表現できる
- main session と child session の関係を表現できる
- turn ごとの context breakdown を表示できる
- delegated context / child session context を辿れる
- token estimate を表示できる
- configuration / runtime context を分けて集計できる
- source file preview ができる
- source file を editor で開ける
- context reconstruction の信頼度が表示される
- JSON export ができる
- unknown context が明示される
- Codex 固有実装が adapter に閉じている

---

## 18. Risks

### 18.1 Agent internals may change

coding agent の session artifact / config format / context construction は変わる可能性がある。
Mitigation:

- agent adapter に閉じ込める
- parser を version-aware にする
- unknown context を許容する
- exact reconstruction を promise しない

### 18.2 Exact context may not be observable

hidden provider prompt や internal summaries は見えない可能性がある。
Mitigation:

- reconstruction confidence を表示する
- unknown context を first-class に扱う
- observable reconstruction として position する

### 18.3 Child session relationship may be incomplete

delegated task / subagent が別 session として記録される場合、parent session との link が明示されていない可能性がある。
Mitigation:

- explicit link がある場合はそれを使う
- tool call / timestamp / workspace / task name から推定 link を作る
- 推定 link は信頼度を下げる
- link 不明の child session は orphan session として扱う

### 18.4 Agent comparison may be imperfect

agent ごとに artifact の粒度が異なるため、完全に公平な比較は難しい。
Mitigation:

- normalized schema を使う
- unknown ratio を比較指標に含める
- comparison は exact measurement ではなく diagnostic として扱うÏ
