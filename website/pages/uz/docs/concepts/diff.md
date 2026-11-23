---
title: Diff va ma'lum farqlar
description: diff har bir yozib olingan holatni Rust'ga qarshi qayta ijro etadi. known-diffs.jsonl ataylab qilingan farqlarni qayd etadi, eskirgan yozuvlar esa gate'dan o'tmaydi.
---

# Diff va ma'lum farqlar

```sh
everything-rust build <crate>
everything-rust diff <crate>                     # N/N bo'lishi shart
everything-rust diff <crate> --json
```

`diff` `cases.jsonl` ni build qilingan Rust moduliga qarshi, uni asl nusxa chaqiruvchilari chaqirgandek chaqirib, qayta ijro etadi va nomuvofiqliklarni ro'yxatlaydi. Nomuvofiqlik xatti-harakatdagi farqdir: portni tuzating, hech qachon yozuvni emas.

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

## Ma'lum farqlar

Ba'zan port ataylab farq qilishi kerak: foydalanuvchi u asl nusxadagi xatoni tuzatishini xohlaydi. Avval so'rang, so'ng har bir holat uchun `<crate>/known-diffs.jsonl` ga bitta qator qo'shing:

**known-diffs.jsonl**
```json
{"fn": "...", "args": [...], "actual": ..., "reason": "..."}
```

`fn`, `args` va `actual` ni `diff --json` dan ko'chiring. Shundan keyin bu holat faqat aynan shu natija bilan o'tadi, farq yo'qolgach esa `gate` muvaffaqiyatsiz bo'ladi, shunda qatorni olib tashlang.

## Eskirgan yozuvlar

`record` asl birlik fayllarining barmoq izini `recorded-from.json` ga saqlaydi. Asl nusxa keyinroq o'zgarsa, asl nusxa avval nima qilganiga qarshi o'tib ketish o'rniga, siz qayta yozib olmaguningizcha `diff` ogohlantiradi va `gate` muvaffaqiyatsiz bo'ladi. Agar `gate` yozuv eskirganini aytsa, port qilishni davom ettirishdan oldin qayta yozib oling.

## Port to'xtab qolganda

`diff` build'lar bo'ylab rivojlanishni kuzatadi. U port to'xtab qolganini aytganda (uning `trend.stalled` i: `port.rs` ga ketma-ket uchta o'zgartirish birorta ham mos holat qo'shmagan), tahrirlashni to'xtating. Qolgan nomuvofiqliklar faqat foydalanuvchi biladigan qoidani talab qilishi mumkin yoki bu funksiyalar hozircha asl nusxada qolishi mumkin.

## Float tolerantligi

Agar crate `--float-tolerance` bilan yozib olingan bo'lsa, `diff` undan foydalanadi; `diff` dagi `--float-tolerance` uni bitta ishga tushirish uchun almashtiradi. [Float tolerantligi](/docs/concepts/float-tolerance) sahifasiga qarang.
