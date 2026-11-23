---
title: PHP (preview)
description: PHP preview holatida. analyze PHP ilovasini rejalashtiradi, PHP CLI yoki serverini esa yozib olish, qayta ijro etish va Rust front door'i ortiga qo'yish mumkin.
---

# PHP (preview)

| | |
| --- | --- |
| Birlik | funksiyalar va `public static` metodlardan iborat papka |
| Ko'prik | navbatda: C ABI orqali PHP FFI |
| Bugun | butun ilovalar: CLI'lar va serverlar (`php -S`) uchun `app record`, `app replay` va front door |
| Holat | Preview: faqat `analyze` va butun ilovalar; birliklarni hozircha ko'prik orqali ulab bo'lmaydi |

`analyze` PHP ilovasini rejalashtiradi: Composer paketlari va PSR-4 autoloading, erkin taqqoslash va butun sonning float'ga to'lib o'tishi kabi xavflar hamda crate'larga moslangan Composer paketlari. Tiplar PHP tiplari va phpdoc'dan olinadi.

PHP CLI yoki serverini yozib olish, qayta ijro etish va Rust front door'i ortiga qo'yish mumkin:

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

Uning birliklarini PHP FFI orqali Rust'ga port qilish keyingi qadam ([issue #2](https://github.com/marprojector/everything-rust/issues/2)). Ungacha `bridge` PHP birliklarini aniq xabar bilan rad etadi.

CI PHP'ni o'rnatmaydi, shuning uchun PHP testlari faqat u o'rnatilgan mashinada ishlaydi.
