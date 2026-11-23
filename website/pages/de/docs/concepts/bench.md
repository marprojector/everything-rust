---
title: Bench
description: bench misst die aufgezeichneten Fälle auf dem Original und auf dem Rust, Bridge eingeschlossen, und gibt ein Urteil ab. Rust ist nicht immer schneller.
---

# Bench

```sh
everything-rust bench <crate>
```

`bench` misst die aufgezeichneten Fälle auf dem Original und auf dem gebauten Rust-Modul, Bridge eingeschlossen, und liefert die Zeilen und ein Urteil mit Empfehlung: ausliefern, die langsameren Funktionen in der Quellsprache belassen oder eine größere Unit portieren. Ein Speedup unter 1 bedeutet, dass die Portierung langsamer ist, so wie ihre Aufrufer sie aufrufen. `--min-ms` legt die minimale Messzeit pro Funktion und Seite fest (Standard 200).

Folge dem Urteil. Liefere keine Portierung aus, die Aufrufer langsamer macht, ohne das zu sagen.

## Warum eine Portierung langsamer sein kann

Eine Portierung, die einmal pro Element aufgerufen wird, bezahlt die Bridge einmal pro Element. Was entscheidet:

- **Interpretierte Originale (Python, Ruby)**: Der Aufruf nach Rust kostet etwa 100-300 ns, und der Rust-Code ist viel schneller als der Interpreter, daher gewinnen selbst kleine Funktionen.
- **Kompilierte Originale (Go, Java nach dem JIT)**: Die ursprüngliche Blattfunktion braucht 4-90 ns; ein einzelner cgo-Aufruf kostet allein etwa 80 ns, ein JNI-Aufruf mit Strings mehr. Diese Sprachen gewinnen nur, wenn ein Aufruf viel Arbeit leistet: Portiere eine Funktion zusammen mit ihren Aufrufern.
- **C**: Es gibt keinen Übergang; der Rust-Code wird direkt mit dem C-Code verglichen.

Pro Aufruf gemessen an echtem Code ([sechs Sprachen](/docs/real-world/six-languages)):

| Sprache | Rust vs. das Original (`bench`, geometrisches Mittel) | Urteil |
| --- | --- | --- |
| Ruby | **4.4x schneller** (3.6-7x pro Funktion) | ausliefern |
| Python | **1.35x schneller** (1.5-1.9x bei den meisten Aufrufen) | ausliefern |
| C | 0.87x: ctype 1.1-1.4x schneller, die optimierten `strtod`, `strspn`, `strcmp` von glibc 0.5-0.7x | optimieren oder die langsameren belassen |
| Java | 0.18x | größere Units portieren |
| Go | 0.04x | größere Units portieren |

Alle 29 Funktionen des Kerns von x18n, die sich ohne API-Änderung verlagern lassen, wurden portiert: Jedes Gate besteht und 7,925 aufgezeichnete Fälle stimmen exakt überein, und `bench` zeigte, dass die Portierungen, aus JS aufgerufen, langsamer als das TS sind, weil die Bridge mehr kostet als die Arbeit ([Bericht](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-core/README.md)). vercel/ms lief über seine aufgezeichneten Fälle mit 0.56x pro Aufruf ([Fallstudie](/docs/real-world/vercel-ms)).

## Was gewinnt: die Schleife verlagern

`profile` findet die Schleife um eine heiße Funktion, und `--batch` verlagert die ganze Schleife nach Rust, auf alle Kerne. Siehe [Batches und Hot Paths](/docs/concepts/batches).

::: tip Auf einem ruhigen Rechner messen
Führe `bench` auf einem ruhigen Rechner aus, führe es zweimal aus und nenne die Hardware und die Anzahl der Kerne neben jeder veröffentlichten Zahl.
:::
