---
title: Float tolerantligi
description: Sonli kod port qilingach kamdan-kam hollarda bit darajasida bir xil float'lar beradi. --float-tolerance float'larga kattaroq qiymatning ma'lum ulushi doirasida mos kelishga ruxsat beradi.
---

# Float tolerantligi

```sh
everything-rust record <crate> --float-tolerance 1e-9
everything-rust app record . --float-tolerance 1e-9 -- python3 cli.py
```

Sonli kod (fizika, statistika, grafika) port qilingach kamdan-kam hollarda bit darajasida bir xil float'lar beradi: amallar tartibi, fused multiply-add va matematik kutubxona kompilyatorlar orasida farq qiladi. Buni qabul qilsangiz, `--float-tolerance` va o'z qiymatingiz bilan yozib oling. Shunda float'lar kattaroq qiymatning shu ulushi doirasida mos keladi (1 dan kichik qiymatlar uchun mutlaq), `diff` va `gate` esa undan foydalanadi. `0` uni olib tashlaydi.

Hali ham aniq mos kelishi kerak bo'lgan narsalar:

- butun sonlar, matn va tuzilma;
- boshqacha yozilgan bir xil qiymat (`3` va `3.0`);
- formatlash. Sonni matnga formatlash tillar orasida farq qiladi (`1e21`, `-0`, raqamlar soni); tuzoqlar bu qiymatlarni o'z ichiga oladi, `--float-tolerance` esa faqat yaxlitlashni kechiradi, formatlashni hech qachon.

Nomuvofiqlikdan qutulish uchun hech qachon tolerantlikni kengaytirmang: undan katta farq portdagi xatodir.
