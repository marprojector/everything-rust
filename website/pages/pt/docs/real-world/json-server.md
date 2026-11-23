---
title: json-server
description: A primeira aplicação completa migrada do princípio ao fim. Um binário Rust que responde o mesmo que o servidor Node, 514/514 casos HTTP registados.
---

# json-server

O [json-server](https://github.com/typicode/json-server) (75k estrelas, MIT, commit `89a34a4`,
1.0.0-beta.15) disponibiliza uma API REST a partir de um ficheiro JSON: Node, tinyhttp e lowdb. É a primeira
aplicação completa migrada do princípio ao fim com o fluxo de aplicação completa (`app record`, `front-door`, `wire`, `lift`):
o resultado é um único binário Rust que responde o mesmo que o servidor Node. Está tudo em
[`demo/real-world/json-server-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/json-server-rs).

| Etapa | Resultado |
| --- | --- |
| `analyze --whole-app` | um servidor, 9 unidades (7 prontas, 2 com cuidado), 13 dependências a substituir (tinyhttp, lowdb, sirv, eta, milliparsec, dot-prop, sort-on, inflection...) |
| `app record --http` | 514 casos: 268 pedidos fornecidos com `--inputs` (todas as rotas e métodos, os operadores where, sort, paginação, embeds, preflight CORS, ficheiros estáticos e ranges, 29 corpos JSON malformados, `/__proto__` e outros nomes de protótipo, escapes inválidos) e 245 armadilhas; cada um registado duas vezes, ids aleatórios deixados como `[..]` |
| `front-door` | 513/514 com 99.8% de fallback; a falha é um `ü` cru no URL, a que o Node responde com 400 e que o proxy volta a codificar |
| unidades | `where-operators` 207, `parse-where` 289, `matches-where` 1,346, `paginate` 853 casos (incluindo chamadas dos próprios testes do json-server); `service` 461 e `app` 243 através de dois pequenos adaptadores que executam chamadas ao Service e pedidos HTTP reais; o `gate` passa nas seis |
| `wire` | primeiro as leituras: 514/514 com 22.6% de fallback, com o Node a responder às escritas e o Rust a recarregar o ficheiro que o Node escreveu; depois todas as rotas: 514/514 com 0% |
| `lift` | um binário totalmente em Rust (5.3 MB), 514/514; a linha de comandos do `bin.ts` migrada manualmente (ajuda, versão, verificações do ficheiro, banner de arranque) |

Depois do lift, as unidades não têm bindings, por isso o `tests/recorded.rs` de cada crate reproduz os seus
casos registados na migração: todos os 3,399 correm com `cargo test`.

O que a migração tinha de acertar, tudo detetado por casos registados:

- O router do tinyhttp não faz o despacho uma única vez: depois de cada `next()` volta a comparar todos os middlewares e
  rotas, no URL que o último middleware reescreveu, e executa todas as rotas para `HEAD`. Assim,
  `HEAD /posts` executa o handler de GET, depois o de POST, e responde com o 400 do POST; `//posts` salta os cabeçalhos
  CORS e o parser do corpo. A migração executa esse ciclo tal como é.
- O milliparsec interpreta todos os corpos de POST, PUT, PATCH e DELETE como JSON, seja qual for o seu tipo, e um corpo
  inválido dá um 500 com a mensagem do V8: `Expected double-quoted property name in JSON at position 7
  (line 1 column 8)`, `Unexpected token 'x', ..."bbbbbbb": x}" is not valid JSON`. A migração
  reproduz os erros do parser do V8, incluindo as posições em unidades UTF-16; verificado face ao Node em
  3,806 entradas malformadas.
- A ordenação utiliza `localeCompare` (ICU), feita com o collator do ICU4X e verificada em 3,000 pares;
  `Number()`, `JSON.stringify` (`1e+21`, chaves inteiras primeiro), os dois parsers de query string do Node
  (que descodificam `é%E9` de forma diferente do WHATWG), o dot-prop com as suas mensagens de erro e o inflection
  para embeds (`person` e `people`) são cada um verificados face ao Node em `json-server-js`.
- `db.data[name]` lê `Object.prototype`: `/__proto__` responde `{}`, `/constructor` um 200
  vazio, e `PATCH /__proto__` substitui o protótipo da base de dados, o que as pesquisas seguintes veem.
- O parser HTTP do Node responde 400 e fecha a ligação perante alvos não-ASCII crus e métodos desconhecidos.

Encontrado no json-server: depois de cada uma das suas próprias escritas, o seu observador de ficheiros recarrega o ficheiro. Quando o
pedido seguinte escreve antes de esse recarregamento terminar, o recarregamento pode apagar essa escrita: o primeiro
registo, enviado tão depressa quanto o servidor respondia, tinha um POST respondido com 201 que leituras posteriores não
mostravam. O `app record --pace 50` espera entre pedidos, e os casos são registados dessa forma. A migração
só recarrega quando outra pessoa alterou o ficheiro, por isso não perde essa escrita: uma
diferença deliberada.

Benchmark: Apple M4 (10 núcleos, 16 GB), macOS 27.3, Node 24.15.0 a executar `lib/bin.js` tal como o
`tsc` o compila para o npm, o binário Rust compilado com `--release`. Ambos servem uma cópia da mesma
base de dados em loopback; o `ab -k` envia os mesmos pedidos, 20,000 por linha (40 para o embed lento na
base de dados grande) depois de um aquecimento. Duas execuções, cada uma iniciada com a carga média de 1 minuto abaixo de 3;
as células indicam execução 1 / execução 2 em pedidos por segundo. O [`bench/bench.py`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/json-server-rs/bench/bench.py)
é o script.

A base de dados das fixtures (6 posts, 3 comentários):

| Pedido | Node, 1 ligação | Rust, 1 ligação | Node, 50 ligações | Rust, 50 ligações |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts` | 11,1k / 10,8k | 27,3k / 23,9k | 19,7k / 20,4k | 115,0k / 110,7k |
| `GET /posts/1` | 11,0k / 11,3k | 27,7k / 25,4k | 21,9k / 22,6k | 119,9k / 99,0k |
| filtro, ordenação, página | 10,5k / 10,0k | 25,2k / 21,5k | 18,3k / 16,4k | 107,5k / 96,3k |
| `?_embed=comments` | 8,7k / 6,6k | 22,5k / 21,8k | 13,8k / 11,0k | 98,3k / 100,6k |
| `GET /` (página de índice) | 7,3k / 6,3k | 27,6k / 21,7k | 10,2k / 9,5k | 135,1k / 119,3k |
| `GET /test.html` (estático) | 12,0k / 11,0k | 25,8k / 25,9k | 30,3k / 26,1k | 103,9k / 104,4k |
| `POST /comments`, 10 ligações | | | 2,973 / 2,467 | 5,821 / 5,976 |

Uma base de dados gerada (10,000 posts, 20,000 comentários, 3.6 MB):

| Pedido | Node, 1 ligação | Rust, 1 ligação | Node, 50 (10) ligações | Rust, 50 (10) ligações |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts/5000` | 8,4k / 9,2k | 14,6k / 14,8k | 15,5k / 16,1k | 69,5k / 67,4k |
| `?_page=3&_per_page=10` | 4,6k / 4,2k | 8,8k / 8,4k | 6,5k / 6,8k | 47,4k / 46,2k |
| `title:contains`, ordenado | 431 / 396 | 600 / 560 | 465 / 453 | 2,516 / 2,329 |
| `?_embed=comments`, uma página de 5 (10 ligações) | 2.4 / 2.4 | 81 / 103 | 2.4 / 2.2 | 253 / 216 |
| `GET /posts`, a lista inteira (10 ligações) | 328 / 319 | 399 / 407 | 381 / 387 | 1,514 / 1,657 |
| `POST /comments` (10 ligações) | | | 236 / 241 | 292 / 264 |

| | Node | Rust |
| --- | ---: | ---: |
| Do arranque à primeira resposta, base de dados das fixtures | 85 / 79 ms | 9.9 / 7.9 ms |
| Do arranque à primeira resposta, 10,000 posts | 95 / 91 ms | 48 / 47 ms |
| Memória (RSS) em repouso, base de dados das fixtures | 74 MB | 3.5 MB |
| Memória (RSS) em repouso, 10,000 posts | 94 MB | 41 MB |
| Memória (RSS) máxima durante as execuções, base de dados das fixtures | 424 / 390 MB | 18 MB |
| Memória (RSS) máxima durante as execuções, 10,000 posts | 462 / 485 MB | 289 / 291 MB |

Onde a diferença é pequena: uma ligação a ler a lista inteira de 10,000 posts (1,2x: ambos gastam o
tempo a escrever 1.5 MB de JSON), escritas na base de dados grande (1.1 a 1,2x: ambos reescrevem o ficheiro inteiro de
3.6 MB em cada escrita, com o Rust a agrupar as escritas em fila tal como o lowdb faz), e a pesquisa ordenada por título
numa ligação (1,4x). Os grandes ganhos vêm da concorrência, que um processo Node não
tem, e do embed, onde o json-server filtra todos os comentários para cada post (a migração
indexa os comentários uma vez por pedido, com o mesmo resultado).

O primeiro binário após o lift era mais lento do que o Node na base de dados grande (33 contra 298 pedidos por
segundo para a lista inteira, 39 contra 217 para escritas). O profiling encontrou, por ordem: ausência de
`TCP_NODELAY` (cada resposta grande esperava cerca de 40 ms por um ACK atrasado), a base de dados inteira
clonada para acrescentar `$schema` antes de cada escrita, cada item listado clonado antes de ser escrito, um único lock
para todos os pedidos, e `item.id` com hash SipHash para cada um dos 10,000 itens. Tudo corrigido; os 3,399
casos das unidades e os 514 casos HTTP passam depois de cada alteração.

Encontrado na ferramenta, e corrigido:

- Um servidor que escreve o seu estado não podia ser registado duas vezes a partir do mesmo ponto de partida:
  `--fresh <path>` copia agora a base de dados (e `public/`) para um diretório de trabalho novo em cada
  arranque, a partir de um snapshot guardado com os casos; `--arg` dá a cada build os argumentos do original;
  `--pace` está descrito acima.
- O `bridge` mantinha `paginate<T>(items: T[])` em TS (genérico sobre qualquer coisa, incluindo funções);
  `--as-json T` faz a bridge com valores JSON.
- As rotas com parâmetros do front door respondiam com o próprio 400 do axum para `/posts/%E0`, antes de a
  unidade ver o pedido; os parâmetros são agora lidos a partir do URI.
- O `lift` preparava apenas a unidade ligada, não os crates de que depende por caminho, e escrevia um `lib.rs`
  que o `cargo fmt --check` rejeitava; ambos corrigidos.
- Os servidores que gera não definiam `TCP_NODELAY`: uma resposta escrita em duas partes esperava
  pelo ACK atrasado do cliente, cerca de 40 ms. Agora definem-no.

O que não abrange: bases de dados JSON5 (o binário indica-o e termina), o limite de 100 KiB para o corpo
(o do Node depende de como o corpo chega em leituras de 64 KiB; a migração assume leituras completas), strings
com surrogates isolados (tornam-se U+FFFD) e um valor `1e400` na base de dados (o Node mantém `Infinity`
em memória até ao recarregamento seguinte; a migração guarda `null`, que é o que vai para o ficheiro).
