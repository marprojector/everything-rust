---
title: Настройка агента
description: everything-rust setup регистрирует MCP-сервер и навык рабочего процесса в Claude Code, Codex, Cursor и Gemini CLI.
---

# Настройка агента

```sh
everything-rust setup
```

`setup` добавляет MCP-сервер в каждый найденный агент и устанавливает навык рабочего процесса туда, откуда агент читает навыки. Остальные настройки сохраняются, повторный запуск безопасен, а `--remove` отменяет изменения.

## Поддерживаемые агенты

| Агент | Что добавляет `everything-rust setup` |
| --- | --- |
| Claude Code | MCP-сервер в `~/.claude.json`, навык в `~/.claude/skills/` |
| Codex | MCP-сервер в `~/.codex/config.toml`, навык в `~/.codex/skills/` |
| Cursor | MCP-сервер в `~/.cursor/mcp.json` |
| Gemini CLI | MCP-сервер в `~/.gemini/settings.json` |
| Любой другой MCP-клиент | добавьте `everything-rust mcp` как stdio-сервер вручную (см. ниже) |

Перед изменением существующего файла `setup` сохраняет копию рядом с ним (`<file>.everything-rust.bak`). `setup --remove` удаляет только записи everything-rust; сохранённые копии остаются, пока вы их не удалите.

## MCP-сервер

`everything-rust mcp` предоставляет каждый шаг как MCP-инструмент через stdio: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate`, а для целого приложения `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Все инструменты, их параметры и то, что они могут затронуть: [Инструменты MCP](/docs/reference/mcp-tools). Сервер также предлагает два промпта, `port_unit` и `port_whole_app`, собранных из навыка.

Чтобы вместо этого добавить MCP-сервер вручную в один проект:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## Навык

Навык рабочего процесса ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) объясняет агенту, как проходить цикл: профилировать, составить план и остановиться для одобрения, портировать один модуль, доказать его корректность, выкатить за теневым режимом и считать работу завершённой, только когда проходит `gate`. Для целого приложения он следует [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md); см. [Портирование целого приложения](/docs/whole-apps/overview).

## Плагин для Claude Code

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

Плагин приносит MCP-сервер и навык; CLI должен быть в вашем `PATH`.
