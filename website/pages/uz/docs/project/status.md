---
title: Holat
description: everything-rust'da bugun nimalar ishlaydi va har bir qism oxirgi marta qanday tekshirilgan.
---

# Holat

Nimalar ishlaydi va har bir qism oxirgi marta qanday tekshirilgan. Yangilangan sana: 2026-11-18.

**Tekshirilgan (Verified)**: quyida nomlangan, boshqa birov yozgan haqiqiy kodda boshidan oxirigacha (bridge, record, port, diff, gate)
ishga tushirilgan. **Sinovdan o'tgan (Tested)**: bu repozitoriy testlari ular uchun yozilgan fixture'larda qamrab olgan, hali
haqiqiy loyihada ishga tushirilmagan. **Preview**: tavsiflanganidek, jarayonning bir qismi uchun ishlaydi.

## Birliklarni port qilish

| Til | Holat | Oxirgi haqiqiy ishga tushirish | Natija |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Tekshirilgan | [Prettier](https://github.com/marprojector/everything-rust/blob/main/docs/real-world.md#prettier) (2026-11-18), [json-server](/docs/real-world/json-server) (2026-11-18), [Fuse.js](/docs/real-world/fuse-js) (2026-11-18), [vercel/ms](/docs/real-world/vercel-ms) (2026-11-18), x18n'ning `packages/core` | Prettier: `doc-printer-rs` 2,848/2,848 va `js-format-rs` 1,279/1,279, gate ikkalasida ham o'tadi; json-server: oltita birlik, 3,399 holat, gate har birida o'tadi; Fuse.js, vercel/ms, x18n: 3,414/3,414, 975/975 va 9,000+ holat mos keladi |
| Python | Tekshirilgan | CPython 3.13'dagi `colorsys.py` (2026-11-16) | 2,400/2,400; gate o'tadi |
| Go | Tekshirilgan | Go 1.24'dagi `path` (2026-11-16) | 2,642/2,642; gate o'tadi |
| Ruby | Tekshirilgan | Ruby 3.3'dagi `shellwords.rb` (2026-11-16) | 1,617/1,617; gate o'tadi |
| Java | Tekshirilgan | Fasad ortidagi JDK 21 (2026-11-16) | 4,492/4,492; gate o'tadi |
| C | Tekshirilgan | Fasad ortidagi glibc (2026-11-16) | bitta aniqlanmagan xatti-harakat holati olib tashlangandan keyin 1,859/1,859; gate o'tadi |
| PHP | Preview | yo'q | faqat `analyze` va butun ilovalar; birliklarni hozircha ko'prik orqali ulab bo'lmaydi |
| C# | Preview | yo'q | faqat `analyze` va butun ilovalar; birliklarni hozircha ko'prik orqali ulab bo'lmaydi |

Har bir ishga tushirish tafsilotlari: [real-world.md](/docs/real-world/overview) (TS/JS) va
[real-world-languages.md](/docs/real-world/six-languages) (qolganlari).

## Buyruqlar

| Buyruq | Holat | Izohlar |
| --- | --- | --- |
| `analyze` | Tekshirilgan | yuqoridagi har bir til, x18n'ning 142 ta birligi |
| `bridge`, `record`, `diff`, `gate`, `build` | Tekshirilgan | yuqoridagi ishga tushirishlar; json-server'ning `paginate` funksiyasida `bridge --as-json` |
| `bench` | Tekshirilgan | yuqoridagi ishga tushirishlar; raqamlar [real-world.md](/docs/real-world/overview) da |
| `record --from-tests` | Tekshirilgan (TS/JS), Sinovdan o'tgan (Python) | vercel/ms'ning Jest to'plami |
| `record` va `gate` da asl nusxa qamrovi | Tekshirilgan (TS/JS) | Fuse.js: holatlar u import qiladigan qatorlarning 81% ini ishga tushiradi; boshqa tillar uni hozircha o'lchamaydi |
| `--batch` | Tekshirilgan | Fuse.js, vercel/ms, x18n qidiruvi |
| wasm ko'prigi (`--kind wasm`) | Tekshirilgan | x18n qidiruvi |
| `shadow` | Tekshirilgan | x18n'ning `packages/core` |
| `profile` | Sinovdan o'tgan | |
| `app record`, `app replay` | Tekshirilgan (serverlar va CLI'lar) | json-server: `--fresh`, `--arg`, `--pace` bilan 514 HTTP holat; Prettier: `--inputs` bilan 3,080 CLI holat; `tests/app.rs` dagi CLI'lar |
| Front door (`front-door`), `wire` | Tekshirilgan (TS server va CLI) | Prettier: bitta birlik ulangan, 57.8% fallback bilan 3,080/3,080, bog'lovchi kod `original::run_with_stdin` bilan har bir holat bo'yicha rad etadi; json-server: avval o'qishlar ulangan (22.6% fallback), so'ng har bir marshrut (0%); `tests/http.rs` da sakkiz tildagi serverlar |
| `lift` | Tekshirilgan (TS server), Sinovdan o'tgan (CLI'lar) | json-server: toza Rust binary'si, 514/514; `tests/app.rs` da barcha sakkiz tildagi CLI'lar |
| MCP server (`mcp`) | Sinovdan o'tgan | `src/mcp.rs` dagi protokol testi |

## Testlar qayerda ishlaydi

CI macOS va Windows'da Node, Python, Go, Ruby va Java'ni o'rnatadi. U PHP yoki
.NET'ni o'rnatmaydi, shuning uchun PHP va C# testlari u yerda o'tkazib yuboriladi va faqat ular o'rnatilgan mashinada ishlaydi.
