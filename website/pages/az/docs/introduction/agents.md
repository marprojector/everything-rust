---
title: Agentinizi qurun
description: everything-rust setup MCP serveri və iş axını skill-ini Claude Code, Codex, Cursor və Gemini CLI-da qeydiyyatdan keçirir.
---

# Agentinizi qurun

```sh
everything-rust setup
```

`setup` tapdığı hər agentə MCP serveri əlavə edir və iş axını skill-ini agentin skill-ləri oxuduğu yerə quraşdırır. Digər ayarlarınızı saxlayır, yenidən işə salmaq təhlükəsizdir, `--remove` isə etdiklərini geri qaytarır.

## Dəstəklənən agentlər

| Agent | `everything-rust setup` əlavə edir |
| --- | --- |
| Claude Code | `~/.claude.json`-da MCP server, `~/.claude/skills/`-də skill |
| Codex | `~/.codex/config.toml`-da MCP server, `~/.codex/skills/`-də skill |
| Cursor | `~/.cursor/mcp.json`-da MCP server |
| Gemini CLI | `~/.gemini/settings.json`-da MCP server |
| İstənilən digər MCP müştərisi | `everything-rust mcp`-ni əllə stdio server kimi əlavə edin (aşağıda) |

Mövcud faylı dəyişməzdən əvvəl `setup` onun yanında bir surətini saxlayır (`<file>.everything-rust.bak`). `setup --remove` yalnız everything-rust-in qeydlərini silir; saxladığı surətlər siz onları silənə qədər qalır.

## MCP server

`everything-rust mcp` hər addımı stdio üzərindən MCP aləti kimi təqdim edir: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate`, bütöv tətbiq üçün isə `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Hər alət, onun parametrləri və nəyə toxuna biləcəyi: [MCP alətləri](/docs/reference/mcp-tools). Server həmçinin skill-dən qurulmuş iki prompt təklif edir: `port_unit` və `port_whole_app`.

Bunun əvəzinə MCP serveri bir layihəyə əllə əlavə etmək üçün:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## Skill

İş axını skill-i ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) agentə dövrü necə işlətməyi deyir: profil çıxarmaq, planlaşdırmaq və təsdiq üçün dayanmaq, bir vahidi port etmək, onu sübut etmək, shadow arxasında buraxmaq və yalnız `gate` keçəndə hazır saymaq. Bütöv tətbiq üçün [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md)-ə əməl edir; bax [Bütöv tətbiqi port etmək](/docs/whole-apps/overview).

## Claude Code plugin-i

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

Plugin MCP serveri və skill-i gətirir; CLI `PATH`-da olmalıdır.
