---
title: Echte Portierungen
description: Was passierte, als everything-rust auf echten Code traf, mit den langsamen Ergebnissen neben den schnellen, und was sich dadurch am Tool geändert hat.
---

# Echte Portierungen

Was passierte, als everything-rust auf echten Code traf, und was sich dadurch am Tool geändert hat. Jede Zahl unten stammt aus einem Benchmark, der zuerst prüft, dass die Ergebnisse identisch sind, und die langsamen Fälle stehen neben den schnellen.

| Portierung | Was es ist | Ergebnis |
| --- | --- | --- |
| [json-server](/docs/real-world/json-server) | ein ganzer Node-Server, durchgehend portiert mit `app record`, `front-door`, `wire` und `lift` | eine reine Rust-Binary, die 514/514 aufgezeichnete HTTP-Fälle besteht; sechs Units bestehen `gate` |
| [Fuse.js](/docs/real-world/fuse-js) | Fuzzy-Suche, der Pfad, auf dem eine Suche ihre Zeit verbringt | 3,414/3,414 Fälle stimmen überein; 32x schneller bei 50,000 Strings (5.3x auf einem Kern) |
| [vercel/ms](/docs/real-world/vercel-ms) | ein winziger Parser für Zeitdauern, durchgehend | 975/975 Fälle stimmen überein; 0.56x pro Aufruf, 18x als `parse`-Batch, 0.8x als `format`-Batch |
| [x18n](/docs/real-world/x18n) | das Repository der Fallstudie | jedes Gate besteht bei `packages/core`, über eine Bridge langsamer als TS; seine Suche 18-22x als Batch |
| [Sechs Sprachen](/docs/real-world/six-languages) | Gos `path`, Rubys `shellwords`, Pythons `colorsys`, Verhalten von JDK und glibc hinter kleinen Fassaden | jede Portierung besteht ihr Gate, insgesamt 13,010 aufgezeichnete Fälle |
| [Fixtures und statische Läufe](/docs/real-world/fixtures) | Fixtures, die geschrieben wurden, um das Tool zu fordern, und Bibliotheken, die analysiert, aber nicht ausgeführt wurden | |

Etwas Echtes portiert? Ein Bericht ist willkommen: siehe [Mitwirken](/docs/project/contributing#share-a-real-port).
