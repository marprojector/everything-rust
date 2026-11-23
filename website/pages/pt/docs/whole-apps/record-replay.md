---
title: Registar e reproduzir
description: O app record executa a aplicação completa sobre os seus exemplos documentados, as suas entradas e armadilhas; o app replay executa qualquer build contra os casos.
---

# Registar e reproduzir

O `record` e o `diff` comprovam uma unidade; o `app record` e o `app replay` comprovam a aplicação, para CLIs e servidores em qualquer uma das seis linguagens. A aplicação é uma caixa negra.

```sh
everything-rust app record . -- python3 cli.py          # o comando que executa o original
everything-rust app replay .                            # o original tem de passar a 100%
everything-rust app replay . --build "target/release/mytool"
```

## De onde vêm os casos

O `app record` executa o original sobre:

- os próprios exemplos documentados da aplicação: linhas `$ mytool ...` nos seus ficheiros Markdown e de shell, com os ficheiros que referem;
- um ficheiro JSONL `--inputs`, com um `{"name", "args", "stdin", "env", "files"}` por linha;
- armadilhas semânticas derivadas desses casos: strings não ASCII e com emoji, divisão e módulo negativos, overflow, floats muito grandes e muito pequenos, ordem de mapas, ficheiros em falta.

Cada caso é executado duas vezes; o que difere entre as duas execuções passa a ser um wildcard `[..]`. `--normalize timestamps|uuids|key-order` ou `--normalize name=regex` acrescentam normalizadores, guardados com os casos. O `app record` reporta o que diferiu por tipo: valores (agora wildcards), ordem (linhas comparadas em qualquer ordem) e linhas (um `...` que coincide com qualquer coisa). Analise cada um deles.

## Os casos são ficheiros simples

Os casos são ficheiros [trycmd](https://docs.rs/trycmd) em `rust/app-cases/`:

| Ficheiro | Contém |
| --- | --- |
| `<case>.toml` | argv, env, código de saída |
| `<case>.stdin` | entrada padrão |
| `<case>.stdout`, `<case>.stderr` | o que o original imprimiu |
| `<case>.in/` | ficheiros de entrada |
| `<case>.out/` | ficheiros escritos |

Leia-os e edite-os à mão; um teste em Rust pode executá-los com `trycmd` assim que a migração tiver um binário.

## Replay

- O `app replay` executa qualquer build contra os casos, imprime `passed/total`, escreve `rust/app-cases/replay-report.md` com um diff por cada caso que falhou e termina com um código diferente de zero em caso de qualquer divergência.
- Uma build que acrescente linhas ao ficheiro indicado por `EVERYTHING_RUST_FALLBACK_LOG` (uma linha por cada recurso ao original), como faz a [front door](/docs/whole-apps/front-door), também tem a sua taxa de fallback reportada.
- `app replay --case <name>` volta a executar um caso pelo nome que o relatório lhe dá: o ciclo rápido e repetível para corrigir uma divergência antes de voltar a executar tudo.

## Replay na CI

O relatório não tem tempos nem caminhos temporários: dois replays da mesma build escrevem o mesmo ficheiro. Faça commit dele e depois execute na CI:

```sh
everything-rust app replay .
git diff --exit-code rust/app-cases/replay-report.md
```

Uma aplicação migrada a meio passa então a falhar a CI em qualquer alteração no que passa, para melhor ou para pior, até que o relatório seja atualizado com ela.

## Saída numérica

`app record --float-tolerance 1e-9` permite que os números coincidam dentro dessa fração do maior valor. Consulte [Tolerância de floats](/docs/concepts/float-tolerance).
