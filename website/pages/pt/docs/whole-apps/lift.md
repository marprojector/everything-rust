---
title: Lift
description: O app lift torna rust/app numa aplicação em Rust puro assim que o replay passa com taxa de fallback 0 e todas as unidades passam o gate.
---

# Lift

Quando o replay passa com taxa de fallback 0 e todas as unidades passam o gate, transforme-a numa aplicação em Rust puro:

```sh
everything-rust app lift .
```

- O `app lift` recusa-se a avançar até que ambas as condições se verifiquem, e diz o que o bloqueia: os casos que ainda recorrem ao fallback, ligações que ainda são um stub, unidades que falham o gate.
- Primeiro numa cópia, remove o caminho de fallback e os interruptores de desativação, transforma cada crate de unidade numa simples biblioteca de `rust/app` (sem `cdylib`, bindings nem wrappers gerados) e adiciona `rust/app/tests/app_cases.rs`, que executa `rust/app-cases` com trycmd no `cargo test`, para que a CI não precise de nada além do cargo.
- Depois executa a lista de verificação do lift: fmt, clippy `-D warnings` e testes em todas as unidades e em `rust/app`, cada alvo indicado em `rust/DECISIONS.md` compilado ou verificado onde estiver instalado, ficheiros lidos em tempo de execução e pastas de recursos não incorporados, e replay no binário puro. Só quando tudo isto passa é que as alterações são escritas de volta. O original nunca é alterado.

## Depois do lift

As unidades deixam de ter bindings. No json-server, o `tests/recorded.rs` de cada crate reproduz os seus casos registados na migração: todos os 3,399 correm com `cargo test`, ao lado dos 514 casos HTTP.

Depois de o original desaparecer, reestruture o Rust à vontade; o `app replay` continua a comprovar o comportamento.

## Depois, a transição

A skill do fluxo de trabalho faz com que o agente faça o que conseguir e escreva `rust/cutover.sh`, um script interativo para os passos que só você pode fazer (deploys, DNS, segredos), com uma forma de voltar atrás em cada passo.
