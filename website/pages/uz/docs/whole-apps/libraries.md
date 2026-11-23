---
title: Kutubxonalar
description: Kutubxonada qayta ijro etiladigan jarayon yo'q. Uning ommaviy API'si front door, har bir birlikning yozib olingan holatlari esa isbot.
---

# Kutubxonalar

Kutubxonada qayta ijro etiladigan jarayon yo'q: uning ommaviy API'si front door, har bir birlikning
yozib olingan holatlari esa isbot.

```sh
everything-rust app front-door .        # rust/app: ommaviy API'li Rust crate'i
# har bir birlik sikldan o'tadi: bridge, record, port.rs, build, diff, gate; so'ng:
everything-rust app wire . textkit-rs   # rust/app'dan qayta eksport qilinadi; asl nusxa uning shadow'ini import qiladi
everything-rust app replay .            # har bir ulangan birlik holatlari; fallback = hali asl nusxadagi eksportlar
everything-rust app lift .
```

- Repozitoriyda `main` ham, server ham bo'lmasa (aks holda `--library`), `app front-door` ommaviy
  API'ni (har bir birlik eksportlarini) topadi va `rust/app` ni kutubxona crate'i sifatida yaratadi. Har bir
  ulangan birlik uning moduliga aylanadi: `pub use textkit_rs::port as textkit;`.
- `app wire` `bridge` yaratgan crate'ni oladi, `gate` ni ishga tushiradi va birlikning shadow o'rinbosarini yozadi:
  asl paket birlik o'rniga uni import qiladi, shunda uning chaqiruvchilari asl API'ni saqlaydi va
  Rust'ga yetib boradi. `EVERYTHING_RUST_UNIT_<NAME>=original` birlikni orqaga qaytaradi (C: shadow
  flag'larisiz build qiling).
- `app replay` har bir ulangan birlikning yozib olingan holatlarini uning binding'lari orqali ishga tushiradi. Fallback ulushi
  Rust hali javob bermaydigan ommaviy eksportlar ulushi: ulanmagan birliklar va `bridge` asl tilda
  qoldiradigan eksportlar.
- `app lift` uchun fallback 0 bo'lishi va har bir birlik `gate` dan o'tishi kerak; u birliklardan binding'lar, wrapper'lar va
  shadow'larni olib tashlaydi hamda `rust/app` ni bitta toza Rust kutubxonasi sifatida qoldiradi. U hech qaysi
  `port.rs` ga tegmaydi, shuning uchun binding'lar orqali mos kelgan holatlar uni hali ham tavsiflaydi; nazorat ro'yxati
  asl nusxaning hali ham shadow'larni import qiladigan fayllari haqida ogohlantiradi.

Beshalasi ham MCP vositalari: `app_record`, `app_replay`, `app_front_door`, `app_wire` va
`app_lift`.
