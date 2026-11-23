---
title: 安裝
description: 從 crates.io、npm 或 release 執行檔安裝 everything-rust，以及每種語言需要的執行環境。
---

# 安裝

## 從 crates.io、npm 或 release 安裝

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

npm 安裝的是同一個執行檔，方便 JavaScript 專案使用。

macOS、Linux 與 Windows 的預先建置執行檔附在每個 [GitHub release](https://github.com/marprojector/everything-rust/releases) 中。在 Claude Code 中也可以將它安裝為外掛（包含 MCP 伺服器與技能；CLI 必須在你的 `PATH` 上）：

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## 從原始碼安裝

```sh
cargo install --path .
```

`record`、`diff` 與 `bench` 會在原始程式自己的執行環境中執行它：TS 需要 Node 22.6+（它直接執行 `.ts`
原始檔），Python 與 C（ctypes）需要 Python 3.9+，Go 需要 1.21+，Ruby 需要 3+，Java 需要 JDK 17+
（`javac` 與 `java`）。在 Node 上執行 `profile` 需要 Node 24+，才能在 `NODE_OPTIONS` 中使用 `--cpu-prof`。C 還需要 C 編譯器。wasm 橋接還需要 `rustup target add wasm32-unknown-unknown` 以及
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked`；`build` 會指出
確切的版本。

## Windows

原生 Windows CI 使用 `x86_64-pc-windows-msvc` Rust 目標。請安裝穩定版 MSVC
工具鏈，以及含有 **Desktop development with C++** 工作負載的 Visual Studio Build Tools；
C 橋接與原生 Rust 擴充需要其中的 C 編譯器與連結器。請安裝你要遷移的來源語言的執行環境。
Python 可能以 `python3` 或 `python` 提供；everything-rust 兩者都能偵測。CI 並未測試 GNU Windows 目標。

請在 Developer PowerShell 提示字元中執行原生橋接整合測試套件：

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

此測試套件會建置並載入真正以 DLL 為基礎的 Node 與 Python 擴充，以及 `PATH` 上每個現有執行環境
所支援的原生橋接。缺少的選用執行環境會回報為略過；Cargo、目前使用的 Rust 標準函式庫與 MSVC 連結器
則是必要的。如果 Cargo 回報找不到 `std`，請先執行
`rustup target remove x86_64-pc-windows-msvc`，再執行
`rustup target add x86_64-pc-windows-msvc` 重新安裝目前的目標。如果連結失敗，請在 Visual Studio Developer
PowerShell 中執行，並確認 `where.exe link` 能找到 MSVC 連結器。測試會使用暫存資料夾，
包括含有空格與 Unicode 字元的路徑，並在成功執行後移除產生的檔案。

對於 AI 代理，`everything-rust setup` 會註冊所有東西（見快速開始）。若要改為手動將 MCP 伺服器加入
單一專案（`.mcp.json`）：

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

每個工具、它的參數以及它可能動到的內容：[docs/tools.md](/docs/reference/mcp-tools)。伺服器也
提供兩個由技能建構的提示：`port_unit` 與 `port_whole_app`。
## 更新或移除

```sh
cargo install everything-rust                                    # 更新（Cargo）
npm install -g everything-rust@latest                            # 更新（npm）
everything-rust setup                                            # 重新整理 AI 代理設定
everything-rust setup --remove && cargo uninstall everything-rust  # 移除（Cargo）
everything-rust setup --remove && npm uninstall -g everything-rust # 移除（npm）
```

`setup --remove` 只會移除 everything-rust 的項目；它儲存的副本
（`<file>.everything-rust.bak`）會保留，直到你自行刪除。
