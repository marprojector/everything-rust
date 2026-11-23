---
title: Geçirmek
description: port.rs ýazmak, siziň ýa-da agentiňiziň ýazýan ýeke faýly, we ony asyl nusga bilen gabat getirýän düzgünler.
---

# Geçirmek

Porty `src/port.rs` içinde siz ýa-da agentiňiz ýazýar. Crate-däki galan hemme zat döredilýär. `port.rs` içinde signatura üýtgedilenden soň `everything-rust refresh <crate>` işlediň.

## Diff-iň bir tapgyryny tygşytlaýan düzgünler

- Kod ýazmazdan öň meýilnamanyň birlik üçin sanaýan her töwekgelçiligini onuň `file:line` ýerinde okaň.
- **Setir süýşmeleri**: JS we Java UTF-16 kod birlikleri boýunça indeksleýär, Rust bolsa UTF-8 baýtlary boýunça. Döredilen `src/utf16.rs` ulanyň.
- **Ýalňyşlyklar**: `Err("TypeName: message")` gaýtaryň. Ol Python-da, Ruby-de we Java-da şol exception, Go-da bolsa `error` teksti bolýar. Rust panic hiç haçan ýazga alnan ýalňyşlyga gabat gelmeýär.
- **Python we Ruby-de sanlar**: asyl nusganyň üýtgetmän geçirýän int-leri int bolup gaýtmaly. Python portlarynda int-or-float üçin `serde_json::Value` däl, `crate::pynum::Num` ulanyň.
- **C**: `crash:` hökmünde ýazga alnan girişler kesgitlenmedik özüni alyp barşdyr; islendik netije geçýär.
- **Tertipli map-ler we obýekt açarlary**: tertip çykyşa ýetýän ýerinde `indexmap`.
- **Regex lookaround ýa-da backreference**: `regex` däl, `fancy-regex`. Islendik ýagdaýda `\w`, `\b`, `\d` we registre duýgur däl gabat gelmek ASCII däl tekstde çeşme diliniň motoryndan başgaça işleýär. Asyl nusganyň `\w`-i diňe ASCII bolan ýerinde (`u`-syz JS, Java) `unicode(false)` goýuň ýa-da ASCII klaslaryny aç-açan ýazyň (`[A-Za-z0-9_]`).
- **`unsafe` ýok.** Hakykatdan gerek bolsa: iň az, izolirlenen, `// SAFETY:` teswiri bilen.
- **Kömekçileri hususy saklaň.** Diňe birligiň eksportlary `pub`; `pub` kömekçi bridge edilen API-niň bölegi bolýar.
- Pikir ýöredýän ýagdaýlaryňyz üçin `port.rs` içinde Rust unit testlerini goşuň.

Döredilen `js.rs` Rust-yň tapawutlanýan ýerinde JavaScript-i gaýtalaýan kömekçileri göterýär, meselem `js::number_to_string` (JS `String(n)`) we `js::math_round` (`Math.round`).

## Mysal: orny süýşüren emoji

[x18n](https://github.com/x18n-dev/x18n)-iň `packages/core/src/fuzzy.ts` faýly, iki usulda hem geçirildi,
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo) içinde:

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: ready, wasm-bindgen (core runs in browsers)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# src/port.rs ýazyň
everything-rust gate demo/fuzzy-rs
```

Rust `char` boýunça indeksleýän ilkinji port ýönekeý testlerden geçdi, emma `diff`-den geçmedi:

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

JS `indexOf` UTF-16 kod birliklerini sanaýar, şonuň üçin emoji iki orun tutýar.
`demo/*/src/port.rs` içindäki port döredilen `utf16` kömekçilerini ulanýar we 2000/2000 ýagdaýdan hem-de gate-den geçýär,
napi-rs (`demo/fuzzy-napi`) we wasm-bindgen (`demo/fuzzy-rs`) hökmünde.

everything-rust-iň tutuş x18n üçin ýazan meýilnamasy: [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md).
## Ol näçe uly bolmaly?

`gate` portuň setirlerini asyl nusganyňky bilen deňeşdirýär we 3x-dan geçende şowsuz bolýar (import we signaturalar üçin goşmaça 50 setir). Ondan köp bolmagy adatça crate-iň eýýäm edýän zadyny el bilen ýazmagy ýa-da ýagdaýlary birme-bir işlemegi aňladýar. `analyze` ulanmaly crate-leri sanaýar.
