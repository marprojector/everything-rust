---
title: Агентті баптау
description: everything-rust setup MCP серверді және workflow skill-ін Claude Code, Codex, Cursor және Gemini CLI ішінде тіркейді.
---

# Агентті баптау

```sh
everything-rust setup
```

`setup` тапқан әр агентке MCP серверді қосады және workflow skill-ін агент skill-дерді оқитын жерге орнатады. Ол басқа баптауларыңызды сақтайды, оны қайта іске қосу қауіпсіз, ал `--remove` барлығын кері қайтарады.

## Үйлесімді агенттер

| Агент | `everything-rust setup` қосатыны |
| --- | --- |
| Claude Code | `~/.claude.json` ішінде MCP сервер, `~/.claude/skills/` ішінде skill |
| Codex | `~/.codex/config.toml` ішінде MCP сервер, `~/.codex/skills/` ішінде skill |
| Cursor | `~/.cursor/mcp.json` ішінде MCP сервер |
| Gemini CLI | `~/.gemini/settings.json` ішінде MCP сервер |
| Кез келген басқа MCP клиент | `everything-rust mcp` командасын stdio сервер ретінде қолмен қосыңыз (төменде) |

Бар файлды өзгертпес бұрын `setup` оның көшірмесін қасына сақтайды (`<file>.everything-rust.bak`). `setup --remove` тек everything-rust жазбаларын алып тастайды; ол сақтаған көшірмелер өзіңіз жоймайынша қала береді.

## MCP сервер

`everything-rust mcp` әр қадамды stdio арқылы MCP құралы ретінде ұсынады: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate`, ал бүкіл қолданба үшін `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Әр құрал, оның параметрлері және ол неге қол тигізуі мүмкін: [MCP құралдары](/docs/reference/mcp-tools). Сервер сондай-ақ skill негізінде құрылған екі prompt ұсынады: `port_unit` және `port_whole_app`.

MCP серверді оның орнына бір жобаға қолмен қосу үшін:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## Skill

Workflow skill ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) агентке циклды қалай жүргізу керектігін айтады: профайлдау, жоспарлап, мақұлдау үшін тоқтау, бір бірлікті порттау, оны дәлелдеу, shadow артында шығару және тек `gate` өткенде ғана дайын деп санау. Бүкіл қолданба үшін ол [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) файлын ұстанады; [Бүкіл қолданбаны порттау](/docs/whole-apps/overview) бетін қараңыз.

## Claude Code плагині

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

Плагин MCP серверді және skill-ді әкеледі; CLI сіздің `PATH` ішінде болуы керек.
