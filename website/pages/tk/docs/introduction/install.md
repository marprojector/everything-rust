---
title: Gurnamak
description: everything-rust-i crates.io, npm ýa-da release binar faýlyndan gurnaň, şeýle hem her diliň talap edýän runtime-laryny.
---

# Gurnamak

## crates.io, npm ýa-da release arkaly

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

npm JavaScript taslamalary üçin şol bir binar faýly gurnaýar.

macOS, Linux we Windows üçin taýýar binar faýllar her [GitHub release](https://github.com/marprojector/everything-rust/releases) sahypasyna goşulan. Claude Code-da ony plugin hökmünde hem gurnap bilersiňiz (MCP serwer we skill; CLI siziň `PATH` içiňizde bolmaly):

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## Çeşme kodundan

```sh
cargo install --path .
```

`record`, `diff` we `bench` asyl nusgany onuň öz runtime-ynda işledýär: TS üçin Node 22.6+ (ol `.ts`
çeşme faýllaryny göni işledýär), Python we C (ctypes) üçin Python 3.9+, Go 1.21+, Ruby 3+, JDK 17+
(`javac` we `java`). Node-da `profile` üçin Node 24+ gerek, ol `NODE_OPTIONS` içinde `--cpu-prof` ulanmaga rugsat berýär. C üçin şeýle hem C kompilýatory gerek. wasm bridge-leri üçin goşmaça `rustup target add wasm32-unknown-unknown` we
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked` gerek; takyk wersiýany
`build` aýdýar.

## Windows

Ýerli Windows CI `x86_64-pc-windows-msvc` Rust target-ini ulanýar. Durnukly MSVC
toolchain-ini we **Desktop development with C++** iş ýükli Visual Studio Build Tools-y gurnaň;
C kompilýatory we linker C bridge-leri we ýerli Rust giňeltmeleri üçin zerur. Geçirýän çeşme
diliňiziň runtime-yny gurnaň. Python `python3` ýa-da
`python` hökmünde elýeterli bolup biler; everything-rust ikisini hem tapýar. GNU Windows target-i CI-de barlanmaýar.

Ýerli bridge integrasiýa toplumyny Developer PowerShell penjiresinden işlediň:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

Toplum DLL esasly hakyky Node we Python giňeltmelerini, şeýle hem `PATH` içinde bar bolan her runtime üçin
goldanýan ýerli bridge-leri gurýar we ýükleýär. Ýok bolan goşmaça runtime-lar geçirilen (skipped) diýip görkezilýär;
Cargo, işjeň Rust standart kitaphanasy we MSVC linker zerur. Eger Cargo
`std` tapyp bilmeýändigini habar berse, işjeň target-i
`rustup target remove x86_64-pc-windows-msvc`, soňra
`rustup target add x86_64-pc-windows-msvc` bilen täzeden gurnaň. Baglanyşdyrmak (linking) şowsuz bolsa, Visual Studio Developer
PowerShell-den işlediň we `where.exe link` buýrugynyň MSVC linker-i tapýandygyny barlaň. Testler wagtlaýyn bukjalary,
şol sanda boşluklary we Unicode nyşanlary bolan ýoly ulanýar we üstünlikli işletmeden soň döredilen
artefaktlary aýyrýar.

Agentler üçin `everything-rust setup` hemme zady hasaba alýar (Çalt başlangyç sahypasyna serediň). MCP serweri
ýeke taslama el bilen goşmak üçin (`.mcp.json`):

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

Her gural, onuň parametrleri we onuň nämä degip biljekdigi: [docs/tools.md](/docs/reference/mcp-tools). Serwer şeýle hem
skill-den gurlan iki prompt hödürleýär: `port_unit` we `port_whole_app`.
## Täzelemek ýa-da aýyrmak

```sh
cargo install everything-rust                                    # täzelemek (Cargo)
npm install -g everything-rust@latest                            # täzelemek (npm)
everything-rust setup                                            # agent sazlamalaryny täzelemek
everything-rust setup --remove && cargo uninstall everything-rust  # aýyrmak (Cargo)
everything-rust setup --remove && npm uninstall -g everything-rust # aýyrmak (npm)
```

`setup --remove` diňe everything-rust-iň ýazgylaryny aýyrýar; onuň saklan nusgalary
(`<file>.everything-rust.bak`) siz olary pozýançaňyz galýar.
