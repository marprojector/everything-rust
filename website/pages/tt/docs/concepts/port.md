---
title: Порт
description: port.rs язу, сез яки агентыгыз яза торган бердәнбер файл, һәм аны оригиналга туры килүдә тотучы кагыйдәләр.
---

# Порт

Портны сез яки агентыгыз `src/port.rs` эчендә яза. Crate'тагы калган һәрнәрсә ясалган. `port.rs` эчендә сигнатураны үзгәрткәннән соң `everything-rust refresh <crate>` җибәрегез.

## Diff'ның бер әйләнешен экономияләүче кагыйдәләр

- Код язганчы, план берәмлек өчен күрсәткән һәр куркынычны аның `file:line` урынында укыгыз.
- **Юл офсетлары**: JS һәм Java UTF-16 код берәмлекләре буенча индекслый, Rust UTF-8 байтлары буенча. Ясалган `src/utf16.rs` кулланыгыз.
- **Хаталар**: `Err("TypeName: message")` кайтарыгыз. Ул Python, Ruby һәм Java'да шул исключениегә, ә Go'да `error` текстына әйләнә. Rust panic беркайчан да язып алынган хатага туры килми.
- **Python һәм Ruby'дагы саннар**: оригинал аша үткән int'лар int булып кайтырга тиеш. Python портларында int яки float өчен `serde_json::Value` түгел, `crate::pynum::Num` кулланыгыз.
- **C**: `crash:` дип язып алынган кертмәләр билгеләнмәгән тәртип; теләсә нинди нәтиҗә уза.
- **Тәртипле map'лар һәм объект ачкычлары**: тәртип чыгышка тәэсир итсә, `indexmap`.
- **Regex lookaround яки кире сылтамалар**: `regex` түгел, `fancy-regex`. Ничек кенә булмасын, `\w`, `\b`, `\d` һәм регистрга карамаган эзләү ASCII булмаган текстны чыганак телнең движогыннан башкача эшкәртә. Оригиналның `\w`'ы бары ASCII булган урында (`u`'сыз JS, Java) `unicode(false)` куегыз яки ASCII класларын ачык языгыз (`[A-Za-z0-9_]`).
- **`unsafe` юк.** Чынлап кирәк булса: минималь, аерымланган, `// SAFETY:` аңлатмасы белән.
- **Ярдәмчеләрне ябык тотыгыз.** Бары берәмлек экспортлары гына `pub`; `pub` ярдәмче bridge аша чыккан API өлешенә әйләнә.
- Сез уйлап тикшергән очраклар өчен `port.rs` эчендә Rust unit тестлары өстәгез.

Ясалган `js.rs` Rust аерылган урыннарда JavaScript'ны кабатлаучы ярдәмчеләр йөртә, мәсәлән, `js::number_to_string` (JS `String(n)`) һәм `js::math_round` (`Math.round`).

## Мисал: позициясен күчергән эмодзи

[x18n](https://github.com/x18n-dev/x18n) эченнән `packages/core/src/fuzzy.ts`, ике ысул белән дә
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo) эчендә күчерелгән:

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: ready, wasm-bindgen (core браузерларда эшли)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# src/port.rs языгыз
everything-rust gate demo/fuzzy-rs
```

Rust `char` буенча индекслаган беренче порт гади тестларны узды, ләкин `diff`'ны узмады:

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

JS `indexOf` UTF-16 код берәмлекләрен саный, шуңа күрә эмодзи ике позиция алып тора.
`demo/*/src/port.rs` эчендәге порт ясалган `utf16` ярдәмчеләрен куллана һәм 2000/2000 очракны һәм gate'ны уза,
napi-rs (`demo/fuzzy-napi`) һәм wasm-bindgen (`demo/fuzzy-rs`) итеп тә.

everything-rust бөтен x18n өчен язган план: [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md).
## Ул нинди зурлыкта булырга тиеш?

`gate` порт юлларын оригиналныкы белән чагыштыра һәм 3x'тан артса (өстәвенә импортлар һәм сигнатуралар өчен 50 юл) узмый. Моннан артуы гадәттә crate инде эшләгән нәрсәне кулдан язуны яки очракларны берәм-берәм эшкәртүне аңлата. `analyze` кулланырга кирәкле crate'ларны күрсәтә.
