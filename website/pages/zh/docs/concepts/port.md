---
title: 移植
description: 编写 port.rs（你或你的智能体唯一需要编写的文件），以及让它与原实现保持一致的规则。
---

# 移植

你或你的智能体在 `src/port.rs` 中编写移植代码。crate 中的其他一切都是生成的。修改 `port.rs` 中的签名之后，请运行 `everything-rust refresh <crate>`。

## 能省掉一轮 diff 的规则

- 编写代码之前，先到 `file:line` 处阅读规划为该单元列出的每一项风险。
- **字符串偏移**：JS 和 Java 按 UTF-16 码元索引，Rust 按 UTF-8 字节索引。请使用生成的 `src/utf16.rs`。
- **错误**：返回 `Err("TypeName: message")`。在 Python、Ruby 和 Java 中它会变成对应的异常，在 Go 中会变成 `error` 文本。Rust 的 panic 永远无法匹配录制的错误。
- **Python 和 Ruby 中的数字**：原实现原样传递的整数，返回时也必须是整数。在 Python 移植版中，需要“整数或浮点数”时请使用 `crate::pynum::Num`，而不是 `serde_json::Value`。
- **C**：录制为 `crash:` 的输入属于未定义行为；任何结果都算通过。
- **有序 Map 和对象键**：当顺序会影响输出时，使用 `indexmap`。
- **正则环视或反向引用**：使用 `fancy-regex`，而不是 `regex`。无论哪种，`\w`、`\b`、`\d` 和忽略大小写匹配处理非 ASCII 文本的方式都与源语言的引擎不同。当原实现的 `\w` 只匹配 ASCII 时（不带 `u` 的 JS、Java），请设置 `unicode(false)` 或写出 ASCII 字符类（`[A-Za-z0-9_]`）。
- **不要用 `unsafe`。** 如果确实需要：尽量少、单独隔离，并加上 `// SAFETY:` 注释。
- **辅助函数保持私有。** 只有单元的导出项才是 `pub`；`pub` 的辅助函数会成为桥接 API 的一部分。
- 对你推理过的用例，在 `port.rs` 中添加 Rust 单元测试。

生成的 `js.rs` 中包含在 Rust 行为不同的地方复现 JavaScript 行为的辅助函数，例如 `js::number_to_string`（JS 的 `String(n)`）和 `js::math_round`（`Math.round`）。

## 示例：移动了一个位置的 emoji

来自 [x18n](https://github.com/x18n-dev/x18n) 的 `packages/core/src/fuzzy.ts`，在
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo) 中用两种方式移植：

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts：就绪，wasm-bindgen（core 在浏览器中运行）
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# 编写 src/port.rs
everything-rust gate demo/fuzzy-rs
```

第一版移植按 Rust `char` 做索引，通过了简单测试，却没能通过 `diff`：

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

JS 的 `indexOf` 按 UTF-16 码元计数，所以一个 emoji 占两个位置。
`demo/*/src/port.rs` 中的移植版使用生成的 `utf16` 辅助函数，通过了 2000/2000 条用例和门禁，
包括 napi-rs 版（`demo/fuzzy-napi`）和 wasm-bindgen 版（`demo/fuzzy-rs`）。

everything-rust 为整个 x18n 编写的规划：[`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md)。
## 应该写多大？

`gate` 会比较移植版与原实现的行数，超过 3 倍（另加 50 行用于导入和签名）就会失败。超出通常意味着在手写某个 crate 已经提供的功能，或者在逐个处理各种情况。`analyze` 会列出应该使用的 crate。
