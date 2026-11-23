---
title: Java
description: public static metodlardan ibarət paketi orijinal metodlara malik wrapper siniflərdə tipli JNI və yaradılmış binary wire vasitəsilə Rust-a port edin.
---

# Java

| | |
| --- | --- |
| Vahid | `public static` metodlardan ibarət paket |
| Körpü | orijinal metodlara malik wrapper siniflərdə tipli JNI və yaradılmış binary wire |
| Qeydə alan və təkrar oynadan | yaradılmış reflection drayveri |
| Tələb olunur | JDK 17+ (`javac` və `java`) |
| Vəziyyət | Fasad arxasında JDK 21 üzərində yoxlanılıb: 4,492/4,492 (226-sı xəta atır), gate keçir |

Batch-lər `new FnBatch(items)`-dir; işiniz bitəndə onları bağlayın. `profile` JFR-dən istifadə edir.

## Portun düzgün etməli olduğu şeylər

- `Integer.parseInt` `Character.digit` vasitəsilə istənilən Unicode onluq rəqəmini (`١٢`, `２`) qəbul edir.
- `Double.toString` (JDK 19+) ən qısa onluq formanı çap edir, amma heç vaxt iki rəqəmdən az yox: `4.9E-324`, Rust-un ən qısa forması isə `5e-324`-dür.
- `\s` yalnız ASCII-dir; Rust-un `split_whitespace` funksiyası U+00A0-da da bölür.
- Float-lar gediş-gəlişdən sağ çıxmalıdır: `serde_json`-un standart float parser-i həmişə düzgün yuvarlaqlaşdırmır, buna görə everything-rust və onun yaratdığı hər crate onun `float_roundtrip` xüsusiyyətindən istifadə edir.

## Sürət

JIT-dən sonra yarpaq funksiyalar bir neçə nanosaniyə çəkir, string-li JNI çağırışı isə bundan baha başa gəlir. JDK fasadında `bench` portu çağırış başına 0.18x ölçdü: daha böyük vahidləri port edin. 4 nüvədə batch kimi, 50,000 string üzərində `slugify` (Java-da 43 ms) **3x daha sürətli** işlədi (çağırış başına port: 0.68x).

Təfərrüatlar: [Altı dil](/docs/real-world/six-languages).
