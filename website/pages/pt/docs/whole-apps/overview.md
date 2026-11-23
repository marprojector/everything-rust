---
title: Migrar uma aplicação completa
description: Do primeiro registo a um binário em Rust puro. O objetivo é uma aplicação Rust que se comporta como o original, mantendo o original como referência.
---

# Migrar uma aplicação completa

O objetivo é uma aplicação em Rust puro que se comporta como o original, mantendo o original como referência. O seu agente escreve o Rust; o everything-rust ordena o trabalho, cria a estrutura e comprova cada passo. CLIs, servidores HTTP e bibliotecas.

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in    pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down proved by
 decisions to make      outside         original answers                                           replay
```

## 1. Planear

```sh
everything-rust analyze . --whole-app
```

O `--whole-app` acrescenta a forma da aplicação (CLI, servidor ou biblioteca) e os pontos de entrada, todas as dependências externas (importadas ou declaradas) com a respetiva crate Rust e grau de adequação, a ordem de migração em peças (unidades que se importam mutuamente avançam juntas), e decisões numeradas com uma resposta recomendada: unidades a manter, unidades bloqueadas, dependências sem crate ou com uma crate parcial.

## 2. Registar a aplicação completa e depois mover o ponto de entrada

1. [`app record` e `app replay`](/docs/whole-apps/record-replay) comprovam o programa a partir do exterior, como uma caixa negra.
2. [`app front-door`](/docs/whole-apps/front-door) coloca um binário Rust à frente do original; todos os comandos continuam a executar o original.
3. Cada unidade passa pelo [ciclo](/docs/concepts/the-loop), e depois o `app wire` entrega os seus comandos ao Rust.
4. [`app lift`](/docs/whole-apps/lift) remove o original assim que o replay passa com taxa de fallback 0 e todas as unidades passam o gate.

[Servidores](/docs/whole-apps/servers) e [bibliotecas](/docs/whole-apps/libraries) utilizam os mesmos comandos.

## Como o agente avança

A [skill de aplicação completa](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) guia o agente ao longo do processo, começando pelas decisões que só você pode tomar:

- **Interrogatório**: o agente pergunta por rondas, cada pergunta com a respetiva resposta recomendada, até que todas as decisões de que a migração depende estejam resolvidas, e regista cada uma como um ADR em `rust/DECISIONS.md`, com `rust/GLOSSARY.md` a mapear os termos do original para nomes em Rust.
- **Investigação**: agentes em segundo plano leem as fontes primárias de cada dependência e escrevem `rust/research/<topic>.md`, com todas as armadilhas semânticas que encontraram.
- **Entre sessões**: `rust/MAP.md` lista todas as unidades com o respetivo estado; uma sessão reserva uma unidade antes de começar, e cada ponto de controlo deixa um `rust/HANDOFF.md` para a sessão seguinte. Unidades da mesma vaga podem ser migradas em paralelo por subagentes.
- **Transição**: o agente faz o que consegue e escreve `rust/cutover.sh`, um script interativo para os passos que só você pode fazer (deploys, DNS, segredos), com uma forma de voltar atrás em cada passo.

O interrogatório, a investigação e estas práticas foram adaptados das [skills de Matt Pocock](https://github.com/mattpocock/skills).

## Isto não prende o Rust ao design antigo?

Só enquanto ambos correm lado a lado. Os registos verificam o comportamento nas fronteiras da aplicação (a saída de uma CLI, as respostas de um servidor, as funções exportadas de um módulo), não os seus detalhes internos. Espelhe a estrutura antiga enquanto as unidades ainda atravessam a bridge; assim que o `app lift` tiver removido o original, reestruture o Rust à vontade, e o `app replay` continua a comprovar o comportamento.

## Um caso real

O [json-server](/docs/real-world/json-server) passou por todos os passos: 514 casos HTTP registados, uma front door com 99.8% de fallback, primeiro as leituras ligadas (22.6% de fallback), depois todas as rotas (0%), e um lift para um binário em Rust puro que passa 514/514.
