---
title: vercel/ms
description: vercel/ms durchgehend portiert. Korrekt bei jedem aufgezeichneten Fall, langsamer pro Aufruf, schneller nur, wenn ein Programm viele Zeitdauern auf einmal parst.
---

# vercel/ms

[vercel/ms](https://github.com/vercel/ms) (5.5k Sterne, Commit `4ff48ce`) wandelt `"2 days"` in
Millisekunden um und zurück; die meisten Node-Projekte ziehen es über irgendeine Abhängigkeit herein. Durchgehend portiert
mit everything-rust 1.5.1; das Crate liegt in
[`demo/real-world/vercel-ms-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/vercel-ms-rs).

| Schritt | Ergebnis |
| --- | --- |
| `analyze` | 1 Unit (`src/index.ts`), bereit, Bereitschaft 96 |
| `bridge --kind napi` | `ms`, `parse`, `parseStrict` und `format` gebridget, keine in TS belassen |
| `record --from-tests -- jest` | 975 Fälle: generierte, Fallen und 340 Aufrufe, erfasst aus den 167 Jest-Tests von vercel/ms (die erste Portierung nutzte 149 von Hand daraus kopierte Eingaben) |
| Portierung | 200 Zeilen Rust für 184 Zeilen TS, inklusive eines `FormatIndex`, der `options` einmal pro Batch liest |
| `diff` | 784/784 stimmten beim ersten Build überein; 975/975 mit den erfassten Aufrufen |
| `gate` | besteht (fmt, clippy, Tests, Fehlerpfade, Größe, Verhalten) |
| `bench`, pro Aufruf | 0.56x über die aufgezeichneten Fälle (anfangs 0.39x): `parse` auf einem normalen String braucht 79 ns, gleichauf mit JS, aber Randfälle und geworfene Fehler kosten über die Bridge weiterhin mehr |
| Batch, `parse` x100,000 | JS 11.8 ms, Rust 0.65 ms: **18x**; 3.4x mit der einmaligen Übergabe der Strings (2.1x, bevor Strings zusammengefügt übergeben wurden) |
| Batch, `format` x100,000 | JS 2.1 ms, Rust 2.5 ms mit der Übergabe: **0.8x** (anfangs 0.2x); `format(x, { long: true })` 1.2x |

Was die Portierung richtig machen musste, alles durch aufgezeichnete Fälle oder Fallen gefunden:

- `/.../i` ohne das Flag `u` faltet nur ASCII-Buchstaben, während Rusts `regex` Unicode faltet; so
  wären `"1 ſ"` (langes s) und das Kelvin-Zeichen als Sekunden geparst worden. Die Portierung gleicht Einheiten von
  Hand ab, nur ASCII.
- `Math.round` rundet Hälften auf (`-2.5` ergibt `-2`); Rusts `round` rundet von null weg.
- `${n}` gibt `1e+21`, `5e-324` und `0` für `-0` aus; die Portierung bildet die Zahl-zu-String-Umwandlung von JS nach.
  Ihre erste Version hatte außerdem einen Fehler bei Gleichständen (`25.728225708007812` als `...813` ausgegeben), den die
  ganzzahligen Ausgaben von ms nie erreichen.
- `str.length` zählt UTF-16-Einheiten, also zählt die Grenze von 100 Zeichen ein Emoji als zwei.

Im Tool gefunden und behoben: Eine napi-Batch-Methode mit einem strukturierten Parameter (`options` von `format`)
kompilierte nicht, da das generierte Binding einen `serde_json::Value` nahm, den napi
ohne sein Feature `serde-json` nicht konvertieren kann. Crates mit Batch-Klassen aktivieren es jetzt.

Aus dieser Portierung ins Tool übernommen:

- Die generierte `js.rs` enthält `js::number_to_string` und `js::math_round`, gegen Node geprüft mit
  20,000 zufälligen Doubles; die Portierung nutzt sie statt eigener Kopien. `number_to_string` nimmt
  seine Ziffern aus dem Printer von serde_json: 5x schneller als `format!` bei Ganzzahlen, 3x bei Brüchen.
- napi-Batches senden String-Elemente als einen zusammengefügten String mit UTF-16-End-Offsets, Zahlen-Elemente als
  ein `Float64Array`, und geben String-Ergebnisse zusammengefügt zurück (`index.cjs` trennt sie mit `slice`):
  2x schneller bei Strings und 20x bei Zahlen als ein JS-Wert pro Element. `new Batch(items)` und
  `run()` behalten ihre Form.
- `analyze` markiert `Math.round`, `toFixed` und `/i`-Regexe ohne `u`, die `s` oder `k` erwähnen,
  jeweils mit einer Anleitung.
- Aufrufe kosten weniger: Bei napi geht jede synchrone Funktion über den binären Wire ohne napi-Argumente
  (ein Argument kostet etwa 55 ns, der ganze Aufruf 22 ns), ein JS-Wrapper pro Funktion statt
  zwei, `serde_json::Value`-Parameter überqueren die Bridge als getaggte Werte beliebiger Form, und Fehler kommen
  im Wire-Puffer zurück. `parse` pro Aufruf sank von 121 auf 79 ns.
- `record --from-tests` ersetzt das Kopieren von Testeingaben von Hand: Es zeichnet jeden Aufruf auf, den die Suite
  während der Tests macht, wobei der Erfassungscode aus der Coverage herausgehalten wird (ms erzwingt 100%).

Das Urteil: `ms` ist eine gute Portierung und ein schlechtes Ziel für Geschwindigkeit. Korrekt bei jedem aufgezeichneten Fall, schneller
nur, wenn ein Programm viele Zeitdauern auf einmal parst.
