---
title: Порттоо
description: port.rs'ти жазуу (сиз же агентиңиз жаза турган жалгыз файл) жана аны түп нуска менен дал келтирип турган эрежелер.
---

# Порттоо

Портту сиз же агентиңиз `src/port.rs` ичинде жазасыз. Crate'теги калган нерселердин баары генерацияланат. `port.rs`'теги сигнатураны өзгөрткөндөн кийин `everything-rust refresh <crate>` иштетиңиз.

## Diff'тин бир айлампасын үнөмдөгөн эрежелер

- Код жазуудан мурун план бирдик үчүн тизмектеген ар бир тобокелдикти анын `file:line` жеринде окуңуз.
- **Саптагы позициялар**: JS жана Java UTF-16 код бирдиктери боюнча индекстейт, Rust UTF-8 байттары боюнча. Генерацияланган `src/utf16.rs` колдонуңуз.
- **Каталар**: `Err("TypeName: message")` кайтарыңыз. Ал Python, Ruby жана Java'да ошол exception'го, Go'до `error` текстине айланат. Rust panic жазылган катага эч качан дал келбейт.
- **Python жана Ruby'деги сандар**: түп нуска өткөргөн int'тер int болуп кайтышы керек. Python портторунда int-же-float үчүн `serde_json::Value` эмес, `crate::pynum::Num` колдонуңуз.
- **C**: `crash:` деп жазылган киргизүүлөр аныкталбаган жүрүм-турум; каалаган натыйжа өтөт.
- **Тартиптелген map'тер жана объекттин ачкычтары**: тартип чыгышка жеткен жерде `indexmap`.
- **Regex lookaround же backreference**: `regex` эмес, `fancy-regex`. Кандай болсо да, `\w`, `\b`, `\d` жана регистрге көз карандысыз дал келүү ASCII эмес текстти баштапкы тилдин кыймылдаткычынан башкача иштетет. Түп нусканын `\w`'сы ASCII гана болгон жерде (`u`'сыз JS, Java) `unicode(false)` коюңуз же ASCII класстарын ачык жазыңыз (`[A-Za-z0-9_]`).
- **`unsafe` жок.** Чындап керек болсо: минималдуу, обочолонгон, `// SAFETY:` комментарийи менен.
- **Жардамчыларды private кармаңыз.** Бирдиктин экспорттору гана `pub`; `pub` жардамчы bridge аркылуу өткөн API'нин бөлүгү болуп калат.
- Өзүңүз ойлонгон кейстер үчүн `port.rs` ичине Rust unit тесттерин кошуңуз.

Генерацияланган `js.rs` Rust айырмаланган жерде JavaScript'ти кайталаган жардамчыларды камтыйт, мисалы `js::number_to_string` (JS `String(n)`) жана `js::math_round` (`Math.round`).

## Мисал: позицияны жылдырган эмодзи

[x18n](https://github.com/x18n-dev/x18n) долбоорундагы `packages/core/src/fuzzy.ts`, эки жол менен порттолгон,
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo) ичинде:

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: ready, wasm-bindgen (core runs in browsers)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# write src/port.rs
everything-rust gate demo/fuzzy-rs
```

Rust `char` боюнча индекстеген биринчи порт жөнөкөй тесттерден өттү, бирок `diff`'тен өтпөй калды:

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

JS `indexOf` UTF-16 код бирдиктерин санайт, ошондуктан эмодзи эки позицияны ээлейт.
`demo/*/src/port.rs` ичиндеги порт генерацияланган `utf16` жардамчыларын колдонуп, 2000/2000 кейстен жана gate'тен өтөт,
napi-rs катары да (`demo/fuzzy-napi`), wasm-bindgen катары да (`demo/fuzzy-rs`).

everything-rust бүт x18n үчүн жазган план: [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md).
## Ал канчалык чоң болушу керек?

`gate` порттун саптарын түп нусканыкы менен салыштырат жана 3x'тен ашса өтпөйт (плюс импорттор жана сигнатуралар үчүн 50 сап). Андан ашуу көбүнчө crate буга чейин кылган нерсени кол менен жазуу же кейстерди бир-бирден иштетүү дегенди билдирет. `analyze` колдонула турган crate'терди тизмектейт.
