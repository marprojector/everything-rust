---
title: 配置你的智能体
description: everything-rust setup 会把 MCP 服务器和工作流技能注册到 Claude Code、Codex、Cursor 和 Gemini CLI。
---

# 配置你的智能体

```sh
everything-rust setup
```

`setup` 会把 MCP 服务器添加到它找到的每一个智能体中，并把工作流技能安装到该智能体读取技能的位置。它会保留你的其他配置，可以安全地重复运行，`--remove` 则会撤销这些改动。

## 适配工具

| 智能体 | `everything-rust setup` 会添加 |
| --- | --- |
| Claude Code | `~/.claude.json` 中的 MCP 服务器，`~/.claude/skills/` 中的技能 |
| Codex | `~/.codex/config.toml` 中的 MCP 服务器，`~/.codex/skills/` 中的技能 |
| Cursor | `~/.cursor/mcp.json` 中的 MCP 服务器 |
| Gemini CLI | `~/.gemini/settings.json` 中的 MCP 服务器 |
| 其他任意 MCP 客户端 | 手动将 `everything-rust mcp` 添加为 stdio 服务器（见下文） |

修改已存在的文件之前，`setup` 会在它旁边保存一份备份（`<file>.everything-rust.bak`）。`setup --remove` 只会移除 everything-rust 写入的条目；它保存的备份会一直保留，直到你手动删除。

## MCP 服务器

`everything-rust mcp` 通过 stdio 把每个步骤作为 MCP 工具提供：`profile`、`analyze`、`bridge`、`refresh`、`record`、`build`、`diff`、`bench`、`shadow`、`gate`，以及面向整个应用的 `app_record`、`app_replay`、`app_front_door`、`app_wire`、`app_lift`。每个工具、它的参数以及它可能改动的内容：[MCP 工具](/docs/reference/mcp-tools)。服务器还提供两个基于技能构建的提示词：`port_unit` 和 `port_whole_app`。

如果只想手动把 MCP 服务器添加到单个项目：

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## 技能

工作流技能（[`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)）告诉智能体如何运行核心闭环：做性能分析，制定计划并停下来等待批准，迁移一个单元，证明它正确，以影子运行的方式上线，并且只有在 `gate` 通过时才算完成。对于整个应用，它会遵循 [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md)；见[迁移整个应用](/docs/whole-apps/overview)。

## Claude Code 插件

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

该插件包含 MCP 服务器和技能；CLI 必须位于 `PATH` 中。
