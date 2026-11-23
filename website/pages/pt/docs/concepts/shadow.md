---
title: Shadow
description: O shadow gera um módulo de substituição direta que executa o original e o Rust lado a lado e reporta as divergências.
---

# Shadow

```sh
everything-rust shadow <crate>
```

O `shadow` gera um substituto direto com a API do original: um módulo TS ou Python, um pacote Go, um ficheiro Ruby, classes Java ou um ficheiro C. Carregue-o onde a aplicação carregava o original. Executa ambos, devolve o resultado do original e reporta as divergências do Rust enquanto ganha confiança em produção.

| `EVERYTHING_RUST_MODE` | Comportamento |
| --- | --- |
| `shadow` (predefinição) | executa ambos, devolve o resultado do original, reporta as divergências |
| `new` | muda para o Rust |
| `old` | desliga o Rust |

A API pública não muda. Quem chama importa as mesmas funções, ou o módulo shadow no lugar delas.
