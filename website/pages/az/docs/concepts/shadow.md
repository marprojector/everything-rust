---
title: Shadow
description: shadow orijinalı və Rust-u yan-yana işə salan və uyğunsuzluqları bildirən drop-in modul yaradır.
---

# Shadow

```sh
everything-rust shadow <crate>
```

`shadow` orijinalın API-si ilə drop-in yaradır: TS və ya Python modulu, Go paketi, Ruby faylı, Java sinifləri və ya C faylı. Onu tətbiqin orijinalı yüklədiyi yerdə yükləyin. O, hər ikisini işə salır, orijinalın nəticəsini qaytarır və siz production-da inam qazanarkən Rust uyğunsuzluqlarını bildirir.

| `EVERYTHING_RUST_MODE` | Davranış |
| --- | --- |
| `shadow` (standart) | hər ikisini işə salır, orijinalın nəticəsini qaytarır, uyğunsuzluqları bildirir |
| `new` | Rust-a keçir |
| `old` | Rust-u söndürür |

Açıq API dəyişmir. Çağıranlar eyni funksiyaları və ya onların yerinə shadow modulunu import edir.
