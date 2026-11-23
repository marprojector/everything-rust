---
title: Doctor
description: O comando everything-rust doctor verifica os runtimes das linguagens, a configuração do agente e o servidor MCP, com o comando que corrige cada problema.
---

# Doctor

```sh
everything-rust doctor
everything-rust doctor --json
```

O `doctor` verifica os runtimes das linguagens, a configuração do agente e o servidor MCP, e mostra o comando que corrige cada problema. Termina com o código 1 quando falta algo obrigatório.

Execute-o quando uma etapa falhar porque um runtime, a configuração do agente ou o servidor MCP está em falta ou desatualizado. A skill do fluxo de trabalho diz aos agentes para fazerem o mesmo.

## O que cada linguagem precisa

O `record`, o `diff` e o `bench` executam o original no seu próprio runtime: Node 22.6+ para TS (executa diretamente fontes `.ts`), Python 3.9+ para Python e C (ctypes), Go 1.21+, Ruby 3+, JDK 17+ (`javac` e `java`). O `profile` em Node precisa do Node 24+, que permite `--cpu-prof` em `NODE_OPTIONS`. O C precisa também de um compilador de C. As bridges wasm precisam ainda de `rustup target add wasm32-unknown-unknown` e do `wasm-bindgen-cli` correspondente; o `build` indica a versão exata.

## Para contribuidores

O `cargo run -- doctor` lista os runtimes que tem instalados, as respetivas versões e o comando que instala ou atualiza cada um. Consulte [Contribuir](/docs/project/contributing).
