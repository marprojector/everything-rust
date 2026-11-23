---
title: Gate
description: gate ist die Definition von „fertig“ für eine Portierung. Jede Prüfung, die es ausführt, und wie man eine fehlschlagende behebt.
---

# Gate

```sh
everything-rust gate <crate>
```

Fertig heißt, `gate` besteht. Es prüft:

- `cargo fmt`, clippy mit `-D warnings` und die cargo-Tests;
- `unsafe` nur mit `// SAFETY:`-Kommentaren und nichts mit `unimplemented!`;
- einen aufgezeichneten Fehlerfall für jede Stelle, an der das Original absichtlich fehlschlägt (`throw`, `raise`, ein `error`-Ergebnis). Füge einen mit `record --inputs` hinzu, wenn gate ihn nennt;
- kein `unwrap`, `expect` oder `panic!` in einer Funktion, deren Original einen Fehler zurückgibt. Gib `Err` zurück; für einen Zustand, den auch das Original als unmöglich behandelt, einen `// INFALLIBLE: <why>`-Kommentar;
- die Portierung höchstens 3x so viele Zeilen wie das Original (plus 50 Zeilen für Imports und Signaturen);
- das Modul baut, und jeder Export ist vorhanden;
- jeder aufgezeichnete Fall stimmt überein, [bekannte Abweichungen](/docs/concepts/diff#known-diffs) eingeschlossen, und die Aufzeichnung ist nicht [veraltet](/docs/concepts/diff#stale-recordings);
- für TS/JS eine Zeile zur [Abdeckung](/docs/concepts/coverage): wie viel vom Original die Fälle ausgeführt haben.

Jede fehlgeschlagene Prüfung trägt einen `fix`: Führe den zuerst aus.

## Dann berichten und warten

Wenn das Gate besteht, lässt der Workflow-Skill den Agent berichten, was sich geändert hat (Dateien, Funktionen, Bridge, Batches), die Gate-Ausgabe, die diff-Zahlen und bench-Werte, verbleibende Verhaltensunterschiede oder Risiken und was im Plan noch übrig ist, und vor der nächsten Unit auf die Freigabe warten.
