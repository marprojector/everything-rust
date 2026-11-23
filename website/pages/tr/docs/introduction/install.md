---
title: Kurulum
description: everything-rust'i crates.io, npm veya bir release binary'sinden kurun ve her dilin ihtiyaç duyduğu runtime'ları hazırlayın.
---

# Kurulum

## crates.io, npm veya bir release'ten

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

npm, JavaScript projeleri için aynı binary'yi kurar.

macOS, Linux ve Windows için hazır derlenmiş binary'ler her [GitHub release](https://github.com/marprojector/everything-rust/releases)'e eklenmiştir. Claude Code'da bunu bir plugin olarak da kurabilirsiniz (MCP sunucusu ve skill; CLI'ın `PATH` içinde olması gerekir):

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## Kaynaktan

```sh
cargo install --path .
```

`record`, `diff` ve `bench` orijinali kendi runtime'ında çalıştırır: TS için Node 22.6+ (`.ts`
kaynaklarını doğrudan çalıştırır), Python ve C için Python 3.9+ (ctypes), Go 1.21+, Ruby 3+, JDK 17+
(`javac` ve `java`). Node üzerinde `profile` için Node 24+ gerekir; bu sürüm `NODE_OPTIONS` içinde `--cpu-prof`'a izin verir. C için ayrıca bir C derleyicisi gerekir. wasm köprüleri ayrıca `rustup target add wasm32-unknown-unknown` ve
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked` ister; tam
sürümü `build` söyler.

## Windows

Yerel Windows CI, `x86_64-pc-windows-msvc` Rust hedefini kullanır. Kararlı MSVC
toolchain'ini ve **Desktop development with C++** iş yüküyle Visual Studio Build Tools'u kurun;
C derleyicisi ve linker, C köprüleri ve yerel Rust eklentileri için gereklidir. Taşıdığınız kaynak
dilin runtime'ını kurun. Python, `python3` ya da `python` olarak bulunabilir;
everything-rust ikisini de algılar. GNU Windows hedefi CI tarafından test edilmez.

Yerel köprü entegrasyon test takımını bir Developer PowerShell isteminden çalıştırın:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

Test takımı, gerçek DLL tabanlı Node ve Python eklentilerini ve `PATH` üzerinde bulunan her runtime
için desteklenen yerel köprüleri derler ve yükler. Eksik isteğe bağlı runtime'lar atlandı olarak
raporlanır; Cargo, etkin Rust standart kütüphanesi ve MSVC linker'ı zorunludur. Cargo `std`'yi
bulamadığını bildirirse etkin hedefi önce
`rustup target remove x86_64-pc-windows-msvc`, ardından
`rustup target add x86_64-pc-windows-msvc` ile yeniden kurun. Linkleme başarısız olursa bir Visual Studio Developer
PowerShell'inden çalıştırın ve `where.exe link`'in MSVC linker'ını bulduğunu doğrulayın. Testler,
boşluk ve Unicode karakter içeren bir yol dahil geçici klasörler kullanır ve başarılı bir
çalıştırmadan sonra üretilen dosyaları siler.

Ajanlar için `everything-rust setup` her şeyi kaydeder (bkz. Hızlı başlangıç). MCP sunucusunu bunun yerine
tek bir projeye elle eklemek için (`.mcp.json`):

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

Her araç, parametreleri ve neye dokunabileceği: [docs/tools.md](/docs/reference/mcp-tools). Sunucu ayrıca
skill'den üretilmiş iki prompt sunar: `port_unit` ve `port_whole_app`.
## Güncelleme veya kaldırma

```sh
cargo install everything-rust                                    # güncelle (Cargo)
npm install -g everything-rust@latest                            # güncelle (npm)
everything-rust setup                                            # ajan ayarlarını yenile
everything-rust setup --remove && cargo uninstall everything-rust  # kaldır (Cargo)
everything-rust setup --remove && npm uninstall -g everything-rust # kaldır (npm)
```

`setup --remove` yalnızca everything-rust'in girdilerini kaldırır; kaydettiği kopyalar
(`<file>.everything-rust.bak`) siz silene kadar kalır.
