---
title: 门禁
description: gate 是一次迁移完成的定义。它运行的每一项检查，以及如何修复失败的检查。
---

# 门禁

```sh
everything-rust gate <crate>
```

完成的定义就是 `gate` 通过。它会检查：

- `cargo fmt`、带 `-D warnings` 的 clippy，以及 cargo 测试；
- `unsafe` 必须带 `// SAFETY:` 注释，并且没有 `unimplemented!`；
- 原实现每一个有意失败的地方（`throw`、`raise`、`error` 结果）都有一条录制的失败用例。当 gate 指出缺少时，用 `record --inputs` 添加一条；
- 在原实现会返回错误的函数中没有 `unwrap`、`expect` 或 `panic!`。请返回 `Err`；对于原实现也视为不可能出现的状态，加上 `// INFALLIBLE: <why>` 注释；
- 移植版的行数最多是原实现的 3 倍（另加 50 行用于导入和签名）；
- 模块能构建，并且每个导出项都存在；
- 每条录制用例都一致（包括[已知差异](/docs/concepts/diff#known-diffs)），并且录制结果没有[过期](/docs/concepts/diff#stale-recordings)；
- 对于 TS/JS，一行[覆盖率](/docs/concepts/coverage)信息：这些用例运行了原实现的多少代码。

每一项失败的检查都带有一个 `fix`：请先运行它。

## 然后汇报，并等待

门禁通过后，工作流技能会让智能体汇报改动了什么（文件、函数、桥接层、批处理）、门禁输出、diff 计数和基准测试数字、遗留的行为差异或风险，以及规划中还剩下什么，并在开始下一个单元之前等待批准。
