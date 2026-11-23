---
title: ۋاكالەتچىڭىزنى تەڭشەش
description: everything-rust setup بۇيرۇقى MCP مۇلازىمېتىرى ۋە خىزمەت ئېقىمى ماھارىتىنى Claude Code، Codex، Cursor ۋە Gemini CLI غا تىزىملىتىدۇ.
---

# ۋاكالەتچىڭىزنى تەڭشەش

```sh
everything-rust setup
```

`setup` تاپقان ھەر بىر ۋاكالەتچىگە MCP مۇلازىمېتىرىنى قوشىدۇ ۋە خىزمەت ئېقىمى ماھارىتىنى ۋاكالەتچى ماھارەتلەرنى ئوقۇيدىغان جايغا ئورنىتىدۇ. ئۇ باشقا تەڭشەكلىرىڭىزنى ساقلاپ قالىدۇ، قايتا ئىجرا قىلىش بىخەتەر، `--remove` بولسا ئۇنى ئەمەلدىن قالدۇرىدۇ.

## قوللايدىغانلار

| ۋاكالەتچى | `everything-rust setup` قوشىدىغىنى |
| --- | --- |
| Claude Code | `~/.claude.json` دا MCP مۇلازىمېتىرى، `~/.claude/skills/` دا ماھارەت |
| Codex | `~/.codex/config.toml` دا MCP مۇلازىمېتىرى، `~/.codex/skills/` دا ماھارەت |
| Cursor | `~/.cursor/mcp.json` دا MCP مۇلازىمېتىرى |
| Gemini CLI | `~/.gemini/settings.json` دا MCP مۇلازىمېتىرى |
| باشقا ھەر قانداق MCP خېرىدارى | `everything-rust mcp` نى stdio مۇلازىمېتىرى سۈپىتىدە قولدا قوشۇڭ (تۆۋەندە) |

مەۋجۇت بىر ھۆججەتنى ئۆزگەرتىشتىن بۇرۇن، `setup` ئۇنىڭ يېنىغا بىر نۇسخىسىنى ساقلايدۇ (`<file>.everything-rust.bak`). `setup --remove` پەقەت everything-rust نىڭ تۈرلىرىنىلا چىقىرىۋېتىدۇ؛ ئۇ ساقلىغان نۇسخىلار سىز ئۆچۈرمىگۈچە قالىدۇ.

## MCP مۇلازىمېتىرى

`everything-rust mcp` ھەر بىر قەدەمنى stdio ئارقىلىق MCP قورالى سۈپىتىدە تەمىنلەيدۇ: `profile`، `analyze`، `bridge`، `refresh`، `record`، `build`، `diff`، `bench`، `shadow`، `gate`، پۈتۈن ئەپ ئۈچۈن بولسا `app_record`، `app_replay`، `app_front_door`، `app_wire`، `app_lift`. ھەر بىر قورال، ئۇنىڭ پارامېتىرلىرى ۋە نېمىگە تېگىشى مۇمكىنلىكى: [MCP قوراللىرى](/docs/reference/mcp-tools). مۇلازىمېتىر يەنە ماھارەتتىن ياسالغان ئىككى ئەسكەرتىش (prompt) تەمىنلەيدۇ: `port_unit` ۋە `port_whole_app`.

MCP مۇلازىمېتىرىنى ئۇنىڭ ئورنىغا بىر تۈرگىلا قولدا قوشۇش ئۈچۈن:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## ماھارەت

خىزمەت ئېقىمى ماھارىتى ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) ۋاكالەتچىگە ئايلانمىنى قانداق ئىجرا قىلىشنى ئېيتىدۇ: ئانالىز قىلىش، پىلانلاش ۋە تەستىق ئۈچۈن توختاش، بىر بىرلىكنى كۆچۈرۈش، ئۇنى ئىسپاتلاش، سايە ئارقىسىدا ئېلان قىلىش، ۋە پەقەت `gate` ئۆتكەندىلا تامام دەپ قاراش. پۈتۈن ئەپ ئۈچۈن ئۇ [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) غا ئەمەل قىلىدۇ؛ [پۈتۈن ئەپنى كۆچۈرۈش](/docs/whole-apps/overview) نى كۆرۈڭ.

## Claude Code قىستۇرمىسى

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

قىستۇرما MCP مۇلازىمېتىرى ۋە ماھارەتنى ئېلىپ كېلىدۇ؛ CLI چوقۇم `PATH` تا بولۇشى كېرەك.
