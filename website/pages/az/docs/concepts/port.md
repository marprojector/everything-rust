---
title: Port
description: port.rs-in yazılması, sizin və ya agentinizin yazdığı yeganə fayl, və onu orijinala uyğun saxlayan qaydalar.
---

# Port

Portu siz və ya agentiniz `src/port.rs`-də yazırsınız. Crate-dəki qalan hər şey yaradılır. `port.rs`-də imzanı dəyişdikdən sonra `everything-rust refresh <crate>` işə salın.

## Bir diff raundunu qənaət edən qaydalar

- Kod yazmazdan əvvəl planın vahid üçün sadaladığı hər riski onun `file:line` yerində oxuyun.
- **String offset-ləri**: JS və Java UTF-16 kod vahidləri ilə, Rust isə UTF-8 baytları ilə indeksləyir. Yaradılmış `src/utf16.rs`-dən istifadə edin.
- **Xətalar**: `Err("TypeName: message")` qaytarın. O, Python, Ruby və Java-da həmin exception-a, Go-da isə `error` mətninə çevrilir. Rust panic-i heç vaxt qeydə alınmış xətaya uyğun gəlmir.
- **Python və Ruby-də rəqəmlər**: orijinalın olduğu kimi ötürdüyü int-lər int kimi qayıtmalıdır. Python portlarında int və ya float üçün `serde_json::Value` yox, `crate::pynum::Num` istifadə edin.
- **C**: `crash:` kimi qeydə alınmış girişlər qeyri-müəyyən davranışdır; istənilən nəticə keçir.
- **Sıralı map-lər və obyekt açarları**: ardıcıllığın çıxışa çatdığı yerdə `indexmap`.
- **Regex lookaround və ya backreference-lər**: `regex` yox, `fancy-regex`. Hər iki halda `\w`, `\b`, `\d` və hərf registrindən asılı olmayan uyğunluq ASCII olmayan mətnlə mənbə dilinin mühərrikindən fərqli işləyir. Orijinalın `\w`-si yalnız ASCII olan yerdə (`u` olmadan JS, Java) `unicode(false)` təyin edin və ya ASCII siniflərini açıq yazın (`[A-Za-z0-9_]`).
- **`unsafe` yoxdur.** Həqiqətən lazımdırsa: minimal, təcrid olunmuş, `// SAFETY:` şərhi ilə.
- **Köməkçiləri private saxlayın.** Yalnız vahidin export-ları `pub`-dur; `pub` köməkçi körpü ilə bağlanmış API-nin bir hissəsinə çevrilir.
- Haqqında düşündüyünüz hallar üçün `port.rs`-ə Rust unit testləri əlavə edin.

Yaradılmış `js.rs` Rust-un fərqləndiyi yerlərdə JavaScript-i təkrarlayan köməkçilər daşıyır, məsələn `js::number_to_string` (JS `String(n)`) və `js::math_round` (`Math.round`).

## Nümunə: mövqeyi sürüşdürən emoji

[x18n](https://github.com/x18n-dev/x18n)-dən `packages/core/src/fuzzy.ts`, hər iki yolla
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo)-da port edilib:

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: hazırdır, wasm-bindgen (core brauzerlərdə işləyir)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# src/port.rs-i yazın
everything-rust gate demo/fuzzy-rs
```

Rust `char` ilə indeksləyən ilk port sadə testlərdən keçdi, amma `diff`-dən keçmədi:

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

JS `indexOf` UTF-16 kod vahidlərini sayır, buna görə emoji iki mövqe tutur.
`demo/*/src/port.rs`-dəki port yaradılmış `utf16` köməkçilərindən istifadə edir və 2000/2000 haldan və gate-dən keçir,
həm napi-rs (`demo/fuzzy-napi`), həm də wasm-bindgen (`demo/fuzzy-rs`) kimi.

everything-rust-in bütün x18n üçün yazdığı plan: [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md).
## Nə qədər böyük olmalıdır?

`gate` portun sətirlərini orijinalınkı ilə müqayisə edir və 3 qatdan (üstəgəl import-lar və imzalar üçün 50 sətir) çox olduqda uğursuz olur. Bundan çox adətən crate-in artıq etdiyi şeyi əllə yazmaq və ya halları bir-bir emal etmək deməkdir. `analyze` istifadə ediləcək crate-ləri siyahıya alır.
