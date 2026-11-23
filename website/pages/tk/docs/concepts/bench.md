---
title: Bench
description: bench ýazga alnan ýagdaýlaryň asyl nusgada we Rust-da wagtyny ölçeýär, bridge hem hasaba alnanda, we netije berýär. Rust hemişe has çalt däl.
---

# Bench

```sh
everything-rust bench <crate>
```

`bench` ýazga alnan ýagdaýlaryň asyl nusgada we gurlan Rust modulynda wagtyny ölçeýär, bridge hem hasaba alnanda, we setirleri hem-de maslahatly netijäni gaýtarýar: çykarmak, haýal funksiýalary çeşme dilinde galdyrmak ýa-da has uly birligi geçirmek. 1-den kiçi tizlenme portuň çagyryjylary çagyrşy ýaly çagyrylanda has haýaldygyny aňladýar. `--min-ms` her funksiýa we tarap üçin iň az ölçenýän wagty kesgitleýär (adaty 200).

Netijä eýeriň. Çagyryjylary haýallaşdyrýan porty muny aýtman çykarmaň.

## Port näme üçin haýal bolup biler

Her element üçin bir gezek çagyrylýan port her element üçin bir gezek bridge üçin töleýär. Muny kesgitleýän zatlar:

- **Interpretirlenýän asyl nusgalar (Python, Ruby)**: Rust-a çagyryş takmynan 100-300 ns durýar, Rust kody bolsa interpretatordan has çalt, şonuň üçin kiçi funksiýalar hem utýar.
- **Kompilirlenýän asyl nusgalar (Go, JIT-den soň Java)**: asyl ýaprak funksiýasy 4-90 ns alýar; ýeke cgo çagyryşynyň özi takmynan 80 ns durýar, setirli JNI çagyryşy has köp. Bu diller diňe bir çagyryş köp iş edende utýar: funksiýany çagyryjylary bilen bilelikde geçiriň.
- **C**: serhet geçişi ýok; Rust kody C kody bilen göni deňeşdirilýär.

Hakyky kodda bir çagyryş boýunça ölçeldi ([alty dil](/docs/real-world/six-languages)):

| Dil | Rust asyl nusga garşy (`bench`, geometrik orta) | Netije |
| --- | --- | --- |
| Ruby | **4.4x has çalt** (her funksiýa üçin 3.6-7x) | çykarmak |
| Python | **1.35x has çalt** (köp çagyryşlarda 1.5-1.9x) | çykarmak |
| C | 0.87x: ctype 1.1-1.4x has çalt, glibc-iň sazlanan `strtod`, `strspn`, `strcmp` funksiýalary 0.5-0.7x | optimizirlemek ýa-da haýallaryny galdyrmak |
| Java | 0.18x | has uly birlikleri geçirmek |
| Go | 0.04x | has uly birlikleri geçirmek |

x18n ýadrosynyň API üýtgetmezden göçürip bolýan 29 funksiýasynyň hemmesi geçirildi: her gate geçýär we ýazga alnan 7,925 ýagdaý takyk gabat gelýär, `bench` bolsa portlaryň JS-den çagyrylanda TS-den haýaldygyny görkezdi, sebäbi bridge işiň özünden köp durýar ([hasabat](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-core/README.md)). vercel/ms ýazga alnan ýagdaýlarynda bir çagyryşda 0.56x işledi ([tejribe mysaly](/docs/real-world/vercel-ms)).

## Näme utýar: aýlawy göçürmek

`profile` gyzgyn funksiýanyň daşyndaky aýlawy tapýar, `--batch` bolsa tutuş aýlawy ähli ýadrolarda Rust-a göçürýär. [Batch-lar we hot paths](/docs/concepts/batches) sahypasyna serediň.

::: tip Asuda maşynda ölçäň
`bench`-i asuda maşynda işlediň, ony iki gezek işlediň we çap edilen her sanyň ýanynda enjamy we ýadrolaryň sanyny görkeziň.
:::
