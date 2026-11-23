---
title: Der Ablauf
description: profile, analyze, bridge, record, port, diff, bench, shadow und gate. Die Schritte, die jede Unit durchläuft.
---

# Der Ablauf

Jede Portierung, von einer heißen Funktion bis zu einer ganzen App, durchläuft dieselben Schritte, eine Unit nach der anderen. Eine Portierung ist erst fertig, wenn `gate` besteht.

Wie everything-rust eine Portierung prüft, was seine Befehle entscheiden und wo seine Grenzen liegen. Zurück zur [README](/).

## Wie es funktioniert

```
profile ──> analyze ──> bridge ──> record ──> port.rs ──> diff ──> bench ──> shadow ──> gate
 where the    what to     the Rust   what the    you or      every     faster?    both side   done means
 time goes    port first  crate and  original    your agent  recorded  the        by side in  this passes
                          wrapper    does        write it    case      verdict    production
```

Das Original bleibt die maßgebliche Quelle: `record` führt es mit Randfällen (und deinen eigenen Eingaben) aus und
speichert, was es zurückgibt oder wirft; `diff` spielt jeden Fall gegen das Rust ab; nichts wird ausgeliefert, bevor
alle übereinstimmen und `gate` besteht. Aufrufer behalten die ganze Zeit die ursprüngliche API.

Eine ganze App durchläuft denselben Ablauf Unit für Unit, innerhalb eines Rahmens, der das ganze
Programm beweist ([Eine ganze App portieren](/docs/whole-apps/overview)):

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop above, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in         pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down      proved by
 decisions to make      outside         original answers                                                replay
```

## Der Ablauf

| Schritt | Befehl | Was er tut |
| --- | --- | --- |
| 0. Profilieren | `everything-rust profile <repo> -- node bench.mjs` | Führt eine echte Arbeitslast unter dem Profiler der Sprache aus (node, cProfile, `go test` pprof, JFR, ein Ruby-Tracer): heiße Funktionen nach inklusiver Zeit, ihre Aufrufer und welche Schleifen als Batch laufen sollten; markiert bereits kompilierten Code (numba, Cython, JAX), bei dem Rust wenig bringt |
| 1. Planen | `everything-rust analyze <repo>` | Units, Portierungsreihenfolge, Risiken mit `file:line`, Bridge pro Unit, Passung von Abhängigkeiten zu Crates, beste erste Portierungen; `--whole-app` ergänzt Einstiegspunkte und Form, jede Abhängigkeit (importiert oder deklariert) mit ihrem Crate, Stücke (Units, die sich gegenseitig importieren, wandern zusammen) und nummerierte Entscheidungen mit einer empfohlenen Antwort |
| 2. Bridge | `everything-rust bridge <repo> <unit>` | Crate mit den Bindings (`lib.rs`, plus dem Wrapper, den die Quellsprache lädt) und Stubs zum Portieren (`port.rs`); `--batch fn:param` fügt eine Klasse hinzu, die viele `param`-Werte in Rust hält; `--as-json T` bridget einen Typparameter, den die App nur mit Daten füllt, als JSON-Werte |
| 3. Aufzeichnen | `everything-rust record <crate>` | Führt das Original mit generierten Randfällen aus (Nicht-ASCII, Emoji, `-0`, `NaN`, Ints, wo Floats erwartet werden, ...) und mit jeder semantischen Falle in jedem Parameter (Stringlänge und Groß-/Kleinschreibung, vorzeichenbehaftete Division, Überlaufgrenzen, Float-Ausgabe, Map- und Sortierreihenfolge) und speichert `cases.jsonl`; `--from-tests -- npx jest` (oder `-- pytest`) zeichnet zusätzlich jeden Aufruf auf, den die eigene Testsuite des Originals macht; `--float-tolerance 1e-9` lässt Floats bei numerischem Code innerhalb dieses Anteils übereinstimmen |
| 3. Bauen | `everything-rust build <crate>` | Baut die Rust-Seite: `.node`, wasm-Paket, Python-Erweiterung oder Shared Library |
| 3. Diff | `everything-rust diff <crate>` | Spielt die aufgezeichneten Fälle gegen Rust ab, aufgerufen so, wie die Aufrufer des Originals es aufrufen; listet Abweichungen |
| 3. Bench | `everything-rust bench <crate>` | Misst die aufgezeichneten Fälle auf dem Original und auf Rust, Bridge eingeschlossen |
| 4. Shadow | `everything-rust shadow <crate>` | Direkter Ersatz, der beide ausführt, das Ergebnis des Originals zurückgibt und Abweichungen meldet; `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. Gate | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, Tests, `unsafe`-Prüfung, nichts unportiert, ein aufgezeichneter Fehlerfall für jeden Fehlerpfad des Originals, kein `unwrap`/`expect`/`panic!`, wo das Original einen Fehler zurückgibt (außer markiert mit `// INFALLIBLE: <why>`), Build, API, Verhalten |
| Agents | `everything-rust setup` | Registriert den MCP-Server (`everything-rust mcp`, jeder Schritt als Tool) und den Workflow-Skill ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) bei deinen Coding-Agents |

## Hot Paths: die Daten in Rust halten

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

## Was `bridge` entscheidet

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

## Worauf `analyze` schaut

- Pakete (`package.json`) und Quelldateien; überspringt `node_modules`, Build-Ausgaben, Fixtures,
  Playgrounds, Tool-Konfigurationen. `<script>`-Blöcke in Vue/Svelte/Astro werden nach Imports geparst.
- **Units**: ein Ordner mit Quelldateien oder eine Datei im Quell-Wurzelverzeichnis eines Pakets.
- **Wellen**: Die Imports einer Unit kommen in früheren Wellen; Import-Zyklen werden zusammen portiert.
- **Bridge**: napi-rs, wenn die Unit oder etwas, das sie importiert, Node braucht; wasm-bindgen für portablen
  Code; in JS belassen für UI- und Editor-Code.
- **Bereitschaft** (wie sicher) und **Priorität** (sicher und lohnend: Schleifen und String-Arbeit, Tests, Größe).

| Risiko | Schweregrad | Beispiel |
| --- | --- | --- |
| String-Offsets | hoch | `slice`, `indexOf`, `matchAll`, das Flag `d`: UTF-16 vs. UTF-8 |
| Regex-Syntax | hoch | Lookaround, Rückverweise, `y`: nicht im Crate `regex` |
| Dynamische Regex | hoch | `new RegExp(userPattern)`: JS-Syntax zur Laufzeit |
| Dynamischer Code | hoch | `eval`, `new Function`, berechnetes `import()` |
| Dynamische Typisierung | mittel | `any`, `'key' in value` |
| Geordnete Collections | mittel | `Map`, `Set`, `Object.keys`: Einfügereihenfolge |
| Async, Generatoren | mittel | Grenzen von Promises und Generatoren |
| Klassenvererbung, Reflection | mittel | `extends`, `Proxy`, `Reflect`, `Symbol` |
| Callback-artige APIs | mittel | Parameter und Methoden mit Funktionstyp: gesprächige Bridges |
| Exceptions, Zahlen, JSON | niedrig | `throw`, bitweise Operationen, Reihenfolge bei `JSON.stringify` |

## Was es nicht erkennt

Bestehen heißt, das Rust stimmt mit jedem aufgezeichneten Fall überein, mehr nicht. Was Neufassungen zum Verhängnis wird,
liegt meist außerhalb der Aufzeichnung, also kenne die Grenzen:

- **Eingaben, die niemand aufgezeichnet hat.** Die generierten Fälle, die Fallen, die Testsuite des Originals
  (`record --from-tests`) und deine eigenen `--inputs` sind die Obergrenze. Füge echte Eingaben hinzu (Produktionsbeispiele,
  Fehlerberichte); die Nutzereingabe, die die Produktion lahmlegt, ist selten eine, auf die ein Generator kommt.
- **Ein Original, das sich weiterentwickelt hat.** `record` speichert einen Fingerabdruck der Dateien der Unit
  (`recorded-from.json`); ändert sich das Original später, warnt `diff` und `gate` schlägt fehl, bis du
  erneut aufzeichnest, statt gegen das zu bestehen, was das Original früher tat.
- **Regexe.** Das Crate `regex` hat weder Lookaround noch Rückverweise, und selbst ein Muster, das
  kompiliert, kann `\w`, ``, `\d` und Groß-/Kleinschreibung ignorierendes Matching bei Nicht-ASCII-Text anders behandeln
  als JS, Python oder Java. Ein Wechsel zu `fancy-regex` oder ein umgeschriebenes Muster kann jeden
  aufgezeichneten Fall bestehen und trotzdem bei einem türkischen punktlosen `ı` oder einem Zero-Width-Joiner abweichen. `record` fügt
  Regex-Fallen (diese, das Kelvin-Zeichen, das lange s, arabisch-indische Ziffern, Zeilentrenner, finales Sigma)
  zu jeder Funktion in einer Datei hinzu, die Regexe nutzt, aber fuzze deine echten Muster auch gegen echten Text.
- **Was sich zwischen Läufen ändert.** `record` führt das Original zweimal aus und sortiert, was sich unterschied:
  Geänderte Werte (Zeiten, IDs) werden zu Platzhaltern; dieselben Zeilen in anderer Reihenfolge (Hash-Maps,
  Nebenläufigkeit) werden in beliebiger Reihenfolge verglichen, sodass eine Portierung trotzdem genau diese Zeilen erzeugen muss; eine
  andere Anzahl von Zeilen wird zu `...`, das auf alles passt. `record` listet jede Art auf:
  Sieh dir jeden Fall an, denn ein Unterschied durch Hash-Reihenfolge oder Nebenläufigkeit ist genau das, was sich
  in einer Portierung tatsächlich ändern kann.
- **Floats.** Die Umwandlung von Zahlen in Text unterscheidet sich zwischen Sprachen (`1e21`, `-0`, Anzahl der Ziffern).
  Die Fallen enthalten diese Werte, und sie müssen exakt übereinstimmen; `--float-tolerance` verzeiht
  nur Rundung, nie Formatierung.
- **Alles, was nicht in der Ausgabe steht.** Laufzeit, Speicher, Nebenläufigkeit unter Last und andere Seiteneffekte
  als stdout, stderr, Exit-Codes, geschriebene Dateien und HTTP-Antworten (Netzwerkaufrufe, Datenbanken,
  Signale) werden nicht verglichen.
- **Andere Plattformen.** Eine Aufzeichnung beweist das Verhalten auf dem Rechner, der sie ausgeführt hat. C, dessen Ergebnisse
  von Typgrößen, Endianness oder dem Compiler abhängen, kann sich auf dem Zielsystem anders verhalten; `analyze`
  belässt hardwarenahes C aus diesem Grund in C.
## Jeder Schritt

| Schritt | Befehl | Was er tut |
| --- | --- | --- |
| 0. Profilieren | `everything-rust profile <repo> -- node bench.mjs` | Führt eine echte Arbeitslast unter dem Profiler der Sprache aus (node, cProfile, `go test` pprof, JFR, ein Ruby-Tracer): heiße Funktionen nach inklusiver Zeit, ihre Aufrufer und welche Schleifen als Batch laufen sollten; markiert bereits kompilierten Code (numba, Cython, JAX), bei dem Rust wenig bringt |
| 1. Planen | `everything-rust analyze <repo>` | Units, Portierungsreihenfolge, Risiken mit `file:line`, Bridge pro Unit, Passung von Abhängigkeiten zu Crates, beste erste Portierungen; `--whole-app` ergänzt Einstiegspunkte und Form, jede Abhängigkeit (importiert oder deklariert) mit ihrem Crate, Stücke (Units, die sich gegenseitig importieren, wandern zusammen) und nummerierte Entscheidungen mit einer empfohlenen Antwort |
| 2. Bridge | `everything-rust bridge <repo> <unit>` | Crate mit den Bindings (`lib.rs`, plus dem Wrapper, den die Quellsprache lädt) und Stubs zum Portieren (`port.rs`); `--batch fn:param` fügt eine Klasse hinzu, die viele `param`-Werte in Rust hält; `--as-json T` bridget einen Typparameter, den die App nur mit Daten füllt, als JSON-Werte |
| 3. Aufzeichnen | `everything-rust record <crate>` | Führt das Original mit generierten Randfällen aus (Nicht-ASCII, Emoji, `-0`, `NaN`, Ints, wo Floats erwartet werden, ...) und mit jeder semantischen Falle in jedem Parameter (Stringlänge und Groß-/Kleinschreibung, vorzeichenbehaftete Division, Überlaufgrenzen, Float-Ausgabe, Map- und Sortierreihenfolge) und speichert `cases.jsonl`; `--from-tests -- npx jest` (oder `-- pytest`) zeichnet zusätzlich jeden Aufruf auf, den die eigene Testsuite des Originals macht; `--float-tolerance 1e-9` lässt Floats bei numerischem Code innerhalb dieses Anteils übereinstimmen |
| 3. Bauen | `everything-rust build <crate>` | Baut die Rust-Seite: `.node`, wasm-Paket, Python-Erweiterung oder Shared Library |
| 3. Diff | `everything-rust diff <crate>` | Spielt die aufgezeichneten Fälle gegen Rust ab, aufgerufen so, wie die Aufrufer des Originals es aufrufen; listet Abweichungen |
| 3. Bench | `everything-rust bench <crate>` | Misst die aufgezeichneten Fälle auf dem Original und auf Rust, Bridge eingeschlossen |
| 4. Shadow | `everything-rust shadow <crate>` | Direkter Ersatz, der beide ausführt, das Ergebnis des Originals zurückgibt und Abweichungen meldet; `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. Gate | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, Tests, `unsafe`-Prüfung, nichts unportiert, ein aufgezeichneter Fehlerfall für jeden Fehlerpfad des Originals, kein `unwrap`/`expect`/`panic!`, wo das Original einen Fehler zurückgibt (außer markiert mit `// INFALLIBLE: <why>`), Build, API, Verhalten |
| Agents | `everything-rust setup` | Registriert den MCP-Server (`everything-rust mcp`, jeder Schritt als Tool) und den Workflow-Skill ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) bei deinen Coding-Agents |
## Weiter

Jeder Schritt hat eine eigene Seite: [Profilieren und analysieren](/docs/concepts/analyze), [Bridge](/docs/concepts/bridge), [Record](/docs/concepts/record), [Port](/docs/concepts/port), [Diff](/docs/concepts/diff), [Bench](/docs/concepts/bench), [Shadow](/docs/concepts/shadow) und [Gate](/docs/concepts/gate).
