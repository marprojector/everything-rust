---
title: Yozib olish va qayta ijro
description: app record butun ilovani uning hujjatlashtirilgan misollari, sizning kirish ma'lumotlaringiz va tuzoqlar ustida ishga tushiradi; app replay istalgan build'ni holatlarga qarshi ishga tushiradi.
---

# Yozib olish va qayta ijro

`record` va `diff` bitta birlikni isbotlaydi; `app record` va `app replay` esa oltita tilning istalganidagi CLI'lar va serverlar uchun ilovani isbotlaydi. Ilova qora quti hisoblanadi.

```sh
everything-rust app record . -- python3 cli.py          # asl nusxani ishga tushiradigan buyruq
everything-rust app replay .                            # asl nusxa 100% o'tishi kerak
everything-rust app replay . --build "target/release/mytool"
```

## Holatlar qayerdan keladi

`app record` asl nusxani quyidagilar ustida ishga tushiradi:

- ilovaning o'z hujjatlashtirilgan misollari: uning Markdown va shell fayllaridagi `$ mytool ...` qatorlari, ular nomlagan fayllar bilan;
- `--inputs` JSONL fayli, har qatorda bitta `{"name", "args", "stdin", "env", "files"}`;
- shu holatlardan olingan semantik tuzoqlar: ASCII bo'lmagan va emoji'li string'lar, manfiy bo'lish va qoldiq, to'lib ketish, katta va juda kichik float'lar, map tartibi, yo'q fayllar.

Har bir holat ikki marta ishga tushiriladi; ikki ishga tushirish orasida farq qilgan narsa `[..]` wildcard'iga aylanadi. `--normalize timestamps|uuids|key-order` yoki `--normalize name=regex` holatlar bilan birga saqlanadigan normalizatorlar qo'shadi. `app record` nima farq qilganini turi bo'yicha xabar qiladi: qiymatlar (endi wildcard'lar), tartib (qatorlar istalgan tartibda solishtiriladi) va qatorlar (istalgan narsaga mos keladigan `...`). Har birini ko'rib chiqing.

## Holatlar oddiy fayllar

Holatlar `rust/app-cases/` dagi [trycmd](https://docs.rs/trycmd) fayllari:

| Fayl | Nimani saqlaydi |
| --- | --- |
| `<case>.toml` | argv, env, exit kodi |
| `<case>.stdin` | standart kirish |
| `<case>.stdout`, `<case>.stderr` | asl nusxa chiqargan narsa |
| `<case>.in/` | kiruvchi fayllar |
| `<case>.out/` | yozilgan fayllar |

Ularni qo'lda o'qing va tahrirlang; port'da binary paydo bo'lgach, Rust testi ularni `trycmd` bilan ishga tushirishi mumkin.

## Qayta ijro

- `app replay` istalgan build'ni holatlarga qarshi ishga tushiradi, `passed/total` ni chiqaradi, har bir muvaffaqiyatsiz holat uchun diff bilan `rust/app-cases/replay-report.md` ni yozadi va har qanday nomuvofiqlikda noldan farqli kod bilan tugaydi.
- [Front door](/docs/whole-apps/front-door) kabi `EVERYTHING_RUST_FALLBACK_LOG` nomlagan faylga yozib boradigan build (asl nusxaga har bir fallback uchun bitta qator) o'zining fallback ulushini ham hisobotda oladi.
- `app replay --case <name>` hisobot bergan nom bo'yicha bitta holatni qayta ishga tushiradi: hammasini qayta ishga tushirishdan oldin nomuvofiqlikni tuzatish uchun tez va takrorlanadigan sikl.

## CI'da qayta ijro

Hisobotda vaqt o'lchovlari yoki vaqtinchalik yo'llar yo'q: bir xil build'ning ikki marta qayta ijrosi bir xil faylni yozadi. Uni commit qiling, so'ng CI'da ishga tushiring:

```sh
everything-rust app replay .
git diff --exit-code rust/app-cases/replay-report.md
```

Shunda yarim port qilingan ilova nima o'tishidagi har qanday o'zgarishda, yaxshi yoki yomon, hisobot u bilan yangilanmaguncha CI'dan o'tmaydi.

## Sonli chiqish

`app record --float-tolerance 1e-9` sonlarga kattaroq qiymatning shu ulushi doirasida mos kelishga ruxsat beradi. [Float tolerantligi](/docs/concepts/float-tolerance) sahifasiga qarang.
