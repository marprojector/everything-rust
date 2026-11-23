---
title: Bridge
description: bridge legt ein Rust-Crate für eine Unit an, mit Bindings und einem Wrapper, sodass Aufrufer die ursprüngliche API behalten.
---

# Bridge

```sh
everything-rust bridge <repo> <unit-id>          # Crate in <repo>/rust/<name>; überschreibt nie
everything-rust refresh <crate>                  # nach dem Ändern einer Signatur in port.rs
```

`bridge` erstellt ein Crate mit den Bindings (`lib.rs`, plus dem Wrapper, den die Quellsprache lädt) und Stubs zum Portieren (`port.rs`). Du schreibst die Portierung nur in `src/port.rs`; `src/lib.rs` und die Wrapper werden generiert, und `refresh` erzeugt sie aus den Signaturen in `port.rs` neu und fügt neue Batch-Klassen hinzu. `port.rs` wird nie angefasst.

Optionen:

- `--batch fn:param` fügt eine Klasse hinzu, die viele `param`-Werte in Rust hält. Siehe [Batches](/docs/concepts/batches).
- `--as-json T` bridget einen Typparameter, den die App nur mit Daten füllt (`T` in `paginate<T>(items: T[])`), als JSON-Werte.
- `--keep "name: reason"` belässt weitere Exporte in der Quellsprache, z. B. Code, der von `Intl`-Daten abhängt.
- `--kind` überschreibt die Bridge, die der Plan gewählt hat (napi-rs oder wasm für TypeScript).

## Die Bridge pro Sprache

| Sprache | Bridge |
| --- | --- |
| TypeScript/JavaScript | napi-rs oder wasm-bindgen mit einem generierten binären Wire; Klassen und async inklusive |
| Python | PyO3 (abi3), direkte Konvertierung, typisierte Exceptions |
| C | `extern "C"`-Funktionen mit den ursprünglichen Namen und einem Header |
| Go | typisierte cgo-Aufrufe und ein generierter binärer Wire, in einem Package mit der ursprünglichen API |
| Ruby | eine magnus-Erweiterung unter den ursprünglichen Modulen |
| Java | typisiertes JNI und ein generierter binärer Wire, in Wrapper-Klassen mit den ursprünglichen Methoden |

PHP- und C#-Units lassen sich noch nicht bridgen; siehe [PHP](/docs/languages/php) und [C#](/docs/languages/csharp).

## Was bridge entscheidet

- Exporte, die keine Bridge überqueren können, bleiben in der Quellsprache, mit Begründung, und das Shadow-Modul
  exportiert das Original weiter. Für TS:
  Funktionen, die Funktionen nehmen oder zurückgeben, Generatoren und Iteratoren, RegExp, `instanceof`-Prüfungen,
  Generics (`--as-json T` bridget eines, das die App nur mit Daten füllt) und Werte, deren Typen (verfolgt über Imports, Re-Exporte und `extends`)
  Funktionen enthalten. `--keep "name: reason"` belässt weitere, z. B. Code, der von `Intl`-Daten abhängt.
- TS-Typen werden auf Rust-Typen abgebildet, wobei Typ-Aliase über Imports aufgelöst und Parametertypen
  aus Standardwerten abgeleitet werden; strukturierte Werte überqueren die Bridge als generierter binärer Wire (JSON nur
  als Rückfall für Formen, die der Wire nicht modelliert).
- JSON kann `undefined`, NaN, Infinity, -0 oder einzelne Surrogate nicht ausdrücken: Strukturierte Werte tragen
  Markierungen (`{"$undefined": true}`, `{"$num": "NaN"}`, `{"$utf16": [...]}`), die das generierte
  `index.cjs` übersetzt, mit Helfern in der `js.rs` des Crates.

`record` führt TS-Quellen so aus, wie sie sind: Ein Node-Resolve-Hook folgt Imports im Bundler-Stil
(`./errors` zu `./errors.ts`), `--resolve-from` liefert npm-Pakete, wenn im Repo keine
installiert sind, und TS, das transformiert werden muss (Enums, Parameter Properties), wird mit
`--experimental-transform-types` erneut versucht.
