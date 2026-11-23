---
title: Diff 与已知差异
description: diff 在 Rust 实现上回放每一条录制用例。known-diffs.jsonl 记录有意为之的差异，过期的录制结果会让门禁失败。
---

# Diff 与已知差异

```sh
everything-rust build <crate>
everything-rust diff <crate>                     # 必须是 N/N
everything-rust diff <crate> --json
```

`diff` 以原实现调用方的调用方式，在构建好的 Rust 模块上回放 `cases.jsonl`，并列出不一致之处。不一致就是行为差异：要修的是移植版，而不是录制结果。

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

## 已知差异

有时移植版本来就应该不同：用户希望它修复原实现的某个 bug。请先询问，然后在 `<crate>/known-diffs.jsonl` 中为每条用例添加一行：

**known-diffs.jsonl**
```json
{"fn": "...", "args": [...], "actual": ..., "reason": "..."}
```

`fn`、`args` 和 `actual` 从 `diff --json` 中复制。之后这条用例只有在结果完全等于该值时才算通过；一旦它不再有差异，`gate` 就会失败，这时请删除这一行。

## 过期的录制结果

`record` 会在 `recorded-from.json` 中保存原单元文件的指纹。如果原实现之后发生变化，`diff` 会发出警告，`gate` 会失败，直到你重新录制，而不是对照原实现过去的行为通过。如果 `gate` 提示录制结果已过期，请先重新录制，再继续迁移。

## 迁移停滞时

`diff` 会跨构建跟踪进度。当它提示迁移已停滞（它的 `trend.stalled`：连续三次修改 `port.rs` 都没有新增一致的用例）时，请停止修改。剩下的不一致可能需要只有用户才知道的规则，或者这些函数暂时应该留在原实现中。

## 浮点容差

如果 crate 是用 `--float-tolerance` 录制的，`diff` 会使用该容差；在 `diff` 上加 `--float-tolerance` 可以为单次运行覆盖它。见[浮点容差](/docs/concepts/float-tolerance)。
