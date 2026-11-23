---
title: Java
description: public static metodlardan iborat paketni asl metodlarga ega wrapper klasslarda tiplangan JNI va generatsiya qilingan binary wire orqali Rust'ga port qiling.
---

# Java

| | |
| --- | --- |
| Birlik | `public static` metodlardan iborat paket |
| Ko'prik | asl metodlarga ega wrapper klasslarda tiplangan JNI va generatsiya qilingan binary wire |
| Yozib oluvchi va qayta ijro etuvchi | generatsiya qilingan reflection drayveri |
| Talablar | JDK 17+ (`javac` va `java`) |
| Holat | Fasad ortidagi JDK 21'da tekshirilgan: 4,492/4,492 (226 tasi exception tashlaydi), gate o'tadi |

Batch'lar `new FnBatch(items)`; ish tugagach ularni yoping. `profile` JFR'dan foydalanadi.

## Port nimani to'g'ri qilishi kerak

- `Integer.parseInt` `Character.digit` orqali istalgan Unicode o'nlik raqamini (`١٢`, `２`) qabul qiladi.
- `Double.toString` (JDK 19+) eng qisqa o'nlik ko'rinishni chiqaradi, lekin hech qachon ikki raqamdan kam emas: `4.9E-324`, Rust'ning eng qisqa shakli esa `5e-324`.
- `\s` faqat ASCII; Rust'ning `split_whitespace` U+00A0 da ham bo'ladi.
- Float'lar borib-qaytishdan o'zgarmay chiqishi kerak: `serde_json` ning standart float parser'i har doim ham to'g'ri yaxlitlamaydi, shuning uchun everything-rust va u generatsiya qiladigan har bir crate uning `float_roundtrip` imkoniyatidan foydalanadi.

## Tezlik

JIT'dan keyin oxirgi (leaf) funksiyalar bir necha nanosoniya oladi, string'lar bilan JNI chaqiruvi esa bundan qimmatroq. JDK fasadida `bench` portni har bir chaqiruvda 0.18x deb o'lchadi: kattaroq birliklarni port qiling. 4 yadroda batch sifatida 50,000 string ustida `slugify` (Java'da 43 ms) **3x tezroq** ishladi (har bir chaqiruvli port: 0.68x).

Batafsil: [Olti til](/docs/real-world/six-languages).
