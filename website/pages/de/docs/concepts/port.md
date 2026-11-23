---
title: Port
description: port.rs schreiben, die eine Datei, die du oder dein Agent schreibt, und die Regeln, mit denen sie zum Original passt.
---

# Port

Du oder dein Agent schreibt die Portierung in `src/port.rs`. Alles andere im Crate wird generiert. Führe nach dem Ändern einer Signatur in `port.rs` `everything-rust refresh <crate>` aus.

## Regeln, die eine Runde diff sparen

- Lies jedes Risiko, das der Plan für die Unit auflistet, an seiner `file:line`, bevor du Code schreibst.
- **String-Offsets**: JS und Java indizieren UTF-16-Codeeinheiten, Rust UTF-8-Bytes. Nutze die generierte `src/utf16.rs`.
- **Fehler**: Gib `Err("TypeName: message")` zurück. Daraus wird diese Exception in Python, Ruby und Java und der `error`-Text in Go. Ein Rust-Panic stimmt nie mit einem aufgezeichneten Fehler überein.
- **Zahlen in Python und Ruby**: Ints, die das Original durchreicht, müssen als Ints zurückkommen. Nutze in Python-Portierungen `crate::pynum::Num` für Int-oder-Float, nicht `serde_json::Value`.
- **C**: Eingaben, die als `crash:` aufgezeichnet sind, sind undefiniertes Verhalten; jedes Ergebnis besteht.
- **Geordnete Maps und Objekt-Keys**: `indexmap`, wo die Reihenfolge die Ausgabe erreicht.
- **Regex mit Lookaround oder Rückverweisen**: `fancy-regex`, nicht `regex`. So oder so behandeln `\w`, `\b`, `\d` und Groß-/Kleinschreibung ignorierendes Matching Nicht-ASCII-Text anders als die Engine der Quellsprache. Setze `unicode(false)` oder schreibe ASCII-Klassen aus (`[A-Za-z0-9_]`), wo das `\w` des Originals nur ASCII kennt (JS ohne `u`, Java).
- **Kein `unsafe`.** Wenn wirklich nötig: minimal, isoliert, mit einem `// SAFETY:`-Kommentar.
- **Halte Helfer privat.** Nur die Exporte der Unit sind `pub`; ein `pub`-Helfer wird Teil der gebridgten API.
- Füge Rust-Unit-Tests in `port.rs` für die Fälle hinzu, über die du nachdenkst.

Die generierte `js.rs` enthält Helfer, die JavaScript nachbilden, wo Rust abweicht, etwa `js::number_to_string` (JS `String(n)`) und `js::math_round` (`Math.round`).

## Ein Beispiel: das Emoji, das eine Position verschob

`packages/core/src/fuzzy.ts` aus [x18n](https://github.com/x18n-dev/x18n), auf beide Arten portiert in
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo):

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: bereit, wasm-bindgen (core läuft im Browser)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# src/port.rs schreiben
everything-rust gate demo/fuzzy-rs
```

Eine erste Portierung, die über Rust-`char` indizierte, bestand einfache Tests, scheiterte aber an `diff`:

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

JS `indexOf` zählt UTF-16-Codeeinheiten, also belegt ein Emoji zwei Positionen. Die Portierung in
`demo/*/src/port.rs` nutzt die generierten `utf16`-Helfer und besteht 2000/2000 Fälle und das Gate,
als napi-rs (`demo/fuzzy-napi`) und als wasm-bindgen (`demo/fuzzy-rs`).

Der Plan, den everything-rust für ganz x18n schreibt: [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md).
## Wie groß sollte sie sein?

`gate` vergleicht die Zeilen der Portierung mit denen des Originals und schlägt oberhalb von 3x fehl (plus 50 Zeilen für Imports und Signaturen). Darüber bedeutet es meist, dass von Hand geschrieben wird, was ein Crate bereits tut, oder dass Fälle einzeln behandelt werden. `analyze` listet die zu verwendenden Crates.
