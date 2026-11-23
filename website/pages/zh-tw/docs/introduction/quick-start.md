---
title: 快速開始
description: 安裝 everything-rust，在你的程式開發 AI 代理中註冊它，並在第一個單元上執行工作循環。
---

# 快速開始

## 讓你的 AI 代理安裝它

將以下內容貼到 Claude Code、Codex、Cursor 或 Gemini CLI：

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## 或自行安裝

::: tabs
== cargo
```sh
cargo install everything-rust   # 從 crates.io 安裝
everything-rust setup           # 在 Claude Code、Codex、Cursor 與 Gemini CLI 中註冊
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

還沒有 Rust 工具鏈？macOS、Linux 與 Windows 的預先建置執行檔附在每個 [GitHub release](https://github.com/marprojector/everything-rust/releases) 中，而 JavaScript 專案可以用 npm 安裝同一個執行檔。

`setup` 會把 MCP 伺服器加入它找到的每個 AI 代理，並將工作流程技能安裝到該代理讀取技能的位置。它會變更哪些內容，請參閱[設定你的 AI 代理](/docs/introduction/agents)。

## 詢問你的 AI 代理

- *「用 everything-rust 把這個 CLI 從頭到尾以 Rust 重寫，並證明它的行為完全相同。」*
- *「這個搜尋感覺很慢。用 everything-rust 做效能剖析，然後把熱點迴圈移到 Rust。」*
- *「用 everything-rust 規劃把 `src/parser/` 遷移到 Rust；移植第一個單元並證明它正確。」*
- *「對 `rust/text-rs` 中的 Rust 移植版本與原始版本做基準測試，告訴我是否該出貨。」*

## 或自行執行各個步驟

以 x18n 的 `fuzzyScore` 為例的步驟：`profile` 找出熱點迴圈，`bridge --batch` 產生 crate，`gate` 通過，基準測試再將它與 TypeScript 版本比較。

```sh
everything-rust profile . -- node bench.mjs              # 哪裡是熱點，以及誰呼叫了它
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# 撰寫 src/port.rs（或交給你的 AI 代理），然後：
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # 每個已錄製的案例都必須一致
everything-rust bench rust/fuzzy-rs                      # 更快嗎？看結論就知道
everything-rust gate rust/fuzzy-rs                       # 這個通過才算完成
```

每個步驟的說明請見[工作循環](/docs/concepts/the-loop)。

::: tip 缺少東西時
如果某個步驟因為執行環境、AI 代理設定或 MCP 伺服器缺少或版本過舊而失敗，請執行 [`everything-rust doctor`](/docs/introduction/doctor)。
:::
