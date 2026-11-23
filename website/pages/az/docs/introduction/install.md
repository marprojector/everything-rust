---
title: Quraşdırma
description: everything-rust-i crates.io, npm və ya release binary-sindən quraşdırın, həmçinin hər dilə lazım olan runtime-ları.
---

# Quraşdırma

## crates.io, npm və ya release-dən

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

npm JavaScript layihələri üçün eyni binary-ni quraşdırır.

macOS, Linux və Windows üçün hazır binary fayllar hər [GitHub release](https://github.com/marprojector/everything-rust/releases)-ə əlavə olunub. Claude Code-da onu plugin kimi də quraşdıra bilərsiniz (MCP server və skill; CLI `PATH`-da olmalıdır):

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## Mənbədən

```sh
cargo install --path .
```

`record`, `diff` və `bench` orijinalı öz runtime-ında işə salır: TS üçün Node 22.6+ (`.ts`
mənbələrini birbaşa işə salır), Python və C (ctypes) üçün Python 3.9+, Go 1.21+, Ruby 3+, JDK 17+
(`javac` və `java`). Node-da `profile` üçün `NODE_OPTIONS`-da `--cpu-prof`-a icazə verən Node 24+ lazımdır. C üçün həmçinin C kompilyatoru lazımdır. wasm körpüləri üçün həmçinin `rustup target add wasm32-unknown-unknown` və
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked` lazımdır; dəqiq versiyanı
`build` göstərir.

## Windows

Yerli Windows CI `x86_64-pc-windows-msvc` Rust target-indən istifadə edir. Stabil MSVC
toolchain-ini və **Desktop development with C++** iş yükü ilə Visual Studio Build Tools-u quraşdırın;
C kompilyatoru və linker C körpüləri və yerli Rust genişlənmələri üçün tələb olunur. Köçürdüyünüz
mənbə dilinin runtime-ını quraşdırın. Python `python3` və ya `python` kimi mövcud ola bilər;
everything-rust hər ikisini tanıyır. GNU Windows target-i CI-da yoxlanılmır.

Yerli körpü inteqrasiya dəstini Developer PowerShell pəncərəsindən işə salın:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

Dəst `PATH`-da olan hər runtime üçün real DLL əsaslı Node və Python genişlənmələrini və dəstəklənən
yerli körpüləri qurur və yükləyir. Çatışmayan ixtiyari runtime-lar ötürülmüş kimi bildirilir;
Cargo, aktiv Rust standart kitabxanası və MSVC linker tələb olunur. Cargo `std`-ni tapa bilmədiyini
bildirirsə, aktiv target-i
`rustup target remove x86_64-pc-windows-msvc` və ardınca
`rustup target add x86_64-pc-windows-msvc` ilə yenidən quraşdırın. Linkləmə uğursuz olarsa, Visual Studio Developer
PowerShell-dən işə salın və `where.exe link`-in MSVC linker-i tapdığını yoxlayın. Testlər boşluq və
Unicode simvolları olan yol da daxil olmaqla müvəqqəti qovluqlardan istifadə edir və uğurlu işləmədən
sonra yaradılmış artefaktları silir.

Agentlər üçün `everything-rust setup` hər şeyi qeydiyyatdan keçirir (bax Sürətli başlanğıc). Bunun əvəzinə MCP serveri
bir layihəyə əllə əlavə etmək üçün (`.mcp.json`):

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

Hər alət, onun parametrləri və nəyə toxuna biləcəyi: [docs/tools.md](/docs/reference/mcp-tools). Server həmçinin
skill-dən qurulmuş iki prompt təklif edir: `port_unit` və `port_whole_app`.
## Yeniləmə və ya silmə

```sh
cargo install everything-rust                                    # yeniləmə (Cargo)
npm install -g everything-rust@latest                            # yeniləmə (npm)
everything-rust setup                                            # agent ayarlarını yeniləmə
everything-rust setup --remove && cargo uninstall everything-rust  # silmə (Cargo)
everything-rust setup --remove && npm uninstall -g everything-rust # silmə (npm)
```

`setup --remove` yalnız everything-rust-in qeydlərini silir; saxladığı surətlər
(`<file>.everything-rust.bak`) siz onları silənə qədər qalır.
