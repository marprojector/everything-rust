---
title: Record
description: record asl nusxani generatsiya qilingan chegaraviy holatlar, semantik tuzoqlar, uning o'z testlari va sizning kirish ma'lumotlaringiz ustida ishga tushiradi va natijalarni cases.jsonl sifatida saqlaydi.
---

# Record

```sh
everything-rust record <crate>
everything-rust record <crate> --from-tests -- npx jest      # yoki: -- pytest
everything-rust record <crate> --inputs extra.jsonl
```

`record` asl funksiyalarni ishga tushiradi va har biri nima qaytarishi yoki qanday xato tashlashini crate yonidagi `cases.jsonl` ga saqlaydi: port mos kelishi shart bo'lgan xatti-harakat. Keyinchalik nomuvofiqlik xatti-harakatdagi farqdir: portni tuzating, hech qachon yozuvni emas.

## Holatlar qayerdan keladi

- **Generatsiya qilingan chegaraviy holatlar**: ASCII bo'lmagan belgilar, emoji, `-0`, `NaN`, float kutilgan joyda int'lar va boshqalar (`--count` har bir funksiya uchun nechta ekanini belgilaydi).
- Har bir parametrda **semantik tuzoqlar**: string uzunligi va registri, ishorali bo'lish, to'lib ketish chegaralari, float'ni chiqarish, map va saralash tartibi.
- Regex'lar ishlatiladigan fayldagi har bir funksiya uchun **regex tuzoqlari**: turkcha nuqtasiz `ı`, Kelvin belgisi, uzun s, arab-hind raqamlari, qator ajratgichlari, so'z oxiridagi sigma, zero-width joiner'lar.
- **Asl nusxaning o'z testlari**: `--from-tests -- <test command>` test to'plami birlikning eksport qilingan funksiyalariga qiladigan har bir chaqiruvni yozib oladi (istalgan runner bilan TS/JS, pytest yoki unittest bilan Python). Manba keyin tiklanadi. vercel/ms'da u 167 ta Jest testidan 340 ta chaqiruvni yozib oldi.
- **Sizning kirish ma'lumotlaringiz**: har qatorda bitta `{"fn": name, "args": [...]}` bo'lgan `--inputs cases.jsonl`. Haqiqiy kirish ma'lumotlarini qo'shing (production namunalari, xato hisobotlari); prod'ni buzadigan foydalanuvchi kiritmasi kamdan-kam hollarda generator o'ylab topadigan narsa bo'ladi.

Python chaqiruvchilari `1` ni `1.0` kabi tez-tez uzatadi, shuning uchun yozib olish har to'rtinchi Python holatida butun qiymatli sonlarni int sifatida uzatadi.

## cases.jsonl dagi belgilar

JSON `undefined`, NaN, Infinity, -0 yoki yolg'iz surrogatlarni ifodalay olmaydi. Tuzilmali qiymatlar generatsiya qilingan `index.cjs` tarjima qiladigan belgilarni olib yuradi, yordamchilar crate'ning `js.rs` ida:

| Belgi | Ma'nosi |
| --- | --- |
| `{"$undefined": true}` | JavaScript `undefined` |
| `{"$num": "NaN"}` | JSON saqlay olmaydigan son (`NaN`, `Infinity`, `-0`) |
| `{"$utf16": [...]}` | yolg'iz surrogatlar uchun UTF-16 kod birliklari sifatida berilgan string |

Funksiya shartnomasidan tashqaridagi C kirish ma'lumotlari asl nusxani crash qilishi mumkin. C drayveri har bir holatni fork qilingan child jarayonda ishga tushiradi va `crash: SIGSEGV` ni yozib oladi; `diff` u yerda portning istalgan xatti-harakatini qabul qiladi, chunki C'niki aniqlanmagan.

## Ikki ishga tushirish va nima farq qiladi

`record` asl nusxani ikki marta ishga tushiradi va ishga tushirishlar orasida nima farq qilganini ajratadi: o'zgargan qiymatlar (vaqtlar, id'lar) wildcard'larga aylanadi; boshqa tartibdagi bir xil qatorlar (hash map'lar, parallellik) istalgan tartibda solishtiriladi; qatorlar sonining farqi istalgan narsaga mos keladigan `...` ga aylanadi. `record` har bir turni ro'yxatlaydi. Har birini ko'rib chiqing, chunki hash tartibi yoki parallellik farqi aynan portda haqiqatan o'zgarishi mumkin bo'lgan narsa.

## Yozuv o'z asl nusxasini eslab qoladi

`record` birlik fayllarining barmoq izini saqlaydi (`recorded-from.json`). Asl nusxa keyinroq o'zgarsa, asl nusxa avval nima qilganiga qarshi o'tib ketish o'rniga, siz qayta yozib olmaguningizcha `diff` ogohlantiradi va `gate` muvaffaqiyatsiz bo'ladi. [Diff](/docs/concepts/diff#stale-recordings) sahifasiga qarang.

## TypeScript xususiyatlari

`record` TS manbalarini o'z holicha ishga tushiradi: Node resolve hook'i bundler uslubidagi importlarga amal qiladi (`./errors` dan `./errors.ts` ga), repoda npm paketlari o'rnatilmagan bo'lsa `--resolve-from` ularni yetkazib beradi, transformatsiya talab qiladigan TS (enum'lar, parametr xossalari) esa `--experimental-transform-types` bilan qayta urinib ko'riladi. Node'da to'g'ridan-to'g'ri ishlay olmaydigan TS'ni build natijasidan `--module dist/x.mjs` bilan yoki `--node-arg=--import=tsx` bilan yozib olish mumkin.

## Sonli kod

Float'lar kamdan-kam hollarda bitma-bit mos keladigan kod uchun `--float-tolerance` bilan yozib oling. [Float tolerantligi](/docs/concepts/float-tolerance) sahifasiga qarang.
