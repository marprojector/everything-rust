---
title: 快速开始
description: 安装 everything-rust，将它注册到你的编程智能体，并在第一个单元上跑通核心闭环。
---

# 快速开始

## 让你的智能体来安装

把下面这段话粘贴给 Claude Code、Codex、Cursor 或 Gemini CLI：

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## 或者自己安装

::: tabs
== cargo
```sh
cargo install everything-rust   # 从 crates.io 安装
everything-rust setup           # 注册到 Claude Code、Codex、Cursor 和 Gemini CLI
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

还没有安装 Rust 工具链？每个 [GitHub release](https://github.com/marprojector/everything-rust/releases) 都附带 macOS、Linux 和 Windows 的预编译二进制文件；对于 JavaScript 项目，npm 安装的也是同一个二进制文件。

`setup` 会把 MCP 服务器添加到它找到的每一个智能体中，并把工作流技能安装到该智能体读取技能的位置。它具体改动了什么，见[配置你的智能体](/docs/introduction/agents)。

## 向你的智能体提需求

- *"使用 everything-rust 将这个 CLI 端到端重写为 Rust，并证明其行为完全一致。"*
- *"这个搜索感觉很慢。用 everything-rust 做性能分析，把热点循环迁移到 Rust。"*
- *"用 everything-rust 规划 `src/parser/` 向 Rust 的迁移；先迁移第一个单元并验证。"*
- *"将 `rust/text-rs` 中的 Rust 移植版与原实现做基准对比，告诉我是否值得上线。"*

## 或者自己执行每一步

以 x18n 的 `fuzzyScore` 为例：`profile` 找到热点循环，`bridge --batch` 生成 crate，`gate` 通过，基准测试再把它与 TypeScript 版本做对比。

```sh
everything-rust profile . -- node bench.mjs              # 哪里是热点，谁在调用它
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# 编写 src/port.rs（或交给智能体），然后：
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # 每条录制用例都必须一致
everything-rust bench rust/fuzzy-rs                      # 更快吗？看结论
everything-rust gate rust/fuzzy-rs                       # 这一步通过才算完成
```

每一步的说明见[核心闭环](/docs/concepts/the-loop)。

::: tip 缺少某些东西时
如果某一步因为运行时、智能体配置或 MCP 服务器缺失或过旧而失败，请运行 [`everything-rust doctor`](/docs/introduction/doctor)。
:::
