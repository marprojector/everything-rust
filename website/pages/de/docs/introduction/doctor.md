---
title: Doctor
description: everything-rust doctor prüft die Sprach-Runtimes, die Agent-Einrichtung und den MCP-Server und nennt zu jedem Problem den Befehl, der es behebt.
---

# Doctor

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor` prüft die Sprach-Runtimes, die Agent-Einrichtung und den MCP-Server und gibt zu jedem Problem den Befehl aus, der es behebt. Es beendet sich mit 1, wenn etwas Erforderliches fehlt.

Führe es aus, wenn ein Schritt fehlschlägt, weil eine Runtime, die Agent-Einrichtung oder der MCP-Server fehlt oder veraltet ist. Der Workflow-Skill weist Agents an, dasselbe zu tun.

## Was jede Sprache braucht

`record`, `diff` und `bench` führen das Original in seiner eigenen Runtime aus: Node 22.6+ für TS (es führt `.ts`-Quellen direkt aus), Python 3.9+ für Python und C (ctypes), Go 1.21+, Ruby 3+, JDK 17+ (`javac` und `java`). `profile` braucht unter Node Node 24+, das `--cpu-prof` in `NODE_OPTIONS` erlaubt. C braucht außerdem einen C-Compiler. wasm-Bridges brauchen zusätzlich `rustup target add wasm32-unknown-unknown` und die passende `wasm-bindgen-cli`; `build` nennt die genaue Version.

## Für Mitwirkende

`cargo run -- doctor` listet auf, welche Runtimes du hast, ihre Versionen und den Befehl, der jede installiert oder aktualisiert. Siehe [Mitwirken](/docs/project/contributing).
