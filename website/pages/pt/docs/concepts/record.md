---
title: Record
description: O record executa o original com casos-limite gerados, armadilhas semânticas, os seus próprios testes e as suas entradas, e guarda os resultados em cases.jsonl.
---

# Record

```sh
everything-rust record <crate>
everything-rust record <crate> --from-tests -- npx jest      # ou: -- pytest
everything-rust record <crate> --inputs extra.jsonl
```

O `record` executa as funções originais e guarda o que cada uma devolve ou lança em `cases.jsonl`, junto da crate: o comportamento a que a migração tem de corresponder. Uma divergência mais tarde é uma diferença de comportamento: corrija a migração, nunca o registo.

## De onde vêm os casos

- **Casos-limite gerados**: não ASCII, emoji, `-0`, `NaN`, ints onde se esperam floats, entre outros (`--count` define quantos por função).
- **Armadilhas semânticas** em cada parâmetro: comprimento e maiúsculas/minúsculas de strings, divisão com sinal, limites de overflow, impressão de floats, ordem de mapas e de ordenação.
- **Armadilhas de regex** para cada função de um ficheiro que use regexes: o `ı` turco sem ponto, o sinal Kelvin, o s longo, os dígitos árabe-índicos, os separadores de linha, o sigma final, os zero-width joiners.
- **Os próprios testes do original**: `--from-tests -- <test command>` regista todas as chamadas que a suite de testes faz às funções exportadas da unidade (TS/JS com qualquer runner, Python com pytest ou unittest). O código-fonte é reposto no final. No vercel/ms, capturou 340 chamadas a partir de 167 testes Jest.
- **As suas entradas**: `--inputs cases.jsonl` com um `{"fn": name, "args": [...]}` por linha. Adicione entradas reais (amostras de produção, relatórios de bugs); a entrada do utilizador que parte a produção raramente é uma em que um gerador pense.

Os chamadores Python passam `1` tantas vezes quanto `1.0`, por isso o registo passa números inteiros como ints em cada quarto caso Python.

## Marcadores em cases.jsonl

O JSON não consegue exprimir `undefined`, NaN, Infinity, -0 nem surrogates isolados. Os valores estruturados levam marcadores que o `index.cjs` gerado traduz, com helpers no `js.rs` da crate:

| Marcador | Significa |
| --- | --- |
| `{"$undefined": true}` | `undefined` do JavaScript |
| `{"$num": "NaN"}` | um número que o JSON não consegue representar (`NaN`, `Infinity`, `-0`) |
| `{"$utf16": [...]}` | uma string dada como unidades de código UTF-16, para surrogates isolados |

Entradas C fora do contrato de uma função podem fazer o original falhar. O driver C executa cada caso num processo filho criado por fork e regista `crash: SIGSEGV`; o `diff` aceita aí qualquer comportamento da migração, já que o do C é indefinido.

## Duas execuções, e o que difere

O `record` executa o original duas vezes e separa o que diferiu entre as execuções: valores alterados (horas, ids) passam a wildcards; as mesmas linhas noutra ordem (hash maps, concorrência) são comparadas em qualquer ordem; um número diferente de linhas passa a `...`, que corresponde a qualquer coisa. O `record` lista cada tipo. Analise cada um, pois uma diferença de ordem de hash ou de concorrência é exatamente o que pode mudar a sério numa migração.

## O registo lembra-se do seu original

O `record` guarda uma impressão digital dos ficheiros da unidade (`recorded-from.json`). Quando o original muda mais tarde, o `diff` avisa e o `gate` falha até registar de novo, em vez de passar contra o que o original costumava fazer. Consulte [Diff](/docs/concepts/diff#stale-recordings).

## Especificidades do TypeScript

O `record` executa os ficheiros TS tal como estão: um resolve hook do Node segue imports ao estilo dos bundlers (`./errors` para `./errors.ts`), `--resolve-from` fornece os pacotes npm quando o repositório não tem nenhum instalado, e o TS que precisa de transformação (enums, parameter properties) é tentado de novo com `--experimental-transform-types`. O TS que não pode correr diretamente no Node pode ser registado a partir do output compilado com `--module dist/x.mjs`, ou com `--node-arg=--import=tsx`.

## Código numérico

Para código em que os floats raramente correspondem bit a bit, registe com `--float-tolerance`. Consulte [Tolerância de floats](/docs/concepts/float-tolerance).
