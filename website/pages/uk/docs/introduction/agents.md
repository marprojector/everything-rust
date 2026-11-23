---
title: Налаштування агента
description: everything-rust setup реєструє сервер MCP і навичку робочого процесу в Claude Code, Codex, Cursor і Gemini CLI.
---

# Налаштування агента

```sh
everything-rust setup
```

`setup` додає сервер MCP до кожного агента, якого знаходить, і встановлює навичку робочого процесу туди, звідки агент читає навички. Інші ваші налаштування він не чіпає, його безпечно запускати повторно, а `--remove` скасовує зміни.

## Працює з

| Агент | Що додає `everything-rust setup` |
| --- | --- |
| Claude Code | сервер MCP у `~/.claude.json`, навичку в `~/.claude/skills/` |
| Codex | сервер MCP у `~/.codex/config.toml`, навичку в `~/.codex/skills/` |
| Cursor | сервер MCP у `~/.cursor/mcp.json` |
| Gemini CLI | сервер MCP у `~/.gemini/settings.json` |
| Будь-який інший клієнт MCP | додайте `everything-rust mcp` як stdio-сервер вручну (нижче) |

Перш ніж змінити наявний файл, `setup` зберігає його копію поруч (`<file>.everything-rust.bak`). `setup --remove` прибирає лише записи everything-rust; збережені копії лишаються, доки ви їх не видалите.

## Сервер MCP

`everything-rust mcp` надає кожен крок як інструмент MCP через stdio: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate`, а для цілого застосунку `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Усі інструменти, їхні параметри і те, що вони можуть змінювати: [Інструменти MCP](/docs/reference/mcp-tools). Сервер також пропонує два промпти, `port_unit` і `port_whole_app`, складені з навички.

Щоб натомість додати сервер MCP вручну до одного проєкту:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## Навичка

Навичка робочого процесу ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) пояснює агенту, як проходити цикл: профілювати, скласти план і зупинитися для схвалення, портувати один модуль, довести його правильність, випустити за shadow і вважати роботу завершеною лише тоді, коли проходить `gate`. Для цілого застосунку вона дотримується [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md); див. [Портування цілого застосунку](/docs/whole-apps/overview).

## Плагін для Claude Code

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

Плагін приносить сервер MCP і навичку; CLI має бути у вашому `PATH`.
