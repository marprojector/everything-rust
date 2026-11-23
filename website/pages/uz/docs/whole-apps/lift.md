---
title: Lift
description: Qayta ijro 0 fallback ulushi bilan o'tgach va har bir birlik gate'dan o'tgach, app lift rust/app'ni toza Rust ilovasiga aylantiradi.
---

# Lift

Qayta ijro 0 fallback ulushi bilan o'tganda va har bir birlik gate'dan o'tganda, uni toza Rust ilovasiga aylantiring:

```sh
everything-rust app lift .
```

- `app lift` ikkala shart bajarilmaguncha rad etadi va nima to'sqinlik qilayotganini aytadi: hali fallback'ga tushayotgan holatlar, hali stub bo'lib turgan bog'lovchi kod, gate'dan o'tmayotgan birliklar.
- Avval nusxada u fallback yo'lini va o'chirish kalitlarini olib tashlaydi, har bir birlik crate'ini `rust/app` ning oddiy kutubxonasiga aylantiradi (`cdylib`, binding'lar yoki generatsiya qilingan wrapper'larsiz) va `rust/app/tests/app_cases.rs` ni qo'shadi, u `cargo test` da trycmd bilan `rust/app-cases` ni ishga tushiradi, shunda CI'ga cargo'dan boshqa hech narsa kerak bo'lmaydi.
- So'ng u lift nazorat ro'yxatini bajaradi: har bir birlik va `rust/app` da fmt, clippy `-D warnings` va testlar, `rust/DECISIONS.md` da nomlangan har bir target o'rnatilgan joyda build qilinadi yoki tekshiriladi, ishga tushish vaqtida o'qiladigan fayllar va asset papkalari ichiga joylanmagan, toza binary'da qayta ijro. Faqat bularning hammasi o'tgandagina o'zgarishlar qayta yoziladi. Asl nusxaga hech qachon tegilmaydi.

## Lift'dan keyin

Birliklarda endi binding'lar yo'q. json-server'da har bir crate'ning `tests/recorded.rs` i o'zining yozib olingan holatlarini portda qayta ijro etadi: 514 ta HTTP holat bilan birga barcha 3,399 tasi `cargo test` bilan ishlaydi.

Asl nusxa yo'qolgach, Rust kodini erkin qayta tuzing; `app replay` xatti-harakatni baribir isbotlaydi.

## So'ng o'tish (cutover)

Workflow skill'i agentga qo'lidan kelganini qildiradi va `rust/cutover.sh` ni yozdiradi, bu faqat siz bajara oladigan qadamlar (deploy'lar, DNS, maxfiy kalitlar) uchun interaktiv skript bo'lib, har bir qadamda orqaga qaytish yo'li bor.
