---
title: Агентиңизди жөндөө
description: everything-rust setup MCP серверди жана workflow skill'ин Claude Code, Codex, Cursor жана Gemini CLI менен каттайт.
---

# Агентиңизди жөндөө

```sh
everything-rust setup
```

`setup` тапкан ар бир агентке MCP серверди кошот жана workflow skill'ин агент skill'дерди окуган жерге орнотот. Ал башка жөндөөлөрүңүздү сактайт, аны кайра иштетүү коопсуз, ал эми `--remove` баарын артка кайтарат.

## Шайкеш агенттер

| Агент | `everything-rust setup` кошкону |
| --- | --- |
| Claude Code | `~/.claude.json` ичинде MCP сервер, `~/.claude/skills/` ичинде skill |
| Codex | `~/.codex/config.toml` ичинде MCP сервер, `~/.codex/skills/` ичинде skill |
| Cursor | `~/.cursor/mcp.json` ичинде MCP сервер |
| Gemini CLI | `~/.gemini/settings.json` ичинде MCP сервер |
| Башка каалаган MCP кардар | `everything-rust mcp` буйругун stdio сервер катары кол менен кошуңуз (төмөндө) |

Бар файлды өзгөртүүдөн мурун `setup` анын көчүрмөсүн жанына сактайт (`<file>.everything-rust.bak`). `setup --remove` everything-rust'тин жазууларын гана алып салат; ал сактаган көчүрмөлөр сиз өчүрмөйүнчө калат.

## MCP сервер

`everything-rust mcp` ар бир кадамды stdio аркылуу MCP куралы катары берет: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate`, ал эми бүт колдонмо үчүн `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Ар бир курал, анын параметрлери жана ал эмнеге тие алары: [MCP куралдары](/docs/reference/mcp-tools). Сервер ошондой эле skill'ден курулган эки prompt сунуштайт: `port_unit` жана `port_whole_app`.

Анын ордуна MCP серверди бир долбоорго кол менен кошуу үчүн:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## Skill

Workflow skill ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) агентке циклди кантип жүргүзүүнү айтат: профилдөө, пландоо жана макулдук үчүн токтоо, бир бирдикти порттоо, аны далилдөө, shadow'дун артында чыгаруу жана `gate` өткөндө гана даяр деп эсептөө. Бүт колдонмо үчүн ал [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) боюнча иштейт; [Бүт колдонмону порттоо](/docs/whole-apps/overview) барагын караңыз.

## Claude Code плагини

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

Плагин MCP серверди жана skill'ди алып келет; CLI сиздин `PATH` ичинде болушу керек.
