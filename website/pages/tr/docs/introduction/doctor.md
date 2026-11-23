---
title: Doctor
description: everything-rust doctor dil runtime'larını, ajan kurulumunu ve MCP sunucusunu kontrol eder ve her sorunu düzelten komutu gösterir.
---

# Doctor

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor` dil runtime'larını, ajan kurulumunu ve MCP sunucusunu kontrol eder ve her sorunu düzelten komutu yazdırır. Gerekli bir şey eksik olduğunda 1 koduyla çıkar.

Bir runtime, ajan kurulumu ya da MCP sunucusu eksik veya güncel olmadığı için bir adım başarısız olduğunda bunu çalıştırın. İş akışı skill'i de ajanlara aynısını yapmalarını söyler.

## Her dilin ihtiyaçları

`record`, `diff` ve `bench` orijinali kendi runtime'ında çalıştırır: TS için Node 22.6+ (`.ts` kaynaklarını doğrudan çalıştırır), Python ve C için Python 3.9+ (ctypes), Go 1.21+, Ruby 3+, JDK 17+ (`javac` ve `java`). Node üzerinde `profile` için Node 24+ gerekir; bu sürüm `NODE_OPTIONS` içinde `--cpu-prof`'a izin verir. C için ayrıca bir C derleyicisi gerekir. wasm köprüleri ayrıca `rustup target add wasm32-unknown-unknown` ve buna uyan `wasm-bindgen-cli` ister; tam sürümü `build` söyler.

## Katkıda bulunanlar için

`cargo run -- doctor`, hangi runtime'lara sahip olduğunuzu, sürümlerini ve her birini kuran ya da güncelleyen komutu listeler. Bkz. [Katkıda bulunma](/docs/project/contributing).
