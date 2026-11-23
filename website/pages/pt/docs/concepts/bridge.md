---
title: Bridge
description: O bridge cria a estrutura de uma crate Rust para uma unidade, com bindings e um wrapper para que os chamadores mantenham a API original.
---

# Bridge

```sh
everything-rust bridge <repo> <unit-id>          # crate em <repo>/rust/<name>; nunca sobrescreve
everything-rust refresh <crate>                  # depois de alterar uma assinatura em port.rs
```

O `bridge` cria uma crate com os bindings (`lib.rs`, mais o wrapper que a linguagem de origem carrega) e stubs a migrar (`port.rs`). Escreve a migração apenas em `src/port.rs`; `src/lib.rs` e os wrappers são gerados, e o `refresh` regenera-os a partir das assinaturas em `port.rs`, acrescentando quaisquer novas classes de lote. Nunca mexe em `port.rs`.

Opções:

- `--batch fn:param` acrescenta uma classe que mantém muitos valores de `param` no Rust. Consulte [Lotes](/docs/concepts/batches).
- `--as-json T` faz passar como valores JSON um parâmetro de tipo que a aplicação só preenche com dados (`T` em `paginate<T>(items: T[])`).
- `--keep "name: reason"` mantém mais exports na linguagem de origem, por exemplo código que depende de dados `Intl`.
- `--kind` substitui a bridge escolhida pelo plano (napi-rs ou wasm para TypeScript).

## A bridge por linguagem

| Linguagem | Bridge |
| --- | --- |
| TypeScript/JavaScript | napi-rs ou wasm-bindgen com um formato binário gerado; classes e async incluídos |
| Python | PyO3 (abi3), conversão direta, exceções tipadas |
| C | funções `extern "C"` com os nomes originais e um header |
| Go | chamadas cgo tipadas e um formato binário gerado, num pacote com a API original |
| Ruby | uma extensão magnus dentro dos módulos originais |
| Java | JNI tipado e um formato binário gerado, em classes wrapper com os métodos originais |

As unidades PHP e C# ainda não podem passar por uma bridge; consulte [PHP](/docs/languages/php) e [C#](/docs/languages/csharp).

## O que o bridge decide

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
