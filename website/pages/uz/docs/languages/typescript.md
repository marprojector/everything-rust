---
title: TypeScript/JavaScript
description: Node.js kodini, TypeScript yoki JavaScript'ni napi-rs yoki wasm-bindgen orqali, asl API'ni saqlagan holda Rust'ga port qiling.
---

# TypeScript/JavaScript

| | |
| --- | --- |
| Birlik | papka yoki fayl |
| Ko'prik | generatsiya qilingan binary wire bilan napi-rs yoki wasm-bindgen; klasslar va async ham |
| Yozib oluvchi va qayta ijro etuvchi | `.ts` manbalarini ishga tushiradigan Node |
| Talablar | Node 22.6+ (`profile` uchun Node 24+ kerak) |
| Holat | Tekshirilgan |

## Ko'prikni tanlash

`analyze` birlik yoki u import qiladigan biror narsa Node'ni talab qilsa napi-rs'ni, ko'chiriladigan kod uchun wasm-bindgen'ni tanlaydi (u brauzerlarda ham ishlaydi), UI va muharrir kodini esa JS'da qoldiradi. `bridge --kind napi|wasm` buni almashtiradi.

Ko'prikdan o'ta olmaydigan eksportlar sababi ko'rsatilgan holda TS'da qoladi va shadow moduli asl nusxani qayta eksport qiladi: funksiya qabul qiladigan yoki qaytaradigan funksiyalar, generator va iteratorlar, RegExp, `instanceof` tekshiruvlari, generic'lar (`--as-json T` ilova faqat ma'lumot bilan to'ldiradigan bittasini ko'prik orqali o'tkazadi) va tiplari funksiya saqlaydigan qiymatlar. [Bridge](/docs/concepts/bridge) sahifasiga qarang.

## TS'ni o'z holicha yozib olish

`record` TS manbalarini to'g'ridan-to'g'ri ishga tushiradi: Node resolve hook'i bundler uslubidagi importlarga amal qiladi, repoda npm paketlari o'rnatilmagan bo'lsa `--resolve-from` ularni yetkazib beradi, transformatsiya talab qiladigan TS (enum'lar, parametr xossalari) esa `--experimental-transform-types` bilan qayta urinib ko'riladi. `record --from-tests -- npx jest` istalgan runner bilan asl nusxaning o'z test to'plami qiladigan har bir chaqiruvni yozib oladi. Asl nusxaning qamrovi TS/JS uchun o'lchanadi ([Qamrov](/docs/concepts/coverage)).

## Port nimani to'g'ri qilishi kerak

Haqiqiy kodda yozib olingan holatlar orqali topilgan:

- `str.length`, `indexOf` va `slice` UTF-16 kod birliklarini sanaydi, shuning uchun emoji ikki pozitsiyani egallaydi. Generatsiya qilingan `utf16` yordamchilaridan foydalaning.
- `u` bayrog'isiz `/.../i` faqat ASCII harflarini registrdan mustaqil qiladi, Rust'ning `regex` i esa Unicode'ni qamraydi, shuning uchun `"1 ſ"` (uzun s) va Kelvin belgisi vercel/ms'da soniya sifatida parse qilingan bo'lardi.
- `Math.round` yarimlarni yuqoriga yaxlitlaydi (`-2.5` `-2` beradi); Rust'ning `round` i noldan uzoqlashadi. Generatsiya qilingan `js.rs` da `js::math_round` bor.
- `${n}` `1e+21`, `5e-324` va `-0` uchun `0` chiqaradi. `js::number_to_string` buni takrorlaydi.
- `String.prototype.trim` Rust'ning `char::is_whitespace` ini emas, JS bo'sh joy belgilarini ishlatadi.

## Tezlik

Har bir chaqiruvda port har safar ko'prik narxini to'laydi, V8 esa kichik funksiyalarni nanosoniyalarda bajaradi. vercel/ms yozib olingan holatlari ustida har bir chaqiruvda 0.56x ishladi; x18n'ning yadro portlari JS'dan chaqirilganda TS'dan sekinroq. Batch'lar sifatida sikl Rust'ga ko'chadi:

| Ish yuki | Asl nusxa | everything-rust bilan |
| --- | ---: | ---: |
| x18n'ning yon panel qidiruvi, 50,000 kalit (Node) | har so'rovga 10 ms | 0.5 ms: 20x tezroq |
| Xuddi shu qidiruv brauzerda (wasm, bitta thread) | 10 ms | 0.95 ms: 11.5x |
| Fuse.js, 50,000 string | 91.2 ms | 2.8 ms: 32x |
| vercel/ms `parse` x100,000 | 11.8 ms | 0.65 ms: 18x (uzatishni hisobga olganda 3.4x) |
| vercel/ms `format` x100,000 | 2.1 ms | uzatish bilan 2.5 ms: 0.8x |

Amaliy misollar: [json-server](/docs/real-world/json-server), [Fuse.js](/docs/real-world/fuse-js), [vercel/ms](/docs/real-world/vercel-ms), [x18n](/docs/real-world/x18n).
