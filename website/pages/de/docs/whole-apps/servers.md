---
title: Server
description: Dieselben Befehle beweisen HTTP-Server. --http, --fresh, --arg und --pace, und eine axum-Front-Door, die jede Route weiterleitet, die keine Unit beantwortet.
---

# Server

Dieselben Befehle beweisen HTTP-Server. `--http` sagt, dass der Befehl einen Server startet; er bekommt seinen
Port aus `PORT` oder `{port}` im Befehl (`--port` für einen, der immer auf demselben
Port lauscht):

```sh
everything-rust app record . --http -- node server.js
everything-rust app replay .
everything-rust app front-door .                                    # axum: jede Route weitergeleitet
everything-rust app wire . api --route 'GET /users/{id}' --route '*'
everything-rust app lift .
```

- Anfragen kommen aus `curl`-Zeilen in der Doku der App (`curl -X POST -d ... http://localhost:8080/x`),
  aus `--inputs` (`{"name", "method", "path", "headers", "body"}`) und aus Fallen: Pfadsegmente und Query-Werte
  werden gegen dieselben Fallen wie CLI-Argumente getauscht, Bodies gegen Text- und JSON-Fallen, dazu eine
  unbekannte Route. Jeder Lauf startet den Server frisch und sendet die Anfragen in Namensreihenfolge, sodass der Zustand
  zwischen Anfragen erhalten bleibt.
- Ein Server, der Zustand in Dateien hält, bekommt bei jedem Start eine frische Kopie: `--fresh db.json` (eine Datei
  oder ein Ordner, wiederholbar) wird als Snapshot in `http/files/` abgelegt und jedes Mal in ein neues Arbeitsverzeichnis
  kopiert, wenn das Original, die Front Door oder ein Build startet; `--arg db.json` ist ein
  Argument, das jeder Start nach seinem Befehl bekommt, sodass die Rust-Binary so gestartet wird wie das Original.
  `--pace 50` wartet nach jeder Antwort 50 ms, für Arbeit, die ein Server nach dem Antworten erledigt (ein Datei-Watcher,
  der neu lädt, was er gerade geschrieben hat).
- Fälle sind `rust/app-cases/http/<case>.toml` (die Anfrage, der Status und die relevanten Header)
  und `<case>.body` (der Antwort-Body, mit `[..]` dort, wo sich zwei Läufe unterschieden). Date-, Server- und
  andere Header pro Antwort werden weggelassen; lösche jede Header-Zeile, die dir egal ist.
- Die Front Door ist ein axum-Server, der das Original auf einem eigenen Port startet und
  jede Route weiterleitet, die keine Unit beantwortet. `wire --route` nimmt axum-Routen: `METHOD /path/{param}`, `/path`
  für jede Methode, `*` für alles andere; der Glue-Code bekommt die Route, ihre Parameter und die
  Anfrage. `lift` entfernt den Proxy; HTTP-Fälle laufen dann in der CI mit `app replay`.
## Ein echtes Beispiel

[json-server](/docs/real-world/json-server) brauchte `--fresh`, `--arg` und `--pace`: Er schreibt seine Datenbank bei jedem Schreibzugriff neu, und sein Datei-Watcher kann einen Schreibzugriff rückgängig machen, der schnell darauf folgt.
