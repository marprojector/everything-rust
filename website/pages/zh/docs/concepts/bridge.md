---
title: 桥接层
description: bridge 为一个单元搭建 Rust crate，生成绑定代码和包装层，让调用方继续使用原有 API。
---

# 桥接层

```sh
everything-rust bridge <repo> <unit-id>          # crate 位于 <repo>/rust/<name>；从不覆盖
everything-rust refresh <crate>                  # 修改 port.rs 中的签名之后运行
```

`bridge` 会创建一个 crate，其中包含绑定代码（`lib.rs`，以及源语言加载的包装层）和待移植的桩函数（`port.rs`）。你只需要在 `src/port.rs` 中编写移植代码；`src/lib.rs` 和各种包装层都是生成的，`refresh` 会根据 `port.rs` 中的签名重新生成它们，并加入新的批处理类。它从不改动 `port.rs`。

选项：

- `--batch fn:param` 会添加一个把大量 `param` 值留在 Rust 中的类。见[批处理](/docs/concepts/batches)。
- `--as-json T` 把应用只用来装数据的类型参数（`paginate<T>(items: T[])` 中的 `T`）作为 JSON 值桥接。
- `--keep "name: reason"` 把更多导出项保留在源语言中，例如依赖 `Intl` 数据的代码。
- `--kind` 覆盖规划所选的桥接层（TypeScript 可选 napi-rs 或 wasm）。

## 各语言的桥接层

| 语言 | 桥接层 |
| --- | --- |
| TypeScript/JavaScript | napi-rs 或 wasm-bindgen，配合生成的二进制传输协议；支持类和异步 |
| Python | PyO3（abi3），直接类型转换，类型化异常 |
| C | 与原函数同名的 `extern "C"` 函数及头文件 |
| Go | 类型化 cgo 调用与生成的二进制传输协议，放在保持原 API 的包中 |
| Ruby | 挂载在原模块下的 magnus 扩展 |
| Java | 类型化 JNI 与生成的二进制传输协议，放在保持原方法的包装类中 |

PHP 和 C# 的单元暂时还不能桥接；见 [PHP](/docs/languages/php) 和 [C#](/docs/languages/csharp)。

## bridge 如何做决定

- 无法跨越桥接层的导出项会保留在源语言中并注明原因，影子
  模块会重新导出原实现。对于 TS：
  接收或返回函数的函数、生成器和迭代器、RegExp、`instanceof` 检查、
  泛型（`--as-json T` 可以桥接一个应用只用来装数据的泛型），以及类型中（沿导入、重新导出和 `extends` 追踪）包含
  函数的值。`--keep "name: reason"` 可以保留更多内容，例如依赖 `Intl` 数据的代码。
- TS 类型会映射为 Rust 类型，类型别名会沿导入解析，参数类型
  会根据默认值推断；结构化的值通过生成的二进制传输协议跨边界（只有传输协议无法描述的结构
  才回退到 JSON）。
- JSON 无法表达 `undefined`、NaN、Infinity、-0 或孤立代理项：结构化的值会携带
  标记（`{"$undefined": true}`、`{"$num": "NaN"}`、`{"$utf16": [...]}`），由生成的
  `index.cjs` 转换，辅助函数位于 crate 的 `js.rs` 中。

`record` 按原样运行 TS 源码：Node 的 resolve 钩子会处理打包工具风格的导入
（`./errors` 解析为 `./errors.ts`），仓库中没有安装 npm 包时可以用 `--resolve-from`
提供，需要转换的 TS（枚举、参数属性）会用
`--experimental-transform-types` 重试。
