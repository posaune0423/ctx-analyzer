# Token Estimation Spec

## 1. Goal

PRD §13.1 #13: 「context segment ごとの token 数を概算できる」。
ただし PRD §9 「正確な billing token count の保証」は **non-goal**。

token 数は常に **estimate** であり、UI は数値の前に `~` を付ける（UI.md §8）。

## 2. Confidence levels

`src/domain/confidence/`:

| 値          | 意味                                                    |
| ----------- | ------------------------------------------------------- |
| `Observed`  | agent 自身の telemetry / event から取得した値           |
| `Estimated` | text の長さから heuristic で算出                        |
| `Unknown`   | text が見えない (encrypted reasoning 等)、estimate 不能 |

## 3. Default heuristic

`src/infra/token/CharsPer4Estimator`:

```text
tokens = ceil(text.chars().count() / 4)
```

- byte 数ではなく **char 数** を使う（multibyte 対応）。
- 空文字は 0 tokens。

これは `tiktoken` 等を使わない粗い近似。MVP の目的は「相対的な大小と category 別比率」を把握することで、絶対精度ではない。

## 4. Observed totals (Codex)

`event_msg.token_count` event を parse した結果のうち、`info.total_token_usage`
が non-null なものの **最後** を session totals として採用する:

```rust
TokenEstimate {
    tokens: total_tokens,
    input: Some(input_tokens),
    cached_input: Some(cached_input_tokens),
    output: Some(output_tokens),
    reasoning_output: Some(reasoning_output_tokens),
    confidence: Confidence::Observed,
}
```

`last_token_usage` (turn 単位の差分) は当該 turn の `last_token_usage` に格納する。

## 5. Severity (UI.md §10)

`build_context_breakdown` view-model が以下の閾値で severity を付与:

| 閾値                  | label    | symbol |
| --------------------- | -------- | ------ |
| `< 1k`                | Low      | (none) |
| `1k–5k`               | Medium   | (none) |
| `5k–15k`              | High     | `!`    |
| `> 15k`               | Critical | `!!`   |
| `Confidence::Unknown` | Unknown  | `?`    |

## 6. Future

- `tiktoken-rs` ベースの精緻 estimator（model 別 BPE 適用）を `src/infra/token/`
  に差し込み、`TokenEstimator` port を実装すれば全 segment が即座に切り替わる。
- model-aware cost 推定は `docs/specs/diagnostics.md` 側で扱う。

## 7. Tests

- `tests/unit/infra/token_heuristic.rs`:
  - 空文字 → 0
  - 10 ASCII → 3 (ceil(10/4))
  - 10 multibyte (`あ`\*10) → 3 (chars 基準)
  - 8 ASCII → 2 (exact multiple)
- `tests/integration/codex/rollout_with_subagent.rs`:
  - bundled fixture の totals が `Observed`、`total_tokens == 1_513_937`
