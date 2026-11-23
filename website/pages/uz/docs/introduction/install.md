---
title: O'rnatish
description: everything-rust'ni crates.io, npm yoki release binary'sidan o'rnating, shuningdek har bir tilga kerak bo'lgan runtime'lar.
---

# O'rnatish

## crates.io, npm yoki release'dan

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

npm JavaScript loyihalari uchun xuddi shu binary'ni o'rnatadi.

macOS, Linux va Windows uchun tayyor binary fayllar har bir [GitHub release](https://github.com/marprojector/everything-rust/releases)'ga biriktirilgan. Claude Code'da uni plugin sifatida ham o'rnatishingiz mumkin (MCP server va skill; CLI `PATH` ichida bo'lishi kerak):

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## Manba kodidan

```sh
cargo install --path .
```

`record`, `diff` va `bench` asl nusxani o'z runtime'ida ishga tushiradi: TS uchun Node 22.6+ (u `.ts`
manbalarini to'g'ridan-to'g'ri ishga tushiradi), Python va C (ctypes) uchun Python 3.9+, Go 1.21+, Ruby 3+, JDK 17+
(`javac` va `java`). Node'da `profile` uchun Node 24+ kerak, u `NODE_OPTIONS` ichida `--cpu-prof` ga ruxsat beradi. C uchun C kompilyatori ham kerak. wasm ko'priklari uchun yana `rustup target add wasm32-unknown-unknown` va
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked` kerak; aniq versiyani
`build` aytadi.

## Windows

Windows'dagi native CI `x86_64-pc-windows-msvc` Rust target'idan foydalanadi. Barqaror MSVC
toolchain'ini va **Desktop development with C++** workload'i bilan Visual Studio Build Tools'ni o'rnating;
C kompilyatori va linker C ko'priklari hamda native Rust kengaytmalari uchun zarur. Ko'chirayotgan
manba tilingiz uchun runtime'ni o'rnating. Python `python3` yoki `python` sifatida mavjud bo'lishi
mumkin; everything-rust ikkalasini ham aniqlaydi. GNU Windows target'i CI'da sinovdan o'tkazilmaydi.

Native ko'prik integratsiya testlar to'plamini Developer PowerShell oynasidan ishga tushiring:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

To'plam haqiqiy DLL asosidagi Node va Python kengaytmalarini hamda `PATH` da mavjud har bir runtime uchun
qo'llab-quvvatlanadigan native ko'priklarni build qiladi va yuklaydi. Yo'q bo'lgan ixtiyoriy runtime'lar
o'tkazib yuborilgan deb ko'rsatiladi; Cargo, faol Rust standart kutubxonasi va MSVC linker zarur. Agar Cargo
`std` ni topa olmayotganini xabar qilsa, faol target'ni
`rustup target remove x86_64-pc-windows-msvc` va undan keyin
`rustup target add x86_64-pc-windows-msvc` bilan qayta o'rnating. Agar linklash muvaffaqiyatsiz bo'lsa, Visual Studio Developer
PowerShell'dan ishga tushiring va `where.exe link` MSVC linker'ini topishini tekshiring. Testlar vaqtinchalik papkalardan,
jumladan bo'sh joy va Unicode belgilar bor yo'ldan foydalanadi hamda muvaffaqiyatli ishga tushirishdan keyin
generatsiya qilingan artefaktlarni o'chiradi.

Agentlar uchun `everything-rust setup` hammasini ro'yxatdan o'tkazadi (Tezkor boshlashga qarang). MCP serverni
buning o'rniga bitta loyihaga qo'lda qo'shish uchun (`.mcp.json`):

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

Har bir vosita, uning parametrlari va u nimalarga tegishi mumkinligi: [docs/tools.md](/docs/reference/mcp-tools). Server
skill asosida tuzilgan ikkita prompt ham taklif qiladi: `port_unit` va `port_whole_app`.
## Yangilash yoki o'chirish

```sh
cargo install everything-rust                                    # yangilash (Cargo)
npm install -g everything-rust@latest                            # yangilash (npm)
everything-rust setup                                            # agent sozlamalarini yangilash
everything-rust setup --remove && cargo uninstall everything-rust  # o'chirish (Cargo)
everything-rust setup --remove && npm uninstall -g everything-rust # o'chirish (npm)
```

`setup --remove` faqat everything-rust yozuvlarini olib tashlaydi; u saqlagan nusxalar
(`<file>.everything-rust.bak`) siz ularni o'chirmaguningizcha qoladi.
