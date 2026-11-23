---
title: Shadow
description: shadow, orijinali ve Rust'ı yan yana çalıştıran ve uyuşmazlıkları raporlayan, doğrudan yerine geçen bir modül üretir.
---

# Shadow

```sh
everything-rust shadow <crate>
```

`shadow`, orijinalin API'sine sahip, doğrudan yerine geçen (drop-in) bir modül üretir: bir TS veya Python modülü, bir Go paketi, bir Ruby dosyası, Java sınıfları ya da bir C dosyası. Uygulamanın orijinali yüklediği yerde onu yükleyin. İkisini de çalıştırır, orijinalin sonucunu döndürür ve siz production'da güven kazanırken Rust uyuşmazlıklarını raporlar.

| `EVERYTHING_RUST_MODE` | Davranış |
| --- | --- |
| `shadow` (varsayılan) | ikisini de çalıştırır, orijinalin sonucunu döndürür, uyuşmazlıkları raporlar |
| `new` | Rust'a geçer |
| `old` | Rust'ı kapatır |

Public API değişmez. Çağıranlar aynı fonksiyonları ya da onların yerine shadow modülünü import eder.
