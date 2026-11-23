---
title: 什么是 everything-rust
description: everything-rust 录制你的应用当前的行为，并对照这份录制结果校验 Rust 重写的每一部分，直到应用变成纯 Rust。
---

# 什么是 everything-rust

<p class="lead">逐块将你的整个应用重写为 Rust。everything-rust 会录制你现有的 Node.js（TypeScript/JavaScript）、Python、C、Go、Ruby 或 Java 应用的行为，然后在每推进一步之前，都对照这份录制结果校验 Rust 重写的每一部分，直到应用变成纯 Rust。</p>

它适用于 CLI、HTTP 服务端和程序库。Rust 代码由你的编程智能体（Claude Code、Codex、Cursor、Gemini CLI）或你本人编写。PHP 和 C# 处于[预览](/docs/languages/php)阶段。

## 它做什么，不做什么

everything-rust 不翻译代码。它提供的是智能体无法可靠猜出的东西：

- **时间花在哪里**：`profile` 运行真实工作负载，并对热点函数排序；
- **先迁移什么，按什么顺序**：`analyze` 规划单元、迁移波次、风险和 crate；
- **一个桥接层**，让其余代码在 API 不变的情况下调用 Rust（`bridge`）；
- **移植版必须匹配的精确行为**：`record` 在边界用例上运行原实现，并保存它的返回值或抛出的异常；
- **一种安全的上线方式**：`shadow` 让两者并行运行；
- **一个判断迁移何时完成的门禁**：`gate`。

原实现始终是唯一的事实来源。`diff` 在 Rust 实现上回放每一条录制用例，在全部一致且 `gate` 通过之前，什么都不会上线。

## 四个步骤

| 步骤 | 发生了什么 |
| --- | --- |
| **录制** | `record` 在生成的边界用例、语义陷阱和你自己的输入上运行原实现，并保存它的返回值或抛出的异常。 |
| **移植** | `bridge` 生成 Rust crate 和一层包装，让调用方继续使用原有 API。Rust 代码由你或你的智能体编写。 |
| **校验** | `diff` 在 Rust 实现上回放每一条录制用例，在全部一致且 `gate` 通过之前，什么都不会上线。 |
| **收口** | 当回放在零回退的情况下通过、且每个单元都通过门禁时，`app lift` 将其变为纯 Rust 应用。原实现始终不会被改动。 |

## Rust 一定更快吗？

不一定，everything-rust 也会如实告诉你。每条数据调用一次的小函数在 Rust 里往往更慢，因为每次调用都要跨过桥接层；V8、JVM 和 Go 运行这类函数只需纳秒级时间。`profile` 会找到包在它外层的循环，`--batch` 把整个循环迁过去，`bench` 则在上线前给出结论。实测数据（包括慢的情况）见[批处理与热点路径](/docs/concepts/batches)。

## 它只能捕获录制覆盖到的行为

通过只代表 Rust 实现与已录制的每条用例一致，仅此而已。在信任一个绿色的门禁之前，请先阅读[它捕获不到什么](/docs/concepts/limits)。

::: warning 请在副本上运行
`record` 和 `app record` 会运行你的原始代码，用大量生成的输入调用每个函数（或整个应用），并且 `app record` 会把每条用例运行两次。`diff`、`gate` 和 `app replay` 会在移植版或原实现上再次运行相同的用例。会写文件、访问数据库或调用网络的代码，每条用例都会执行一次这些操作。请在项目的副本上运行它们，使用测试数据库，并确保环境中没有生产环境凭据。
:::

## 下一步

- [快速开始](/docs/introduction/quick-start)：安装它，并让你的智能体完成第一次迁移。
- [核心闭环](/docs/concepts/the-loop)：一个单元的每一个步骤。
- [迁移整个应用](/docs/whole-apps/overview)：从第一次录制到纯 Rust 二进制。
- [真实项目迁移](/docs/real-world/overview)：json-server、Fuse.js、vercel/ms 等，慢的结果和快的结果并列展示。
