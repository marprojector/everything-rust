---
title: Front door e wire
description: O app front-door coloca um binário Rust à frente do original; o app wire entrega comandos à migração de uma unidade assim que esta passa o gate.
---

# Front door e wire

Passe primeiro o ponto de entrada para Rust e depois entregue-lhe os comandos, uma unidade de cada vez:

```sh
everything-rust app front-door .                            # rust/app: todos os comandos executam o original
everything-rust app wire . textkit --command upper          # depois de textkit-rs passar o gate
# escreva a ligação em rust/app/src/units/textkit.rs e depois:
everything-rust app wire . textkit                          # de novo 100%, com menor taxa de fallback
```

## A front door

O `app front-door` cria a estrutura de `rust/app`, um binário Rust com o nome da aplicação: um parser clap para os comandos e flags presentes nos casos registados, em que cada comando executa o original com os mesmos argv, stdin e ambiente e devolve a sua saída e o seu código de saída. Cada fallback é contabilizado, por isso reproduzir a front door acabada de criar dá 100% de sucesso com 100% de fallback (compila-a e reprodu-la para o mostrar). Nunca substitui `rust/app`: a partir daí, o código é seu.

## Ligar uma unidade

`app wire <repo> <unit> --command <name>`:

1. executa o `gate` na crate da unidade (`--skip-gate` se acabou de passar);
2. adiciona-a a `rust/app` como biblioteca;
3. escreve `src/units/<unit>.rs` uma vez, para que você ou o seu agente o preencham: interpretar os argumentos, chamar `<crate>::port`, imprimir o que o original imprime;
4. encaminha os comandos para ela e depois compila e reproduz.

Execute-o novamente depois de escrever a ligação: 100% de sucesso e uma taxa de fallback menor. `EVERYTHING_RUST_UNIT_<NAME>=original` devolve uma unidade ligada ao original até ao lift.

Ligue o próprio nome da aplicação como comando (`--command mytool`) para argv que não nomeia nenhum comando: sem argumentos, comandos desconhecidos, erros de utilização.

## Nada chama de volta o original

A front door executa o original como um processo separado (ou reencaminha HTTP para ele), as unidades com bridge são chamadas pelo original e nunca o contrário, e os callbacks ficam na linguagem de origem.

Os servidores utilizam rotas em vez de comandos: consulte [Servidores](/docs/whole-apps/servers). As bibliotecas utilizam a sua API pública: consulte [Bibliotecas](/docs/whole-apps/libraries).
