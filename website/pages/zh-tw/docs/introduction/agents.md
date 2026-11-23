---
title: 設定你的 AI 代理
description: everything-rust setup 會在 Claude Code、Codex、Cursor 與 Gemini CLI 中註冊 MCP 伺服器與工作流程技能。
---

# 設定你的 AI 代理

```sh
everything-rust setup
```

`setup` 會把 MCP 伺服器加入它找到的每個 AI 代理，並將工作流程技能安裝到該代理讀取技能的位置。它會保留你的其他設定，可以安全地重複執行，而 `--remove` 可以復原變更。

## 支援的 AI 代理

| AI 代理 | `everything-rust setup` 新增的內容 |
| --- | --- |
| Claude Code | MCP 伺服器寫入 `~/.claude.json`，技能放在 `~/.claude/skills/` |
| Codex | MCP 伺服器寫入 `~/.codex/config.toml`，技能放在 `~/.codex/skills/` |
| Cursor | MCP 伺服器寫入 `~/.cursor/mcp.json` |
| Gemini CLI | MCP 伺服器寫入 `~/.gemini/settings.json` |
| 其他任何 MCP 用戶端 | 手動將 `everything-rust mcp` 新增為 stdio 伺服器（見下方） |

在修改已存在的檔案之前，`setup` 會在旁邊儲存一份副本（`<file>.everything-rust.bak`）。`setup --remove` 只會移除 everything-rust 的項目；它儲存的副本會保留，直到你自行刪除。

## MCP 伺服器

`everything-rust mcp` 透過 stdio 將每個步驟提供為 MCP 工具：`profile`、`analyze`、`bridge`、`refresh`、`record`、`build`、`diff`、`bench`、`shadow`、`gate`，以及用於整個應用程式的 `app_record`、`app_replay`、`app_front_door`、`app_wire`、`app_lift`。每個工具、它的參數以及它可能動到的內容：[MCP 工具](/docs/reference/mcp-tools)。伺服器也提供兩個由技能建構的提示：`port_unit` 與 `port_whole_app`。

若要改為手動將 MCP 伺服器加入單一專案：

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## 技能

工作流程技能（[`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)）會告訴 AI 代理如何執行工作循環：效能剖析、規劃並停下來等待核准、移植一個單元、證明它正確、以影子模式出貨，並且只有在 `gate` 通過時才算完成。對於整個應用程式，它遵循 [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md)；請參閱[移植整個應用程式](/docs/whole-apps/overview)。

## Claude Code 外掛

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

此外掛包含 MCP 伺服器與技能；CLI 必須在你的 `PATH` 上。
