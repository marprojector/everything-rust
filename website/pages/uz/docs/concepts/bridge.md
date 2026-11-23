---
title: Bridge
description: bridge bitta birlik uchun binding'lar va wrapper bilan Rust crate'ini yaratadi, shunda chaqiruvchilar asl API'ni saqlab qoladi.
---

# Bridge

```sh
everything-rust bridge <repo> <unit-id>          # crate <repo>/rust/<name> da; hech qachon qayta yozmaydi
everything-rust refresh <crate>                  # port.rs dagi signaturani o'zgartirgandan keyin
```

`bridge` binding'lar (`lib.rs` va manba tili yuklaydigan wrapper) hamda port qilinadigan stub'lar (`port.rs`) bilan crate yaratadi. Portni faqat `src/port.rs` ga yozasiz; `src/lib.rs` va wrapper'lar generatsiya qilinadi, `refresh` esa ularni `port.rs` dagi signaturalardan qayta generatsiya qiladi va yangi batch klasslarini qo'shadi. U hech qachon `port.rs` ga tegmaydi.

Parametrlar:

- `--batch fn:param` ko'plab `param` qiymatlarini Rust'da saqlaydigan klass qo'shadi. [Batch'lar](/docs/concepts/batches) sahifasiga qarang.
- `--as-json T` ilova faqat ma'lumot bilan to'ldiradigan tip parametrini (`paginate<T>(items: T[])` dagi `T`) JSON qiymatlari sifatida ko'prik orqali o'tkazadi.
- `--keep "name: reason"` qo'shimcha eksportlarni manba tilida qoldiradi, masalan `Intl` ma'lumotlariga bog'liq kodni.
- `--kind` reja tanlagan ko'prikni almashtiradi (TypeScript uchun napi-rs yoki wasm).

## Har bir til uchun ko'prik

| Til | Ko'prik |
| --- | --- |
| TypeScript/JavaScript | generatsiya qilingan binary wire bilan napi-rs yoki wasm-bindgen; klasslar va async ham |
| Python | PyO3 (abi3), to'g'ridan-to'g'ri konvertatsiya, tiplangan exception'lar |
| C | asl nomlar bilan `extern "C"` funksiyalar va header |
| Go | asl API'li paketda tiplangan cgo chaqiruvlari va generatsiya qilingan binary wire |
| Ruby | asl modullar ostida magnus kengaytmasi |
| Java | asl metodlarga ega wrapper klasslarda tiplangan JNI va generatsiya qilingan binary wire |

PHP va C# birliklarini hozircha ko'prik orqali ulab bo'lmaydi; [PHP](/docs/languages/php) va [C#](/docs/languages/csharp) sahifalariga qarang.

## bridge nimani hal qiladi

- Ko'prikdan o'ta olmaydigan eksportlar sababi ko'rsatilgan holda manba tilida qoladi, shadow
  moduli esa asl nusxani qayta eksport qiladi. TS uchun:
  funksiya qabul qiladigan yoki qaytaradigan funksiyalar, generator va iteratorlar, RegExp, `instanceof` tekshiruvlari,
  generic'lar (`--as-json T` ilova faqat ma'lumot bilan to'ldiradigan bittasini ko'prik orqali o'tkazadi) va tiplari (importlar, qayta eksportlar va `extends` orqali kuzatilgan)
  funksiya saqlaydigan qiymatlar. `--keep "name: reason"` qo'shimchalarini qoldiradi, masalan `Intl` ma'lumotlariga bog'liq kodni.
- TS tiplari Rust tiplariga moslanadi, tip taxalluslari importlar orqali aniqlanadi, parametr tiplari esa
  standart qiymatlardan chiqariladi; tuzilmali qiymatlar generatsiya qilingan binary wire sifatida o'tadi (JSON faqat
  wire modellashtirmaydigan shakllar uchun fallback sifatida).
- JSON `undefined`, NaN, Infinity, -0 yoki yolg'iz surrogatlarni ifodalay olmaydi: tuzilmali qiymatlar
  generatsiya qilingan `index.cjs` tarjima qiladigan belgilarni (`{"$undefined": true}`, `{"$num": "NaN"}`, `{"$utf16": [...]}`) olib yuradi,
  yordamchilar crate'ning `js.rs` ida.

`record` TS manbalarini o'z holicha ishga tushiradi: Node resolve hook'i bundler uslubidagi importlarga amal qiladi
(`./errors` dan `./errors.ts` ga), repoda npm paketlari o'rnatilmagan bo'lsa `--resolve-from` ularni
yetkazib beradi, transformatsiya talab qiladigan TS (enum'lar, parametr xossalari) esa
`--experimental-transform-types` bilan qayta urinib ko'riladi.
