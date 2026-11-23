---
title: Durum
description: everything-rust'te bugün neyin çalıştığı ve her parçanın en son nasıl kontrol edildiği.
---

# Durum

Neyin çalıştığı ve her parçanın en son nasıl kontrol edildiği. Güncelleme: 2026-11-18.

**Doğrulandı**: aşağıda adı geçen, başkasının yazdığı gerçek kod üzerinde uçtan uca (bridge, record, port,
diff, gate) çalıştırıldı. **Test edildi**: bu deponun testleri tarafından, onlar için yazılmış fixture'lar
üzerinde kapsanıyor, henüz gerçek bir projede çalıştırılmadı. **Önizleme**: anlatıldığı gibi, akışın bir
kısmı için çalışır.

## Birimleri port etmek

| Dil | Durum | Son gerçek çalıştırma | Sonuç |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Doğrulandı | [Prettier](https://github.com/marprojector/everything-rust/blob/main/docs/real-world.md#prettier) (2026-11-18), [json-server](/docs/real-world/json-server) (2026-11-18), [Fuse.js](/docs/real-world/fuse-js) (2026-11-18), [vercel/ms](/docs/real-world/vercel-ms) (2026-11-18), x18n'in `packages/core`'u | Prettier: `doc-printer-rs` 2,848/2,848 ve `js-format-rs` 1,279/1,279, ikisinde de gate geçiyor; json-server: altı birim, 3,399 vaka, her birinde gate geçiyor; Fuse.js, vercel/ms, x18n: 3,414/3,414, 975/975 ve 9,000+ vaka eşleşiyor |
| Python | Doğrulandı | CPython 3.13'ten `colorsys.py` (2026-11-16) | 2,400/2,400; gate geçiyor |
| Go | Doğrulandı | Go 1.24'ten `path` (2026-11-16) | 2,642/2,642; gate geçiyor |
| Ruby | Doğrulandı | Ruby 3.3'ten `shellwords.rb` (2026-11-16) | 1,617/1,617; gate geçiyor |
| Java | Doğrulandı | Bir cephe arkasında JDK 21 (2026-11-16) | 4,492/4,492; gate geçiyor |
| C | Doğrulandı | Bir cephe arkasında glibc (2026-11-16) | Tanımsız davranışlı bir vaka çıkarıldıktan sonra 1,859/1,859; gate geçiyor |
| PHP | Önizleme | yok | yalnızca `analyze` ve tüm uygulamalar; birimler henüz köprülenemez |
| C# | Önizleme | yok | yalnızca `analyze` ve tüm uygulamalar; birimler henüz köprülenemez |

Her çalıştırmanın ayrıntıları: [real-world.md](/docs/real-world/overview) (TS/JS) ve
[real-world-languages.md](/docs/real-world/six-languages) (diğerleri).

## Komutlar

| Komut | Durum | Notlar |
| --- | --- | --- |
| `analyze` | Doğrulandı | yukarıdaki her dil, x18n'in 142 birimi |
| `bridge`, `record`, `diff`, `gate`, `build` | Doğrulandı | yukarıdaki çalıştırmalar; json-server'ın `paginate`'i üzerinde `bridge --as-json` |
| `bench` | Doğrulandı | yukarıdaki çalıştırmalar; sayılar [real-world.md](/docs/real-world/overview) içinde |
| `record --from-tests` | Doğrulandı (TS/JS), Test edildi (Python) | vercel/ms'in Jest test takımı |
| `record` ve `gate` içinde orijinalin kapsamı | Doğrulandı (TS/JS) | Fuse.js: vakalar import ettiği satırların %81'ini çalıştırıyor; diğer diller bunu henüz ölçmüyor |
| `--batch` | Doğrulandı | Fuse.js, vercel/ms, x18n araması |
| wasm köprüsü (`--kind wasm`) | Doğrulandı | x18n araması |
| `shadow` | Doğrulandı | x18n'in `packages/core`'u |
| `profile` | Test edildi | |
| `app record`, `app replay` | Doğrulandı (sunucular ve CLI'lar) | json-server: `--fresh`, `--arg`, `--pace` ile 514 HTTP vakası; Prettier: `--inputs` ile 3,080 CLI vakası; `tests/app.rs` içindeki CLI'lar |
| Front door (`front-door`), `wire` | Doğrulandı (TS sunucusu ve CLI) | Prettier: bir birim bağlandı, %57.8 fallback ile 3,080/3,080, yapıştırıcı kod vaka başına `original::run_with_stdin` ile geri çeviriyor; json-server: önce okumalar bağlandı (%22.6 fallback), ardından her route (%0); `tests/http.rs` içinde sekiz dilde sunucular |
| `lift` | Doğrulandı (TS sunucusu), Test edildi (CLI'lar) | json-server: saf Rust binary'si, 514/514; `tests/app.rs` içinde sekiz dilin hepsinde CLI'lar |
| MCP sunucusu (`mcp`) | Test edildi | `src/mcp.rs` içinde protokol testi |

## Testler nerede çalışır

CI; macOS ve Windows üzerinde Node, Python, Go, Ruby ve Java kurar. PHP veya .NET kurmaz, bu yüzden
PHP ve C# testleri orada atlanır ve yalnızca bunların kurulu olduğu bir makinede çalışır.
