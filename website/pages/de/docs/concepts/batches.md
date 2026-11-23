---
title: Batches und Hot Paths
description: Eine Portierung, die einmal pro Element aufgerufen wird, verliert gegen einen JIT. Batches verlagern die ganze Schleife nach Rust, auf alle Kerne, und gewinnen selbst dort, wo Portierungen pro Aufruf verlieren.
---

# Batches und Hot Paths

```sh
everything-rust bridge <repo> <unit> --batch fn:param    # oder: everything-rust refresh <crate> --batch fn:param
```

`--batch` erzeugt eine Klasse, die die `param`-Werte einmal entgegennimmt und in Rust hält: `new FnBatch(items)` (TS, Python, Java), `Mod::FnBatch.new(items)` (Ruby), `NewFnBatch(items)` (Go). Pro Anfrage liefert `run(<other args>)` jedes Ergebnis als ein Array, und `rank(<other args>, min)` liefert die Indizes der Elemente, die besten zuerst. Es läuft auf allen Kernen, und die Fehler sind die des Originals. Jede Sprache außer C bekommt Batches.

Eine Portierung, die einmal pro Element aufgerufen wird, bezahlt die Bridge einmal pro Element und verliert gegen einen JIT. Was gewinnt, ist
die Schleife zu verlagern: `profile` findet sie, und `--batch` erzeugt eine Klasse, die die Elemente einmal entgegennimmt
und die Funktion pro Aufruf über alle ausführt, auf allen Kernen, mit Ergebnissen als Typed Arrays.

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore: ein Viertel des Laufs, pro Key von `matched` aufgerufen: als Batch ausführen
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

Ein `FuzzyScoreIndex`-Struct in `port.rs` ersetzt die Standardschleife durch eine, die die
Elemente einmal vorbereitet und selbst ranken kann (`rank`): Es hält die Keys in Kleinbuchstaben mit je einer Zeichenmaske,
eine Tabelle der ersten Position jedes häufigen Zeichens in jedem Key (eine Ein-Buchstaben-Anfrage ist eine
Spalte) und die Treffer der letzten Anfrage (wer nach `s` noch `se` tippt, bewertet nur neu, was `s` getroffen hat). Bei
der Sidebar-Suche von x18n über 50,000 Keys, Buchstabe für Buchstabe getippt, mit identischen Ergebnissen:

| | Pro Anfrage | vs. TS |
| --- | ---: | ---: |
| TS: `map(fuzzyScore)`, filtern, sortieren | 10 ms | 1x |
| TS mit derselben Eingrenzung wie der Index | 4.1-5.4 ms | 2x |
| Dieselbe Portierung, pro Key aufgerufen | - | 0.2-0.4x |
| `FuzzyScoreBatch`, Standardindex (4 Kerne) | 5.1 ms | 1.7x |
| `FuzzyScoreBatch` mit `FuzzyScoreIndex` (napi, 4 Kerne) | 0.45-0.56 ms | **18-22x** |
| Dasselbe mit wasm (ein Thread) | 0.95 ms | **11.5x** |

Gegenüber TS mit derselben Eingrenzung ist Rust immer noch 7-12x schneller (wasm 5x). Wandle die zurückgegebenen
Indizes mit einer einfachen Schleife in Keys um: `Array.from(order, i => keys[i])` kostete 20x mehr, 0.4 ms.

Jede Sprache bekommt Batches, und sie schlagen das Original selbst dort, wo Portierungen pro Aufruf verlieren. Dieselbe
Arbeit auf 4 Kernen, identische Ergebnisse und Fehler (Benchmarks in jedem Demo-Ordner):

| Sprache | Arbeitslast | Original | Portierung pro Aufruf vs. Original | Batch vs. Original |
| --- | --- | ---: | ---: | ---: |
| TS | x18n-Suche, 50,000 Keys | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`, 20,000 Strings | 98 ms | - | **17x** |
| Ruby | `slugify`, 20,000 Strings | 110 ms | - | **12.7x** |
| Ruby | `shellescape` (stdlib), 50,000 gemischte Werte | 88 ms | - | **11.8x** |
| Go | `Slugify`, 50,000 Strings | 49 ms | 0.93x | **4x** |
| Java | `slugify`, 50,000 Strings | 43 ms | 0.68x | **3x** |

Kompilierte Originale gewinnen weniger: Die Arbeit pro Element ist bereits schnell, also gewinnt der Batch um die Anzahl der
Kerne. Batch-Crates allozieren mit mimalloc: Mit dem System-Allocator war der Go-Batch langsamer als
ein einzelner Thread. Demo: [`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md).
## Einen schnellen Index schreiben

- Lass den Aufrufer den Batch einmal bauen, nicht pro Anfrage. Bilde zurückgegebene Indizes mit einer einfachen Schleife auf Elemente ab.
- Um Elemente einmal vorzubereiten (Kleinschreibung, Masken, Positionstabellen), definiere `pub struct FnIndex` in `port.rs` mit `new(items)` und `run(<other args>)`. Es muss Element für Element dasselbe Ergebnis wie die Funktion liefern.
- Wenn das Ranking Arbeit überspringen kann, gib dem Index ein eigenes `rank(<other args>, min)`. Es muss dem Ranking der Funktionsergebnisse entsprechen: Score von hoch nach niedrig, Gleichstände in Elementreihenfolge.
- Ein Batch multipliziert die Geschwindigkeit der Portierung pro Element mit den Kernen. Gegen kompilierte Originale (Go, Java) muss die Portierung selbst schnell sein: ASCII ohne Unicode-Tabellen behandeln, jedes Ergebnis in einem vorab dimensionierten `String` bauen, Zwischenkopien vermeiden.
- Schließe den Batch in Go und Java, wenn du fertig bist (Finalizer sind nur ein Rückfall).
