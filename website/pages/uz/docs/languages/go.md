---
title: Go
description: Go paketini asl API'li paketda tiplangan cgo chaqiruvlari va generatsiya qilingan binary wire orqali Rust'ga port qiling.
---

# Go

| | |
| --- | --- |
| Birlik | paket |
| Ko'prik | asl API'li paketda tiplangan cgo chaqiruvlari va generatsiya qilingan binary wire |
| Yozib oluvchi va qayta ijro etuvchi | o'z moduli sifatida build qilinadigan generatsiya qilingan Go drayveri |
| Talablar | Go 1.21+ |
| Holat | Go 1.24'dagi `path` da tekshirilgan: 2,642/2,642 (542 tasi Go'ning o'z test jadvallaridan), gate o'tadi |

`profile` `go test` orqali ishlaydi: `everything-rust profile <repo> -- go test -run X -bench . ./pkg`. Batch'lar `NewFnBatch(items)`; ish tugagach ularni yoping.

## Port nimani to'g'ri qilishi kerak

- `path.Clean`: Go'ning `lazybuf` i orqaga surilgandan keyin oxirgi saqlangan baytni emas, yozish indeksidagi baytni o'qiydi (9 holat, Go'ning o'z test jadvalidan).
- `strings.ToLower` har bir rune'ni alohida kichik harfga o'tkazadi: `ΣΊΣΥΦΟΣ` `σίσυφοσ` ga aylanadi, Rust'ning `to_lowercase` esa so'z oxiridagi sigma qoidasini qo'llaydi. Go `İ` ni ham `i` ga o'tkazadi, Rust esa `i̇` beradi.
- Go string'larni baytlar bo'yicha kesadi; rune o'rtasidan o'tgan kesim JSON'ga har bir bayt uchun bitta U+FFFD bo'lib yetib boradi.

## Tezlik

Go oxirgi (leaf) funksiyalarni bir necha nanosoniyada bajaradi, bitta cgo chaqiruvining o'zi esa taxminan 80 ns turadi. `path` da `bench` portni har bir chaqiruvda 0.04x deb o'lchadi: kattaroq birliklarni port qiling. Batch sifatida ish barcha yadrolarda Rust'ga ko'chadi: 50,000 string ustida `Slugify` Go'da 49 ms oldi, batch esa **4x tezroq** ishladi (har bir chaqiruvli port: 0.93x). Batch crate'lari mimalloc bilan xotira ajratadi: tizim allokatori bilan Go batch'i bitta thread'dan sekinroq edi.

Batafsil: [Olti til](/docs/real-world/six-languages).
