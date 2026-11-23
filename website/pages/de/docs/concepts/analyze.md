---
title: Profilieren und analysieren
description: Finde mit profile heraus, wohin die Zeit geht, und plane dann mit analyze Units, Portierungsreihenfolge, Risiken und Crates.
---

# Profilieren und analysieren

## Profile: finden, was sich zu verlagern lohnt

Wenn es um Geschwindigkeit geht, miss, bevor du irgendetwas auswählst:

```sh
everything-rust profile <repo> -- node bench.mjs      # oder: python3 bench.py, ruby bench.rb,
                                                    #       java -cp out Main, go test -run X -bench . ./pkg
```

`profile` führt eine echte Arbeitslast unter dem Profiler der Sprache aus (node, cProfile, `go test` pprof, JFR, ein Ruby-Tracer) und listet die heißen Funktionen nach inklusiver Zeit, ihre Aufrufer und welche Schleifen als Batch laufen sollten. Go wird nur über `go test` profiliert; C wird nicht unterstützt.

- Eine kleine Funktion, die einmal pro Element aufgerufen wird, lohnt sich allein **nicht** zu portieren: Ein Aufruf nach Rust kostet mehr als ihre Arbeit, und JITs (V8, die JVM, Go) führen sie in Nanosekunden aus. Portiere sie zusammen mit ihren Aufrufern oder [mach einen Batch daraus](/docs/concepts/batches). Die Empfehlungszeile sagt, welches von beiden, mit dem genauen Befehl.
- Bereits kompilierter Code (numba `@njit`, Cython, `@jax.jit`) läuft als Maschinencode: Eine Rust-Portierung bringt kaum Geschwindigkeit. `profile` listet ihn unter „Already compiled“.

## Analyze: die Portierung planen

```sh
everything-rust analyze <repo> -o plan.md        # für Menschen
everything-rust analyze <repo> --json            # für Agents: Schema everything-rust/analyze@1
everything-rust analyze <repo> --whole-app       # plus Einstiegspunkte, Form, Abhängigkeiten, Entscheidungen
```

`analyze` listet Units, die Portierungsreihenfolge, Risiken mit `file:line`, die Bridge pro Unit, wie gut sich jede Abhängigkeit auf ein Crate abbilden lässt, und `start_here`: die besten ersten Portierungen. `--whole-app` ergänzt die Form der App (CLI, Server oder Bibliothek) und ihre Einstiegspunkte, jede Abhängigkeit (importiert oder deklariert) mit ihrem Crate, Stücke (Units, die sich gegenseitig importieren, wandern zusammen) und nummerierte Entscheidungen mit einer empfohlenen Antwort. Siehe [Eine ganze App portieren](/docs/whole-apps/overview).

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
## Crates für Abhängigkeiten

`analyze` nennt das Rust-Crate für jede Bibliothek, die der Code importiert, mit einer Passung von `good`, `partial` oder `none`. Eine in der Tabelle fehlende Bibliothek wird als `unknown` gemeldet; die Tabelle rät nie ein Crate. Wie du eines hinzufügst, steht unter [Mitwirken](/docs/project/contributing#add-a-crate-mapping).
