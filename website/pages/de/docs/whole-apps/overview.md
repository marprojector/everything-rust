---
title: Eine ganze App portieren
description: Von der ersten Aufzeichnung bis zu einer reinen Rust-Binary. Das Ziel ist eine Rust-App, die sich wie das Original verhält, wobei das Original als Referenz erhalten bleibt.
---

# Eine ganze App portieren

Das Ziel ist eine reine Rust-App, die sich wie das Original verhält, wobei das Original als Referenz erhalten bleibt. Dein Agent schreibt das Rust; everything-rust ordnet die Arbeit, legt das Gerüst an und beweist jeden Schritt. CLIs, HTTP-Server und Bibliotheken.

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in    pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down proved by
 decisions to make      outside         original answers                                           replay
```

## 1. Planen

```sh
everything-rust analyze . --whole-app
```

`--whole-app` ergänzt die Form der App (CLI, Server oder Bibliothek) und ihre Einstiegspunkte, jede externe Abhängigkeit (importiert oder deklariert) mit ihrem Rust-Crate und wie gut es passt, die Portierungsreihenfolge als Stücke (Units, die sich gegenseitig importieren, wandern zusammen) und nummerierte Entscheidungen mit einer empfohlenen Antwort: Units behalten, blockierte Units, Abhängigkeiten ohne oder mit nur teilweise passendem Crate.

## 2. Die ganze App aufzeichnen, dann den Einstiegspunkt verlegen

1. [`app record` und `app replay`](/docs/whole-apps/record-replay) beweisen das Programm von außen, als Blackbox.
2. [`app front-door`](/docs/whole-apps/front-door) stellt eine Rust-Binary vor das Original; jeder Befehl führt weiterhin das Original aus.
3. Jede Unit durchläuft [den Ablauf](/docs/concepts/the-loop), dann übergibt `app wire` ihre Befehle an Rust.
4. [`app lift`](/docs/whole-apps/lift) entfernt das Original, sobald das Abspielen mit Fallback-Rate 0 besteht und jede Unit gate besteht.

[Server](/docs/whole-apps/servers) und [Bibliotheken](/docs/whole-apps/libraries) nutzen dieselben Befehle.

## Wie der Agent sich hindurcharbeitet

Der [Whole-App-Skill](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) führt den Agent hindurch und beginnt mit den Entscheidungen, die nur du treffen kannst:

- **Grilling**: Der Agent fragt in Runden, jede Frage mit ihrer empfohlenen Antwort, bis jede Entscheidung geklärt ist, von der die Portierung abhängt, und schreibt jede als ADR in `rust/DECISIONS.md`, wobei `rust/GLOSSARY.md` die Begriffe des Originals auf Rust-Namen abbildet.
- **Recherche**: Hintergrund-Agents lesen die Primärquellen jeder Abhängigkeit und schreiben `rust/research/<topic>.md`, mit jeder semantischen Falle, die sie gefunden haben.
- **Über Sitzungen hinweg**: `rust/MAP.md` listet jede Unit mit ihrem Zustand; eine Sitzung beansprucht eine Unit, bevor sie beginnt, und jeder Checkpoint hinterlässt eine `rust/HANDOFF.md` für die nächste Sitzung. Units einer Welle können von Subagents parallel portiert werden.
- **Umstellung**: Der Agent erledigt, was er kann, und schreibt `rust/cutover.sh`, ein interaktives Skript für die Schritte, die nur du machen kannst (Deployments, DNS, Secrets), mit einem Weg zurück bei jedem Schritt.

Grilling, Recherche und diese Praktiken sind übernommen aus [Matt Pococks Skills](https://github.com/mattpocock/skills).

## Bindet das das Rust nicht an das alte Design?

Nur solange beide nebeneinander laufen. Die Aufzeichnungen prüfen das Verhalten an den Rändern der App (die Ausgabe einer CLI, die Antworten eines Servers, die exportierten Funktionen eines Moduls), nicht ihr Inneres. Spiegle die alte Struktur, solange Units noch die Bridge überqueren; sobald `app lift` das Original entfernt hat, strukturiere das Rust frei um, und `app replay` beweist weiterhin das Verhalten.

## Ein echtes Beispiel

[json-server](/docs/real-world/json-server) hat jeden Schritt durchlaufen: 514 aufgezeichnete HTTP-Fälle, eine Front Door bei 99.8% Fallback, zuerst die Lesezugriffe verdrahtet (22.6% Fallback), dann jede Route (0%), und ein Lift zu einer reinen Rust-Binary, die 514/514 besteht.
