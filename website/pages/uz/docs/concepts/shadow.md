---
title: Shadow
description: shadow asl nusxani va Rust'ni yonma-yon ishga tushiradigan va nomuvofiqliklarni xabar qiladigan o'rinbosar modulni generatsiya qiladi.
---

# Shadow

```sh
everything-rust shadow <crate>
```

`shadow` asl API'ga ega o'rinbosarni generatsiya qiladi: TS yoki Python moduli, Go paketi, Ruby fayli, Java klasslari yoki C fayli. Uni ilova asl nusxani yuklagan joyda yuklang. U ikkalasini ishga tushiradi, asl nusxa natijasini qaytaradi va production'da ishonch hosil qilayotganingizda Rust nomuvofiqliklarini xabar qiladi.

| `EVERYTHING_RUST_MODE` | Xatti-harakat |
| --- | --- |
| `shadow` (standart) | ikkalasini ishga tushiradi, asl nusxa natijasini qaytaradi, nomuvofiqliklarni xabar qiladi |
| `new` | Rust'ga o'tadi |
| `old` | Rust'ni o'chiradi |

Ommaviy API o'zgarmaydi. Chaqiruvchilar xuddi shu funksiyalarni yoki ularning o'rniga shadow modulini import qiladi.
