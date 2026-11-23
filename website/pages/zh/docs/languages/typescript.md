---
title: TypeScript/JavaScript
description: 通过 napi-rs 或 wasm-bindgen 把 Node.js 代码（TypeScript 或 JavaScript）迁移到 Rust，并保持原有 API。
---

# TypeScript/JavaScript

| | |
| --- | --- |
| 单元 | 文件夹或单个文件 |
| 桥接层 | napi-rs 或 wasm-bindgen，配合生成的二进制传输协议；支持类和异步 |
| 录制与回放方式 | Node，直接运行 `.ts` 源码 |
| 依赖 | Node 22.6+（`profile` 需要 Node 24+） |
| 状态 | 已验证 |

## 选择桥接层

当单元或它导入的任何内容需要 Node 时，`analyze` 选择 napi-rs；可移植的代码选择 wasm-bindgen（它也能在浏览器中运行）；UI 和编辑器代码则保留在 JS 中。`bridge --kind napi|wasm` 可以覆盖这一选择。

无法跨越桥接层的导出项会保留在 TS 中并注明原因，影子模块会重新导出原实现：接收或返回函数的函数、生成器和迭代器、RegExp、`instanceof` 检查、泛型（`--as-json T` 可以桥接一个应用只用来装数据的泛型），以及类型中包含函数的值。见[桥接层](/docs/concepts/bridge)。

## 按原样录制 TS

`record` 直接运行 TS 源码：Node 的 resolve 钩子会处理打包工具风格的导入，仓库中没有安装 npm 包时可以用 `--resolve-from` 提供，需要转换的 TS（枚举、参数属性）会用 `--experimental-transform-types` 重试。`record --from-tests -- npx jest` 会录制原实现自身测试套件发出的每一次调用，适用于任何测试运行器。TS/JS 会测量原实现的覆盖率（[覆盖率](/docs/concepts/coverage)）。

## 移植版必须做对的地方

以下问题都是在真实代码上由录制用例发现的：

- `str.length`、`indexOf` 和 `slice` 按 UTF-16 码元计数，所以一个 emoji 占两个位置。请使用生成的 `utf16` 辅助函数。
- 不带 `u` 标志的 `/.../i` 只对 ASCII 字母做大小写折叠，而 Rust 的 `regex` 会折叠 Unicode，所以在 vercel/ms 中 `"1 ſ"`（长 s）和开尔文符号会被解析成秒。
- `Math.round` 对 .5 向上取整（`-2.5` 得到 `-2`）；Rust 的 `round` 则远离零取整。生成的 `js.rs` 中有 `js::math_round`。
- `${n}` 会输出 `1e+21`、`5e-324`，并把 `-0` 输出为 `0`。`js::number_to_string` 可以复现这些行为。
- `String.prototype.trim` 使用 JS 的空白字符定义，而不是 Rust 的 `char::is_whitespace`。

## 速度

逐次调用时，移植版每次都要付出跨越桥接层的代价，而 V8 运行小函数只需纳秒级时间。在录制用例上，vercel/ms 逐次调用的速度是 0.56x；从 JS 调用时，x18n 的 core 移植版比 TS 更慢。改成批处理后，循环被迁到 Rust：

| 工作负载 | 原实现 | 使用 everything-rust |
| --- | ---: | ---: |
| x18n 侧边栏搜索，50,000 个键（Node） | 每次查询 10 ms | 0.5 ms：快 20 倍 |
| 浏览器中的同一搜索（wasm，单线程） | 10 ms | 0.95 ms：11.5x |
| Fuse.js，50,000 个字符串 | 91.2 ms | 2.8 ms：32x |
| vercel/ms `parse` x100,000 | 11.8 ms | 0.65 ms：18x（计入数据交接为 3.4x） |
| vercel/ms `format` x100,000 | 2.1 ms | 计入数据交接为 2.5 ms：0.8x |

案例研究：[json-server](/docs/real-world/json-server)、[Fuse.js](/docs/real-world/fuse-js)、[vercel/ms](/docs/real-world/vercel-ms)、[x18n](/docs/real-world/x18n)。
