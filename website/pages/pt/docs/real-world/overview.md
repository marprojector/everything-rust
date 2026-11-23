---
title: Migrações reais
description: O que aconteceu quando o everything-rust encontrou código real, com os resultados lentos ao lado dos rápidos, e o que mudou na ferramenta por causa disso.
---

# Migrações reais

O que aconteceu quando o everything-rust encontrou código real, e o que mudou na ferramenta por causa disso. Cada número abaixo vem de um benchmark que primeiro verifica se os resultados são idênticos, e os casos lentos aparecem ao lado dos rápidos.

| Migração | O que é | Resultado |
| --- | --- | --- |
| [json-server](/docs/real-world/json-server) | um servidor Node completo, migrado do princípio ao fim com `app record`, `front-door`, `wire` e `lift` | um binário totalmente em Rust que passa 514/514 casos HTTP registados; seis unidades passam o `gate` |
| [Fuse.js](/docs/real-world/fuse-js) | pesquisa aproximada (fuzzy), o caminho onde uma pesquisa gasta o seu tempo | 3,414/3,414 casos coincidem; 32x mais rápido com 50,000 strings (5,3x num núcleo) |
| [vercel/ms](/docs/real-world/vercel-ms) | um pequeno parser de durações, do princípio ao fim | 975/975 casos coincidem; 0,56x por chamada, 18x como lote de `parse`, 0,8x como lote de `format` |
| [x18n](/docs/real-world/x18n) | o repositório do estudo de caso | todos os gates passam em `packages/core`, mais lento do que o TS através de uma bridge; a sua pesquisa 18-22x como lote |
| [Seis linguagens](/docs/real-world/six-languages) | `path` do Go, `shellwords` do Ruby, `colorsys` do Python, comportamento do JDK e da glibc por trás de pequenas fachadas | cada migração passa o seu gate, 13,010 casos registados no total |
| [Fixtures e execuções estáticas](/docs/real-world/fixtures) | fixtures escritas para pôr a ferramenta à prova, e bibliotecas analisadas mas não executadas | |

Migrou algo real? Um relato é bem-vindo: consulte [Contribuir](/docs/project/contributing#share-a-real-port).
