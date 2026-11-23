---
title: Bridge
description: bridge bir birlik üçin Rust crate-ini binding-ler we wrapper bilen döredýär, şonuň üçin çagyryjylar asyl API-ni saklaýar.
---

# Bridge

```sh
everything-rust bridge <repo> <unit-id>          # <repo>/rust/<name> içinde crate; hiç haçan üstünden ýazmaýar
everything-rust refresh <crate>                  # port.rs içinde signatura üýtgedilenden soň
```

`bridge` binding-leri (`lib.rs`, üstesine çeşme diliniň ýükleýän wrapper-i) we geçirmek üçin stub-lary (`port.rs`) bolan crate döredýär. Porty diňe `src/port.rs` içinde ýazýarsyňyz; `src/lib.rs` we wrapper-ler döredilýär, `refresh` bolsa olary `port.rs` içindäki signaturalardan täzeden döredýär we islendik täze batch klaslaryny goşýar. Ol hiç haçan `port.rs`-e degmeýär.

Opsiýalar:

- `--batch fn:param` köp sanly `param` bahasyny Rust-da saklaýan klas goşýar. [Batch-lar](/docs/concepts/batches) sahypasyna serediň.
- `--as-json T` programmanyň diňe maglumat bilen dolduryýan tip parametrini (`paginate<T>(items: T[])` içindäki `T`) JSON bahalary hökmünde bridge edýär.
- `--keep "name: reason"` has köp eksporty çeşme dilinde galdyrýar, meselem `Intl` maglumatlaryna bagly kody.
- `--kind` meýilnamanyň saýlan bridge-ini üýtgedýär (TypeScript üçin napi-rs ýa-da wasm).

## Her dil üçin bridge

| Dil | Bridge |
| --- | --- |
| TypeScript/JavaScript | döredilen binar wire bilen napi-rs ýa-da wasm-bindgen; klaslar we async hem bar |
| Python | PyO3 (abi3), göni öwrülişik, tipli exception-lar |
| C | asyl atlary we header bilen `extern "C"` funksiýalary |
| Go | tipli cgo çagyryşlary we döredilen binar wire, asyl API-li paketde |
| Ruby | asyl modullaryň aşagyndaky magnus giňeltmesi |
| Java | tipli JNI we döredilen binar wire, asyl usullary bolan wrapper klaslarynda |

PHP we C# birliklerini entek bridge edip bolmaýar; [PHP](/docs/languages/php) we [C#](/docs/languages/csharp) sahypalaryna serediň.

## Bridge näme karar berýär

- Bridge-den geçip bilmeýän eksportlar sebäbi bilen çeşme dilinde galýar, shadow
  moduly bolsa asyl nusgany täzeden eksport edýär. TS üçin:
  funksiýa alýan ýa-da gaýtarýan funksiýalar, generator-lar we iterator-lar, RegExp, `instanceof` barlaglary,
  generic-ler (`--as-json T` programmanyň diňe maglumat bilen dolduryýan birini bridge edýär) we tipleri (importlar, täzeden eksportlar we `extends` arkaly yzarlanýar)
  funksiýa saklaýan bahalar. `--keep "name: reason"` has köpüni galdyrýar, meselem `Intl` maglumatlaryna bagly kody.
- TS tipleri Rust tiplerine laýyklaşdyrylýar, tip lakamlary importlar arkaly çözülýär, parametr tipleri
  adaty bahalardan çaklanýar; gurluşly bahalar döredilen binar wire hökmünde geçýär (JSON diňe
  wire-yň modellemeýän görnüşleri üçin fallback hökmünde).
- JSON `undefined`, NaN, Infinity, -0 ýa-da ýeke surrogate-lary aňladyp bilmeýär: gurluşly bahalar
  döredilen `index.cjs` faýlynyň terjime edýän belliklerini (`{"$undefined": true}`, `{"$num": "NaN"}`, `{"$utf16": [...]}`) göterýär,
  kömekçiler bolsa crate-iň `js.rs` faýlynda.

`record` TS çeşme faýllaryny bolşy ýaly işledýär: Node resolve hook-y bundler görnüşli importlara eýerýär
(`./errors`-dan `./errors.ts`-e), repozitoriýde hiç zat gurnalmadyk bolsa `--resolve-from` npm paketlerini
berýär, öwürmegi talap edýän TS (enum-lar, parameter properties) bolsa
`--experimental-transform-types` bilen täzeden synanyşylýar.
