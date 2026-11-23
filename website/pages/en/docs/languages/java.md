---
title: Java
description: Port a package of public static methods to Rust through typed JNI and a generated binary wire, in wrapper classes with the original methods.
---

# Java

| | |
| --- | --- |
| Unit | a package of `public static` methods |
| Bridge | typed JNI and a generated binary wire, in wrapper classes with the original methods |
| Recorded and replayed by | a generated reflection driver |
| Needs | JDK 17+ (`javac` and `java`) |
| Status | Verified on JDK 21 behind a facade: 4,492/4,492 (226 throw), gate passes |

Batches are `new FnBatch(items)`; close them when done. `profile` uses JFR.

## What a port has to get right

- `Integer.parseInt` accepts any Unicode decimal digit (`١٢`, `２`) through `Character.digit`.
- `Double.toString` (JDK 19+) prints the shortest decimal, but never fewer than two digits: `4.9E-324`, where Rust's shortest form is `5e-324`.
- `\s` is ASCII-only; Rust's `split_whitespace` also splits at U+00A0.
- Floats must survive the round trip: `serde_json`'s default float parser is not always correctly rounded, so everything-rust and every crate it generates use its `float_roundtrip` feature.

## Speed

After the JIT, leaf functions take a few nanoseconds, and a JNI call with strings costs more than that. On the JDK facade, `bench` measured the port at 0.18x per call: port bigger units. As a batch on 4 cores, `slugify` over 50,000 strings (43 ms in Java) ran **3x faster** (the per-call port: 0.68x).

Details: [Six languages](/docs/real-world/six-languages).
