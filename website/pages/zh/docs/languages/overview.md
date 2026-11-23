---
title: 支持语言
description: "TypeScript/JavaScript、Python、C、Go、Ruby 和 Java，另有处于预览阶段的 PHP 和 C#。每种语言的单元、桥接层和录制器。"
---

# 支持语言

六种语言可以走完整个核心闭环：Node.js（TypeScript/JavaScript）、Python、C、Go、Ruby 和 Java。PHP 和 C# 处于预览阶段：目前支持 `analyze` 和整个应用，单元桥接层是下一步。

| 语言 | 单元 | 桥接层 | 录制与回放方式 |
| --- | --- | --- | --- |
| TypeScript/JavaScript | 文件夹或单个文件 | napi-rs 或 wasm-bindgen，配合生成的二进制传输协议；支持类和异步 | Node，直接运行 `.ts` 源码 |
| Python | 包或模块 | PyO3（abi3），直接类型转换，类型化异常 | Python，导入原实现 |
| C | `.c` 文件 | 与原函数同名的 `extern "C"` 函数及头文件 | 将原实现编译为共享库，通过 ctypes 调用，每条用例一个 fork |
| Go | 包 | 类型化 cgo 调用与生成的二进制传输协议，放在保持原 API 的包中 | 生成的 Go 驱动，作为独立模块构建 |
| Ruby | 包含模块函数的文件或文件夹 | 挂载在原模块下的 magnus 扩展 | 生成的 Ruby 驱动 |
| Java | 包含 `public static` 方法的包 | 类型化 JNI 与生成的二进制传输协议，放在保持原方法的包装类中 | 生成的反射驱动 |
| PHP（预览） | 包含函数和 `public static` 方法的文件夹 | 即将推出：基于 C ABI 的 PHP FFI | 目前支持整个应用：`app record`、`app replay` 和前门，适用于 CLI 与服务端（`php -S`） |
| C#（预览） | 包含 `public static` 方法的文件夹 | 即将推出：基于 C ABI 的 P/Invoke | 目前支持整个应用：`app record`、`app replay` 和前门，适用于 CLI 与 ASP.NET 服务端（`dotnet <app>.dll`） |

除 C 以外的每种语言还支持批处理类（`--batch`）：数据只跨边界一次并留在
Rust 侧，每次调用都会在全部 CPU 核心上处理所有数据。

语言会根据仓库自动检测（`go.mod`、`pom.xml`、`.csproj` 或 `.sln`、
`pyproject.toml`、`Gemfile`、`composer.json`、`package.json`，否则按最常见的源码文件判断）；
`--lang` 可以覆盖检测结果。类型来自源码：TS 类型、Python 类型注解、C、Go、Java 和 C#
函数签名、Ruby YARD 标签、PHP 类型和 phpdoc；未标注类型但按数值使用的参数会被推断
为数值。

PHP 处于预览阶段：`analyze` 能为 PHP 应用制定计划（Composer 包和 PSR-4 自动加载，松散比较、
整数溢出变成浮点数等风险，以及 Composer 包到 crate 的映射），并且可以对 PHP CLI 或服务端进行录制、回放，并把它放到 Rust 前门之后。通过 PHP 的 FFI 把它的
单元迁移到 Rust 是下一步（[issue #2](https://github.com/marprojector/everything-rust/issues/2)）。

C# 同样处于预览阶段：`analyze` 会读取 `.csproj` 项目及其 NuGet 包
（映射为 crate），把 `using` 指令解析到对应文件夹，并标出 C# 行为不同的地方
（`decimal`、`dynamic`、LINQ 的延迟查询、`Task`、`lock`、反射）。C# CLI 或 ASP.NET
服务端可以被录制、回放，并放到 Rust 前门之后；通过 P/Invoke 迁移单元
是下一步。
## 各语言状态

| 语言 | 状态 | 最近一次真实运行 |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | 已验证 | json-server、Fuse.js、vercel/ms、x18n 的 `packages/core` |
| [Python](/docs/languages/python) | 已验证 | CPython 3.13 的 `colorsys.py`：2,400/2,400 |
| [Go](/docs/languages/go) | 已验证 | Go 1.24 的 `path`：2,642/2,642 |
| [Ruby](/docs/languages/ruby) | 已验证 | Ruby 3.3 的 `shellwords.rb`：1,617/1,617 |
| [Java](/docs/languages/java) | 已验证 | 门面后的 JDK 21：4,492/4,492 |
| [C](/docs/languages/c) | 已验证 | 门面后的 glibc：去掉一条未定义行为用例后 1,859/1,859 |
| [PHP](/docs/languages/php) | 预览 | 仅支持 `analyze` 和整个应用 |
| [C#](/docs/languages/csharp) | 预览 | 仅支持 `analyze` 和整个应用 |

包含命令的完整表格见[项目状态](/docs/project/status)页面。
