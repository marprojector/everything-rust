---
title: Status
description: Was in everything-rust heute funktioniert und wie jeder Teil zuletzt geprüft wurde.
---

# Status

Was funktioniert und wie jeder Teil zuletzt geprüft wurde. Aktualisiert am 2026-11-18.

**Verifiziert**: durchgehend ausgeführt (bridge, record, port, diff, gate) auf echtem Code, den jemand anderes geschrieben hat,
unten genannt. **Getestet**: von den Tests dieses Repositorys auf eigens dafür geschriebenen Fixtures abgedeckt, noch nicht
auf einem echten Projekt ausgeführt. **Vorschau**: funktioniert für einen Teil des Ablaufs, wie beschrieben.

## Units portieren

| Sprache | Status | Letzter echter Lauf | Ergebnis |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Verifiziert | [Prettier](https://github.com/marprojector/everything-rust/blob/main/docs/real-world.md#prettier) (2026-11-18), [json-server](/docs/real-world/json-server) (2026-11-18), [Fuse.js](/docs/real-world/fuse-js) (2026-11-18), [vercel/ms](/docs/real-world/vercel-ms) (2026-11-18), `packages/core` von x18n | Prettier: `doc-printer-rs` 2,848/2,848 und `js-format-rs` 1,279/1,279, gate besteht bei beiden; json-server: sechs Units, 3,399 Fälle, gate besteht bei jeder; Fuse.js, vercel/ms, x18n: 3,414/3,414, 975/975 und 9,000+ Fälle stimmen überein |
| Python | Verifiziert | `colorsys.py` aus CPython 3.13 (2026-11-16) | 2,400/2,400; gate besteht |
| Go | Verifiziert | `path` aus Go 1.24 (2026-11-16) | 2,642/2,642; gate besteht |
| Ruby | Verifiziert | `shellwords.rb` aus Ruby 3.3 (2026-11-16) | 1,617/1,617; gate besteht |
| Java | Verifiziert | JDK 21 hinter einer Fassade (2026-11-16) | 4,492/4,492; gate besteht |
| C | Verifiziert | glibc hinter einer Fassade (2026-11-16) | 1,859/1,859 nach dem Entfernen eines Falls mit undefiniertem Verhalten; gate besteht |
| PHP | Vorschau | keiner | nur `analyze` und ganze Apps; Units lassen sich noch nicht bridgen |
| C# | Vorschau | keiner | nur `analyze` und ganze Apps; Units lassen sich noch nicht bridgen |

Details zu jedem Lauf: [real-world.md](/docs/real-world/overview) (TS/JS) und
[real-world-languages.md](/docs/real-world/six-languages) (die anderen).

## Befehle

| Befehl | Status | Hinweise |
| --- | --- | --- |
| `analyze` | Verifiziert | jede Sprache oben, die 142 Units von x18n |
| `bridge`, `record`, `diff`, `gate`, `build` | Verifiziert | die Läufe oben; `bridge --as-json` auf `paginate` von json-server |
| `bench` | Verifiziert | die Läufe oben; Zahlen in [real-world.md](/docs/real-world/overview) |
| `record --from-tests` | Verifiziert (TS/JS), Getestet (Python) | die Jest-Suite von vercel/ms |
| Abdeckung des Originals in `record` und `gate` | Verifiziert (TS/JS) | Fuse.js: Die Fälle führen 81% der importierten Zeilen aus; andere Sprachen messen das noch nicht |
| `--batch` | Verifiziert | Fuse.js, vercel/ms, x18n-Suche |
| wasm-Bridge (`--kind wasm`) | Verifiziert | x18n-Suche |
| `shadow` | Verifiziert | `packages/core` von x18n |
| `profile` | Getestet | |
| `app record`, `app replay` | Verifiziert (Server und CLIs) | json-server: 514 HTTP-Fälle mit `--fresh`, `--arg`, `--pace`; Prettier: 3,080 CLI-Fälle mit `--inputs`; CLIs in `tests/app.rs` |
| Front Door (`front-door`), `wire` | Verifiziert (TS-Server und CLI) | Prettier: eine Unit verdrahtet, 3,080/3,080 bei 57.8% Fallback, Glue-Code lehnt pro Fall mit `original::run_with_stdin` ab; json-server: zuerst die Lesezugriffe verdrahtet (22.6% Fallback), dann jede Route (0%); Server in acht Sprachen in `tests/http.rs` |
| `lift` | Verifiziert (TS-Server), Getestet (CLIs) | json-server: reine Rust-Binary, 514/514; CLIs in allen acht Sprachen in `tests/app.rs` |
| MCP-Server (`mcp`) | Getestet | Protokolltest in `src/mcp.rs` |

## Wo die Tests laufen

Die CI installiert Node, Python, Go, Ruby und Java auf macOS und Windows. Sie installiert weder PHP noch
.NET, daher werden die PHP- und C#-Tests dort übersprungen und laufen nur auf einem Rechner, auf dem sie vorhanden sind.
