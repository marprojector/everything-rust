---
title: json-server
description: Die erste ganze App, durchgehend portiert. Eine Rust-Binary, die antwortet, was der Node-Server antwortet, 514/514 aufgezeichnete HTTP-Fälle.
---

# json-server

[json-server](https://github.com/typicode/json-server) (75k Sterne, MIT, Commit `89a34a4`,
1.0.0-beta.15) stellt eine REST-API aus einer JSON-Datei bereit: Node, tinyhttp und lowdb. Es ist die erste ganze
App, die mit dem Ablauf für ganze Apps (`app record`, `front-door`, `wire`, `lift`) durchgehend portiert wurde:
Das Ergebnis ist eine Rust-Binary, die antwortet, was der Node-Server antwortet. Alles liegt in
[`demo/real-world/json-server-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/json-server-rs).

| Schritt | Ergebnis |
| --- | --- |
| `analyze --whole-app` | ein Server, 9 Units (7 bereit, 2 mit Vorsicht), 13 zu ersetzende Abhängigkeiten (tinyhttp, lowdb, sirv, eta, milliparsec, dot-prop, sort-on, inflection...) |
| `app record --http` | 514 Fälle: 268 mit `--inputs` vorgegebene Anfragen (jede Route und Methode, die where-Operatoren, Sortierung, Paginierung, Embeds, CORS-Preflight, statische Dateien und Ranges, 29 fehlerhafte JSON-Bodies, `/__proto__` und andere Prototyp-Namen, fehlerhafte Escapes) und 245 Fallen; jede zweimal aufgezeichnet, zufällige IDs als `[..]` belassen |
| `front-door` | 513/514 bei 99.8% Fallback; der Fehltreffer ist ein rohes `ü` in der URL, das Node mit 400 beantwortet und der Proxy neu kodiert |
| Units | `where-operators` 207, `parse-where` 289, `matches-where` 1,346, `paginate` 853 Fälle (Aufrufe aus den eigenen Tests von json-server eingeschlossen); `service` 461 und `app` 243 über zwei kleine Adapter, die Service-Aufrufe und echte HTTP-Anfragen ausführen; `gate` besteht bei allen sechs |
| `wire` | zuerst die Lesezugriffe: 514/514 bei 22.6% Fallback, Node beantwortet die Schreibzugriffe und Rust lädt die Datei neu, die Node geschrieben hat; dann jede Route: 514/514 bei 0% |
| `lift` | eine reine Rust-Binary (5.3 MB), 514/514; die Kommandozeile von `bin.ts` von Hand portiert (Hilfe, Version, die Dateiprüfungen, das Startbanner) |

Nach dem Lift haben die Units keine Bindings mehr, daher spielt die `tests/recorded.rs` jedes Crates dessen
aufgezeichnete Fälle auf der Portierung ab: Alle 3,399 laufen mit `cargo test`.

Was die Portierung richtig machen musste, alles durch aufgezeichnete Fälle gefunden:

- Der Router von tinyhttp verteilt nicht nur einmal: Nach jedem `next()` gleicht er alle Middlewares und
  Routen erneut ab, auf der URL, die die letzte Middleware umgeschrieben hat, und führt für `HEAD` jede Route aus. So
  führt `HEAD /posts` den GET-Handler aus, dann den von POST, und antwortet mit dem 400 von POST; `//posts` überspringt die CORS-Header
  und den Body-Parser. Die Portierung führt diese Schleife so aus, wie sie ist.
- milliparsec parst jeden POST-, PUT-, PATCH- und DELETE-Body als JSON, egal welchen Typs, und ein fehlerhafter
  ist ein 500 mit der Meldung von V8: `Expected double-quoted property name in JSON at position 7
  (line 1 column 8)`, `Unexpected token 'x', ..."bbbbbbb": x}" is not valid JSON`. Die Portierung
  bildet die Parser-Fehler von V8 nach, Positionen in UTF-16-Einheiten eingeschlossen; geprüft gegen Node mit
  3,806 fehlerhaften Eingaben.
- Sortierung nutzt `localeCompare` (ICU), umgesetzt mit dem Collator von ICU4X und geprüft an 3,000 Paaren;
  `Number()`, `JSON.stringify` (`1e+21`, Ganzzahl-Keys zuerst), die zwei Query-String-Parser von Node
  (die `é%E9` anders dekodieren als WHATWG), dot-prop mit seinen Fehlermeldungen und inflection
  für Embeds (`person` und `people`) werden jeweils in `json-server-js` gegen Node geprüft.
- `db.data[name]` liest `Object.prototype`: `/__proto__` antwortet `{}`, `/constructor` ein leeres
  200, und `PATCH /__proto__` ersetzt den Prototyp der Datenbank, was spätere Zugriffe sehen.
- Der HTTP-Parser von Node antwortet bei rohen Nicht-ASCII-Zielen und unbekannten Methoden mit 400 und schließt die Verbindung.

In json-server gefunden: Nach jedem seiner eigenen Schreibzugriffe lädt sein Datei-Watcher die Datei neu. Wenn die
nächste Anfrage schreibt, bevor dieses Neuladen endet, kann das Neuladen diesen Schreibzugriff auslöschen: Die erste
Aufzeichnung, so schnell gesendet, wie der Server antwortete, hatte einen mit 201 beantworteten POST, den spätere Lesezugriffe nicht
zeigten. `app record --pace 50` wartet zwischen Anfragen, und die Fälle werden so aufgezeichnet. Die Portierung
lädt nur neu, wenn jemand anderes die Datei geändert hat, verliert diesen Schreibzugriff also nicht: ein bewusster
Unterschied.

Benchmark: Apple M4 (10 Kerne, 16 GB), macOS 27.3, Node 24.15.0 mit `lib/bin.js`, so wie
`tsc` es für npm baut, die Rust-Binary mit `--release` gebaut. Beide stellen eine Kopie derselben
Datenbank über Loopback bereit; `ab -k` sendet dieselben Anfragen, 20,000 pro Zeile (40 für das langsame Embed auf
der großen Datenbank) nach einer Aufwärmphase. Zwei Läufe, jeder gestartet mit einem 1-Minuten-Load-Average unter 3;
die Zellen geben Lauf 1 / Lauf 2 in Anfragen pro Sekunde an. [`bench/bench.py`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/json-server-rs/bench/bench.py)
ist das Skript.

Die Fixture-Datenbank (6 Posts, 3 Kommentare):

| Anfrage | Node, 1 Verbindung | Rust, 1 Verbindung | Node, 50 Verbindungen | Rust, 50 Verbindungen |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts` | 11.1k / 10.8k | 27.3k / 23.9k | 19.7k / 20.4k | 115.0k / 110.7k |
| `GET /posts/1` | 11.0k / 11.3k | 27.7k / 25.4k | 21.9k / 22.6k | 119.9k / 99.0k |
| filtern, sortieren, Seite | 10.5k / 10.0k | 25.2k / 21.5k | 18.3k / 16.4k | 107.5k / 96.3k |
| `?_embed=comments` | 8.7k / 6.6k | 22.5k / 21.8k | 13.8k / 11.0k | 98.3k / 100.6k |
| `GET /` (Indexseite) | 7.3k / 6.3k | 27.6k / 21.7k | 10.2k / 9.5k | 135.1k / 119.3k |
| `GET /test.html` (statisch) | 12.0k / 11.0k | 25.8k / 25.9k | 30.3k / 26.1k | 103.9k / 104.4k |
| `POST /comments`, 10 Verbindungen | | | 2,973 / 2,467 | 5,821 / 5,976 |

Eine generierte Datenbank (10,000 Posts, 20,000 Kommentare, 3.6 MB):

| Anfrage | Node, 1 Verbindung | Rust, 1 Verbindung | Node, 50 (10) Verbindungen | Rust, 50 (10) Verbindungen |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts/5000` | 8.4k / 9.2k | 14.6k / 14.8k | 15.5k / 16.1k | 69.5k / 67.4k |
| `?_page=3&_per_page=10` | 4.6k / 4.2k | 8.8k / 8.4k | 6.5k / 6.8k | 47.4k / 46.2k |
| `title:contains`, sortiert | 431 / 396 | 600 / 560 | 465 / 453 | 2,516 / 2,329 |
| `?_embed=comments`, eine Seite mit 5 (10 Verbindungen) | 2.4 / 2.4 | 81 / 103 | 2.4 / 2.2 | 253 / 216 |
| `GET /posts`, die ganze Liste (10 Verbindungen) | 328 / 319 | 399 / 407 | 381 / 387 | 1,514 / 1,657 |
| `POST /comments` (10 Verbindungen) | | | 236 / 241 | 292 / 264 |

| | Node | Rust |
| --- | ---: | ---: |
| Start bis zur ersten Antwort, Fixture-Datenbank | 85 / 79 ms | 9.9 / 7.9 ms |
| Start bis zur ersten Antwort, 10,000 Posts | 95 / 91 ms | 48 / 47 ms |
| Speicher (RSS) im Leerlauf, Fixture-Datenbank | 74 MB | 3.5 MB |
| Speicher (RSS) im Leerlauf, 10,000 Posts | 94 MB | 41 MB |
| Speicher (RSS) Spitze während der Läufe, Fixture-Datenbank | 424 / 390 MB | 18 MB |
| Speicher (RSS) Spitze während der Läufe, 10,000 Posts | 462 / 485 MB | 289 / 291 MB |

Wo der Abstand klein ist: eine Verbindung, die die ganze Liste mit 10,000 Posts liest (1.2x: beide verbringen die
Zeit damit, 1.5 MB JSON auszugeben), Schreibzugriffe auf die große Datenbank (1.1 bis 1.2x: beide schreiben bei jedem Schreibzugriff die ganze
3.6-MB-Datei neu, wobei Rust wartende Schreibzugriffe zusammenfasst, wie lowdb es tut) und die sortierte Titelsuche
über eine Verbindung (1.4x). Die großen Gewinne kommen aus der Nebenläufigkeit, die ein Node-Prozess
nicht hat, und aus dem Embed, bei dem json-server für jeden Post jeden Kommentar filtert (die Portierung
indiziert die Kommentare einmal pro Anfrage, gleiches Ergebnis).

Die erste geliftete Binary war auf der großen Datenbank langsamer als Node (33 gegenüber 298 Anfragen pro
Sekunde für die ganze Liste, 39 gegenüber 217 für Schreibzugriffe). Das Profiling fand, der Reihe nach: kein
`TCP_NODELAY` (jede große Antwort wartete etwa 40 ms auf ein verzögertes ACK), die ganze Datenbank wurde
vor jedem Schreibzugriff geklont, um `$schema` hinzuzufügen, jedes gelistete Element wurde vor der Ausgabe geklont, ein Lock
für alle Anfragen, und `item.id` wurde für jedes der 10,000 Elemente mit SipHash gehasht. Alles behoben; die 3,399
Unit-Fälle und 514 HTTP-Fälle bestehen nach jeder Änderung.

Im Tool gefunden und behoben:

- Ein Server, der seinen Zustand schreibt, ließ sich nicht zweimal vom selben Start aus aufzeichnen:
  `--fresh <path>` kopiert die Datenbank (und `public/`) jetzt bei jedem Start in ein frisches Arbeitsverzeichnis,
  aus einem Snapshot, der bei den Fällen liegt; `--arg` gibt jedem Build die Argumente des Originals;
  `--pace` steht oben.
- `bridge` beließ `paginate<T>(items: T[])` in TS (generisch über alles, Funktionen eingeschlossen);
  `--as-json T` bridget es als JSON-Werte.
- Die Routen der Front Door mit Parametern antworteten bei `/posts/%E0` mit dem eigenen 400 von axum, bevor die
  Unit die Anfrage sah; Parameter werden jetzt aus der URI gelesen.
- `lift` bereitete nur die verdrahtete Unit vor, nicht die Crates, von denen sie per Pfad abhängt, und schrieb eine `lib.rs`,
  die `cargo fmt --check` ablehnte; beides behoben.
- Die Server, die es erzeugt, setzten kein `TCP_NODELAY`: Eine in zwei Teilen geschriebene Antwort wartete
  auf das verzögerte ACK des Clients, etwa 40 ms. Sie setzen es jetzt.

Was sie nicht abdeckt: JSON5-Datenbanken (die Binary sagt das und beendet sich), das Body-Limit von 100 KiB
(das von Node hängt davon ab, wie der Body in 64-KiB-Lesevorgängen ankommt; die Portierung nimmt vollständige Lesevorgänge an), Strings
mit einzelnen Surrogaten (sie werden zu U+FFFD) und einen Datenbankwert von `1e400` (Node hält `Infinity`
bis zum nächsten Neuladen im Speicher; die Portierung speichert `null`, was auch in die Datei geschrieben wird).
