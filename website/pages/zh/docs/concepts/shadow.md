---
title: 影子运行
description: shadow 生成一个可直接替换的模块，让原实现和 Rust 并行运行，并报告不一致。
---

# 影子运行

```sh
everything-rust shadow <crate>
```

`shadow` 生成一个保持原有 API、可直接替换的模块：TS 或 Python 模块、Go 包、Ruby 文件、Java 类或 C 文件。在应用原本加载原实现的地方加载它。它会同时运行两者，返回原实现的结果，并在你于生产环境中建立信心的过程中报告 Rust 的不一致。

| `EVERYTHING_RUST_MODE` | 行为 |
| --- | --- |
| `shadow`（默认） | 同时运行两者，返回原实现的结果，报告不一致 |
| `new` | 切换到 Rust |
| `old` | 关闭 Rust |

公共 API 不会改变。调用方导入的是相同的函数，或者用影子模块代替它们。
