---
title: Installation
description: Installiere everything-rust über crates.io, npm oder eine Release-Binary, dazu die Runtimes, die jede Sprache braucht.
---

# Installation

## Über crates.io, npm oder ein Release

::: tabs
== cargo
```sh
cargo install everything-rust
everything-rust setup
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

npm installiert dieselbe Binary, für JavaScript-Projekte.

Vorkompilierte Binaries für macOS, Linux und Windows hängen an jedem [GitHub-Release](https://github.com/marprojector/everything-rust/releases). In Claude Code kannst du es auch als Plugin installieren (der MCP-Server und der Skill; die CLI muss in deinem `PATH` liegen):

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## Aus dem Quellcode

```sh
cargo install --path .
```

`record`, `diff` und `bench` führen das Original in seiner eigenen Runtime aus: Node 22.6+ für TS (es führt `.ts`-Quellen
direkt aus), Python 3.9+ für Python und C (ctypes), Go 1.21+, Ruby 3+, JDK 17+
(`javac` und `java`). `profile` braucht unter Node Node 24+, das `--cpu-prof` in `NODE_OPTIONS` erlaubt. C braucht außerdem einen C-Compiler. wasm-Bridges brauchen zusätzlich `rustup target add wasm32-unknown-unknown` und
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked`; `build` nennt
die genaue Version.

## Windows

Die native Windows-CI nutzt das Rust-Target `x86_64-pc-windows-msvc`. Installiere die stabile MSVC-Toolchain
und die Visual Studio Build Tools mit dem Workload **Desktop development with C++**; der
C-Compiler und der Linker sind für C-Bridges und native Rust-Erweiterungen erforderlich. Installiere die Runtime
der Quellsprache, die du migrierst. Python kann als `python3` oder als
`python` vorhanden sein; everything-rust erkennt beides. Das GNU-Target für Windows wird von der CI nicht geprüft.

Führe die native Bridge-Integrationssuite in einer Developer-PowerShell aus:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

Die Suite baut und lädt echte DLL-basierte Node- und Python-Erweiterungen sowie die unterstützten nativen
Bridges für jede Runtime, die im `PATH` vorhanden ist. Fehlende optionale Runtimes werden als übersprungen gemeldet;
Cargo, die aktive Rust-Standardbibliothek und der MSVC-Linker sind erforderlich. Meldet Cargo, dass
es `std` nicht findet, installiere das aktive Target neu mit
`rustup target remove x86_64-pc-windows-msvc` gefolgt von
`rustup target add x86_64-pc-windows-msvc`. Schlägt das Linken fehl, arbeite in einer Visual Studio Developer
PowerShell und prüfe, dass `where.exe link` den MSVC-Linker findet. Die Tests nutzen temporäre Ordner,
darunter einen Pfad mit Leerzeichen und Unicode-Zeichen, und entfernen erzeugte Artefakte nach einem
erfolgreichen Lauf.

Für Agents registriert `everything-rust setup` alles (siehe Schnellstart). So fügst du den MCP-Server stattdessen
von Hand für ein einzelnes Projekt hinzu (`.mcp.json`):

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

Jedes Tool, seine Parameter und was es anfassen darf: [docs/tools.md](/docs/reference/mcp-tools). Der Server bietet
außerdem zwei Prompts an, `port_unit` und `port_whole_app`, die aus dem Skill erzeugt werden.
## Aktualisieren oder entfernen

```sh
cargo install everything-rust                                    # aktualisieren (Cargo)
npm install -g everything-rust@latest                            # aktualisieren (npm)
everything-rust setup                                            # Agent-Einstellungen auffrischen
everything-rust setup --remove && cargo uninstall everything-rust  # entfernen (Cargo)
everything-rust setup --remove && npm uninstall -g everything-rust # entfernen (npm)
```

`setup --remove` entfernt nur die Einträge von everything-rust; die gespeicherten Kopien
(`<file>.everything-rust.bak`) bleiben, bis du sie löschst.
