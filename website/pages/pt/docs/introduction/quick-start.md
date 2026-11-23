---
title: Início rápido
description: Instale o everything-rust, registe-o no seu agente de programação e execute o ciclo numa primeira unidade.
---

# Início rápido

## Deixe o seu agente instalá-lo

Cole isto no Claude Code, Codex, Cursor ou Gemini CLI:

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## Ou instale-o você mesmo

::: tabs
== cargo
```sh
cargo install everything-rust   # a partir do crates.io
everything-rust setup           # regista-o no Claude Code, Codex, Cursor e Gemini CLI
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

Ainda não tem a toolchain do Rust? Os binários pré-compilados para macOS, Linux e Windows estão anexados a cada [lançamento no GitHub](https://github.com/marprojector/everything-rust/releases) e, para projetos JavaScript, o npm instala o mesmo binário.

O `setup` adiciona o servidor MCP a cada agente que encontrar e instala a skill do fluxo de trabalho no local onde o agente lê as skills. Consulte [Configurar o seu agente](/docs/introduction/agents) para saber o que altera.

## Peça ao seu agente

- *"Reescreve esta CLI em Rust com o everything-rust, do princípio ao fim, e prova que se comporta da mesma forma."*
- *"Esta pesquisa parece lenta. Faz o profiling com o everything-rust e passa o ciclo crítico para Rust."*
- *"Planeia uma migração de `src/parser/` para Rust com o everything-rust; migra a primeira unidade e prova-a."*
- *"Faz o benchmark da migração para Rust em `rust/text-rs` face ao original e diz-me se devo entregá-la."*

## Ou execute as etapas você mesmo

As etapas no `fuzzyScore` do x18n: o `profile` encontra o ciclo crítico, o `bridge --batch` gera o crate, o `gate` passa e o benchmark compara-o com o TypeScript.

```sh
everything-rust profile . -- node bench.mjs              # o que é crítico, e quem o chama
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# escreva src/port.rs (ou deixe o seu agente fazê-lo) e depois:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # cada caso registado tem de coincidir
everything-rust bench rust/fuzzy-rs                      # mais rápido? o veredicto diz
everything-rust gate rust/fuzzy-rs                       # concluído significa que isto passa
```

Cada etapa é explicada em [O ciclo](/docs/concepts/the-loop).

::: tip Quando falta alguma coisa
Se uma etapa falhar porque um runtime, a configuração do agente ou o servidor MCP está em falta ou desatualizado, execute [`everything-rust doctor`](/docs/introduction/doctor).
:::
