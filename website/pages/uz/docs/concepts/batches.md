---
title: Batch'lar va hot path'lar
description: Har bir element uchun bir marta chaqiriladigan port JIT'dan yutqazadi. Batch'lar butun siklni barcha yadrolarda Rust'ga ko'chiradi va har bir chaqiruvli portlar yutqazadigan joyda ham yutadi.
---

# Batch'lar va hot path'lar

```sh
everything-rust bridge <repo> <unit> --batch fn:param    # yoki: everything-rust refresh <crate> --batch fn:param
```

`--batch` `param` qiymatlarini bir marta qabul qilib, ularni Rust'da saqlaydigan klassni generatsiya qiladi: `new FnBatch(items)` (TS, Python, Java), `Mod::FnBatch.new(items)` (Ruby), `NewFnBatch(items)` (Go). Har bir so'rovda `run(<other args>)` barcha natijalarni bitta massiv sifatida qaytaradi, `rank(<other args>, min)` esa element indekslarini eng yaxshisidan boshlab qaytaradi. U barcha yadrolarda ishlaydi va xatolar asl nusxanikidek bo'ladi. C'dan tashqari har bir til batch'larni oladi.

Har bir element uchun bir marta chaqiriladigan port ko'prik narxini har bir element uchun bir marta to'laydi va JIT'dan yutqazadi. Yutadigan narsa
siklni ko'chirish: `profile` uni topadi, `--batch` esa elementlarni bir marta qabul qiladigan
va har bir chaqiruvda funksiyani ularning hammasi ustida, barcha yadrolarda, tiplangan massiv natijalari bilan bajaradigan klassni generatsiya qiladi.

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore: ishning to'rtdan biri, `matched` tomonidan har bir kalit uchun chaqiriladi: batch qiling
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

`port.rs` dagi `FuzzyScoreIndex` struct'i standart siklni elementlarni bir marta tayyorlaydigan
sikl bilan almashtiradi va o'zi saralay oladi (`rank`): u kalitlarni har biri belgi niqobi bilan
kichik harflarda, har bir kalitda har bir keng tarqalgan belgining birinchi pozitsiyasi jadvalini (bir harfli so'rov bitta
ustun) va oxirgi so'rov mosliklarini saqlaydi (`s` dan keyin `se` yozilsa, faqat `s` mos kelganlar qayta baholanadi). x18n'ning
50,000 kalit bo'yicha yon panel qidiruvida, harfma-harf yozilganda, bir xil natijalar bilan:

| | Har bir so'rovga | TS'ga nisbatan |
| --- | ---: | ---: |
| TS: `map(fuzzyScore)`, filter, sort | 10 ms | 1x |
| Indeks bilan bir xil toraytirishli TS | 4.1-5.4 ms | 2x |
| Xuddi shu port, har bir kalit uchun chaqirilgan | - | 0.2-0.4x |
| `FuzzyScoreBatch`, standart indeks (4 yadro) | 5.1 ms | 1.7x |
| `FuzzyScoreIndex` bilan `FuzzyScoreBatch` (napi, 4 yadro) | 0.45-0.56 ms | **18-22x** |
| Xuddi shu wasm bilan (bitta thread) | 0.95 ms | **11.5x** |

Xuddi shunday toraytirishni bajaradigan TS'ga qarshi ham Rust 7-12x tezroq (wasm 5x). Qaytarilgan
indekslarni oddiy sikl bilan kalitlarga aylantiring: `Array.from(order, i => keys[i])` 20x qimmatroq, 0.4 ms turdi.

Har bir til batch'larni oladi va ular har bir chaqiruvli portlar yutqazadigan joyda ham asl nusxadan o'zib ketadi.
4 yadroda bir xil ish, bir xil natijalar va xatolar (benchmark'lar har bir demo papkasida):

| Til | Ish yuki | Asl nusxa | Har bir chaqiruvli port asl nusxaga nisbatan | Batch asl nusxaga nisbatan |
| --- | --- | ---: | ---: | ---: |
| TS | x18n qidiruvi, 50,000 kalit | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`, 20,000 string | 98 ms | - | **17x** |
| Ruby | `slugify`, 20,000 string | 110 ms | - | **12.7x** |
| Ruby | `shellescape` (stdlib), 50,000 aralash qiymat | 88 ms | - | **11.8x** |
| Go | `Slugify`, 50,000 string | 49 ms | 0.93x | **4x** |
| Java | `slugify`, 50,000 string | 43 ms | 0.68x | **3x** |

Kompilyatsiya qilinadigan asl nusxalar kamroq yutadi: har bir elementdagi ish allaqachon tez, shuning uchun batch yadrolar
soni hisobiga yutadi. Batch crate'lari mimalloc bilan xotira ajratadi: tizim allokatori bilan Go batch'i bitta
thread'dan sekinroq edi. Demo: [`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md).
## Tez indeks yozish

- Chaqiruvchi batch'ni har bir so'rovda emas, bir marta qursin. Qaytarilgan indekslarni oddiy sikl bilan elementlarga qayta moslang.
- Elementlarni bir marta tayyorlash uchun (kichik harf, niqoblar, pozitsiya jadvallari) `port.rs` da `new(items)` va `run(<other args>)` ga ega `pub struct FnIndex` ni aniqlang. U funksiya bilan element-element bir xil natija berishi kerak.
- Saralash ishni o'tkazib yubora olsa, indeksga o'zining `rank(<other args>, min)` ini bering. U funksiya natijalarini saralash bilan mos kelishi kerak: ball yuqoridan pastga, tenglar element tartibida.
- Batch portning har bir elementdagi tezligini yadrolar soniga ko'paytiradi. Kompilyatsiya qilinadigan asl nusxalarga (Go, Java) qarshi portning o'zi tez bo'lishi kerak: ASCII'ni Unicode jadvallarisiz qayta ishlang, har bir natijani oldindan o'lchami belgilangan bitta `String` da quring, oraliq nusxalardan qoching.
- Go va Java'da ish tugagach batch'ni yoping (finalizer'lar faqat zaxira yo'li).
