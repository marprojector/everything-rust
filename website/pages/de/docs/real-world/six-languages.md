---
title: Sechs Sprachen
description: everything-rust durchgehend auf echtem Code in Go, Ruby, Python, Java und C ausgeführt, mit Geschwindigkeit pro Aufruf und als Batch, und den Fehlern, die die Läufe gefunden haben.
---

# Echter Code in sechs Sprachen

everything-rust wurde durchgehend (analyze, bridge, record, port, diff, gate, bench) auf echtem Code in
jeder unterstützten Sprache ausgeführt. Der Code musste bereits auf diesem Rechner liegen: Aus dem
Internet heruntergeladener Code wurde nicht ausgeführt. Daher sind die Ziele Module der Standardbibliotheken, die mit den
installierten Toolchains mitkommen, und für Java und C, wo kein passender Bibliotheksquellcode auf der Platte lag, dienen das JDK
und glibc selbst als Orakel hinter einer dünnen Fassade.

Die Portierungen, aufgezeichneten Fälle und zusätzlichen Eingaben liegen in [`demo/real-world`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/README.md).

| Sprache | Code | Funktionen | Aufgezeichnete Fälle | Ergebnis |
| --- | --- | ---: | ---: | --- |
| Go | `path` aus der Standardbibliothek von Go 1.24 | 7 (+1 belassen) | 2,642 (542 aus Gos eigenen Testtabellen) | 2,642/2,642, gate besteht |
| Ruby | `shellwords.rb` aus Ruby 3.3 | 3 | 1,617 (446 lösen eine Exception aus) | 1,617/1,617, gate besteht |
| Python | `colorsys.py` aus CPython 3.13 | 6 | 2,400 | 2,400/2,400, gate besteht |
| Java | JDK 21 hinter einer Fassade (`Integer`, `Long`, `Math`, `Double.toString`, `URLEncoder`, `Base64`, `toUpperCase`) | 11 | 4,492 (226 werfen) | 4,492/4,492, gate besteht |
| C | glibc hinter einer Fassade (`strtol`, `atoi`, `strtod`, `strspn`, `strcmp`, ctype) | 9 | 1,860 (6 Abstürze) | 1,859/1,859 nach dem Entfernen eines Falls mit undefiniertem Verhalten, gate besteht |
| C | aklomp/base64 (in der Runtime von mypyc eingebunden) | 0 von 7 | - | nur Plan: jede Funktion nimmt Ausgabepuffer oder ein Zustands-Struct |

## Geschwindigkeit

Die ersten Bridges für Python, Go, Ruby und Java schickten jedes Argument und jedes Ergebnis als JSON-Text,
was 10-50x mehr kostete als die Arbeit in diesen Funktionen. Sie wurden so umgebaut, dass sie
Werte direkt konvertieren. `bench` misst jetzt nur die Aufrufe (Argumente einmal dekodiert, im Prozess) und sagt,
ob ausgeliefert werden soll.

| Sprache | Pro Aufruf vorher | Pro Aufruf jetzt | Rust vs. das Original (`bench`, geometrisches Mittel) | Urteil |
| --- | ---: | ---: | --- | --- |
| Ruby | 6.9-9.5 µs (Fiddle + JSON) | 0.36-1.15 µs (magnus-Erweiterung) | **4.4x schneller** (3.6-7x pro Funktion) | ausliefern |
| Python | 11.7 µs (JSON) | etwa 0.3 µs (direkte PyO3-Konvertierung) | **1.35x schneller** (1.5-1.9x bei den meisten Aufrufen) | ausliefern |
| C | - | kein Übergang: direkt gelinkt | 0.87x: ctype 1.1-1.4x schneller, die optimierten `strtod`, `strspn`, `strcmp` von glibc 0.5-0.7x | optimieren oder die langsameren belassen |
| Java | etwa 1.7 µs (JNI + JSON) | 30-800 ns (typisiertes JNI und binärer Wire) | 0.18x | größere Units portieren |
| Go | 2.6 µs (cgo + JSON) | 0.11-0.42 µs (typisiertes cgo und binärer Wire) | 0.04x | größere Units portieren |

Listen, Maps, Structs und mehrere Ergebnisse überqueren die Bridge in Go und Java als binärer Wire, der pro
Typ generiert wird: Gos `WordFreq` (eine Map) ging von 5.1 auf 1.0 µs, `Midpoint` (ein Struct) von 7.2 auf 1.0 µs;
Javas `frequencies` (eine `Map`) von 8.4 auf 1.0 µs und `words` (eine `List<String>`) von 6.0 µs auf
0.57 µs, 1.39x schneller als Java selbst.

TypeScript (x18n-Kern, neun Units, 27 Funktionen auf denselben Fällen): Der binäre Wire senkte die
Kosten der Bridge im Schnitt um ein Drittel (`rangeContains` 3.6 -> 1.1 µs, `applyTextEdits` 10.4 -> 5.7
µs, `toWireRange` 5.0 -> 2.6 µs), und `buildTree` ging von gleichauf zu 2.0x schneller als TS. Die
anderen Funktionen sind Blätter, die V8 in 15-600 ns ausführt: in Rust weiterhin langsamer, wie bei Go und Java.

Was entscheidet:

- **Interpretierte Originale (Python, Ruby)**: Der Aufruf nach Rust kostet etwa 100-300 ns, und der
  Rust-Code ist viel schneller als der Interpreter, daher gewinnen selbst kleine Funktionen.
- **Kompilierte Originale (Go, Java nach dem JIT)**: Die ursprüngliche Blattfunktion braucht 4-90 ns; ein einzelner
  cgo-Aufruf kostet allein etwa 80 ns, ein JNI-Aufruf mit Strings mehr. Keine Bridge kann eine 5-ns-Funktion
  schneller machen. Diese Sprachen gewinnen nur, wenn ein Aufruf viel Arbeit leistet: Portiere eine Funktion
  zusammen mit ihren Aufrufern. `bench` sagt das.
- **C**: Es gibt keinen Übergang; der Rust-Code wird direkt mit dem C-Code verglichen, und das Ergebnis
  ist eine Eigenschaft der Portierung.

### Batches: die Suche von x18n

Die Blatt-Portierungen oben werden pro Aufruf gemessen. Bei der Sidebar-Suche von x18n (50,000 Keys, Buchstabe
für Buchstabe getippt) zeigte `everything-rust profile`, dass `fuzzyScore` etwa ein Viertel des Laufs ausmacht, einmal pro Key aufgerufen von
`matched`, und schlug `--batch fuzzyScore:text` vor. Der generierte `FuzzyScoreBatch` hält die
Keys in Rust; ein `FuzzyScoreIndex` in `port.rs` wandelt sie einmal in UTF-16-Kleinbuchstaben um und hält eine 64-Bit-Maske
der Codeeinheiten in jedem Key, sodass die meisten Keys, die nicht passen können, ungelesen übersprungen werden. Die Ergebnisse
sind für jede Anfrage identisch mit der TS-Pipeline:

| | ms pro Anfrage | vs. TS |
| --- | ---: | ---: |
| TS-Pipeline | 10 | 1x |
| TS mit derselben Eingrenzung | 4.1-5.4 | 2x |
| Batch, Standardindex (Portierung pro Element, 4 Kerne) | 5.1 | 1.7x |
| Batch mit `FuzzyScoreIndex` (Masken), napi, 4 Kerne | 1.2 | 7x |
| Batch mit `FuzzyScoreIndex`, der selbst rankt, napi, 4 Kerne | 0.45-0.56 | 18-22x |
| Dasselbe, wasm, 1 Thread | 0.95 | 11.5x |

Der Schritt von 7x auf 20x: Von den 1.2 ms waren 0.4 ms das eigene `Array.from(order, i =>
keys[i])` des Benchmarks (eine einfache Schleife kostet 0.02 ms). Der Rest kam daher, dass der Index selbst ranken darf:
Er bewertet nur die Treffer der letzten Anfrage neu, wenn die neue Anfrage sie verlängert (ein Key, der nicht
auf `s` passt, kann nicht auf `se` passen), liest Ein-Buchstaben-Anfragen aus einer Tabelle erster Positionen, bewertet
ASCII-Keys als Bytes und ordnet die ganzzahligen Scores mit einem Counting Sort. Callgrind zeigte, dass
Bibliotheksaufrufe (`memchr`, `memcmp`) bei 30-Byte-Keys mehr kosten als einfache Schleifen. Ein Teil des Gewinns
ist der Algorithmus, daher führt der Benchmark auch TS mit derselben Eingrenzung aus: Rust ist 7-12x schneller
als das (wasm 5x).

Dasselbe Muster in den anderen Sprachen, gegen die Originale (4 Kerne, identische Ergebnisse und
Fehler):

| Sprache | Arbeitslast | Original | Portierung pro Aufruf vs. Original | Batch vs. Original |
| --- | --- | ---: | ---: | ---: |
| Python | Fixture `slugify`, 20,000 Strings | 98 ms | - | 17x |
| Ruby | Fixture `slugify`, 20,000 Strings | 110 ms | - | 12.7x |
| Ruby | `Shellwords.shellescape`, 50,000 gemischte Werte | 88 ms | - | 11.8x |
| Go | Fixture `Slugify`, 50,000 Strings | 49 ms | 0.93x | 3.3-4.1x |
| Java | Fixture `slugify`, 50,000 Strings | 43 ms | 0.68x | 3.0x |

Der Go-Batch lief zunächst langsamer als ein einzelner Thread: Die Portierung alloziert pro Element, und der Allocator von glibc
serialisierte die Threads im Go-Prozess. Batch-Crates nutzen jetzt mimalloc. Der shellescape-Batch
fand außerdem einen Fehler in der Portierung, den die Aufzeichnung pro Aufruf übersehen hatte: Ruby-Symbole (`shellescape(:sym)`) kamen
als ihre JSON-Markierung zurück. In der Portierung behoben.

Go und Java gingen dann von 2.5x und 1.7x auf 4x und 3x. Innerhalb des Batches war die Hälfte der Zeit nicht
die Arbeit: Ergebnisse wurden auf einem Thread zusammengefügt und freigegeben, Go kopierte sie ein zweites Mal, und Java
machte einen JNI-Aufruf pro String über modifiziertes UTF-8. String-Ergebnisse gehen jetzt gesammelt zurück, auf allen Kernen
in Gos eigenen Puffer oder ein einzelnes Java-`char[]` geschrieben. Die andere Hälfte waren die Portierungen: Auf einem
Thread war das Rust-`slugify` kaum schneller als das von Go und langsamer als Javas JIT. ASCII ohne Tabellenzugriffe
und ohne Zwischenstring in Kleinbuchstaben umzuwandeln behob das; Javas Portierung behält die Regel für das
finale Sigma bei, indem sie nur das durch Whitespace getrennte Stück, das ein `Σ` enthält, als Ganzes umwandelt.
Pro Aufruf ging die Go-Portierung von 0.73x auf 0.93x, die Java-Portierung von 0.46x auf 0.68x.

## Was die Läufe gefunden haben

### Fehler in everything-rust, behoben

- **Floats um eine ULP verändert** (Java-Lauf). Der Standard-Float-Parser von `serde_json` rundet nicht immer
  korrekt. everything-rust verfälschte aufgezeichnete Eingaben beim Lesen, und generierte Crates
  verfälschten sie erneut. 43 `Double.toString`-Fälle schlugen bei einer korrekten Portierung fehl. Behoben mit dem
  Feature `float_roundtrip` von `serde_json` in everything-rust und in jedem Crate, das es erzeugt (napi-rs,
  wasm und der JSON-Dispatcher).
- **C-Abstürze beendeten die Aufzeichnung** (C-Lauf). Eingaben außerhalb des Vertrags einer C-Funktion (`isalpha(INT_MAX)`)
  lassen glibc mit einem Segfault abstürzen. Der C-Treiber führt jetzt jeden Fall in einem geforkten Kindprozess aus und zeichnet
  `crash: SIGSEGV` auf; `diff` akzeptiert dort jedes Verhalten der Portierung, da das von C undefiniert ist.
- **`module_function :name` nicht erkannt** (Ruby-Lauf). `shellwords` deklariert seine Modulfunktionen
  mit Symbol-Argumenten nach den Methoden; der Exporter kannte nur die bloße Form `module_function`.
  Jetzt werden beide Formen gefunden.
- **Untypisierter Code bekam unbrauchbare Eingaben** (Python-Lauf). `colorsys` hat keine Type Hints, daher waren 1,709 von
  1,800 generierten Fällen `TypeError`s. Nicht annotierte Parameter, die als Zahlen verwendet werden (`r - g`,
  `max(r, g, b)`, `x < 1`), werden jetzt in Python und Ruby als `number` typisiert: 94 Fehler bleiben, alle echt
  (`ZeroDivisionError`).
- **Als Floats übergebene Ints wurden nie aufgezeichnet** (Python-Lauf). Python-Aufrufer übergeben `1` genauso oft wie
  `1.0`, und `colorsys` reicht Ints durch (`hsv_to_rgb(0, 0, 7)` ist `(7, 7, 7)`). Die Aufzeichnung übergibt jetzt
  ganzzahlige Werte in jedem vierten Python-Fall als Ints; das deckte 134 Abweichungen in einer
  als `f64` typisierten Portierung auf.
- **Meldungen aus der TypeScript-Zeit**: „must stay in JS“, „kept in TS“, „characterization tests
  against the TS code“ nennen jetzt die Quellsprache.
- **Wieder geöffnete Kernklassen** (`class String` in `shellwords.rb`) sind jetzt ein Monkeypatching-Risiko.

### Verhalten, das eine Portierung treffen muss, gefunden durch `diff`

Jede Zeile war eine Abweichung in einer ersten Portierung, gefunden durch die aufgezeichneten Fälle. Mit *Fixture* markierte Zeilen
stammen aus den kleinen Testprojekten in `tests/fixtures/`, für jede Sprache geschrieben; der Rest aus
den Läufen mit echtem Code oben.

- Go `path.Clean`: Gos `lazybuf` liest das Byte am Schreibindex, nachdem dieser zurückgesetzt wurde, nicht das
  letzte behaltene Byte (9 Fälle, aus Gos eigener Testtabelle).
- *Fixture* Gos `strings.ToLower` und Rubys `downcase` wandeln jede Rune einzeln um: `ΣΊΣΥΦΟΣ` wird zu `σίσυφοσ`,
  wo Rusts `to_lowercase` die Regel für das finale Sigma anwendet. Go bildet außerdem `İ` auf `i` ab, wo Rust
  `i̇` liefert.
- *Fixture* Go schneidet Strings nach Bytes; ein Schnitt mitten durch eine Rune kommt in JSON als ein U+FFFD pro Byte an.
- *Fixture* Rubys `slugify` löste bei einem negativen `max` einen `NoMethodError` aus (`slug[0, -1]`
  ist `nil`): ein Fehler im Original, den die Portierung beibehalten muss, oder die Aufrufer müssen zuerst korrigiert werden.
- *Fixture* Rubys `Array#sum` nutzt Kahan-Babuska-Summation und beginnt bei der Ganzzahl 0, daher ist `[-0.0].sum` gleich `0.0`.
- Rubys `Float#to_s` nutzt ab 1e15 die Exponentenform (`1e15.to_s` ist `1.0e+15`, `1e14.to_s` ist
  `100000000000000.0`); `nil.inspect` innerhalb von Arrays ist `nil`.
- *Fixture* Javas `\s` gilt nur für ASCII; Rusts `split_whitespace` trennt auch an U+00A0.
- Pythons Int-oder-Float-Ergebnisse: `colorsys` behält Ints bei `+`, `-` und `*`, daher rechnet die getreue Portierung
  mit einem kleinen Int-oder-Float-Typ statt mit `f64` (134 Fälle, sobald Ints aufgezeichnet wurden).
- glibc `strtod("-")` konvertiert nichts und gibt `+0.0` zurück, nicht `-0.0`.

In der ersten Portierung behandelt und durch die aufgezeichneten Fälle bestätigt:

- Javas `Integer.parseInt` akzeptiert über `Character.digit` jede Unicode-Dezimalziffer (`١٢`, `２`).
- Javas `Double.toString` (JDK 19+) gibt die kürzeste Dezimaldarstellung aus, aber nie weniger als zwei Ziffern:
  `4.9E-324`, wo Rusts kürzeste Form `5e-324` ist.
- Pythons `max(nan, 1, 2)` ist `nan` (der erste Wert gewinnt, außer ein späterer ist im Vergleich größer);
  `-0.0 % 1.0` ist `0.0`.
- glibc `toupper(-128)` ist `128` (seine Tabellen akzeptieren vorzeichenbehaftete `char`-Werte); `strtol` mit Basis 0
  liest `077` als Oktalzahl und `0x` ohne Hex-Ziffer als `0`.

### Grenzen, die diese Läufe gezeigt haben

- **C-APIs, die Puffer übergeben.** Die öffentlichen Funktionen von aklomp/base64 nehmen `char *out, size_t *outlen`
  oder ein `struct base64_state *`. Die C-Bridge transportiert nur Skalare und `const char *`, daher belässt sie
  alle in C, mit Begründung. Die Unterstützung des Musters mit Ausgabepuffer ist der nächste Schritt für C.
- **C-Makros.** 43 der 51 Dateien von base64 definieren Funktionen über Makros
  (`BASE64_ENC_FUNCTION(arch)`); der Parser meldet dort Fehler, und der Plan markiert diese Units als
  partiell. SIMD-Intrinsics werden noch nicht als Risiko ausgewiesen.
- **Undefiniertes Verhalten ohne Absturz.** `ispunct(1000)` liest über die Tabelle von glibc hinaus und
  gibt 1 zurück. Keine Portierung sollte das kopieren; der eine Fall wurde von Hand aus `cases.jsonl` entfernt. Es gibt
  noch keinen eingebauten Weg, einen Fall als undefiniert zu markieren.
- **Dynamische Typisierung kostet Code.** Die `shellwords`-Portierung implementiert Rubys `to_s` und `inspect`
  für Zahlen, nil, Arrays und Hashes nach, weil `shellescape` alles akzeptiert. Die `colorsys`-Portierung
  braucht Pythons numerischen Turm. Eine typisierte Portierung ist kürzer, ändert aber das Verhalten für manche
  Aufrufer; `diff` zeigt genau, für welche.
- **Die Fassaden.** Die Läufe für Java und C testen echtes Verhalten von JDK und glibc, aber über Fassaden,
  die für den Lauf geschrieben wurden, nicht über die eigene API einer Bibliothek.

## Wie jede Sprache läuft

| | Aufzeichnen | Diff | Bridge | Shadow |
| --- | --- | --- | --- | --- |
| Go | generierter Treiber, gebaut als eigenes Modul mit einem `replace` auf das Original | derselbe Treiber über dem cgo-Wrapper | JSON-Dispatcher + cgo-Package mit der ursprünglichen API | Package mit der ursprünglichen API, führt beide aus |
| Ruby | generierter Treiber lädt die Originaldateien | derselbe Treiber lädt den Fiddle-Wrapper | JSON-Dispatcher + Fiddle-Wrapper, der dieselben Module definiert | Datei, die nach dem Original per require geladen wird: `prepend`et beide |
| Java | Reflection-Treiber, kompiliert mit den Originalquellen | derselbe Treiber, kompiliert mit den JNI-Wrapper-Klassen | JSON-Dispatcher über JNI + Wrapper-Klassen mit den ursprünglichen statischen Methoden | `<Class>Shadow`-Klassen |
| Python | prozessinterner Treiber importiert das Original | derselbe Treiber importiert das PyO3-Paket | PyO3 | Modul mit der ursprünglichen API |
| C | `cc -shared` des Originals, aufgerufen über ctypes, ein Fork pro Fall | ctypes über dem Rust-`cdylib` | `extern "C"`-Funktionen mit den ursprünglichen Namen und einem Header | Shadow-`.c` mit `-D`-Umbenennungen; das Feature `shadow` des Crates exportiert nur `everything_rust_`-Namen |
