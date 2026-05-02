# Diagnostics Spec (Post-MVP)

## 1. Status

PRD §15 / §16 に挙がっている diagnostics と optimization suggestions は MVP で
は **out of scope**。MVP の現状は:

- `Severity` 表示 (`Low/Medium/High/Critical/Unknown`) を `inspect` UI で表示
- `ParseWarning` を `warnings` array として `export` に含める

## 2. Planned diagnostics

将来 `application/usecases/diagnose/` に追加する想定:

| 診断項目                                           | 閾値の例 / 起源 segment                                 |
| -------------------------------------------------- | ------------------------------------------------------- |
| configuration context が大きすぎる                | total_tokens(Configuration) > 30k                       |
| instruction source が大きすぎる                   | 単一 instruction segment > 10k                          |
| capability definitions が大きすぎる                | total_tokens(Capability) > 20k                          |
| MCP definitions が大きすぎる                       | (Claude Code adapter 後)                                |
| skill が load されたが使用されていない             | Capability/Skill segment & 同 turn 内に対応する FunctionCall 無 |
| MCP が load されたが tool call 無                  | (post-MVP)                                              |
| rules / instructions が重複している                | 同一 hash の Configuration segment が複数               |
| delegated context / child session の context 大   | Delegated segment total > 20k                           |
| child session 結果 summary が小さすぎる            | child summary tokens < 200                              |
| unknown context ratio が高すぎる                   | Unknown / Total > 0.2                                   |
| compact 後に重要 context が消えた可能性             | (compact event の前後 diff、Codex は compact event を吐く) |

## 3. Output shape

`export_json` の `warnings` array に下記 shape で追加する想定:

```jsonc
{
  "level": "warning",
  "code": "configuration_too_large",
  "message": "Configuration context is 32,140 tokens; consider moving project instructions into directory-scoped instructions.",
  "related_segment_ids": [3, 4, 5]
}
```

`level ∈ {info, warning, critical}`。

## 4. Optimization suggestions

PRD §16 の suggestion は diagnostics の延長として、別 array `suggestions` を
schema に追加する。これも post-MVP。

## 5. Tests (planned)

- `tests/unit/application/diagnose.rs`: 各 diagnostic rule の閾値判定
- `tests/integration/codex/diagnostics.rs`: bundled fixture が想定 warnings を生成することを assert（post-MVP）
