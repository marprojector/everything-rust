---
title: O ciclo
description: profile, analyze, bridge, record, port, diff, bench, shadow e gate. Os passos por que passa cada unidade.
---

# O ciclo

Cada migração, de uma função crítica a uma aplicação inteira, passa pelos mesmos passos, uma unidade de cada vez. Uma migração só está concluída quando o `gate` passa.

Como o everything-rust verifica uma migração, o que os seus comandos decidem e onde estão os seus limites. Voltar ao [README](/).

## Como funciona

```
profile ──> analyze ──> bridge ──> record ──> port.rs ──> diff ──> bench ──> shadow ──> gate
 where the    what to     the Rust   what the    you or      every     faster?    both side   done means
 time goes    port first  crate and  original    your agent  recorded  the        by side in  this passes
                          wrapper    does        write it    case      verdict    production
```

O original continua a ser a fonte da verdade: o `record` executa-o com casos-limite (e as suas próprias entradas) e
guarda o que devolve ou lança; o `diff` reproduz cada caso contra o Rust; nada é publicado até que
todos correspondam e o `gate` passe. Os chamadores mantêm a API original durante todo o processo.

Uma aplicação inteira passa pelo mesmo ciclo, unidade a unidade, dentro de uma estrutura que prova o
programa completo ([Migrar uma aplicação inteira](/docs/whole-apps/overview)):

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop above, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in         pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down      proved by
 decisions to make      outside         original answers                                                replay
```

## O ciclo

| Passo | Comando | O que faz |
| --- | --- | --- |
| 0. Profile | `everything-rust profile <repo> -- node bench.mjs` | Executa uma carga de trabalho real sob o profiler da linguagem (node, cProfile, pprof do `go test`, JFR, um tracer Ruby): funções críticas por tempo inclusivo, os seus chamadores e os ciclos a processar em lote; assinala código que já é compilado (numba, Cython, JAX), onde o Rust ganha pouco |
| 1. Planear | `everything-rust analyze <repo>` | Unidades, ordem de migração, riscos com `file:line`, bridge por unidade, correspondência entre dependências e crates, melhores primeiras migrações; `--whole-app` acrescenta os pontos de entrada e a forma, todas as dependências (importadas ou declaradas) com a respetiva crate, as peças (unidades que se importam mutuamente movem-se em conjunto) e decisões numeradas com uma resposta recomendada |
| 2. Bridge | `everything-rust bridge <repo> <unit>` | Crate com os bindings (`lib.rs`, mais o wrapper que a linguagem de origem carrega) e stubs a migrar (`port.rs`); `--batch fn:param` acrescenta uma classe que mantém muitos valores de `param` no Rust; `--as-json T` faz passar como valores JSON um parâmetro de tipo que a aplicação só preenche com dados |
| 3. Record | `everything-rust record <crate>` | Executa o original com casos-limite gerados (não ASCII, emoji, `-0`, `NaN`, ints onde se esperam floats, ...) e com todas as armadilhas semânticas em cada parâmetro (comprimento e maiúsculas/minúsculas de strings, divisão com sinal, limites de overflow, impressão de floats, ordem de mapas e de ordenação), e guarda o `cases.jsonl`; `--from-tests -- npx jest` (ou `-- pytest`) regista também todas as chamadas feitas pela própria suite de testes do original; `--float-tolerance 1e-9` permite que os floats correspondam dentro dessa fração em código numérico |
| 3. Build | `everything-rust build <crate>` | Compila o lado Rust: `.node`, pacote wasm, extensão Python ou biblioteca partilhada |
| 3. Diff | `everything-rust diff <crate>` | Reproduz os casos registados contra o Rust, chamado da mesma forma que os chamadores do original o chamam; lista as divergências |
| 3. Bench | `everything-rust bench <crate>` | Mede o tempo dos casos registados no original e no Rust, bridge incluída |
| 4. Shadow | `everything-rust shadow <crate>` | Substituto direto que executa ambos, devolve o resultado do original e reporta as divergências; `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. Gate | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, testes, auditoria de `unsafe`, nada por migrar, um caso de falha registado para cada caminho de erro do original, nenhum `unwrap`/`expect`/`panic!` onde o original devolve um erro (exceto se marcado com `// INFALLIBLE: <why>`), compilação, API, comportamento |
| Agentes | `everything-rust setup` | Regista o servidor MCP (`everything-rust mcp`, cada passo como uma ferramenta) e a skill do fluxo de trabalho ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) nos seus agentes de programação |

## Caminhos críticos: manter os dados no Rust

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

## O que o `bridge` decide

- Os exports que não conseguem atravessar uma bridge ficam na linguagem de origem, com o motivo, e o módulo
  shadow volta a exportar o original. Para TS:
  funções que recebem ou devolvem funções, generators e iterators, RegExp, verificações `instanceof`,
  genéricos (`--as-json T` faz passar um que a aplicação só preenche com dados) e valores cujos tipos (seguidos através de imports, re-exports e `extends`) contêm
  funções. `--keep "name: reason"` mantém mais, por exemplo código que depende de dados `Intl`.
- Os tipos TS são mapeados para tipos Rust, com os aliases de tipos resolvidos através de imports e os tipos dos parâmetros
  inferidos a partir dos valores predefinidos; os valores estruturados atravessam num formato binário gerado (JSON apenas
  como alternativa para formas que esse formato não modela).
- O JSON não consegue exprimir `undefined`, NaN, Infinity, -0 nem surrogates isolados: os valores estruturados levam
  marcadores (`{"$undefined": true}`, `{"$num": "NaN"}`, `{"$utf16": [...]}`) que o `index.cjs` gerado
  traduz, com helpers no `js.rs` da crate.

O `record` executa os ficheiros TS tal como estão: um resolve hook do Node segue imports ao estilo dos bundlers
(`./errors` para `./errors.ts`), `--resolve-from` fornece os pacotes npm quando o repositório não tem nenhum
instalado, e o TS que precisa de transformação (enums, parameter properties) é tentado de novo com
`--experimental-transform-types`.

## O que o `analyze` examina

- Pacotes (`package.json`) e ficheiros de código-fonte; ignora `node_modules`, output de compilação, fixtures,
  playgrounds e configurações de ferramentas. Os blocos `<script>` de Vue/Svelte/Astro são analisados à procura de imports.
- **Unidades**: uma pasta de ficheiros de código-fonte, ou um ficheiro na raiz do código-fonte de um pacote.
- **Vagas**: os imports de uma unidade surgem em vagas anteriores; ciclos de imports são migrados em conjunto.
- **Bridge**: napi-rs quando a unidade ou algo que ela importa precisa do Node; wasm-bindgen para código
  portável; manter em JS para código de UI e de editor.
- **Prontidão** (quão seguro) e **prioridade** (seguro e compensador: ciclos e trabalho com strings, testes, tamanho).

| Risco | Gravidade | Exemplo |
| --- | --- | --- |
| Offsets de strings | alta | `slice`, `indexOf`, `matchAll`, a flag `d`: UTF-16 vs UTF-8 |
| Sintaxe de regex | alta | lookaround, backreferences, `y`: não existem na crate `regex` |
| Regex dinâmica | alta | `new RegExp(userPattern)`: sintaxe JS em tempo de execução |
| Código dinâmico | alta | `eval`, `new Function`, `import()` calculado |
| Tipagem dinâmica | média | `any`, `'key' in value` |
| Coleções ordenadas | média | `Map`, `Set`, `Object.keys`: ordem de inserção |
| Async, generators | média | fronteiras de promises e generators |
| Herança de classes, reflexão | média | `extends`, `Proxy`, `Reflect`, `Symbol` |
| APIs baseadas em callbacks | média | parâmetros e métodos do tipo função: bridges com muitas chamadas |
| Exceções, números, JSON | baixa | `throw`, operações bit a bit, ordem do `JSON.stringify` |

## O que não consegue detetar

Passar significa que o Rust corresponde a todos os casos registados, nada mais. O que costuma
estragar as reescritas está normalmente fora do registo, por isso conheça os limites:

- **Entradas que ninguém registou.** Os casos gerados, as armadilhas, a suite de testes do original
  (`record --from-tests`) e os seus próprios `--inputs` são o limite. Adicione entradas reais (amostras
  de produção, relatórios de bugs); a entrada do utilizador que parte a produção raramente é uma em que
  um gerador pense.
- **Um original que evoluiu.** O `record` guarda uma impressão digital dos ficheiros da unidade
  (`recorded-from.json`); quando o original muda mais tarde, o `diff` avisa e o `gate` falha até
  registar de novo, em vez de passar contra o que o original costumava fazer.
- **Regexes.** A crate `regex` não tem lookaround nem backreferences, e mesmo um padrão que
  compila pode tratar `\w`, `\b`, `\d` e a correspondência sem distinção de maiúsculas em texto não ASCII
  de forma diferente do JS, Python ou Java. Trocar para `fancy-regex` ou reescrever o padrão pode passar
  todos os casos registados e ainda assim diferir num `ı` turco sem ponto ou num zero-width joiner. O
  `record` adiciona armadilhas de regex (essas, o sinal Kelvin, o s longo, os dígitos árabe-índicos, os
  separadores de linha, o sigma final) a todas as funções de um ficheiro que use regexes, mas faça também
  fuzzing dos seus padrões reais contra texto real.
- **O que muda entre execuções.** O `record` executa o original duas vezes e separa o que diferiu:
  valores alterados (horas, ids) passam a wildcards; as mesmas linhas noutra ordem (hash maps,
  concorrência) são comparadas em qualquer ordem, por isso uma migração continua a ter de produzir
  exatamente essas linhas; um número diferente de linhas passa a `...`, que corresponde a qualquer coisa.
  O `record` lista cada tipo: analise cada um, pois uma diferença de ordem de hash ou de concorrência é
  exatamente o que pode mudar a sério numa migração.
- **Floats.** A formatação de números para texto difere entre linguagens (`1e21`, `-0`, número de
  dígitos). As armadilhas incluem esses valores, e têm de corresponder exatamente; o `--float-tolerance`
  perdoa apenas o arredondamento, nunca a formatação.
- **Tudo o que não está no output.** Tempos, memória, concorrência sob carga e efeitos secundários
  além de stdout, stderr, códigos de saída, ficheiros escritos e respostas HTTP (chamadas de rede,
  bases de dados, sinais) não são comparados.
- **Outras plataformas.** Um registo prova o comportamento na máquina que o executou. Código C cujos
  resultados dependem do tamanho dos tipos, da endianness ou do compilador pode comportar-se de forma
  diferente no destino; o `analyze` mantém em C o código C ligado ao hardware por essa razão.
## Todos os passos

| Passo | Comando | O que faz |
| --- | --- | --- |
| 0. Profile | `everything-rust profile <repo> -- node bench.mjs` | Executa uma carga de trabalho real sob o profiler da linguagem (node, cProfile, pprof do `go test`, JFR, um tracer Ruby): funções críticas por tempo inclusivo, os seus chamadores e os ciclos a processar em lote; assinala código que já é compilado (numba, Cython, JAX), onde o Rust ganha pouco |
| 1. Planear | `everything-rust analyze <repo>` | Unidades, ordem de migração, riscos com `file:line`, bridge por unidade, correspondência entre dependências e crates, melhores primeiras migrações; `--whole-app` acrescenta os pontos de entrada e a forma, todas as dependências (importadas ou declaradas) com a respetiva crate, as peças (unidades que se importam mutuamente movem-se em conjunto) e decisões numeradas com uma resposta recomendada |
| 2. Bridge | `everything-rust bridge <repo> <unit>` | Crate com os bindings (`lib.rs`, mais o wrapper que a linguagem de origem carrega) e stubs a migrar (`port.rs`); `--batch fn:param` acrescenta uma classe que mantém muitos valores de `param` no Rust; `--as-json T` faz passar como valores JSON um parâmetro de tipo que a aplicação só preenche com dados |
| 3. Record | `everything-rust record <crate>` | Executa o original com casos-limite gerados (não ASCII, emoji, `-0`, `NaN`, ints onde se esperam floats, ...) e com todas as armadilhas semânticas em cada parâmetro (comprimento e maiúsculas/minúsculas de strings, divisão com sinal, limites de overflow, impressão de floats, ordem de mapas e de ordenação), e guarda o `cases.jsonl`; `--from-tests -- npx jest` (ou `-- pytest`) regista também todas as chamadas feitas pela própria suite de testes do original; `--float-tolerance 1e-9` permite que os floats correspondam dentro dessa fração em código numérico |
| 3. Build | `everything-rust build <crate>` | Compila o lado Rust: `.node`, pacote wasm, extensão Python ou biblioteca partilhada |
| 3. Diff | `everything-rust diff <crate>` | Reproduz os casos registados contra o Rust, chamado da mesma forma que os chamadores do original o chamam; lista as divergências |
| 3. Bench | `everything-rust bench <crate>` | Mede o tempo dos casos registados no original e no Rust, bridge incluída |
| 4. Shadow | `everything-rust shadow <crate>` | Substituto direto que executa ambos, devolve o resultado do original e reporta as divergências; `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. Gate | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, testes, auditoria de `unsafe`, nada por migrar, um caso de falha registado para cada caminho de erro do original, nenhum `unwrap`/`expect`/`panic!` onde o original devolve um erro (exceto se marcado com `// INFALLIBLE: <why>`), compilação, API, comportamento |
| Agentes | `everything-rust setup` | Regista o servidor MCP (`everything-rust mcp`, cada passo como uma ferramenta) e a skill do fluxo de trabalho ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) nos seus agentes de programação |
## A seguir

Cada passo tem a sua própria página: [Profile e analyze](/docs/concepts/analyze), [Bridge](/docs/concepts/bridge), [Record](/docs/concepts/record), [Port](/docs/concepts/port), [Diff](/docs/concepts/diff), [Bench](/docs/concepts/bench), [Shadow](/docs/concepts/shadow) e [Gate](/docs/concepts/gate).
