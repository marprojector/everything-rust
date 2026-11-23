---
title: Fuse.js
description: Pesquisa em listas de strings do Fuse.js em Rust. 3,414/3,414 casos registados coincidem, resultados idênticos byte a byte, 32x mais rápido com 50,000 strings.
---

# Fuse.js

O [Fuse.js](https://github.com/krisk/Fuse) (20k estrelas, cerca de 20 milhões de downloads no npm por semana,
commit `edf2fb6`) é a biblioteca de pesquisa aproximada por trás de inúmeras caixas de pesquisa. Migrado com
o everything-rust 1.6 (não lançado); o crate está em [`demo/real-world/fuse-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/fuse-rs).

O `analyze` colocou todas as pastas de `src/` num único ciclo de importações, por isso o Fuse não pode ser migrado ficheiro a ficheiro.
Em vez disso, a migração pega no caminho onde uma pesquisa gasta o seu tempo: um adaptador de 54 linhas
(`adapter/relevance.ts`) exporta a pontuação do próprio Fuse para uma string de uma lista de strings, construída a partir do
`BitapSearch`, da norma de campo e do `computeScoreSingle` do Fuse. Essa função é o que uma pesquisa sobre
50,000 strings executa 50,000 vezes. O `bridge --batch relevance:text` mantém as strings em Rust e
executa uma pesquisa sobre todas elas, em todos os núcleos.

| Etapa | Resultado |
| --- | --- |
| adaptador | a ordenação do adaptador é igual a `new Fuse(list).search(q)` em 81 consultas e conjuntos de opções antes de qualquer Rust |
| `record --inputs` | 3,414 casos: títulos das próprias fixtures do Fuse, não-ASCII, emoji, padrões com mais de 32 unidades, todas as opções suportadas e opções com o tipo errado (recusadas, 149 casos) |
| migração | 673 linhas de Rust para as 839 linhas do Fuse que substitui |
| `diff` | 3,414/3,414 coincidem logo no primeiro build |
| `gate` | passa |
| substituto direto | `withRust(Fuse)` responde a pesquisas em listas de strings em Rust e passa tudo o resto ao Fuse; devolve resultados idênticos byte a byte aos do Fuse em 360 pesquisas (12 conjuntos de opções, 15 consultas, com e sem `limit`) |

Tempo de pesquisa por consulta, Apple M4 (10 núcleos), Node 24, resultados verificados como idênticos antes da medição:

| Lista | Fuse.js | Rust | Aceleração |
| ---: | ---: | ---: | ---: |
| 1,000 strings | 1.9 ms | 0.1 ms | 18x |
| 10,000 strings | 19.1 ms | 0.7 ms | 27 a 30x |
| 50,000 strings | 91.2 ms | 2.8 ms | **32x** |
| 50,000 strings, um núcleo | 92.0 ms | 17.5 ms | 5,3x |

O que a migração tinha de acertar:

- O Bitap funciona com inteiros JS de 32 bits: `1 << 31` é negativo e `(1 << 31) - 1` é um double que
  se torna `2147483647` no deslocamento seguinte. A migração mantém esses valores tal como o JS os tem.
- O texto é indexado em unidades de código UTF-16, por isso um emoji ocupa duas posições, como no Fuse.
- O `ignoreDiacritics` é NFD mais uma lista de intervalos de marcas combinatórias e algumas letras indicadas explicitamente
  (`ł`, `ß`, `ı`); a migração utiliza a mesma lista.
- O `String.prototype.trim` decide que strings o Fuse ignora: espaços em branco do JS, não o
  `char::is_whitespace` do Rust (que inclui U+0085 e deixa de fora U+FEFF).

Encontrado na ferramenta, e corrigido: os crates de lote cujos itens eram strings mas cujos resultados não eram
recebiam um helper não utilizado e falhavam o `clippy -D warnings`; e a verificação de tamanho do `gate` contava apenas o
adaptador (54 linhas, portanto 12,5x "demasiado grande") em vez dos ficheiros do Fuse que este importa.

O que não abrange: listas de objetos com `keys`, pesquisa estendida e por tokens, consultas lógicas,
`includeMatches`, um `sortFn` ou `getFn` personalizado. O substituto direto passa esses casos ao Fuse sem alterações. A
aceleração exige que a lista permaneça em Rust entre pesquisas; construí-la custa aproximadamente o mesmo que
o índice do próprio Fuse (3.4 ms contra 2.8 ms para 50,000 strings).
