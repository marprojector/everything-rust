---
title: Bench
description: bench yozib olingan holatlarni asl nusxada va Rust'da, ko'prik bilan birga, vaqt bo'yicha o'lchaydi va hukm chiqaradi. Rust har doim ham tezroq emas.
---

# Bench

```sh
everything-rust bench <crate>
```

`bench` yozib olingan holatlarni asl nusxada va build qilingan Rust modulida, ko'prik bilan birga, vaqt bo'yicha o'lchaydi hamda qatorlar va maslahatli hukmni qaytaradi: chiqarish, sekinroq funksiyalarni manba tilida qoldirish yoki kattaroq birlikni port qilish. 1 dan kichik tezlanish port chaqiruvchilari chaqirgandek sekinroq ekanini anglatadi. `--min-ms` har bir funksiya va tomon uchun minimal o'lchanadigan vaqtni belgilaydi (standart 200).

Hukmga amal qiling. Chaqiruvchilarni sekinlashtiradigan portni buni aytmasdan chiqarmang.

## Nega port sekinroq bo'lishi mumkin

Har bir element uchun bir marta chaqiriladigan port ko'prik narxini har bir element uchun bir marta to'laydi. Buni nima hal qiladi:

- **Interpretatsiya qilinadigan asl nusxalar (Python, Ruby)**: Rust'ga chaqiruv taxminan 100-300 ns turadi va Rust kodi interpretatordan ancha tez, shuning uchun hatto kichik funksiyalar ham yutadi.
- **Kompilyatsiya qilinadigan asl nusxalar (Go, JIT'dan keyin Java)**: asl oxirgi (leaf) funksiya 4-90 ns oladi; bitta cgo chaqiruvining o'zi taxminan 80 ns, string'lar bilan JNI chaqiruvi esa undan ko'p turadi. Bu tillar faqat bitta chaqiruv ko'p ish bajarganda yutadi: funksiyani uning chaqiruvchilari bilan birga port qiling.
- **C**: hech qanday o'tish yo'q; Rust kodi C kodi bilan to'g'ridan-to'g'ri solishtiriladi.

Haqiqiy kodda har bir chaqiruv bo'yicha o'lchangan ([olti til](/docs/real-world/six-languages)):

| Til | Rust asl nusxaga nisbatan (`bench`, geometrik o'rtacha) | Hukm |
| --- | --- | --- |
| Ruby | **4.4x tezroq** (har bir funksiya uchun 3.6-7x) | chiqarish |
| Python | **1.35x tezroq** (ko'pchilik chaqiruvlarda 1.5-1.9x) | chiqarish |
| C | 0.87x: ctype 1.1-1.4x tezroq, glibc'ning sozlangan `strtod`, `strspn`, `strcmp` funksiyalari 0.5-0.7x | optimallashtirish yoki sekinlarini qoldirish |
| Java | 0.18x | kattaroq birliklarni port qilish |
| Go | 0.04x | kattaroq birliklarni port qilish |

x18n yadrosining API o'zgarishisiz ko'chirilishi mumkin bo'lgan barcha 29 funksiyasi port qilindi: har bir gate o'tadi va 7,925 ta yozib olingan holat aynan mos keladi, `bench` esa portlar JS'dan chaqirilganda TS'dan sekinroq ekanini ko'rsatdi, chunki ko'prik ishning o'zidan qimmatroq ([hisobot](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-core/README.md)). vercel/ms yozib olingan holatlari ustida har bir chaqiruvda 0.56x ishladi ([amaliy misol](/docs/real-world/vercel-ms)).

## Nima yutadi: siklni ko'chirish

`profile` issiq funksiya atrofidagi siklni topadi, `--batch` esa butun siklni barcha yadrolarda Rust'ga ko'chiradi. [Batch'lar va hot path'lar](/docs/concepts/batches) sahifasiga qarang.

::: tip Tinch mashinada o'lchang
`bench` ni tinch mashinada, ikki marta ishga tushiring va har bir e'lon qilingan raqam yonida apparat va yadrolar sonini ko'rsating.
:::
