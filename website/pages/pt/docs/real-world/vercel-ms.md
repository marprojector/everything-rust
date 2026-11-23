---
title: vercel/ms
description: O vercel/ms migrado do princípio ao fim. Correto em cada caso registado, mais lento por chamada, mais rápido apenas quando um programa analisa muitas durações de uma só vez.
---

# vercel/ms

O [vercel/ms](https://github.com/vercel/ms) (5,5k estrelas, commit `4ff48ce`) converte `"2 days"` em
milissegundos e vice-versa; a maioria dos projetos Node inclui-o através de alguma dependência. Migrado do princípio ao fim
com o everything-rust 1.5.1; o crate está em
[`demo/real-world/vercel-ms-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/vercel-ms-rs).

| Etapa | Resultado |
| --- | --- |
| `analyze` | 1 unidade (`src/index.ts`), pronta, prontidão 96 |
| `bridge --kind napi` | `ms`, `parse`, `parseStrict` e `format` com bridge, nenhuma mantida em TS |
| `record --from-tests -- jest` | 975 casos: gerados, armadilhas e 340 chamadas capturadas dos 167 testes Jest do vercel/ms (a primeira migração utilizou 149 entradas copiadas manualmente desses testes) |
| migração | 200 linhas de Rust para 184 de TS, incluindo um `FormatIndex` que lê `options` uma vez por lote |
| `diff` | 784/784 coincidiram no primeiro build; 975/975 com as chamadas capturadas |
| `gate` | passa (fmt, clippy, testes, caminhos de erro, tamanho, comportamento) |
| `bench`, por chamada | 0,56x sobre os casos registados (0,39x no início): `parse` numa string normal demora 79 ns, a par do JS, mas os casos extremos e os erros lançados continuam a custar mais através da bridge |
| lote, `parse` x100.000 | JS 11.8 ms, Rust 0.65 ms: **18x**; 3,4x contando com a passagem única das strings (2,1x antes de as strings passarem unidas) |
| lote, `format` x100.000 | JS 2.1 ms, Rust 2.5 ms com a passagem: **0,8x** (0,2x no início); `format(x, { long: true })` 1,2x |

O que a migração tinha de acertar, tudo detetado por casos registados ou armadilhas:

- `/.../i` sem a flag `u` ignora maiúsculas/minúsculas apenas em letras ASCII, enquanto o `regex` do Rust o faz em Unicode, por isso
  `"1 ſ"` (s longo) e o símbolo Kelvin teriam sido interpretados como segundos. A migração compara as unidades
  manualmente, apenas em ASCII.
- O `Math.round` arredonda as metades para cima (`-2.5` dá `-2`); o `round` do Rust afasta-se do zero.
- `${n}` escreve `1e+21`, `5e-324` e `0` para `-0`; a migração reproduz a conversão de número para string do JS.
  A sua primeira versão tinha também um bug de empate (`25.728225708007812` escrito como `...813`) que as
  saídas inteiras do ms nunca atingem.
- `str.length` conta unidades UTF-16, por isso o limite de 100 caracteres conta um emoji como dois.

Encontrado na ferramenta, e corrigido: um método de lote napi com um parâmetro estruturado (o `options` do `format`)
não compilava, porque o binding gerado recebia um `serde_json::Value` que o napi
não consegue converter sem a sua feature `serde-json`. Os crates com classes de lote ativam-na agora.

Levado de volta para a ferramenta a partir desta migração:

- O `js.rs` gerado tem `js::number_to_string` e `js::math_round`, verificados face ao Node em
  20,000 doubles aleatórios; a migração utiliza-os em vez das suas próprias cópias. O `number_to_string` obtém
  os seus dígitos a partir do printer do serde_json: 5x mais rápido do que `format!` para inteiros, 3x para frações.
- Os lotes napi enviam itens string como uma única string unida com offsets finais em UTF-16, itens numéricos como
  um único `Float64Array`, e devolvem os resultados string unidos (o `index.cjs` separa-os com `slice`):
  2x mais rápido para strings e 20x para números do que um valor JS por item. `new Batch(items)` e
  `run()` mantêm a sua forma.
- O `analyze` assinala `Math.round`, `toFixed` e regexes `/i` sem `u` que mencionem `s` ou `k`,
  com orientações para cada caso.
- As chamadas custam menos: no napi, cada função síncrona passa pelo protocolo binário sem argumentos napi
  (um argumento custa cerca de 55 ns, a chamada inteira 22 ns), um wrapper JS por função em vez de
  dois, os parâmetros `serde_json::Value` passam como valores etiquetados de qualquer forma, e os erros regressam
  no buffer do protocolo. O `parse` por chamada passou de 121 para 79 ns.
- O `record --from-tests` substitui a cópia manual das entradas de teste: regista cada chamada que a suite faz
  enquanto os testes correm, com o código de captura excluído da cobertura (o ms exige 100%).

O veredicto: o `ms` é uma boa migração e um mau alvo de velocidade. Correto em cada caso registado, mais rápido
apenas quando um programa analisa muitas durações de uma só vez.
