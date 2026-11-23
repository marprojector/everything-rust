---
title: Java
description: Migre um pacote de métodos public static para Rust através de JNI tipado e de um formato binário gerado, em classes wrapper com os métodos originais.
---

# Java

| | |
| --- | --- |
| Unidade | um pacote de métodos `public static` |
| Bridge | JNI tipado e um formato binário gerado, em classes wrapper com os métodos originais |
| Registado e reproduzido por | um driver de reflexão gerado |
| Requisitos | JDK 17+ (`javac` e `java`) |
| Estado | Verificado no JDK 21 por trás de uma fachada: 4,492/4,492 (226 lançam exceções), o gate passa |

Os lotes são `new FnBatch(items)`; feche-os quando terminar. O `profile` utiliza o JFR.

## O que uma migração tem de acertar

- `Integer.parseInt` aceita qualquer dígito decimal Unicode (`١٢`, `２`) através de `Character.digit`.
- `Double.toString` (JDK 19+) imprime o decimal mais curto, mas nunca com menos de dois dígitos: `4.9E-324`, enquanto a forma mais curta do Rust é `5e-324`.
- `\s` abrange apenas ASCII; o `split_whitespace` do Rust também divide em U+00A0.
- Os floats têm de sobreviver à ida e volta: o parser de floats predefinido do `serde_json` nem sempre arredonda corretamente, por isso o everything-rust e todas as crates que gera utilizam a sua feature `float_roundtrip`.

## Velocidade

Depois do JIT, as funções folha demoram poucos nanossegundos, e uma chamada JNI com strings custa mais do que isso. Na fachada do JDK, o `bench` mediu a migração a 0.18x por chamada: migre unidades maiores. Em lote em 4 núcleos, `slugify` sobre 50,000 strings (43 ms em Java) correu **3x mais rápido** (a migração por chamada: 0.68x).

Detalhes: [Seis linguagens](/docs/real-world/six-languages).
