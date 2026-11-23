---
title: Set up your agent
description: everything-rust setup registers the MCP server and the workflow skill with Claude Code, Codex, Cursor and Gemini CLI.
---

# Set up your agent

```sh
everything-rust setup
```

`setup` adds the MCP server to each agent it finds and installs the workflow skill where the agent reads skills. It keeps your other settings, is safe to run again, and `--remove` undoes it.

## Works with

| Agent | `everything-rust setup` adds |
| --- | --- |
| Claude Code | MCP server in `~/.claude.json`, skill in `~/.claude/skills/` |
| Codex | MCP server in `~/.codex/config.toml`, skill in `~/.codex/skills/` |
| Cursor | MCP server in `~/.cursor/mcp.json` |
| Gemini CLI | MCP server in `~/.gemini/settings.json` |
| Any other MCP client | add `everything-rust mcp` as a stdio server by hand (below) |

Before changing a file that exists, `setup` saves a copy beside it (`<file>.everything-rust.bak`). `setup --remove` takes out only everything-rust's entries; the copies it saved stay until you delete them.

## The MCP server

`everything-rust mcp` serves every step as an MCP tool over stdio: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate`, and for a whole app `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Every tool, its parameters and what it may touch: [MCP tools](/docs/reference/mcp-tools). The server also offers two prompts, `port_unit` and `port_whole_app`, built from the skill.

To add the MCP server to one project by hand instead:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## The skill

The workflow skill ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) tells the agent how to run the loop: profile, plan and stop for approval, port one unit, prove it, ship it behind a shadow, and call it done only when `gate` passes. For a whole app it follows [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md); see [Port a whole app](/docs/whole-apps/overview).

## Claude Code plugin

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

The plugin brings the MCP server and the skill; the CLI must be on your `PATH`.
