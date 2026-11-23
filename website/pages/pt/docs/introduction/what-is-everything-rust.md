---
title: O que é o everything-rust
description: O everything-rust regista o que a sua aplicação faz hoje e compara cada parte da reescrita em Rust com esse registo, até que a aplicação seja totalmente escrita em Rust.
---

# O que é o everything-rust

<p class="lead">Reescreva toda a sua aplicação em Rust, uma peça de cada vez. O everything-rust regista o que a sua aplicação Node.js (TypeScript/JavaScript), Python, C, Go, Ruby ou Java faz hoje e, em seguida, compara cada parte da reescrita em Rust com esse registo antes de avançar, até que a aplicação seja totalmente escrita em Rust.</p>

Funciona com CLIs, servidores HTTP e bibliotecas. O seu agente de programação (Claude Code, Codex, Cursor, Gemini CLI) ou você escreve o código Rust. O PHP e o C# estão em [pré-visualização](/docs/languages/php).

## O que faz, e o que não faz

O everything-rust não traduz código. Fornece aquilo que um agente não consegue adivinhar de forma fiável:

- **para onde vai o tempo**: o `profile` executa uma carga de trabalho real e ordena as funções críticas;
- **o que migrar primeiro, e por que ordem**: o `analyze` planeia unidades, vagas, riscos e crates;
- **uma bridge** para que o resto do código chame o Rust com a API inalterada (`bridge`);
- **o comportamento exato que a migração tem de reproduzir**: o `record` executa o original em casos extremos e guarda o que devolve ou lança;
- **uma forma segura de o entregar**: o `shadow` executa ambos lado a lado;
- **um gate que diz quando uma migração está concluída**: `gate`.

O original continua a ser a fonte de verdade. O `diff` reproduz cada caso registado no Rust, e nada é entregue até que todos coincidam e o `gate` passe.

## Quatro etapas

| Etapa | O que acontece |
| --- | --- |
| **Registar** | O `record` executa o original em casos extremos gerados, armadilhas semânticas e as suas próprias entradas, e guarda o que devolve ou lança. |
| **Migrar** | O `bridge` gera o crate Rust e um wrapper, para que quem chama mantenha a API original. Você ou o seu agente escrevem o Rust. |
| **Verificar** | O `diff` reproduz cada caso registado no Rust, e nada é entregue até que todos coincidam e o `gate` passe. |
| **Elevar** | Quando a reprodução passa sem fallback e cada unidade passa o gate, o `app lift` transforma-a numa aplicação totalmente em Rust. O original nunca é alterado. |

## O Rust é sempre mais rápido?

Não, e o everything-rust di-lo. Uma função pequena chamada uma vez por item é muitas vezes mais lenta em Rust, porque cada chamada atravessa uma bridge; o V8, a JVM e o Go executam essas funções em nanossegundos. O `profile` encontra o ciclo à volta dela, o `--batch` move o ciclo inteiro e o `bench` dá um veredicto antes de entregar. Consulte [Lotes e caminhos críticos](/docs/concepts/batches) para ver números medidos, incluindo os lentos.

## Só consegue detetar aquilo que o registo abrange

Passar significa que o Rust coincide com cada caso que foi registado, nada mais. Leia [O que não consegue detetar](/docs/concepts/limits) antes de confiar num gate verde.

::: warning Execute-o numa cópia
O `record` e o `app record` executam o seu código original, chamando cada função (ou a aplicação inteira) com muitas entradas geradas, e o `app record` executa cada caso duas vezes. O `diff`, o `gate` e o `app replay` executam os mesmos casos novamente, na migração ou no original. Código que escreve ficheiros, comunica com uma base de dados ou faz chamadas à rede fá-lo uma vez por caso. Execute-os numa cópia do projeto, com uma base de dados de teste e sem credenciais de produção no ambiente.
:::

## Próximos passos

- [Início rápido](/docs/introduction/quick-start): instale-o e peça ao seu agente uma primeira migração.
- [O ciclo](/docs/concepts/the-loop): cada etapa para uma unidade.
- [Migrar uma aplicação completa](/docs/whole-apps/overview): do primeiro registo até um binário totalmente em Rust.
- [Migrações reais](/docs/real-world/overview): json-server, Fuse.js, vercel/ms e outros, com os resultados lentos ao lado dos rápidos.
