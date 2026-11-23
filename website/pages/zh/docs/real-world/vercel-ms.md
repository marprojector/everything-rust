---
title: vercel/ms
description: 端到端迁移的 vercel/ms。在每条录制用例上都正确，逐次调用更慢，只有在程序一次解析大量时长时才更快。
---

# vercel/ms

[vercel/ms](https://github.com/vercel/ms)（5.5k stars，commit `4ff48ce`）把 `"2 days"` 转换为
毫秒，也能反向转换；大多数 Node 项目都会通过某个依赖引入它。使用
everything-rust 1.5.1 端到端迁移；crate 位于
[`demo/real-world/vercel-ms-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/vercel-ms-rs)。

| 步骤 | 结果 |
| --- | --- |
| `analyze` | 1 个单元（`src/index.ts`），就绪，就绪度 96 |
| `bridge --kind napi` | 已桥接 `ms`、`parse`、`parseStrict` 和 `format`，没有保留在 TS 中的导出项 |
| `record --from-tests -- jest` | 975 条用例：生成的用例、语义陷阱，以及从 vercel/ms 的 167 个 Jest 测试中捕获的 340 次调用（第一版移植使用的是从中手工复制的 149 个输入） |
| 移植 | 200 行 Rust 对应 184 行 TS，其中包括一个每次批处理只读取一次 `options` 的 `FormatIndex` |
| `diff` | 首次构建 784/784 一致；加入捕获的调用后 975/975 |
| `gate` | 通过（fmt、clippy、测试、错误路径、大小、行为） |
| `bench`，逐次调用 | 在录制用例上为 0.56x（最初为 0.39x）：对普通字符串执行 `parse` 耗时 79 ns，与 JS 持平，但边界用例和抛出的错误经过桥接层后开销仍然更大 |
| 批处理，`parse` x100,000 | JS 11.8 ms，Rust 0.65 ms：**18x**；计入字符串的一次性交接为 3.4x（字符串拼接后跨边界之前为 2.1x） |
| 批处理，`format` x100,000 | JS 2.1 ms，计入交接的 Rust 2.5 ms：**0.8x**（最初为 0.2x）；`format(x, { long: true })` 为 1.2x |

移植版必须做对的地方，全部由录制用例或语义陷阱发现：

- 不带 `u` 标志的 `/.../i` 只对 ASCII 字母做大小写折叠，而 Rust 的 `regex` 会折叠 Unicode，所以
  `"1 ſ"`（长 s）和开尔文符号会被解析成秒。移植版手动匹配单位，
  只匹配 ASCII。
- `Math.round` 对 .5 向上取整（`-2.5` 得到 `-2`）；Rust 的 `round` 则远离零取整。
- `${n}` 会输出 `1e+21`、`5e-324`，并把 `-0` 输出为 `0`；移植版复现了 JS 的数字转字符串。
  它的第一版还有一个舍入平局的 bug（`25.728225708007812` 被输出为 `...813`），而 ms 的
  整数输出永远不会触及这种情况。
- `str.length` 按 UTF-16 码元计数，所以在 100 个字符的上限中，一个 emoji 算两个。

在工具中发现并已修复的问题：带结构化参数（`format` 的
`options`）的 napi 批处理方法无法编译，因为生成的绑定代码接收的是 `serde_json::Value`，而 napi
在没有启用其 `serde-json` 特性时无法转换它。现在带批处理类的 crate 会启用这一特性。

从这次迁移中带回工具的改进：

- 生成的 `js.rs` 中有 `js::number_to_string` 和 `js::math_round`，已在
  20,000 个随机 double 上与 Node 对照检查；移植版使用它们，而不是自己的副本。`number_to_string` 从
  serde_json 的打印器获取数字：对整数比 `format!` 快 5 倍，对小数快 3 倍。
- napi 批处理把字符串数据项作为一个拼接后的字符串加上 UTF-16 结束偏移传入，把数字数据项作为
  一个 `Float64Array` 传入，并把字符串结果拼接后返回（`index.cjs` 用 `slice` 拆分）：
  与每个数据项一个 JS 值相比，字符串快 2 倍，数字快 20 倍。`new Batch(items)` 和
  `run()` 保持原有形式。
- `analyze` 会标出 `Math.round`、`toFixed`，以及提到 `s` 或 `k` 且不带 `u` 的 `/i` 正则，
  并为每一项给出指导。
- 调用开销更低：在 napi 上，每个同步函数都通过二进制传输协议调用，不使用 napi 参数
  （一个参数约 55 ns，而整个调用只需 22 ns），每个函数一个 JS 包装而不是
  两个，`serde_json::Value` 参数以任意结构的带标签值跨边界，错误则通过
  传输缓冲区返回。`parse` 逐次调用从 121 ns 降到 79 ns。
- `record --from-tests` 取代了手工复制测试输入：它在测试运行时录制测试套件发出的每一次调用，
  捕获代码不计入覆盖率（ms 要求 100% 覆盖率）。

结论：`ms` 是一个好的移植，却是一个糟糕的提速目标。它在每条录制用例上都正确，
只有在程序一次解析大量时长时才更快。
