---
title: Port
description: Escrever o port.rs, o único ficheiro que você ou o seu agente escrevem, e as regras que o mantêm igual ao original.
---

# Port

Você ou o seu agente escrevem a migração em `src/port.rs`. Tudo o resto na crate é gerado. Depois de alterar uma assinatura em `port.rs`, execute `everything-rust refresh <crate>`.

## Regras que poupam uma ronda de diff

- Leia cada risco que o plano lista para a unidade, no respetivo `file:line`, antes de escrever código.
- **Offsets de strings**: JS e Java indexam unidades de código UTF-16, o Rust bytes UTF-8. Utilize o `src/utf16.rs` gerado.
- **Erros**: devolva `Err("TypeName: message")`. Torna-se essa exceção em Python, Ruby e Java, e o texto do `error` em Go. Um panic do Rust nunca corresponde a um erro registado.
- **Números em Python e Ruby**: os ints que o original deixa passar têm de voltar como ints. Nas migrações de Python utilize `crate::pynum::Num` para int-ou-float, não `serde_json::Value`.
- **C**: entradas registadas como `crash:` são comportamento indefinido; qualquer resultado passa.
- **Mapas ordenados e chaves de objetos**: `indexmap` onde a ordem chega ao output.
- **Lookaround ou backreferences em regex**: `fancy-regex`, não `regex`. Em qualquer dos casos, `\w`, `\b`, `\d` e a correspondência sem distinção de maiúsculas tratam texto não ASCII de forma diferente do motor da linguagem de origem. Defina `unicode(false)` ou escreva explicitamente classes ASCII (`[A-Za-z0-9_]`) onde o `\w` do original é apenas ASCII (JS sem `u`, Java).
- **Nada de `unsafe`.** Se for mesmo necessário: mínimo, isolado e com um comentário `// SAFETY:`.
- **Mantenha os helpers privados.** Apenas os exports da unidade são `pub`; um helper `pub` passa a fazer parte da API exposta pela bridge.
- Acrescente testes unitários Rust em `port.rs` para os casos sobre os quais raciocina.

O `js.rs` gerado traz helpers que reproduzem o JavaScript onde o Rust difere, como `js::number_to_string` (`String(n)` do JS) e `js::math_round` (`Math.round`).

## Um exemplo: o emoji que deslocou uma posição

`packages/core/src/fuzzy.ts` do [x18n](https://github.com/x18n-dev/x18n), migrado de ambas as formas em
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo):

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: pronto, wasm-bindgen (o core corre em navegadores)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# escrever src/port.rs
everything-rust gate demo/fuzzy-rs
```

Uma primeira migração que indexava por `char` do Rust passou em testes simples, mas falhou no `diff`:

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

O `indexOf` do JS conta unidades de código UTF-16, por isso um emoji ocupa duas posições. A migração em
`demo/*/src/port.rs` utiliza os helpers `utf16` gerados e passa 2000/2000 casos e o gate,
como napi-rs (`demo/fuzzy-napi`) e como wasm-bindgen (`demo/fuzzy-rs`).

O plano que o everything-rust escreve para todo o x18n: [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md).
## Que tamanho deve ter?

O `gate` compara as linhas da migração com as do original e falha acima de 3x (mais 50 linhas para imports e assinaturas). Passar disso costuma significar escrever à mão o que uma crate já faz, ou tratar casos um a um. O `analyze` lista as crates a utilizar.
