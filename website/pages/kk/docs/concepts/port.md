---
title: Порт
description: port.rs жазу, сіз немесе агентіңіз жазатын жалғыз файл, және оны түпнұсқаға сәйкес ұстайтын ережелер.
---

# Порт

Портты `src/port.rs` ішінде сіз немесе агентіңіз жазады. Crate-тегі қалғанның бәрі генерацияланады. `port.rs` ішіндегі сигнатураны өзгерткеннен кейін `everything-rust refresh <crate>` іске қосыңыз.

## Бір diff раундын үнемдейтін ережелер

- Код жазбас бұрын жоспар бірлік үшін тізімдеген әр тәуекелді оның `file:line` орнында оқыңыз.
- **Жол офсеттері**: JS пен Java UTF-16 код бірліктерін, Rust UTF-8 байттарын индекстейді. Генерацияланған `src/utf16.rs` қолданыңыз.
- **Қателер**: `Err("TypeName: message")` қайтарыңыз. Ол Python, Ruby және Java-да сол exception-ға, ал Go-да `error` мәтініне айналады. Rust panic-і ешқашан жазылған қатеге сәйкес келмейді.
- **Python мен Ruby-дегі сандар**: түпнұсқа өзгертпей өткізетін int-тер int болып қайтуы керек. Python порттарында int-немесе-float үшін `serde_json::Value` емес, `crate::pynum::Num` қолданыңыз.
- **C**: `crash:` ретінде жазылған кірістер undefined behavior; кез келген нәтиже өтеді.
- **Реттелген map-тар мен объект кілттері**: рет шығысқа жететін жерде `indexmap`.
- **Регекстегі lookaround немесе backreference**: `regex` емес, `fancy-regex`. Қалай болғанда да, `\w`, `\b`, `\d` және регистрге тәуелсіз салыстыру ASCII емес мәтінді бастапқы тілдің қозғалтқышынан басқаша өңдейді. Түпнұсқаның `\w` тек ASCII болатын жерде (`u`-сыз JS, Java) `unicode(false)` орнатыңыз немесе ASCII кластарын анық жазыңыз (`[A-Za-z0-9_]`).
- **`unsafe` жоқ.** Шынымен керек болса: ең аз, оқшауланған, `// SAFETY:` түсініктемесімен.
- **Көмекшілерді жабық ұстаңыз.** Тек бірліктің экспорттары `pub`; `pub` көмекші bridge жасалған API-дің бөлігіне айналады.
- Өзіңіз ойланатын кейстер үшін `port.rs` ішіне Rust unit тесттерін қосыңыз.

Генерацияланған `js.rs` Rust ерекшеленетін жерде JavaScript-ті қайталайтын көмекшілерді тасымалдайды, мысалы `js::number_to_string` (JS `String(n)`) және `js::math_round` (`Math.round`).

## Мысал: позицияны жылжытқан эмодзи

[x18n](https://github.com/x18n-dev/x18n) жобасындағы `packages/core/src/fuzzy.ts`, екі тәсілмен де
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo) ішінде портталған:

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: дайын, wasm-bindgen (core браузерлерде жұмыс істейді)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# src/port.rs жазыңыз
everything-rust gate demo/fuzzy-rs
```

Rust `char` бойынша индекстеген алғашқы порт қарапайым тесттерден өтті, бірақ `diff`-тен құлады:

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

JS `indexOf` UTF-16 код бірліктерін санайды, сондықтан эмодзи екі позиция алады. `demo/*/src/port.rs`
ішіндегі порт генерацияланған `utf16` көмекшілерін қолданады және 2000/2000 кейстен және gate-тен өтеді,
napi-rs (`demo/fuzzy-napi`) ретінде де, wasm-bindgen (`demo/fuzzy-rs`) ретінде де.

everything-rust бүкіл x18n үшін жазатын жоспар: [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md).
## Ол қаншалықты үлкен болуы керек?

`gate` порттың жолдарын түпнұсқаның жолдарымен салыстырады және 3x-тен асса құлайды (импорттар мен сигнатуралар үшін тағы 50 жол). Одан асу әдетте crate істеп қойған нәрсені қолмен жазуды немесе кейстерді бір-бірлеп өңдеуді білдіреді. `analyze` қолданылатын crate-терді тізімдейді.
