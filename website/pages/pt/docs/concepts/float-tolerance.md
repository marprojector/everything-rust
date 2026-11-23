---
title: Tolerância de floats
description: Código numérico raramente produz floats idênticos bit a bit depois de migrado. --float-tolerance permite que os floats correspondam dentro de uma fração do valor maior.
---

# Tolerância de floats

```sh
everything-rust record <crate> --float-tolerance 1e-9
everything-rust app record . --float-tolerance 1e-9 -- python3 cli.py
```

Código numérico (física, estatística, gráficos) raramente produz floats idênticos bit a bit depois de migrado: a ordem das operações, o fused multiply-add e a biblioteca matemática diferem entre compiladores. Quando aceitar isso, registe com `--float-tolerance` e o seu valor. Os floats passam então a corresponder dentro dessa fração do valor maior (em termos absolutos abaixo de 1), e o `diff` e o `gate` utilizam-na. `0` remove-a.

O que continua a ter de corresponder exatamente:

- inteiros, texto e estrutura;
- o mesmo valor escrito de forma diferente (`3` e `3.0`);
- a formatação. A formatação de números para texto difere entre linguagens (`1e21`, `-0`, número de dígitos); as armadilhas incluem esses valores, e o `--float-tolerance` perdoa apenas o arredondamento, nunca a formatação.

Nunca alargue uma tolerância para fazer desaparecer uma divergência: uma diferença para além dela é um bug na migração.
