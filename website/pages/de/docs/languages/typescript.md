---
title: TypeScript/JavaScript
description: Portiere Node.js-Code, TypeScript oder JavaScript, nach Rust über napi-rs oder wasm-bindgen, mit unveränderter API.
---

# TypeScript/JavaScript

| | |
| --- | --- |
| Unit | ein Ordner oder eine Datei |
| Bridge | napi-rs oder wasm-bindgen mit einem generierten binären Wire; Klassen und async inklusive |
| Aufgezeichnet und abgespielt durch | Node, das die `.ts`-Quellen ausführt |
| Braucht | Node 22.6+ (`profile` braucht Node 24+) |
| Status | Verifiziert |

## Die Bridge wählen

`analyze` wählt napi-rs, wenn die Unit oder etwas, das sie importiert, Node braucht, wasm-bindgen für portablen Code (der läuft auch im Browser), und belässt UI- und Editor-Code in JS. `bridge --kind napi|wasm` überschreibt das.

Exporte, die keine Bridge überqueren können, bleiben in TS, mit Begründung, und das Shadow-Modul exportiert das Original weiter: Funktionen, die Funktionen nehmen oder zurückgeben, Generatoren und Iteratoren, RegExp, `instanceof`-Prüfungen, Generics (`--as-json T` bridget eines, das die App nur mit Daten füllt) und Werte, deren Typen Funktionen enthalten. Siehe [Bridge](/docs/concepts/bridge).

## TS so aufzeichnen, wie es ist

`record` führt TS-Quellen direkt aus: Ein Node-Resolve-Hook folgt Imports im Bundler-Stil, `--resolve-from` liefert npm-Pakete, wenn im Repo keine installiert sind, und TS, das transformiert werden muss (Enums, Parameter Properties), wird mit `--experimental-transform-types` erneut versucht. `record --from-tests -- npx jest` zeichnet jeden Aufruf auf, den die eigene Testsuite des Originals macht, mit jedem Runner. Die Abdeckung des Originals wird für TS/JS gemessen ([Abdeckung](/docs/concepts/coverage)).

## Was eine Portierung richtig machen muss

Gefunden durch aufgezeichnete Fälle in echtem Code:

- `str.length`, `indexOf` und `slice` zählen UTF-16-Codeeinheiten, also belegt ein Emoji zwei Positionen. Nutze die generierten `utf16`-Helfer.
- `/.../i` ohne das Flag `u` faltet nur ASCII-Buchstaben, während Rusts `regex` Unicode faltet; so wären `"1 ſ"` (langes s) und das Kelvin-Zeichen in vercel/ms als Sekunden geparst worden.
- `Math.round` rundet Hälften auf (`-2.5` ergibt `-2`); Rusts `round` rundet von null weg. Das generierte `js.rs` enthält `js::math_round`.
- `${n}` gibt `1e+21`, `5e-324` und `0` für `-0` aus. `js::number_to_string` bildet das nach.
- `String.prototype.trim` nutzt JS-Whitespace, nicht Rusts `char::is_whitespace`.

## Geschwindigkeit

Pro Aufruf bezahlt eine Portierung jedes Mal die Bridge, und V8 führt kleine Funktionen in Nanosekunden aus. vercel/ms lief über seine aufgezeichneten Fälle mit 0.56x pro Aufruf; die Kern-Portierungen von x18n sind, aus JS aufgerufen, langsamer als das TS. Als Batches wandert die Schleife nach Rust:

| Arbeitslast | Original | Mit everything-rust |
| --- | ---: | ---: |
| Sidebar-Suche von x18n, 50,000 Keys (Node) | 10 ms pro Anfrage | 0.5 ms: 20x schneller |
| Dieselbe Suche im Browser (wasm, ein Thread) | 10 ms | 0.95 ms: 11.5x |
| Fuse.js, 50,000 Strings | 91.2 ms | 2.8 ms: 32x |
| vercel/ms `parse` x100,000 | 11.8 ms | 0.65 ms: 18x (3.4x mit Übergabe) |
| vercel/ms `format` x100,000 | 2.1 ms | 2.5 ms mit Übergabe: 0.8x |

Fallstudien: [json-server](/docs/real-world/json-server), [Fuse.js](/docs/real-world/fuse-js), [vercel/ms](/docs/real-world/vercel-ms), [x18n](/docs/real-world/x18n).
