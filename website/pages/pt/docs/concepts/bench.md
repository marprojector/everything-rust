---
title: Bench
description: O bench mede os casos registados no original e no Rust, bridge incluída, e dá um veredicto. O Rust nem sempre é mais rápido.
---

# Bench

```sh
everything-rust bench <crate>
```

O `bench` mede o tempo dos casos registados no original e no módulo Rust compilado, bridge incluída, e devolve as linhas e um veredicto com recomendações: publicar, manter as funções mais lentas na linguagem de origem ou migrar uma unidade maior. Um ganho inferior a 1 significa que a migração é mais lenta tal como os seus chamadores a chamam. `--min-ms` define o tempo mínimo medido por função e por lado (predefinição 200).

Siga o veredicto. Não publique uma migração que torne os chamadores mais lentos sem o dizer.

## Porque é que uma migração pode ser mais lenta

Uma migração chamada uma vez por item paga a bridge uma vez por item. O que decide:

- **Originais interpretados (Python, Ruby)**: a chamada ao Rust custa cerca de 100-300 ns e o código Rust é muito mais rápido do que o interpretador, por isso até funções pequenas ganham.
- **Originais compilados (Go, Java depois do JIT)**: a função folha original demora 4-90 ns; uma única chamada cgo custa cerca de 80 ns, e uma chamada JNI com strings mais. Estas linguagens só ganham quando uma chamada faz muito trabalho: migre uma função juntamente com os seus chamadores.
- **C**: não há travessia; o código Rust é comparado diretamente com o código C.

Medido por chamada em código real ([seis linguagens](/docs/real-world/six-languages)):

| Linguagem | Rust vs o original (`bench`, média geométrica) | Veredicto |
| --- | --- | --- |
| Ruby | **4.4x mais rápido** (3.6-7x por função) | publicar |
| Python | **1.35x mais rápido** (1.5-1.9x na maioria das chamadas) | publicar |
| C | 0.87x: ctype 1.1-1.4x mais rápido, `strtod`, `strspn`, `strcmp` otimizados da glibc 0.5-0.7x | otimizar ou manter as mais lentas |
| Java | 0.18x | migrar unidades maiores |
| Go | 0.04x | migrar unidades maiores |

Todas as 29 funções do núcleo do x18n que podem ser movidas sem alterar a API foram migradas: todos os gates passam e 7,925 casos registados correspondem exatamente, e o `bench` mostrou que as migrações são mais lentas do que o TS quando chamadas a partir de JS, porque a bridge custa mais do que o trabalho ([relatório](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-core/README.md)). O vercel/ms correu a 0.56x por chamada sobre os seus casos registados ([caso de estudo](/docs/real-world/vercel-ms)).

## O que ganha: mover o ciclo

O `profile` encontra o ciclo em torno de uma função crítica, e o `--batch` move o ciclo inteiro para o Rust, em todos os núcleos. Consulte [Lotes e caminhos críticos](/docs/concepts/batches).

::: tip Meça numa máquina sem carga
Execute o `bench` numa máquina sem carga, execute-o duas vezes e indique o hardware e o número de núcleos junto de cada número publicado.
:::
