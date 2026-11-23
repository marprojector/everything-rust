---
title: Batch'ler ve hot path'ler
description: Her öğe için bir kez çağrılan bir port bir JIT'e yenilir. Batch'ler döngünün tamamını tüm çekirdeklerde Rust'a taşır ve çağrı başına portların kaybettiği yerde bile kazanır.
---

# Batch'ler ve hot path'ler

```sh
everything-rust bridge <repo> <unit> --batch fn:param    # ya da: everything-rust refresh <crate> --batch fn:param
```

`--batch`, `param` değerlerini bir kez alıp Rust'ta tutan bir sınıf üretir: `new FnBatch(items)` (TS, Python, Java), `Mod::FnBatch.new(items)` (Ruby), `NewFnBatch(items)` (Go). Her sorguda `run(<other args>)` tüm sonuçları tek bir dizi olarak döndürür, `rank(<other args>, min)` ise öğe indekslerini en iyiden başlayarak döndürür. Tüm çekirdeklerde çalışır ve hatalar orijinalinkilerle aynıdır. C dışındaki her dil batch alır.

Her öğe için bir kez çağrılan bir port, köprünün bedelini her öğe için bir kez öder ve bir JIT'e
yenilir. Kazandıran, döngüyü taşımaktır: `profile` onu bulur ve `--batch`, öğeleri bir kez alıp
her çağrıda fonksiyonu hepsinin üzerinde, tüm çekirdeklerde, typed array sonuçlarla çalıştıran bir sınıf üretir.

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore: çalışmanın dörtte biri, `matched` tarafından anahtar başına çağrılıyor: batch'leyin
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

`port.rs` içindeki bir `FuzzyScoreIndex` struct'ı, varsayılan döngüyü öğeleri bir kez hazırlayan
ve kendi başına sıralayabilen (`rank`) bir döngüyle değiştirir: anahtarları her biri için bir karakter
maskesiyle küçük harfe çevrilmiş olarak, her anahtarda her yaygın karakterin ilk konumunu gösteren
bir tabloyu (tek harfli bir sorgu tek bir sütundur) ve son sorgunun eşleşmelerini (`s`'den sonra `se`
yazmak yalnızca `s`'nin eşleştiklerini yeniden puanlar) tutar. x18n'in 50,000 anahtar üzerindeki
kenar çubuğu aramasında, harf harf yazılarak, aynı sonuçlarla:

| | Sorgu başına | TS'e göre |
| --- | ---: | ---: |
| TS: `map(fuzzyScore)`, filtre, sıralama | 10 ms | 1x |
| İndeksle aynı daraltmayı yapan TS | 4.1-5.4 ms | 2x |
| Aynı port, anahtar başına çağrılarak | - | 0.2-0.4x |
| `FuzzyScoreBatch`, varsayılan indeks (4 çekirdek) | 5.1 ms | 1.7x |
| `FuzzyScoreIndex` ile `FuzzyScoreBatch` (napi, 4 çekirdek) | 0.45-0.56 ms | **18-22x** |
| Aynısı wasm ile (tek thread) | 0.95 ms | **11.5x** |

Aynı daraltmayı yapan TS'e karşı bile Rust hâlâ 7-12x daha hızlıdır (wasm 5x). Döndürülen
indeksleri düz bir döngüyle anahtarlara çevirin: `Array.from(order, i => keys[i])` 20x daha pahalıydı, 0.4 ms.

Her dil batch alır ve çağrı başına portların kaybettiği yerde bile orijinali geçer. Aynı iş
4 çekirdekte, aynı sonuçlar ve hatalarla (benchmark'lar her demo klasöründe):

| Dil | İş yükü | Orijinal | Çağrı başına portun orijinale oranı | Batch'in orijinale oranı |
| --- | --- | ---: | ---: | ---: |
| TS | x18n araması, 50,000 anahtar | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`, 20,000 string | 98 ms | - | **17x** |
| Ruby | `slugify`, 20,000 string | 110 ms | - | **12.7x** |
| Ruby | `shellescape` (stdlib), 50,000 karışık değer | 88 ms | - | **11.8x** |
| Go | `Slugify`, 50,000 string | 49 ms | 0.93x | **4x** |
| Java | `slugify`, 50,000 string | 43 ms | 0.68x | **3x** |

Derlenen orijinaller daha az kazanır: öğe başına iş zaten hızlıdır, bu yüzden batch çekirdek sayısı
kadar kazanır. Batch crate'leri bellek ayırmayı mimalloc ile yapar: sistem ayırıcısıyla Go batch'i
tek thread'den daha yavaştı. Demo: [`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md).

## Hızlı bir indeks yazmak

- Çağıranın batch'i sorgu başına değil, bir kez oluşturmasını sağlayın. Döndürülen indeksleri düz bir döngüyle öğelere geri eşleyin.
- Öğeleri bir kez hazırlamak için (küçük harf, maskeler, konum tabloları) `port.rs` içinde `new(items)` ve `run(<other args>)` içeren bir `pub struct FnIndex` tanımlayın. Öğe öğe, fonksiyonla aynı sonucu vermelidir.
- Sıralama işin bir kısmını atlayabiliyorsa indekse kendi `rank(<other args>, min)` metodunu verin. Fonksiyonun sonuçlarını sıralamakla eşleşmelidir: puan yüksekten düşüğe, eşitlikler öğe sırasıyla.
- Bir batch, portun öğe başına hızını çekirdek sayısıyla çarpar. Derlenen orijinallere (Go, Java) karşı portun kendisi hızlı olmalıdır: ASCII'yi Unicode tabloları olmadan işleyin, her sonucu önceden boyutlandırılmış tek bir `String` içinde oluşturun, ara kopyalardan kaçının.
- Go ve Java'da işiniz bittiğinde batch'i kapatın (finalizer'lar yalnızca bir fallback'tir).
