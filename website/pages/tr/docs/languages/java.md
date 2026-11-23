---
title: Java
description: public static metotlardan oluşan bir paketi, orijinal metotlara sahip wrapper sınıflarda tipli JNI ve üretilmiş bir binary wire üzerinden Rust'a port edin.
---

# Java

| | |
| --- | --- |
| Birim | `public static` metotlardan oluşan bir paket |
| Köprü | orijinal metotlara sahip wrapper sınıflarda tipli JNI ve üretilmiş bir binary wire |
| Kaydeden ve yeniden oynatan | üretilmiş bir reflection sürücüsü |
| Gerekenler | JDK 17+ (`javac` ve `java`) |
| Durum | Bir cephe arkasında JDK 21 üzerinde doğrulandı: 4,492/4,492 (226'sı exception fırlatıyor), gate geçiyor |

Batch'ler `new FnBatch(items)` şeklindedir; işiniz bittiğinde onları kapatın. `profile` JFR kullanır.

## Bir portun doğru yapması gerekenler

- `Integer.parseInt`, `Character.digit` üzerinden herhangi bir Unicode ondalık rakamını (`١٢`, `２`) kabul eder.
- `Double.toString` (JDK 19+) en kısa ondalık gösterimi yazdırır ama asla iki basamaktan azını değil: `4.9E-324`; Rust'ın en kısa biçimi ise `5e-324`'tür.
- `\s` yalnızca ASCII'dir; Rust'ın `split_whitespace`'i U+00A0'da da böler.
- Float'lar gidiş-dönüşü sağlam atlatmalıdır: `serde_json`'ın varsayılan float ayrıştırıcısı her zaman doğru yuvarlamaz, bu yüzden everything-rust ve ürettiği her crate onun `float_roundtrip` özelliğini kullanır.

## Hız

JIT'ten sonra yaprak fonksiyonlar birkaç nanosaniye sürer ve string'li bir JNI çağrısı bundan daha pahalıdır. JDK cephesinde `bench`, portu çağrı başına 0.18x ölçtü: daha büyük birimleri port edin. 4 çekirdekte batch olarak, 50,000 string üzerinde `slugify` (Java'da 43 ms) **3x daha hızlı** çalıştı (çağrı başına port: 0.68x).

Ayrıntılar: [Altı dil](/docs/real-world/six-languages).
