---
title: Agent einrichten
description: everything-rust setup registriert den MCP-Server und den Workflow-Skill bei Claude Code, Codex, Cursor und Gemini CLI.
---

# Agent einrichten

```sh
everything-rust setup
```

`setup` fügt den MCP-Server bei jedem gefundenen Agent hinzu und installiert den Workflow-Skill dort, wo der Agent Skills liest. Deine übrigen Einstellungen bleiben erhalten, du kannst es gefahrlos erneut ausführen, und `--remove` macht es rückgängig.

## Funktioniert mit

| Agent | `everything-rust setup` fügt hinzu |
| --- | --- |
| Claude Code | MCP-Server in `~/.claude.json`, Skill in `~/.claude/skills/` |
| Codex | MCP-Server in `~/.codex/config.toml`, Skill in `~/.codex/skills/` |
| Cursor | MCP-Server in `~/.cursor/mcp.json` |
| Gemini CLI | MCP-Server in `~/.gemini/settings.json` |
| Jeder andere MCP-Client | `everything-rust mcp` von Hand als stdio-Server hinzufügen (siehe unten) |

Bevor `setup` eine vorhandene Datei ändert, speichert es daneben eine Kopie (`<file>.everything-rust.bak`). `setup --remove` entfernt nur die Einträge von everything-rust; die gespeicherten Kopien bleiben, bis du sie löschst.

## Der MCP-Server

`everything-rust mcp` stellt jeden Schritt als MCP-Tool über stdio bereit: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate` und für eine ganze App `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Jedes Tool, seine Parameter und was es anfassen darf: [MCP-Tools](/docs/reference/mcp-tools). Der Server bietet außerdem zwei Prompts an, `port_unit` und `port_whole_app`, die aus dem Skill erzeugt werden.

So fügst du den MCP-Server stattdessen von Hand für ein einzelnes Projekt hinzu:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## Der Skill

Der Workflow-Skill ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) sagt dem Agent, wie er den Ablauf durchläuft: profilieren, planen und auf Freigabe warten, eine Unit portieren, sie beweisen, sie hinter einem Shadow ausliefern und sie erst dann fertig nennen, wenn `gate` besteht. Für eine ganze App folgt er [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md); siehe [Eine ganze App portieren](/docs/whole-apps/overview).

## Claude Code Plugin

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

Das Plugin bringt den MCP-Server und den Skill mit; die CLI muss in deinem `PATH` liegen.
