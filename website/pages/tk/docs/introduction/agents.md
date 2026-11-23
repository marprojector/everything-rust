---
title: Agentiňizi sazlaň
description: everything-rust setup MCP serweri we workflow skill-ini Claude Code, Codex, Cursor we Gemini CLI bilen hasaba alýar.
---

# Agentiňizi sazlaň

```sh
everything-rust setup
```

`setup` tapan her agentine MCP serweri goşýar we workflow skill-ini agentiň skill-leri okaýan ýerine gurnaýar. Ol beýleki sazlamalaryňyzy saklaýar, ony gaýtadan işletmek howpsuz, `--remove` bolsa ony yzyna gaýtarýar.

## Haýsy agentler bilen işleýär

| Agent | `everything-rust setup` näme goşýar |
| --- | --- |
| Claude Code | `~/.claude.json` içinde MCP serwer, `~/.claude/skills/` içinde skill |
| Codex | `~/.codex/config.toml` içinde MCP serwer, `~/.codex/skills/` içinde skill |
| Cursor | `~/.cursor/mcp.json` içinde MCP serwer |
| Gemini CLI | `~/.gemini/settings.json` içinde MCP serwer |
| Islendik başga MCP müşderi | `everything-rust mcp` buýrugyny stdio serwer hökmünde el bilen goşuň (aşakda) |

Bar bolan faýly üýtgetmezden öň `setup` onuň nusgasyny ýanynda saklaýar (`<file>.everything-rust.bak`). `setup --remove` diňe everything-rust-iň ýazgylaryny aýyrýar; onuň saklan nusgalary siz olary pozýançaňyz galýar.

## MCP serwer

`everything-rust mcp` her ädimi stdio arkaly MCP guraly hökmünde hödürleýär: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate`, tutuş programma üçin bolsa `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Her gural, onuň parametrleri we onuň nämä degip biljekdigi: [MCP gurallary](/docs/reference/mcp-tools). Serwer şeýle hem skill-den gurlan iki prompt hödürleýär: `port_unit` we `port_whole_app`.

MCP serweri ýeke taslama el bilen goşmak üçin:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## Skill

Workflow skill-i ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) agente aýlawy nähili işletmelidigini aýdýar: profillemek, meýilleşdirmek we tassyklama üçin saklanmak, bir birligi geçirmek, ony subut etmek, ony shadow arkasynda çykarmak we diňe `gate` geçende işi tamamlanan hasaplamak. Tutuş programma üçin ol [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) faýlyna eýerýär; [Tutuş programmany geçirmek](/docs/whole-apps/overview) sahypasyna serediň.

## Claude Code plugin-i

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

Plugin MCP serweri we skill-i getirýär; CLI siziň `PATH` içiňizde bolmaly.
