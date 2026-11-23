---
title: Go
description: Portiere ein Go-Package nach Rust, über typisierte cgo-Aufrufe und einen generierten binären Wire, in einem Package mit der ursprünglichen API.
---

# Go

| | |
| --- | --- |
| Unit | ein Package |
| Bridge | typisierte cgo-Aufrufe und ein generierter binärer Wire, in einem Package mit der ursprünglichen API |
| Aufgezeichnet und abgespielt durch | einen generierten Go-Treiber, gebaut als eigenes Modul |
| Braucht | Go 1.21+ |
| Status | Verifiziert mit `path` aus Go 1.24: 2,642/2,642 (542 aus Gos eigenen Testtabellen), gate besteht |

`profile` arbeitet über `go test`: `everything-rust profile <repo> -- go test -run X -bench . ./pkg`. Batches sind `NewFnBatch(items)`; schließe sie, wenn du fertig bist.

## Was eine Portierung richtig machen muss

- `path.Clean`: Gos `lazybuf` liest das Byte am Schreibindex, nachdem dieser zurückgesetzt wurde, nicht das letzte behaltene Byte (9 Fälle, aus Gos eigener Testtabelle).
- `strings.ToLower` wandelt jede Rune einzeln um: `ΣΊΣΥΦΟΣ` wird zu `σίσυφοσ`, wo Rusts `to_lowercase` die Regel für das finale Sigma anwendet. Go bildet außerdem `İ` auf `i` ab, wo Rust `i̇` liefert.
- Go schneidet Strings nach Bytes; ein Schnitt mitten durch eine Rune kommt in JSON als ein U+FFFD pro Byte an.

## Geschwindigkeit

Go führt Blattfunktionen in wenigen Nanosekunden aus, und ein einzelner cgo-Aufruf kostet allein etwa 80 ns. Bei `path` maß `bench` die Portierung mit 0.04x pro Aufruf: Portiere größere Units. Als Batch wandert die Arbeit nach Rust, auf alle Kerne: `Slugify` über 50,000 Strings brauchte in Go 49 ms, und der Batch lief **4x schneller** (die Portierung pro Aufruf: 0.93x). Batch-Crates allozieren mit mimalloc: Mit dem System-Allocator war der Go-Batch langsamer als ein einzelner Thread.

Details: [Sechs Sprachen](/docs/real-world/six-languages).
