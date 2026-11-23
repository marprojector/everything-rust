---
title: 环境诊断
description: everything-rust doctor 检查语言运行时、智能体配置和 MCP 服务器，并给出修复每个问题的命令。
---

# 环境诊断

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor` 会检查语言运行时、智能体配置和 MCP 服务器，并打印修复每个问题的命令。当缺少必需的组件时，它以状态码 1 退出。

当某一步因为运行时、智能体配置或 MCP 服务器缺失或过旧而失败时，请运行它。工作流技能也会让智能体这样做。

## 每种语言需要什么

`record`、`diff` 和 `bench` 会在原实现自己的运行时中运行它：TS 需要 Node 22.6+（它直接运行 `.ts` 源码），Python 和 C（ctypes）需要 Python 3.9+，Go 需要 1.21+，Ruby 需要 3+，Java 需要 JDK 17+（`javac` 和 `java`）。在 Node 上使用 `profile` 需要 Node 24+，它允许在 `NODE_OPTIONS` 中使用 `--cpu-prof`。C 还需要 C 编译器。wasm 桥接层还需要 `rustup target add wasm32-unknown-unknown` 和对应版本的 `wasm-bindgen-cli`；`build` 会给出确切的版本号。

## 面向贡献者

`cargo run -- doctor` 会列出你已安装的运行时、它们的版本，以及安装或更新每一项的命令。见[参与贡献](/docs/project/contributing)。
