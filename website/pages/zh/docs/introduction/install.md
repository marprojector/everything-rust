---
title: 安装
description: 从 crates.io、npm 或 release 二进制安装 everything-rust，以及每种语言所需的运行时。
---

# 安装

## 从 crates.io、npm 或 release 安装

::: tabs
== cargo
```sh
cargo install everything-rust
everything-rust setup
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

npm 安装的是同一个二进制文件，方便 JavaScript 项目使用。

每个 [GitHub release](https://github.com/marprojector/everything-rust/releases) 都附带 macOS、Linux 和 Windows 的预编译二进制文件。在 Claude Code 中，你也可以把它作为插件安装（包含 MCP 服务器和技能；CLI 必须位于 `PATH` 中）：

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## 从源码安装

```sh
cargo install --path .
```

`record`、`diff` 和 `bench` 会在原实现自己的运行时中运行它：TS 需要 Node 22.6+（它直接运行 `.ts`
源码），Python 和 C（ctypes）需要 Python 3.9+，Go 需要 1.21+，Ruby 需要 3+，Java 需要 JDK 17+
（`javac` 和 `java`）。在 Node 上使用 `profile` 需要 Node 24+，它允许在 `NODE_OPTIONS` 中使用 `--cpu-prof`。C 还需要 C 编译器。wasm 桥接层还需要 `rustup target add wasm32-unknown-unknown` 和
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked`；`build` 会给出
确切的版本号。

## Windows

原生 Windows CI 使用 `x86_64-pc-windows-msvc` Rust target。请安装稳定版 MSVC
工具链，以及带 **Desktop development with C++** 工作负载的 Visual Studio Build Tools；
C 桥接层和原生 Rust 扩展需要其中的 C 编译器和链接器。再安装你要迁移的源语言对应的运行时。Python 可能以 `python3` 或
`python` 的名字提供，everything-rust 两者都能识别。CI 不覆盖 GNU Windows target。

在 Developer PowerShell 提示符中运行原生桥接集成测试：

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

该测试套件会构建并加载真实的、基于 DLL 的 Node 和 Python 扩展，以及 `PATH` 中存在的每种运行时所支持的原生
桥接层。缺少的可选运行时会被报告为已跳过；Cargo、当前使用的 Rust 标准库和 MSVC 链接器则是必需的。如果 Cargo 报告
找不到 `std`，请先运行
`rustup target remove x86_64-pc-windows-msvc`，再运行
`rustup target add x86_64-pc-windows-msvc`，重新安装当前 target。如果链接失败，请在 Visual Studio Developer
PowerShell 中运行，并确认 `where.exe link` 能找到 MSVC 链接器。测试使用临时文件夹，
其中包括带空格和 Unicode 字符的路径，并会在成功运行后删除生成的产物。

对于智能体，`everything-rust setup` 会注册所需的一切（见快速开始）。如果只想手动把 MCP 服务器添加到
单个项目（`.mcp.json`）：

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

每个工具、它的参数以及它可能改动的内容：[docs/tools.md](/docs/reference/mcp-tools)。服务器还
提供两个基于技能构建的提示词：`port_unit` 和 `port_whole_app`。
## 更新或卸载

```sh
cargo install everything-rust                                    # 更新（Cargo）
npm install -g everything-rust@latest                            # 更新（npm）
everything-rust setup                                            # 刷新智能体配置
everything-rust setup --remove && cargo uninstall everything-rust  # 卸载（Cargo）
everything-rust setup --remove && npm uninstall -g everything-rust # 卸载（npm）
```

`setup --remove` 只会移除 everything-rust 写入的条目；它保存的备份
（`<file>.everything-rust.bak`）会一直保留，直到你手动删除。
