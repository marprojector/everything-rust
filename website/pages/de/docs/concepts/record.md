---
title: Record
description: record führt das Original mit generierten Randfällen, semantischen Fallen, seinen eigenen Tests und deinen Eingaben aus und speichert die Ergebnisse als cases.jsonl.
---

# Record

```sh
everything-rust record <crate>
everything-rust record <crate> --from-tests -- npx jest      # oder: -- pytest
everything-rust record <crate> --inputs extra.jsonl
```

`record` führt die ursprünglichen Funktionen aus und speichert, was jede zurückgibt oder wirft, in `cases.jsonl` neben dem Crate: das Verhalten, das die Portierung treffen muss. Eine spätere Abweichung ist ein Verhaltensunterschied: Korrigiere die Portierung, nie die Aufzeichnung.

## Woher die Fälle kommen

- **Generierte Randfälle**: Nicht-ASCII, Emoji, `-0`, `NaN`, Ints, wo Floats erwartet werden, und mehr (`--count` legt fest, wie viele pro Funktion).
- **Semantische Fallen** in jedem Parameter: Stringlänge und Groß-/Kleinschreibung, vorzeichenbehaftete Division, Überlaufgrenzen, Float-Ausgabe, Map- und Sortierreihenfolge.
- **Regex-Fallen** für jede Funktion in einer Datei, die Regexe nutzt: das türkische punktlose `ı`, das Kelvin-Zeichen, das lange s, arabisch-indische Ziffern, Zeilentrenner, finales Sigma, Zero-Width-Joiner.
- **Die eigenen Tests des Originals**: `--from-tests -- <test command>` zeichnet jeden Aufruf auf, den die Testsuite an die exportierten Funktionen der Unit macht (TS/JS mit jedem Runner, Python mit pytest oder unittest). Der Quellcode wird danach wiederhergestellt. Bei vercel/ms wurden 340 Aufrufe aus 167 Jest-Tests erfasst.
- **Deine Eingaben**: `--inputs cases.jsonl` mit einem `{"fn": name, "args": [...]}` pro Zeile. Füge echte Eingaben hinzu (Produktionsbeispiele, Fehlerberichte); die Nutzereingabe, die die Produktion lahmlegt, ist selten eine, auf die ein Generator kommt.

Python-Aufrufer übergeben `1` genauso oft wie `1.0`, daher übergibt die Aufzeichnung ganzzahlige Werte in jedem vierten Python-Fall als Ints.

## Markierungen in cases.jsonl

JSON kann `undefined`, NaN, Infinity, -0 oder einzelne Surrogate nicht ausdrücken. Strukturierte Werte tragen Markierungen, die das generierte `index.cjs` übersetzt, mit Helfern in der `js.rs` des Crates:

| Markierung | Bedeutet |
| --- | --- |
| `{"$undefined": true}` | JavaScript `undefined` |
| `{"$num": "NaN"}` | eine Zahl, die JSON nicht halten kann (`NaN`, `Infinity`, `-0`) |
| `{"$utf16": [...]}` | ein String als UTF-16-Codeeinheiten, für einzelne Surrogate |

C-Eingaben außerhalb des Vertrags einer Funktion können das Original abstürzen lassen. Der C-Treiber führt jeden Fall in einem geforkten Kindprozess aus und zeichnet `crash: SIGSEGV` auf; `diff` akzeptiert dort jedes Verhalten der Portierung, da das von C undefiniert ist.

## Zwei Läufe und was sich unterscheidet

`record` führt das Original zweimal aus und sortiert, was sich zwischen den Läufen unterschied: Geänderte Werte (Zeiten, IDs) werden zu Platzhaltern; dieselben Zeilen in anderer Reihenfolge (Hash-Maps, Nebenläufigkeit) werden in beliebiger Reihenfolge verglichen; eine andere Anzahl von Zeilen wird zu `...`, das auf alles passt. `record` listet jede Art auf. Sieh dir jeden Fall an, denn ein Unterschied durch Hash-Reihenfolge oder Nebenläufigkeit ist genau das, was sich in einer Portierung tatsächlich ändern kann.

## Die Aufzeichnung merkt sich ihr Original

`record` speichert einen Fingerabdruck der Dateien der Unit (`recorded-from.json`). Ändert sich das Original später, warnt `diff` und `gate` schlägt fehl, bis du erneut aufzeichnest, statt gegen das zu bestehen, was das Original früher tat. Siehe [Diff](/docs/concepts/diff#stale-recordings).

## Besonderheiten bei TypeScript

`record` führt TS-Quellen so aus, wie sie sind: Ein Node-Resolve-Hook folgt Imports im Bundler-Stil (`./errors` zu `./errors.ts`), `--resolve-from` liefert npm-Pakete, wenn im Repo keine installiert sind, und TS, das transformiert werden muss (Enums, Parameter Properties), wird mit `--experimental-transform-types` erneut versucht. TS, das nicht direkt in Node laufen kann, lässt sich aus der Build-Ausgabe mit `--module dist/x.mjs` oder mit `--node-arg=--import=tsx` aufzeichnen.

## Numerischer Code

Für Code, bei dem Floats selten Bit für Bit übereinstimmen, zeichne mit `--float-tolerance` auf. Siehe [Float-Toleranz](/docs/concepts/float-tolerance).
