---
title: Java
description: Portiere ein Package mit public static-Methoden nach Rust, über typisiertes JNI und einen generierten binären Wire, in Wrapper-Klassen mit den ursprünglichen Methoden.
---

# Java

| | |
| --- | --- |
| Unit | ein Package mit `public static`-Methoden |
| Bridge | typisiertes JNI und ein generierter binärer Wire, in Wrapper-Klassen mit den ursprünglichen Methoden |
| Aufgezeichnet und abgespielt durch | einen generierten Reflection-Treiber |
| Braucht | JDK 17+ (`javac` und `java`) |
| Status | Verifiziert mit JDK 21 hinter einer Fassade: 4,492/4,492 (226 werfen), gate besteht |

Batches sind `new FnBatch(items)`; schließe sie, wenn du fertig bist. `profile` nutzt JFR.

## Was eine Portierung richtig machen muss

- `Integer.parseInt` akzeptiert über `Character.digit` jede Unicode-Dezimalziffer (`١٢`, `２`).
- `Double.toString` (JDK 19+) gibt die kürzeste Dezimaldarstellung aus, aber nie weniger als zwei Ziffern: `4.9E-324`, wo Rusts kürzeste Form `5e-324` ist.
- `\s` gilt nur für ASCII; Rusts `split_whitespace` trennt auch an U+00A0.
- Floats müssen den Hin- und Rückweg überstehen: Der Standard-Float-Parser von `serde_json` rundet nicht immer korrekt, daher nutzen everything-rust und jedes Crate, das es erzeugt, dessen Feature `float_roundtrip`.

## Geschwindigkeit

Nach dem JIT brauchen Blattfunktionen wenige Nanosekunden, und ein JNI-Aufruf mit Strings kostet mehr als das. Auf der JDK-Fassade maß `bench` die Portierung mit 0.18x pro Aufruf: Portiere größere Units. Als Batch auf 4 Kernen lief `slugify` über 50,000 Strings (43 ms in Java) **3x schneller** (die Portierung pro Aufruf: 0.68x).

Details: [Sechs Sprachen](/docs/real-world/six-languages).
