---
title: 程序库
description: 程序库没有可以回放的进程。它的公共 API 就是前门，每个单元的录制用例就是证明。
---

# 程序库

程序库没有可以回放的进程：它的公共 API 就是前门，每个单元的录制
用例就是证明。

```sh
everything-rust app front-door .        # rust/app：包含公共 API 的 Rust crate
# 每个单元走一遍核心闭环：bridge、record、port.rs、build、diff、gate；然后：
everything-rust app wire . textkit-rs   # 从 rust/app 重新导出；原实现导入它的影子模块
everything-rust app replay .            # 所有已接入单元的用例；回退 = 仍在原实现中的导出项
everything-rust app lift .
```

- 当仓库既没有 `main` 也没有服务端时（否则使用 `--library`），`app front-door` 会找出公共 API（每个单元的导出项），
  并把 `rust/app` 搭建为一个库 crate。每个已接入的
  单元都会成为它的一个模块：`pub use textkit_rs::port as textkit;`。
- `app wire` 接收由 `bridge` 生成的 crate，运行 `gate`，并写出该单元可直接替换的影子模块：
  原包会导入它来代替该单元，这样调用方继续使用原有 API，
  并调用到 Rust。`EVERYTHING_RUST_UNIT_<NAME>=original` 可以把该单元切回原实现（C：构建时不加
  影子模块的编译参数）。
- `app replay` 通过绑定代码运行所有已接入单元的录制用例。回退率是
  Rust 尚未接管的公共导出项所占的比例：未接入的单元，以及 `bridge` 保留在
  原语言中的导出项。
- `app lift` 要求回退率为 0 且每个单元都通过 `gate`；它会剥离各单元的绑定代码、包装层和
  影子模块，让 `rust/app` 成为一个纯 Rust 库。它不会改动任何
  `port.rs`，因此之前通过绑定代码一致的用例依然能描述它；检查清单
  会提醒原项目中仍在导入影子模块的文件。

这五个命令也都是 MCP 工具：`app_record`、`app_replay`、`app_front_door`、`app_wire` 和
`app_lift`。
