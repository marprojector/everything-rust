---
title: TypeScript/JavaScript
description: Migre código Node.js, TypeScript ou JavaScript, para Rust através de napi-rs ou wasm-bindgen, mantendo a API original.
---

# TypeScript/JavaScript

| | |
| --- | --- |
| Unidade | uma pasta ou um ficheiro |
| Bridge | napi-rs ou wasm-bindgen com um formato binário gerado; classes e async incluídos |
| Registado e reproduzido por | Node, a executar os ficheiros `.ts` |
| Requisitos | Node 22.6+ (o `profile` precisa de Node 24+) |
| Estado | Verificado |

## Escolher a bridge

O `analyze` escolhe napi-rs quando a unidade ou algo que ela importa precisa do Node, wasm-bindgen para código portável (que também corre em navegadores), e mantém o código de UI e de editor em JS. `bridge --kind napi|wasm` sobrepõe-se a essa escolha.

Exportações que não conseguem atravessar uma bridge ficam em TS, com o motivo, e o módulo shadow reexporta o original: funções que recebem ou devolvem funções, geradores e iteradores, RegExp, verificações `instanceof`, genéricos (`--as-json T` cria a bridge para um genérico que a aplicação só preenche com dados), e valores cujos tipos contêm funções. Consulte [Bridge](/docs/concepts/bridge).

## Registar o TS tal como está

O `record` executa diretamente os ficheiros TS: um hook de resolução do Node segue os imports ao estilo dos bundlers, `--resolve-from` fornece pacotes npm quando o repositório não tem nenhum instalado, e TS que precisa de transformação (enums, parameter properties) é tentado novamente com `--experimental-transform-types`. `record --from-tests -- npx jest` regista todas as chamadas feitas pela própria suite de testes do original, com qualquer runner. A cobertura do original é medida para TS/JS ([Cobertura](/docs/concepts/coverage)).

## O que uma migração tem de acertar

Encontrado por casos registados em código real:

- `str.length`, `indexOf` e `slice` contam unidades de código UTF-16, por isso um emoji ocupa duas posições. Utilize os helpers `utf16` gerados.
- `/.../i` sem a flag `u` só ignora maiúsculas/minúsculas em letras ASCII, enquanto o `regex` do Rust o faz em Unicode, por isso `"1 ſ"` (s longo) e o símbolo Kelvin teriam sido interpretados como segundos no vercel/ms.
- `Math.round` arredonda as metades para cima (`-2.5` dá `-2`); o `round` do Rust afasta-se de zero. O `js.rs` gerado tem `js::math_round`.
- `${n}` imprime `1e+21`, `5e-324` e `0` para `-0`. `js::number_to_string` reproduz isso.
- `String.prototype.trim` utiliza o espaço em branco de JS, não o `char::is_whitespace` do Rust.

## Velocidade

Por chamada, uma migração paga a bridge todas as vezes, e o V8 executa funções pequenas em nanossegundos. O vercel/ms correu a 0.56x por chamada sobre os seus casos registados; as migrações do core do x18n são mais lentas do que o TS quando chamadas a partir de JS. Em lotes, o ciclo passa para Rust:

| Carga de trabalho | Original | Com everything-rust |
| --- | ---: | ---: |
| Pesquisa na barra lateral do x18n, 50,000 chaves (Node) | 10 ms por consulta | 0.5 ms: 20x mais rápido |
| A mesma pesquisa no navegador (wasm, uma thread) | 10 ms | 0.95 ms: 11.5x |
| Fuse.js, 50,000 strings | 91.2 ms | 2.8 ms: 32x |
| vercel/ms `parse` x100,000 | 11.8 ms | 0.65 ms: 18x (3.4x contando a passagem de dados) |
| vercel/ms `format` x100,000 | 2.1 ms | 2.5 ms com a passagem de dados: 0.8x |

Casos de estudo: [json-server](/docs/real-world/json-server), [Fuse.js](/docs/real-world/fuse-js), [vercel/ms](/docs/real-world/vercel-ms), [x18n](/docs/real-world/x18n).
