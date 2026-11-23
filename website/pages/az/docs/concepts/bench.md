---
title: Bench
description: bench qeydə alınmış halların orijinalda və Rust-da, körpü daxil, vaxtını ölçür və hökm verir. Rust həmişə daha sürətli deyil.
---

# Bench

```sh
everything-rust bench <crate>
```

`bench` qeydə alınmış halların orijinalda və qurulmuş Rust modulunda, körpü daxil, vaxtını ölçür və sətirləri və tövsiyəli hökmü qaytarır: buraxın, yavaş funksiyaları mənbə dilində saxlayın və ya daha böyük vahidi port edin. 1-dən aşağı sürətlənmə portun çağıranlarının çağırdığı kimi daha yavaş olduğu deməkdir. `--min-ms` funksiya və tərəf başına minimum ölçülən vaxtı təyin edir (standart 200).

Hökmə əməl edin. Çağıranları yavaşladan portu bunu demədən buraxmayın.

## Port niyə daha yavaş ola bilər

Hər element üçün bir dəfə çağırılan port körpünün haqqını hər element üçün bir dəfə ödəyir. Bunu həll edən amillər:

- **İnterpretasiya olunan orijinallar (Python, Ruby)**: Rust-a çağırış təxminən 100-300 ns tutur və Rust kodu interpretatordan xeyli sürətlidir, buna görə hətta kiçik funksiyalar da qazanır.
- **Kompilyasiya olunan orijinallar (Go, JIT-dən sonra Java)**: orijinal yarpaq funksiya 4-90 ns çəkir; tək bir cgo çağırışı təxminən 80 ns tutur, string-li JNI çağırışı isə daha çox. Bu dillər yalnız bir çağırış çox iş görəndə qazanır: funksiyanı onu çağıranlarla birlikdə port edin.
- **C**: körpü keçidi yoxdur; Rust kodu birbaşa C kodu ilə müqayisə olunur.

Real kodda çağırış başına ölçülüb ([altı dil](/docs/real-world/six-languages)):

| Dil | Rust orijinala qarşı (`bench`, həndəsi orta) | Hökm |
| --- | --- | --- |
| Ruby | **4.4x daha sürətli** (funksiya başına 3.6-7x) | buraxın |
| Python | **1.35x daha sürətli** (əksər çağırışlarda 1.5-1.9x) | buraxın |
| C | 0.87x: ctype 1.1-1.4x daha sürətli, glibc-nin optimallaşdırılmış `strtod`, `strspn`, `strcmp` funksiyaları 0.5-0.7x | optimallaşdırın və ya yavaşları saxlayın |
| Java | 0.18x | daha böyük vahidləri port edin |
| Go | 0.04x | daha böyük vahidləri port edin |

x18n-in core-unun API dəyişikliyi olmadan köçə bilən 29 funksiyasının hamısı port edildi: hər gate keçir və 7,925 qeydə alınmış hal dəqiq uyğun gəlir, `bench` isə portların JS-dən çağırıldıqda TS-dən daha yavaş olduğunu göstərdi, çünki körpü işin özündən baha başa gəlir ([hesabat](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-core/README.md)). vercel/ms qeydə alınmış halları üzrə çağırış başına 0.56x işlədi ([nümunə araşdırması](/docs/real-world/vercel-ms)).

## Nə qazandırır: dövrü köçürmək

`profile` isti funksiyanın ətrafındakı dövrü tapır, `--batch` isə bütöv dövrü bütün nüvələrdə Rust-a köçürür. Bax [Batch-lər və hot path-lar](/docs/concepts/batches).

::: tip Sakit maşında ölçün
`bench`-i sakit maşında işə salın, iki dəfə işə salın və dərc olunan hər rəqəmin yanında aparatı və nüvə sayını göstərin.
:::
