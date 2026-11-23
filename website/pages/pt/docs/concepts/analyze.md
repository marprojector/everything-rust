---
title: Profile e analyze
description: Descubra para onde vai o tempo com o profile e depois planeie unidades, ordem de migração, riscos e crates com o analyze.
---

# Profile e analyze

## Profile: encontrar o que vale a pena mover

Quando o objetivo é velocidade, meça antes de escolher qualquer coisa:

```sh
everything-rust profile <repo> -- node bench.mjs      # ou: python3 bench.py, ruby bench.rb,
                                                    #     java -cp out Main, go test -run X -bench . ./pkg
```

O `profile` executa uma carga de trabalho real sob o profiler da linguagem (node, cProfile, pprof do `go test`, JFR, um tracer Ruby) e lista as funções críticas por tempo inclusivo, os seus chamadores e os ciclos a processar em lote. Em Go, o profiling só funciona através do `go test`; C não é suportado.

- Uma função pequena chamada uma vez por item **não** vale a pena migrar sozinha: uma chamada ao Rust custa mais do que o trabalho que faz, e os JITs (V8, a JVM, Go) executam-na em nanossegundos. Migre-a juntamente com os seus chamadores ou [processe-a em lote](/docs/concepts/batches). A linha de recomendação indica qual, com o comando exato.
- Código que já é compilado (numba `@njit`, Cython, `@jax.jit`) corre como código máquina: uma migração para Rust ganha pouca velocidade. O `profile` lista-o em "Already compiled".

## Analyze: planear a migração

```sh
everything-rust analyze <repo> -o plan.md        # para pessoas
everything-rust analyze <repo> --json            # para agentes: schema everything-rust/analyze@1
everything-rust analyze <repo> --whole-app       # mais pontos de entrada, forma, dependências, decisões
```

O `analyze` lista as unidades, a ordem de migração, os riscos com `file:line`, a bridge por unidade, o grau de correspondência de cada dependência a uma crate e `start_here`: as melhores primeiras migrações. `--whole-app` acrescenta a forma da aplicação (CLI, servidor ou biblioteca) e os pontos de entrada, todas as dependências (importadas ou declaradas) com a respetiva crate, as peças (unidades que se importam mutuamente movem-se em conjunto) e decisões numeradas com uma resposta recomendada. Consulte [Migrar uma aplicação inteira](/docs/whole-apps/overview).

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
## Crates para dependências

O `analyze` indica a crate Rust para cada biblioteca que o código importa, com uma correspondência `good`, `partial` ou `none`. Uma biblioteca que falte na sua tabela é reportada como `unknown`; a tabela nunca adivinha uma crate. Para adicionar uma, consulte [Contribuir](/docs/project/contributing#add-a-crate-mapping).
