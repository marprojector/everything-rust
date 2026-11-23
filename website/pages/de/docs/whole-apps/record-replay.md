---
title: Aufzeichnen und abspielen
description: app record führt die ganze App mit ihren dokumentierten Beispielen, deinen Eingaben und Fallen aus; app replay führt jeden beliebigen Build gegen die Fälle aus.
---

# Aufzeichnen und abspielen

`record` und `diff` beweisen eine Unit; `app record` und `app replay` beweisen die App, für CLIs und Server in jeder der sechs Sprachen. Die App ist eine Blackbox.

```sh
everything-rust app record . -- python3 cli.py          # der Befehl, der das Original ausführt
everything-rust app replay .                            # das Original muss zu 100% bestehen
everything-rust app replay . --build "target/release/mytool"
```

## Woher die Fälle kommen

`app record` führt das Original aus mit:

- den eigenen dokumentierten Beispielen der App: `$ mytool ...`-Zeilen in ihren Markdown- und Shell-Dateien, mit den Dateien, die sie nennen;
- einer `--inputs`-JSONL-Datei, ein `{"name", "args", "stdin", "env", "files"}` pro Zeile;
- semantischen Fallen, die aus diesen Fällen abgeleitet werden: Nicht-ASCII- und Emoji-Strings, negative Division und Modulo, Überlauf, große und winzige Floats, Map-Reihenfolge, fehlende Dateien.

Jeder Fall läuft zweimal; was sich zwischen den beiden Läufen unterscheidet, wird zu einem `[..]`-Platzhalter. `--normalize timestamps|uuids|key-order` oder `--normalize name=regex` fügt Normalisierer hinzu, die mit den Fällen gespeichert werden. `app record` meldet, was sich unterschied, nach Art: Werte (jetzt Platzhalter), Reihenfolge (Zeilen in beliebiger Reihenfolge verglichen) und Zeilen (ein `...`, das auf alles passt). Sieh dir jeden einzelnen an.

## Die Fälle sind einfache Dateien

Fälle sind [trycmd](https://docs.rs/trycmd)-Dateien in `rust/app-cases/`:

| Datei | Enthält |
| --- | --- |
| `<case>.toml` | argv, env, Exit-Code |
| `<case>.stdin` | Standardeingabe |
| `<case>.stdout`, `<case>.stderr` | was das Original ausgegeben hat |
| `<case>.in/` | Eingabedateien |
| `<case>.out/` | geschriebene Dateien |

Lies und bearbeite sie von Hand; ein Rust-Test kann sie mit `trycmd` ausführen, sobald die Portierung eine Binary hat.

## Abspielen

- `app replay` führt jeden beliebigen Build gegen die Fälle aus, gibt `passed/total` aus, schreibt `rust/app-cases/replay-report.md` mit einem Diff pro fehlschlagendem Fall und beendet sich bei jeder Abweichung mit einem Wert ungleich null.
- Ein Build, der an die in `EVERYTHING_RUST_FALLBACK_LOG` genannte Datei anhängt (eine Zeile pro Fallback auf das Original), wie es die [Front Door](/docs/whole-apps/front-door) tut, bekommt auch seine Fallback-Rate gemeldet.
- `app replay --case <name>` führt einen Fall unter dem Namen erneut aus, den der Bericht ihm gibt: die schnelle, wiederholbare Schleife, um eine Abweichung zu beheben, bevor wieder alles läuft.

## Abspielen in der CI

Der Bericht enthält keine Zeitmessungen oder temporären Pfade: Zwei Abspielläufe desselben Builds schreiben dieselbe Datei. Committe sie und führe dann in der CI aus:

```sh
everything-rust app replay .
git diff --exit-code rust/app-cases/replay-report.md
```

Eine halb portierte App lässt die CI dann bei jeder Änderung dessen, was besteht, fehlschlagen, ob besser oder schlechter, bis der Bericht entsprechend aktualisiert ist.

## Numerische Ausgabe

`app record --float-tolerance 1e-9` lässt Zahlen innerhalb dieses Anteils des größeren Werts übereinstimmen. Siehe [Float-Toleranz](/docs/concepts/float-tolerance).
