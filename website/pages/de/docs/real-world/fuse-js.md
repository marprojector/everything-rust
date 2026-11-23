---
title: Fuse.js
description: Die String-Listen-Suche von Fuse.js in Rust. 3,414/3,414 aufgezeichnete Fälle stimmen überein, byte-identische Ergebnisse, 32x schneller bei 50,000 Strings.
---

# Fuse.js

[Fuse.js](https://github.com/krisk/Fuse) (20k Sterne, etwa 20 Millionen npm-Downloads pro Woche,
Commit `edf2fb6`) ist die Fuzzy-Search-Bibliothek hinter unzähligen Suchfeldern. Portiert mit
everything-rust 1.6 (unveröffentlicht); das Crate liegt in [`demo/real-world/fuse-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/fuse-rs).

`analyze` stellte jeden Ordner von `src/` in einen einzigen Import-Zyklus, daher lässt sich Fuse nicht Datei für Datei portieren.
Die Portierung nimmt stattdessen den Pfad, auf dem eine Suche ihre Zeit verbringt: Ein Adapter mit 54 Zeilen
(`adapter/relevance.ts`) exportiert Fuses eigenen Score für einen String einer String-Liste, gebaut aus
Fuses `BitapSearch`, der Feldnorm und `computeScoreSingle`. Diese Funktion führt eine Suche über
50,000 Strings 50,000-mal aus. `bridge --batch relevance:text` hält die Strings in Rust und
führt eine Suche über alle aus, auf allen Kernen.

| Schritt | Ergebnis |
| --- | --- |
| Adapter | das Ranking des Adapters entspricht `new Fuse(list).search(q)` bei 81 Anfragen und Optionssätzen, noch vor jeglichem Rust |
| `record --inputs` | 3,414 Fälle: Titel aus Fuses eigenen Fixtures, Nicht-ASCII, Emoji, Muster länger als 32 Einheiten, jede unterstützte Option und Optionen mit falschem Typ (abgelehnt, 149 Fälle) |
| Portierung | 673 Zeilen Rust für die 839 Zeilen Fuse, die sie ersetzt |
| `diff` | 3,414/3,414 stimmen beim ersten Build überein |
| `gate` | besteht |
| Ersatz | `withRust(Fuse)` beantwortet String-Listen-Suchen in Rust und reicht alles andere an Fuse weiter; es liefert bei 360 Suchen (12 Optionssätze, 15 Anfragen, mit und ohne `limit`) byte-identische Ergebnisse zu Fuse |

Suchzeit pro Anfrage, Apple M4 (10 Kerne), Node 24, Ergebnisse vor der Messung auf Gleichheit geprüft:

| Liste | Fuse.js | Rust | Speedup |
| ---: | ---: | ---: | ---: |
| 1,000 Strings | 1.9 ms | 0.1 ms | 18x |
| 10,000 Strings | 19.1 ms | 0.7 ms | 27 bis 30x |
| 50,000 Strings | 91.2 ms | 2.8 ms | **32x** |
| 50,000 Strings, ein Kern | 92.0 ms | 17.5 ms | 5.3x |

Was die Portierung richtig machen musste:

- Bitap läuft auf 32-Bit-JS-Ganzzahlen: `1 << 31` ist negativ, und `(1 << 31) - 1` ist ein Double, das
  beim nächsten Shift zu `2147483647` wird. Die Portierung hält diese Werte so, wie JS sie hat.
- Text wird in UTF-16-Codeeinheiten indiziert, also belegt ein Emoji zwei Positionen, wie in Fuse.
- `ignoreDiacritics` ist NFD plus eine Liste von Bereichen kombinierender Zeichen und ein paar ausgeschriebene Buchstaben
  (`ł`, `ß`, `ı`); die Portierung nutzt dieselbe Liste.
- `String.prototype.trim` entscheidet, welche Strings Fuse überspringt: JS-Whitespace, nicht Rusts
  `char::is_whitespace` (das U+0085 einschließt und U+FEFF auslässt).

Im Tool gefunden und behoben: Batch-Crates, deren Elemente Strings waren, deren Ergebnisse aber nicht,
bekamen einen ungenutzten Helfer und scheiterten an `clippy -D warnings`; und die Größenprüfung von `gate` zählte nur den
Adapter (54 Zeilen, also 12.5x „zu groß“) statt der Fuse-Dateien, die er importiert.

Was sie nicht abdeckt: Objektlisten mit `keys`, erweiterte und Token-Suche, logische Abfragen,
`includeMatches`, eine eigene `sortFn` oder `getFn`. Der Ersatz reicht diese unverändert an Fuse weiter. Der
Speedup setzt voraus, dass die Liste zwischen Suchen in Rust bleibt; sie aufzubauen kostet etwa so viel wie
Fuses eigener Index (3.4 ms gegenüber 2.8 ms bei 50,000 Strings).
