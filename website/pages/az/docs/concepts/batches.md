---
title: Batch-lər və hot path-lar
description: Hər element üçün bir dəfə çağırılan port JIT-ə uduzur. Batch-lər bütöv dövrü bütün nüvələrdə Rust-a köçürür və hətta çağırış başına portların uduzduğu yerdə qazanır.
---

# Batch-lər və hot path-lar

```sh
everything-rust bridge <repo> <unit> --batch fn:param    # və ya: everything-rust refresh <crate> --batch fn:param
```

`--batch` `param` dəyərlərini bir dəfə qəbul edib Rust-da saxlayan sinif yaradır: `new FnBatch(items)` (TS, Python, Java), `Mod::FnBatch.new(items)` (Ruby), `NewFnBatch(items)` (Go). Hər sorğu üçün `run(<other args>)` bütün nəticələri bir massiv kimi qaytarır, `rank(<other args>, min)` isə element indekslərini, ən yaxşısı birinci olmaqla, qaytarır. O, bütün nüvələrdə işləyir, xətalar isə orijinalınkıdır. C-dən başqa hər dil batch-lər alır.

Hər element üçün bir dəfə çağırılan port körpünün haqqını hər element üçün bir dəfə ödəyir və JIT-ə uduzur. Qazandıran şey
dövrü köçürməkdir: `profile` onu tapır, `--batch` isə elementləri bir dəfə qəbul edən
və funksiyanı hər çağırışda hamısının üzərində bütün nüvələrdə, typed array nəticələri ilə işlədən sinif yaradır.

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore: işləmənin dörddə biri, `matched` tərəfindən hər açar üçün çağırılır: batch edin
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

`port.rs`-dəki `FuzzyScoreIndex` struct-ı standart dövrü elementləri bir dəfə hazırlayan dövrlə əvəz edir
və özü sıralaya bilir (`rank`): açarları kiçik hərflərlə, hər birini simvol maskası ilə saxlayır,
hər açarın hər ümumi simvolunun ilk mövqeyinin cədvəlini (bir hərfli sorğu bir sütundur) və son
sorğunun uyğunluqlarını saxlayır (`s`-dən sonra `se` yazmaq yalnız `s`-in uyğun gəldiklərini yenidən qiymətləndirir).
x18n-in 50,000 açar üzrə yan panel axtarışında, hərf-hərf yazılaraq, eyni nəticələrlə:

| | Sorğu başına | TS-ə qarşı |
| --- | ---: | ---: |
| TS: `map(fuzzyScore)`, filter, sort | 10 ms | 1x |
| İndekslə eyni daraltma ilə TS | 4.1-5.4 ms | 2x |
| Eyni port, hər açar üçün çağırılır | - | 0.2-0.4x |
| `FuzzyScoreBatch`, standart indeks (4 nüvə) | 5.1 ms | 1.7x |
| `FuzzyScoreIndex` ilə `FuzzyScoreBatch` (napi, 4 nüvə) | 0.45-0.56 ms | **18-22x** |
| Eynisi wasm ilə (bir thread) | 0.95 ms | **11.5x** |

Eyni daraltmanı edən TS-ə qarşı Rust yenə də 7-12x daha sürətlidir (wasm 5x). Qaytarılan
indeksləri sadə dövrlə açarlara çevirin: `Array.from(order, i => keys[i])` 20x baha başa gəldi, 0.4 ms.

Hər dil batch-lər alır və onlar hətta çağırış başına portların uduzduğu yerdə orijinalı ötür.
4 nüvədə eyni iş, eyni nəticələr və xətalar (benchmark-lar hər demo qovluğundadır):

| Dil | İş yükü | Orijinal | Çağırış başına port orijinala qarşı | Batch orijinala qarşı |
| --- | --- | ---: | ---: | ---: |
| TS | x18n axtarışı, 50,000 açar | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`, 20,000 string | 98 ms | - | **17x** |
| Ruby | `slugify`, 20,000 string | 110 ms | - | **12.7x** |
| Ruby | `shellescape` (stdlib), 50,000 qarışıq dəyər | 88 ms | - | **11.8x** |
| Go | `Slugify`, 50,000 string | 49 ms | 0.93x | **4x** |
| Java | `slugify`, 50,000 string | 43 ms | 0.68x | **3x** |

Kompilyasiya olunan orijinallar daha az qazanır: element başına iş artıq sürətlidir, buna görə batch nüvələrin sayı
qədər qazanır. Batch crate-ləri mimalloc ilə yaddaş ayırır: sistem allocator-u ilə Go batch-i bir
thread-dən daha yavaş idi. Demo: [`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md).
## Sürətli indeks yazmaq

- Çağıranın batch-i hər sorğu üçün yox, bir dəfə qurmasını təmin edin. Qaytarılan indeksləri sadə dövrlə elementlərə geri çevirin.
- Elementləri bir dəfə hazırlamaq üçün (kiçik hərflər, maskalar, mövqe cədvəlləri) `port.rs`-də `new(items)` və `run(<other args>)` ilə `pub struct FnIndex` təyin edin. O, element-element funksiya ilə eyni nəticəni verməlidir.
- Sıralama işi ötürə bildikdə indeksə öz `rank(<other args>, min)` metodunu verin. O, funksiyanın nəticələrinin sıralanmasına uyğun gəlməlidir: bal yüksəkdən aşağıya, bərabərlikdə element ardıcıllığı ilə.
- Batch portun element başına sürətini nüvələrin sayına vurur. Kompilyasiya olunan orijinallara (Go, Java) qarşı portun özü sürətli olmalıdır: ASCII-ni Unicode cədvəlləri olmadan emal edin, hər nəticəni əvvəlcədən ölçüsü təyin olunmuş bir `String`-də qurun, aralıq surətlərdən qaçın.
- Go və Java-da işiniz bitəndə batch-i bağlayın (finalizer-lər yalnız fallback-dır).
