---
title: Port
description: port.rs ni yozish, siz yoki agentingiz yozadigan yagona fayl, va uni asl nusxaga mos holda saqlaydigan qoidalar.
---

# Port

Portni siz yoki agentingiz `src/port.rs` ga yozadi. Crate'dagi qolgan hamma narsa generatsiya qilinadi. `port.rs` dagi signaturani o'zgartirgandan keyin `everything-rust refresh <crate>` ni ishga tushiring.

## Bir diff aylanishini tejaydigan qoidalar

- Kod yozishdan oldin reja birlik uchun ko'rsatgan har bir xavfni uning `file:line` joyida o'qing.
- **String ofsetlari**: JS va Java UTF-16 kod birliklari bo'yicha, Rust esa UTF-8 baytlari bo'yicha indekslaydi. Generatsiya qilingan `src/utf16.rs` dan foydalaning.
- **Xatolar**: `Err("TypeName: message")` qaytaring. U Python, Ruby va Java'da shu exception'ga, Go'da esa `error` matniga aylanadi. Rust panic'i hech qachon yozib olingan xatoga mos kelmaydi.
- **Python va Ruby'dagi sonlar**: asl nusxa o'tkazib yuboradigan int'lar int sifatida qaytishi kerak. Python portlarida int yoki float uchun `serde_json::Value` emas, `crate::pynum::Num` dan foydalaning.
- **C**: `crash:` sifatida yozib olingan kirish ma'lumotlari aniqlanmagan xatti-harakat; istalgan natija o'tadi.
- **Tartiblangan map'lar va obyekt kalitlari**: tartib chiqishga ta'sir qiladigan joyda `indexmap`.
- **Regex lookaround yoki backreference'lar**: `regex` emas, `fancy-regex`. Har qanday holatda `\w`, `\b`, `\d` va registrdan mustaqil moslashtirish ASCII bo'lmagan matnni manba tili dvigatelidan boshqacha talqin qiladi. Asl nusxadagi `\w` faqat ASCII bo'lgan joyda (`u` siz JS, Java) `unicode(false)` ni o'rnating yoki ASCII klasslarini (`[A-Za-z0-9_]`) to'liq yozing.
- **`unsafe` yo'q.** Agar haqiqatan kerak bo'lsa: minimal, ajratilgan, `// SAFETY:` izohi bilan.
- **Yordamchilarni private saqlang.** Faqat birlik eksportlari `pub`; `pub` yordamchi ko'prik orqali ulangan API'ning bir qismiga aylanadi.
- O'zingiz mulohaza qiladigan holatlar uchun `port.rs` ga Rust unit testlarini qo'shing.

Generatsiya qilingan `js.rs` Rust farq qiladigan joylarda JavaScript'ni takrorlaydigan yordamchilarni o'z ichiga oladi, masalan `js::number_to_string` (JS `String(n)`) va `js::math_round` (`Math.round`).

## Misol: pozitsiyani siljitgan emoji

[x18n](https://github.com/x18n-dev/x18n)'dagi `packages/core/src/fuzzy.ts`, ikkala usulda
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo) ichida port qilingan:

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: tayyor, wasm-bindgen (core brauzerlarda ishlaydi)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# src/port.rs ni yozing
everything-rust gate demo/fuzzy-rs
```

Rust `char` bo'yicha indekslagan birinchi port oddiy testlardan o'tdi, lekin `diff` dan o'tmadi:

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

JS `indexOf` UTF-16 kod birliklarini sanaydi, shuning uchun emoji ikki pozitsiyani egallaydi.
`demo/*/src/port.rs` dagi port generatsiya qilingan `utf16` yordamchilaridan foydalanadi va napi-rs (`demo/fuzzy-napi`)
hamda wasm-bindgen (`demo/fuzzy-rs`) sifatida 2000/2000 holatdan va gate'dan o'tadi.

everything-rust butun x18n uchun yozadigan reja: [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md).
## U qanchalik katta bo'lishi kerak?

`gate` port qatorlarini asl nusxa qatorlari bilan solishtiradi va 3x dan oshsa (importlar va signaturalar uchun yana 50 qator) muvaffaqiyatsiz bo'ladi. Bundan oshishi odatda crate allaqachon qiladigan ishni qo'lda yozish yoki holatlarni birma-bir ko'rib chiqishni anglatadi. `analyze` ishlatiladigan crate'larni ro'yxatlaydi.
