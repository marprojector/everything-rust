---
title: Lotes e caminhos críticos
description: Uma migração chamada uma vez por item perde para um JIT. Os lotes movem o ciclo inteiro para o Rust, em todos os núcleos, e ganham mesmo onde as migrações por chamada perdem.
---

# Lotes e caminhos críticos

```sh
everything-rust bridge <repo> <unit> --batch fn:param    # ou: everything-rust refresh <crate> --batch fn:param
```

O `--batch` gera uma classe que recebe os valores de `param` uma vez e os mantém no Rust: `new FnBatch(items)` (TS, Python, Java), `Mod::FnBatch.new(items)` (Ruby), `NewFnBatch(items)` (Go). Por consulta, `run(<other args>)` devolve todos os resultados como um único array, e `rank(<other args>, min)` devolve os índices dos itens, os melhores primeiro. Corre em todos os núcleos, e os erros são os do original. Todas as linguagens exceto C têm lotes.

Uma migração chamada uma vez por item paga a bridge uma vez por item e perde para um JIT. O que ganha é
mover o ciclo: o `profile` encontra-o, e o `--batch` gera uma classe que recebe os itens uma vez
e executa a função sobre todos eles em cada chamada, em todos os núcleos, com resultados em typed arrays.

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore: um quarto da execução, chamada por chave por `matched`: processe-a em lote
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

Uma struct `FuzzyScoreIndex` em `port.rs` substitui o ciclo predefinido por um que prepara os
itens uma vez e consegue ordenar por si só (`rank`): mantém as chaves em minúsculas, cada uma com uma
máscara de caracteres, uma tabela da primeira posição de cada carácter comum em cada chave (uma consulta
de uma letra é uma coluna) e as correspondências da última consulta (escrever `se` depois de `s` volta a
pontuar apenas o que `s` encontrou). Na pesquisa da barra lateral do x18n sobre 50,000 chaves, escrita
letra a letra, com resultados idênticos:

| | Por consulta | vs TS |
| --- | ---: | ---: |
| TS: `map(fuzzyScore)`, filtrar, ordenar | 10 ms | 1x |
| TS com a mesma filtragem prévia do índice | 4.1-5.4 ms | 2x |
| A mesma migração, chamada por chave | - | 0.2-0.4x |
| `FuzzyScoreBatch`, índice predefinido (4 núcleos) | 5.1 ms | 1.7x |
| `FuzzyScoreBatch` com `FuzzyScoreIndex` (napi, 4 núcleos) | 0.45-0.56 ms | **18-22x** |
| O mesmo com wasm (uma thread) | 0.95 ms | **11.5x** |

Contra TS que faz a mesma filtragem prévia, o Rust continua 7-12x mais rápido (wasm 5x). Converta os
índices devolvidos em chaves com um ciclo simples: `Array.from(order, i => keys[i])` custou 20x mais, 0.4 ms.

Todas as linguagens têm lotes, e estes superam o original mesmo onde as migrações por chamada perdem. O
mesmo trabalho em 4 núcleos, resultados e erros idênticos (benchmarks em cada pasta de demo):

| Linguagem | Carga de trabalho | Original | Migração por chamada vs original | Lote vs original |
| --- | --- | ---: | ---: | ---: |
| TS | pesquisa do x18n, 50,000 chaves | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`, 20,000 strings | 98 ms | - | **17x** |
| Ruby | `slugify`, 20,000 strings | 110 ms | - | **12.7x** |
| Ruby | `shellescape` (stdlib), 50,000 valores mistos | 88 ms | - | **11.8x** |
| Go | `Slugify`, 50,000 strings | 49 ms | 0.93x | **4x** |
| Java | `slugify`, 50,000 strings | 43 ms | 0.68x | **3x** |

Os originais compilados ganham menos: o trabalho por item já é rápido, por isso o lote ganha pelo número
de núcleos. As crates de lote alocam com mimalloc: com o alocador do sistema, o lote em Go era mais lento do que
uma única thread. Demo: [`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md).
## Escrever um índice rápido

- Faça com que o chamador construa o lote uma vez, não por consulta. Converta os índices devolvidos de volta em itens com um ciclo simples.
- Para preparar os itens uma vez (minúsculas, máscaras, tabelas de posições), defina `pub struct FnIndex` em `port.rs` com `new(items)` e `run(<other args>)`. Tem de dar o mesmo resultado que a função, item a item.
- Quando a ordenação puder evitar trabalho, dê ao índice o seu próprio `rank(<other args>, min)`. Tem de corresponder à ordenação dos resultados da função: pontuação da mais alta para a mais baixa, empates pela ordem dos itens.
- Um lote multiplica a velocidade por item da migração pelo número de núcleos. Contra originais compilados (Go, Java), a própria migração tem de ser rápida: trate ASCII sem tabelas Unicode, construa cada resultado numa única `String` pré-dimensionada, evite cópias intermédias.
- Feche o lote quando terminar, em Go e Java (os finalizers são apenas um recurso de reserva).
