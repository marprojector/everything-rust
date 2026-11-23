---
title: Agentingizni sozlash
description: everything-rust setup MCP serverni va workflow skill'ini Claude Code, Codex, Cursor va Gemini CLI'da ro'yxatdan o'tkazadi.
---

# Agentingizni sozlash

```sh
everything-rust setup
```

`setup` topgan har bir agentiga MCP serverni qo'shadi va workflow skill'ini agent skill'larni o'qiydigan joyga o'rnatadi. U boshqa sozlamalaringizni saqlaydi, qayta ishga tushirish xavfsiz, `--remove` esa qilganlarini bekor qiladi.

## Qo'llab-quvvatlanadigan agentlar

| Agent | `everything-rust setup` qo'shadi |
| --- | --- |
| Claude Code | `~/.claude.json` da MCP server, `~/.claude/skills/` da skill |
| Codex | `~/.codex/config.toml` da MCP server, `~/.codex/skills/` da skill |
| Cursor | `~/.cursor/mcp.json` da MCP server |
| Gemini CLI | `~/.gemini/settings.json` da MCP server |
| Istalgan boshqa MCP mijozi | `everything-rust mcp` ni qo'lda stdio server sifatida qo'shing (pastda) |

Mavjud faylni o'zgartirishdan oldin `setup` uning yoniga nusxasini saqlaydi (`<file>.everything-rust.bak`). `setup --remove` faqat everything-rust yozuvlarini olib tashlaydi; u saqlagan nusxalar siz ularni o'chirmaguningizcha qoladi.

## MCP server

`everything-rust mcp` har bir qadamni stdio orqali MCP vositasi sifatida taqdim etadi: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate`, butun ilova uchun esa `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Har bir vosita, uning parametrlari va u nimalarga tegishi mumkinligi: [MCP vositalari](/docs/reference/mcp-tools). Server skill asosida tuzilgan ikkita prompt ham taklif qiladi: `port_unit` va `port_whole_app`.

MCP serverni buning o'rniga bitta loyihaga qo'lda qo'shish uchun:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## Skill

Workflow skill'i ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) agentga siklni qanday yuritishni aytadi: profil olish, rejalashtirish va tasdiq uchun to'xtash, bitta birlikni port qilish, uni isbotlash, shadow ortida chiqarish va faqat `gate` o'tgandagina tayyor deb hisoblash. Butun ilova uchun u [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) ga amal qiladi; [Butun ilovani port qilish](/docs/whole-apps/overview) sahifasiga qarang.

## Claude Code plugin'i

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

Plugin MCP serverni va skill'ni olib keladi; CLI `PATH` ichida bo'lishi kerak.
