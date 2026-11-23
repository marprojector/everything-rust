---
title: Was ist everything-rust
description: everything-rust zeichnet auf, was deine App heute tut, und prüft jedes Stück der Rust-Neufassung gegen diese Aufzeichnung, bis die App reines Rust ist.
---

# Was ist everything-rust

<p class="lead">Schreib deine ganze App in Rust neu, ein Stück nach dem anderen. everything-rust zeichnet auf, was deine Node.js- (TypeScript/JavaScript), Python-, C-, Go-, Ruby- oder Java-App heute tut, und prüft dann jedes Stück der Rust-Neufassung gegen diese Aufzeichnung, bevor es weitergeht, bis die App reines Rust ist.</p>

Es funktioniert mit CLIs, HTTP-Servern und Bibliotheken. Dein Coding-Agent (Claude Code, Codex, Cursor, Gemini CLI) oder du schreibst das Rust. PHP und C# sind in der [Vorschau](/docs/languages/php).

## Was es tut und was nicht

everything-rust übersetzt keinen Code. Es liefert, was ein Agent nicht zuverlässig erraten kann:

- **wohin die Zeit geht**: `profile` führt eine echte Arbeitslast aus und ordnet die heißen Funktionen;
- **was zuerst portiert wird und in welcher Reihenfolge**: `analyze` plant Units, Wellen, Risiken und Crates;
- **eine Bridge**, damit der restliche Code das Rust mit unveränderter API aufruft (`bridge`);
- **das genaue Verhalten, das die Portierung treffen muss**: `record` führt das Original mit Randfällen aus und speichert, was es zurückgibt oder wirft;
- **einen sicheren Weg zur Auslieferung**: `shadow` lässt beide nebeneinander laufen;
- **ein Gate, das sagt, wann eine Portierung fertig ist**: `gate`.

Das Original bleibt die maßgebliche Quelle. `diff` spielt jeden aufgezeichneten Fall gegen das Rust ab, und nichts wird ausgeliefert, bevor alle übereinstimmen und `gate` besteht.

## Vier Schritte

| Schritt | Was passiert |
| --- | --- |
| **Aufzeichnen** | `record` führt das Original mit generierten Randfällen, semantischen Fallen und deinen eigenen Eingaben aus und speichert, was es zurückgibt oder wirft. |
| **Portieren** | `bridge` erzeugt das Rust-Crate und einen Wrapper, sodass Aufrufer die ursprüngliche API behalten. Du oder dein Agent schreibt das Rust. |
| **Prüfen** | `diff` spielt jeden aufgezeichneten Fall gegen das Rust ab, und nichts wird ausgeliefert, bevor alle übereinstimmen und `gate` besteht. |
| **Lift** | Wenn das Abspielen ohne Fallback besteht und jede Unit gate besteht, macht `app lift` daraus eine reine Rust-App. Das Original wird nie angefasst. |

## Ist Rust immer schneller?

Nein, und everything-rust sagt das auch. Eine kleine Funktion, die einmal pro Element aufgerufen wird, ist in Rust oft langsamer, weil jeder Aufruf eine Bridge überquert; V8, die JVM und Go führen solche Funktionen in Nanosekunden aus. `profile` findet die Schleife drumherum, `--batch` verlegt die ganze Schleife, und `bench` gibt ein Urteil ab, bevor du auslieferst. Gemessene Zahlen, die langsamen eingeschlossen, findest du unter [Batches und Hot Paths](/docs/concepts/batches).

## Es erkennt nur, was die Aufzeichnung abdeckt

Bestehen heißt: Das Rust stimmt mit jedem aufgezeichneten Fall überein, mehr nicht. Lies [Was es nicht erkennt](/docs/concepts/limits), bevor du einem grünen Gate vertraust.

::: warning Auf einer Kopie ausführen
`record` und `app record` führen deinen ursprünglichen Code aus, rufen jede Funktion (oder die ganze App) mit vielen generierten Eingaben auf, und `app record` führt jeden Fall zweimal aus. `diff`, `gate` und `app replay` führen dieselben Fälle erneut aus, auf der Portierung oder dem Original. Code, der Dateien schreibt, mit einer Datenbank spricht oder das Netzwerk aufruft, tut das einmal pro Fall. Führe sie auf einer Kopie des Projekts aus, mit einer Testdatenbank und ohne Produktionszugangsdaten in der Umgebung.
:::

## Nächste Schritte

- [Schnellstart](/docs/introduction/quick-start): installieren und deinen Agent um eine erste Portierung bitten.
- [Der Ablauf](/docs/concepts/the-loop): jeder Schritt für eine Unit.
- [Eine ganze App portieren](/docs/whole-apps/overview): von der ersten Aufzeichnung bis zu einer reinen Rust-Binary.
- [Echte Portierungen](/docs/real-world/overview): json-server, Fuse.js, vercel/ms und mehr, mit den langsamen Ergebnissen neben den schnellen.
