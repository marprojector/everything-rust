---
title: Агентны көйләү
description: everything-rust setup MCP серверны һәм workflow skill'ны Claude Code, Codex, Cursor һәм Gemini CLI'да теркәп куя.
---

# Агентны көйләү

```sh
everything-rust setup
```

`setup` тапкан һәр агентка MCP серверны өсти һәм workflow skill'ны агент skill'ларны укыган урынга урнаштыра. Ул сезнең башка көйләнмәләрегезне саклый, аны кабат җибәрү куркынычсыз, ә `--remove` аны кире кайтара.

## Кайсы агентлар белән эшли

| Агент | `everything-rust setup` нәрсә өсти |
| --- | --- |
| Claude Code | `~/.claude.json` эчендә MCP сервер, `~/.claude/skills/` эчендә skill |
| Codex | `~/.codex/config.toml` эчендә MCP сервер, `~/.codex/skills/` эчендә skill |
| Cursor | `~/.cursor/mcp.json` эчендә MCP сервер |
| Gemini CLI | `~/.gemini/settings.json` эчендә MCP сервер |
| Башка теләсә нинди MCP клиент | `everything-rust mcp` командасын stdio сервер итеп кулдан өстәгез (түбәндә) |

Булган файлны үзгәртер алдыннан `setup` аның күчермәсен янәшәдә саклый (`<file>.everything-rust.bak`). `setup --remove` everything-rust язмаларын гына алып ташлый; ул саклаган күчермәләр сез аларны бетергәнче кала.

## MCP сервер

`everything-rust mcp` һәр адымны stdio аша MCP коралы итеп тәкъдим итә: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate`, ә бөтен кушымта өчен `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Һәр корал, аның параметрлары һәм ул нәрсәгә кагыла ала: [MCP кораллары](/docs/reference/mcp-tools). Сервер шулай ук skill нигезендә төзелгән ике prompt тәкъдим итә: `port_unit` һәм `port_whole_app`.

MCP серверны бер проектка кулдан өстәргә теләсәгез:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## Skill

Workflow skill ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) агентка циклны ничек алып барырга кирәклеген әйтә: профильләү, планлаштыру һәм раслау өчен туктау, бер берәмлекне күчерү, аны исбатлау, shadow артында чыгару һәм `gate` узгач кына эшне тәмам дип санау. Бөтен кушымта өчен ул [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) буенча эшли; [Бөтен кушымтаны күчерү](/docs/whole-apps/overview) битен карагыз.

## Claude Code плагины

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

Плагин MCP серверны һәм skill'ны китерә; CLI сезнең `PATH` эчендә булырга тиеш.
