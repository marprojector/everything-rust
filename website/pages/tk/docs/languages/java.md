---
title: Java
description: public static usullaryndan ybarat paketi tipli JNI we döredilen binar wire arkaly, asyl usullary bolan wrapper klaslarynda Rust-a geçiriň.
---

# Java

| | |
| --- | --- |
| Birlik | `public static` usullaryndan ybarat paket |
| Bridge | tipli JNI we döredilen binar wire, asyl usullary bolan wrapper klaslarynda |
| Kim ýazga alýar we gaýtadan işledýär | döredilen reflection draýweri |
| Näme gerek | JDK 17+ (`javac` we `java`) |
| Ýagdaý | Fasadyň arkasyndaky JDK 21-de barlandy: 4,492/4,492 (226-sy exception taşlaýar), gate geçýär |

Batch-lar `new FnBatch(items)`; iş gutarandan soň olary ýapyň. `profile` JFR ulanýar.

## Portuň dogry etmeli zatlary

- `Integer.parseInt` `Character.digit` arkaly islendik Unicode onluk sifrini (`١٢`, `２`) kabul edýär.
- `Double.toString` (JDK 19+) iň gysga onluk görnüşi çap edýär, ýöne hiç haçan iki sifrden az däl: `4.9E-324`, Rust-yň iň gysga görnüşi bolsa `5e-324`.
- `\s` diňe ASCII; Rust-yň `split_whitespace` funksiýasy U+00A0-da hem bölýär.
- Float-lar iki tarapa geçişden üýtgemän çykmaly: `serde_json`-yň adaty float parser-i hemişe dogry tegeleklemeýär, şonuň üçin everything-rust we onuň döredýän her crate-i onuň `float_roundtrip` aýratynlygyny ulanýar.

## Tizlik

JIT-den soň ýaprak funksiýalary birnäçe nanosekunt alýar, setirli JNI çagyryşy bolsa ondan köp durýar. JDK fasadynda `bench` portuň bir çagyryşda 0.18x bolandygyny ölçedi: has uly birlikleri geçiriň. 4 ýadroda batch hökmünde 50,000 setir boýunça `slugify` (Java-da 43 ms) **3x has çalt** işledi (her çagyryş porty: 0.68x).

Jikme-jiklikler: [Alty dil](/docs/real-world/six-languages).
