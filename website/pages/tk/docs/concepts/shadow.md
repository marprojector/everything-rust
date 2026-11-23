---
title: Shadow
description: shadow asyl nusgany we Rust-y ýanaşyk işledýän we gabat gelmezlikleri habar berýän çalşyp goýulýan modul döredýär.
---

# Shadow

```sh
everything-rust shadow <crate>
```

`shadow` asyl nusganyň API-si bilen çalşyp goýulýan modul döredýär: TS ýa-da Python moduly, Go paketi, Ruby faýly, Java klaslary ýa-da C faýly. Ony programmanyň asyl nusgany ýükleýän ýerinde ýükläň. Ol ikisini hem işledýär, asyl nusganyň netijesini gaýtarýar we siz production-da ynam toplaýan wagtyňyz Rust gabat gelmezliklerini habar berýär.

| `EVERYTHING_RUST_MODE` | Özüni alyp barşy |
| --- | --- |
| `shadow` (adaty) | ikisini hem işledýär, asyl nusganyň netijesini gaýtarýar, gabat gelmezlikleri habar berýär |
| `new` | Rust-a geçýär |
| `old` | Rust-y öçürýär |

Köpçülikleýin API üýtgemeýär. Çagyryjylar şol bir funksiýalary ýa-da olaryň ýerine shadow modulyny import edýär.
