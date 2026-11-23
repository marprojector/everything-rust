---
title: Front door va wire
description: app front-door asl nusxaning oldiga Rust binary'sini qo'yadi; app wire birlik porti gate'dan o'tgach, buyruqlarni unga topshiradi.
---

# Front door va wire

Avval kirish nuqtasini Rust'ga ko'chiring, so'ng unga buyruqlarni birma-bir birlik bo'yicha topshiring:

```sh
everything-rust app front-door .                            # rust/app: har bir buyruq asl nusxani ishga tushiradi
everything-rust app wire . textkit --command upper          # textkit-rs gate'dan o'tgandan keyin
# bog'lovchi kodni rust/app/src/units/textkit.rs ga yozing, so'ng:
everything-rust app wire . textkit                          # yana 100%, fallback ulushi pastroq
```

## Front door

`app front-door` `rust/app` ni yaratadi, bu ilova nomi bilan atalgan Rust binary'si: yozib olingan holatlardagi buyruqlar va flag'lar uchun clap parser'i, unda har bir buyruq asl nusxani xuddi shu argv, stdin va muhit bilan ishga tushiradi hamda uning chiqishi va exit kodini o'tkazib yuboradi. Har bir fallback sanaladi, shuning uchun yangi front door'ni qayta ijro etish 100% fallback bilan 100% o'tish beradi (u buni ko'rsatish uchun build qiladi va qayta ijro etadi). U hech qachon `rust/app` ni qayta yozmaydi: shundan keyin kod sizniki.

## Birlikni ulash

`app wire <repo> <unit> --command <name>`:

1. birlik crate'ida `gate` ni ishga tushiradi (agar u hozirgina o'tgan bo'lsa, `--skip-gate`);
2. uni `rust/app` ga kutubxona sifatida qo'shadi;
3. siz yoki agentingiz to'ldirishi uchun `src/units/<unit>.rs` ni bir marta yozadi: argumentlarni parse qilish, `<crate>::port` ni chaqirish, asl nusxa chiqaradigan narsani chiqarish;
4. buyruqlarni unga yo'naltiradi, so'ng build qiladi va qayta ijro etadi.

Bog'lovchi kodni yozgandan keyin uni yana ishga tushiring: 100% o'tish va pastroq fallback ulushi. `EVERYTHING_RUST_UNIT_<NAME>=original` ulangan birlikni lift'gacha asl nusxaga qaytaradi.

Hech qanday buyruq nomlamaydigan argv uchun (argumentsiz, noma'lum buyruqlar, foydalanish xatolari) ilovaning o'z nomini buyruq sifatida ulang (`--command mytool`).

## Hech narsa asl nusxaga qayta murojaat qilmaydi

Front door asl nusxani alohida jarayon sifatida ishga tushiradi (yoki HTTP'ni unga uzatadi), ko'prik orqali ulangan birliklarni asl nusxa chaqiradi va hech qachon aksincha emas, callback'lar esa manba tilida qoladi.

Serverlar buyruqlar o'rniga marshrutlardan foydalanadi: [Serverlar](/docs/whole-apps/servers) sahifasiga qarang. Kutubxonalar o'zlarining ommaviy API'sidan foydalanadi: [Kutubxonalar](/docs/whole-apps/libraries) sahifasiga qarang.
