---
title: Bibliotecas
description: Uma biblioteca não tem nenhum processo para reproduzir. A sua API pública é a front door, e os casos registados de cada unidade são a prova.
---

# Bibliotecas

Uma biblioteca não tem nenhum processo para reproduzir: a sua API pública é a front door, e os casos registados de cada
unidade são a prova.

```sh
everything-rust app front-door .        # rust/app: uma crate Rust com a API pública
# cada unidade passa pelo ciclo: bridge, record, port.rs, build, diff, gate; depois:
everything-rust app wire . textkit-rs   # reexportada a partir de rust/app; o original importa o seu shadow
everything-rust app replay .            # os casos de todas as unidades ligadas; fallback = exportações ainda no original
everything-rust app lift .
```

- O `app front-door` encontra a API pública (as exportações de todas as unidades) quando o repositório não tem `main`
  nem servidor (caso contrário, `--library`), e cria `rust/app` como uma crate de biblioteca. Cada unidade
  ligada passa a ser um módulo dela: `pub use textkit_rs::port as textkit;`.
- O `app wire` recebe uma crate criada pelo `bridge`, executa o `gate` e escreve o substituto shadow da unidade:
  o pacote original importa-o no lugar da unidade, de modo que quem o chama mantém a API original e
  chega ao Rust. `EVERYTHING_RUST_UNIT_<NAME>=original` devolve a unidade ao original (C: compilar sem as
  flags do shadow).
- O `app replay` executa os casos registados de todas as unidades ligadas através dos respetivos bindings. A taxa de fallback é
  a parte das exportações públicas a que o Rust ainda não responde: unidades não ligadas e exportações que o `bridge` mantém
  na linguagem original.
- O `app lift` exige fallback 0 e todas as unidades aprovadas no `gate`; remove os bindings, os wrappers e os
  shadows das unidades e deixa `rust/app` como uma única biblioteca em Rust puro. Não toca em nenhum
  `port.rs`, por isso os casos que coincidiram através dos bindings continuam a descrevê-lo; a lista de verificação
  avisa sobre ficheiros do original que ainda importam os shadows.

Os cinco são também ferramentas MCP: `app_record`, `app_replay`, `app_front_door`, `app_wire` e
`app_lift`.
