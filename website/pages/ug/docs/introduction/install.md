---
title: ئورنىتىش
description: everything-rust نى crates.io، npm ياكى release ئىككىلىك ھۆججىتىدىن ئورنىتىش، ۋە ھەر بىر تىلغا كېرەكلىك ئىجرا مۇھىتلىرى.
---

# ئورنىتىش

## crates.io، npm ياكى release دىن

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

npm ئوخشاش ئىككىلىك ھۆججەتنى JavaScript تۈرلىرى ئۈچۈن ئورنىتىدۇ.

macOS، Linux ۋە Windows ئۈچۈن ئالدىن تۈزۈلگەن ئىككىلىك ھۆججەتلەر ھەر بىر [GitHub release](https://github.com/marprojector/everything-rust/releases) غا قوشۇلغان. Claude Code دا ئۇنى قىستۇرما سۈپىتىدىمۇ ئورناتقىلى بولىدۇ (MCP مۇلازىمېتىرى ۋە ماھارەت؛ CLI چوقۇم `PATH` تا بولۇشى كېرەك):

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## مەنبە كودىدىن

```sh
cargo install --path .
```

`record`، `diff` ۋە `bench` ئەسلى نۇسخىنى ئۆزىنىڭ ئىجرا مۇھىتىدا ئىجرا قىلىدۇ: TS ئۈچۈن Node 22.6+ (ئۇ `.ts`
مەنبە ھۆججەتلىرىنى بىۋاسىتە ئىجرا قىلىدۇ)، Python ۋە C ئۈچۈن Python 3.9+ (ctypes)، Go 1.21+، Ruby 3+، JDK 17+
(`javac` ۋە `java`). Node دا `profile` ئۈچۈن Node 24+ كېرەك، ئۇ `NODE_OPTIONS` تا `--cpu-prof` قا يول قويىدۇ. C ئۈچۈن يەنە C تەرجىمانى كېرەك. wasm كۆۋرۈكلىرىگە يەنە `rustup target add wasm32-unknown-unknown` ۋە
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked` كېرەك؛ ئېنىق نەشرىنى
`build` ئېيتىپ بېرىدۇ.

## Windows

Windows تىكى يەرلىك CI `x86_64-pc-windows-msvc` Rust نىشانىنى ئىشلىتىدۇ. مۇقىم MSVC
قورال زەنجىرى ۋە **Desktop development with C++** خىزمەت يۈكى بىلەن Visual Studio Build Tools نى ئورنىتىڭ؛
C تەرجىمانى ۋە ئۇلىغۇچ (linker) C كۆۋرۈكلىرى ۋە يەرلىك Rust كېڭەيتمىلىرى ئۈچۈن زۆرۈر. كۆچۈرۈۋاتقان مەنبە تىلنىڭ
ئىجرا مۇھىتىنى ئورنىتىڭ. Python `python3` ياكى `python` بولۇپ مەۋجۇت بولۇشى مۇمكىن؛
everything-rust ھەر ئىككىسىنى بايقايدۇ. GNU Windows نىشانى CI دا سىنالمايدۇ.

يەرلىك كۆۋرۈك بىرىكتۈرۈش سىناق توپلىمىنى Developer PowerShell كۆزنىكىدىن ئىجرا قىلىڭ:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

سىناق توپلىمى ھەقىقىي DLL ئاساسىدىكى Node ۋە Python كېڭەيتمىلىرىنى، شۇنداقلا `PATH` تا بار ھەر بىر ئىجرا مۇھىتى ئۈچۈن قوللىنىدىغان يەرلىك
كۆۋرۈكلەرنى قۇرۇپ يۈكلەيدۇ. كەم ئىختىيارىي ئىجرا مۇھىتلىرى ئاتلاپ ئۆتۈلدى دەپ دوكلات قىلىنىدۇ؛
Cargo، ئاكتىپ Rust ئۆلچەملىك كۇتۇپخانىسى ۋە MSVC ئۇلىغۇچى زۆرۈر. ئەگەر Cargo `std` نى تاپالمىدى دېسە،
ئاكتىپ نىشاننى `rustup target remove x86_64-pc-windows-msvc` ئاندىن
`rustup target add x86_64-pc-windows-msvc` ئارقىلىق قايتا ئورنىتىڭ. ئۇلاش مەغلۇپ بولسا، Visual Studio Developer
PowerShell دىن ئىجرا قىلىپ، `where.exe link` نىڭ MSVC ئۇلىغۇچىنى تاپىدىغانلىقىنى جەزملەڭ. سىناقلار ۋاقىتلىق قىسقۇچلارنى ئىشلىتىدۇ،
بۇنىڭ ئىچىدە بوشلۇق ۋە Unicode ھەرپلىرى بار يولمۇ بار، ھەمدە مۇۋەپپەقىيەتلىك ئىجرادىن كېيىن ھاسىل قىلىنغان نەرسىلەرنى ئۆچۈرىدۇ.

ۋاكالەتچىلەر ئۈچۈن `everything-rust setup` ھەممىنى تىزىملىتىدۇ (تېز باشلاشنى كۆرۈڭ). MCP مۇلازىمېتىرىنى ئۇنىڭ ئورنىغا
بىر تۈرگىلا قولدا قوشۇش ئۈچۈن (`.mcp.json`):

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

ھەر بىر قورال، ئۇنىڭ پارامېتىرلىرى ۋە نېمىگە تېگىشى مۇمكىنلىكى: [docs/tools.md](/docs/reference/mcp-tools). مۇلازىمېتىر يەنە
ماھارەتتىن ياسالغان ئىككى ئەسكەرتىش تەمىنلەيدۇ: `port_unit` ۋە `port_whole_app`.
## يېڭىلاش ياكى ئۆچۈرۈش

```sh
cargo install everything-rust                                    # يېڭىلاش (Cargo)
npm install -g everything-rust@latest                            # يېڭىلاش (npm)
everything-rust setup                                            # ۋاكالەتچى تەڭشەكلىرىنى يېڭىلاش
everything-rust setup --remove && cargo uninstall everything-rust  # ئۆچۈرۈش (Cargo)
everything-rust setup --remove && npm uninstall -g everything-rust # ئۆچۈرۈش (npm)
```

`setup --remove` پەقەت everything-rust نىڭ تۈرلىرىنىلا چىقىرىۋېتىدۇ؛ ئۇ ساقلىغان نۇسخىلار
(`<file>.everything-rust.bak`) سىز ئۆچۈرمىگۈچە قالىدۇ.
