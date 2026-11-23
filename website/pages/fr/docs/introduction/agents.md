---
title: Configurer votre agent
description: everything-rust setup enregistre le serveur MCP et le skill de workflow auprès de Claude Code, Codex, Cursor et Gemini CLI.
---

# Configurer votre agent

```sh
everything-rust setup
```

`setup` ajoute le serveur MCP à chaque agent qu'il trouve et installe le skill de workflow là où l'agent lit ses skills. Il conserve vos autres réglages, peut être relancé sans risque, et `--remove` annule tout.

## Compatible avec

| Agent | Ce que `everything-rust setup` ajoute |
| --- | --- |
| Claude Code | serveur MCP dans `~/.claude.json`, skill dans `~/.claude/skills/` |
| Codex | serveur MCP dans `~/.codex/config.toml`, skill dans `~/.codex/skills/` |
| Cursor | serveur MCP dans `~/.cursor/mcp.json` |
| Gemini CLI | serveur MCP dans `~/.gemini/settings.json` |
| Tout autre client MCP | ajoutez `everything-rust mcp` à la main comme serveur stdio (voir plus bas) |

Avant de modifier un fichier existant, `setup` en enregistre une copie à côté (`<file>.everything-rust.bak`). `setup --remove` ne retire que les entrées de everything-rust ; les copies enregistrées restent jusqu'à ce que vous les supprimiez.

## Le serveur MCP

`everything-rust mcp` expose chaque étape comme un outil MCP via stdio : `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate`, et pour une application entière `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Chaque outil, ses paramètres et ce qu'il peut toucher : [Outils MCP](/docs/reference/mcp-tools). Le serveur propose aussi deux prompts, `port_unit` et `port_whole_app`, construits à partir du skill.

Pour ajouter plutôt le serveur MCP à la main dans un seul projet :

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## Le skill

Le skill de workflow ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) indique à l'agent comment dérouler la boucle : profiler, planifier et s'arrêter pour validation, porter une unité, la prouver, la livrer derrière un shadow, et ne la déclarer terminée que lorsque `gate` passe. Pour une application entière, il suit [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) ; voir [Porter une application entière](/docs/whole-apps/overview).

## Plugin Claude Code

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

Le plugin apporte le serveur MCP et le skill ; la CLI doit être dans votre `PATH`.
