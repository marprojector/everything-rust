---
title: Ajanınızı kurun
description: everything-rust setup, MCP sunucusunu ve iş akışı skill'ini Claude Code, Codex, Cursor ve Gemini CLI'a kaydeder.
---

# Ajanınızı kurun

```sh
everything-rust setup
```

`setup`, bulduğu her ajana MCP sunucusunu ekler ve iş akışı skill'ini ajanın skill'leri okuduğu yere kurar. Diğer ayarlarınızı korur, tekrar çalıştırmak güvenlidir ve `--remove` yaptıklarını geri alır.

## Desteklenen ajanlar

| Ajan | `everything-rust setup` şunları ekler |
| --- | --- |
| Claude Code | `~/.claude.json` içinde MCP sunucusu, `~/.claude/skills/` içinde skill |
| Codex | `~/.codex/config.toml` içinde MCP sunucusu, `~/.codex/skills/` içinde skill |
| Cursor | `~/.cursor/mcp.json` içinde MCP sunucusu |
| Gemini CLI | `~/.gemini/settings.json` içinde MCP sunucusu |
| Diğer herhangi bir MCP istemcisi | `everything-rust mcp`'yi elle stdio sunucusu olarak ekleyin (aşağıda) |

Var olan bir dosyayı değiştirmeden önce `setup` yanına bir kopyasını kaydeder (`<file>.everything-rust.bak`). `setup --remove` yalnızca everything-rust'in girdilerini kaldırır; kaydettiği kopyalar siz silene kadar kalır.

## MCP sunucusu

`everything-rust mcp` her adımı stdio üzerinden bir MCP aracı olarak sunar: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate` ve tüm uygulama için `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Her araç, parametreleri ve neye dokunabileceği: [MCP araçları](/docs/reference/mcp-tools). Sunucu ayrıca skill'den üretilmiş iki prompt sunar: `port_unit` ve `port_whole_app`.

MCP sunucusunu bunun yerine tek bir projeye elle eklemek için:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## Skill

İş akışı skill'i ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) ajana döngüyü nasıl çalıştıracağını anlatır: profil çıkar, plan yap ve onay için dur, bir birimi port et, kanıtla, bir shadow arkasında yayına al ve ancak `gate` geçtiğinde bitti de. Tüm bir uygulama için [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) dosyasını izler; bkz. [Tüm uygulamayı port etmek](/docs/whole-apps/overview).

## Claude Code plugin'i

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

Plugin, MCP sunucusunu ve skill'i getirir; CLI'ın `PATH` içinde olması gerekir.
