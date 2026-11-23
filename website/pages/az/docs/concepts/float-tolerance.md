---
title: Float tolerantlığı
description: Rəqəmsal kod port edildikdən sonra nadir hallarda bit-bit eyni float-lar verir. --float-tolerance float-ların böyük dəyərin müəyyən payı daxilində uyğun gəlməsinə icazə verir.
---

# Float tolerantlığı

```sh
everything-rust record <crate> --float-tolerance 1e-9
everything-rust app record . --float-tolerance 1e-9 -- python3 cli.py
```

Rəqəmsal kod (fizika, statistika, qrafika) port edildikdən sonra nadir hallarda bit-bit eyni float-lar verir: əməliyyat ardıcıllığı, fused multiply-add və riyazi kitabxana kompilyatorlar arasında fərqlənir. Bunu qəbul edirsinizsə, `--float-tolerance` və öz rəqəminizlə qeydə alın. Bundan sonra float-lar böyük dəyərin həmin payı daxilində uyğun gəlir (1-dən kiçik dəyərlərdə mütləq), `diff` və `gate` isə ondan istifadə edir. `0` onu ləğv edir.

Hələ də dəqiq uyğun gəlməli olanlar:

- tam ədədlər, mətn və struktur;
- fərqli yazılmış eyni dəyər (`3` və `3.0`);
- formatlama. Rəqəmdən mətnə formatlama dillər arasında fərqlənir (`1e21`, `-0`, rəqəm sayları); tələlər bu dəyərləri əhatə edir, `--float-tolerance` isə yalnız yuvarlaqlaşdırmanı bağışlayır, formatlamanı heç vaxt.

Uyğunsuzluğu aradan qaldırmaq üçün tolerantlığı heç vaxt genişləndirməyin: ondan kənar fərq portdakı xətadır.
