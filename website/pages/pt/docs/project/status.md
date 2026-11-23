---
title: Estado
description: O que funciona hoje no everything-rust e como cada parte foi verificada pela última vez.
---

# Estado

O que funciona e como cada parte foi verificada pela última vez. Atualizado a 2026-11-18.

**Verificado**: executado de ponta a ponta (bridge, record, port, diff, gate) em código real escrito por outra pessoa,
indicado abaixo. **Testado**: coberto pelos testes deste repositório em fixtures escritas para o efeito, ainda não
executado num projeto real. **Pré-visualização**: funciona para parte do fluxo, conforme descrito.

## Migração de unidades

| Linguagem | Estado | Última execução real | Resultado |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Verificado | [Prettier](https://github.com/marprojector/everything-rust/blob/main/docs/real-world.md#prettier) (2026-11-18), [json-server](/docs/real-world/json-server) (2026-11-18), [Fuse.js](/docs/real-world/fuse-js) (2026-11-18), [vercel/ms](/docs/real-world/vercel-ms) (2026-11-18), `packages/core` do x18n | Prettier: `doc-printer-rs` 2,848/2,848 e `js-format-rs` 1,279/1,279, o gate passa em ambos; json-server: seis unidades, 3,399 casos, o gate passa em cada uma; Fuse.js, vercel/ms, x18n: 3,414/3,414, 975/975 e mais de 9,000 casos coincidem |
| Python | Verificado | `colorsys.py` do CPython 3.13 (2026-11-16) | 2,400/2,400; o gate passa |
| Go | Verificado | `path` do Go 1.24 (2026-11-16) | 2,642/2,642; o gate passa |
| Ruby | Verificado | `shellwords.rb` do Ruby 3.3 (2026-11-16) | 1,617/1,617; o gate passa |
| Java | Verificado | JDK 21 por trás de uma fachada (2026-11-16) | 4,492/4,492; o gate passa |
| C | Verificado | glibc por trás de uma fachada (2026-11-16) | 1,859/1,859 depois de retirar um caso de comportamento indefinido; o gate passa |
| PHP | Pré-visualização | nenhuma | apenas `analyze` e aplicações completas; ainda não é possível criar a bridge das unidades |
| C# | Pré-visualização | nenhuma | apenas `analyze` e aplicações completas; ainda não é possível criar a bridge das unidades |

Detalhes de cada execução: [real-world.md](/docs/real-world/overview) (TS/JS) e
[real-world-languages.md](/docs/real-world/six-languages) (as restantes).

## Comandos

| Comando | Estado | Notas |
| --- | --- | --- |
| `analyze` | Verificado | todas as linguagens acima, as 142 unidades do x18n |
| `bridge`, `record`, `diff`, `gate`, `build` | Verificado | as execuções acima; `bridge --as-json` no `paginate` do json-server |
| `bench` | Verificado | as execuções acima; números em [real-world.md](/docs/real-world/overview) |
| `record --from-tests` | Verificado (TS/JS), Testado (Python) | a suite Jest do vercel/ms |
| Cobertura do original em `record` e `gate` | Verificado (TS/JS) | Fuse.js: os casos executam 81% das linhas que importa; as outras linguagens ainda não a medem |
| `--batch` | Verificado | Fuse.js, vercel/ms, pesquisa do x18n |
| Bridge wasm (`--kind wasm`) | Verificado | pesquisa do x18n |
| `shadow` | Verificado | `packages/core` do x18n |
| `profile` | Testado | |
| `app record`, `app replay` | Verificado (servidores e CLIs) | json-server: 514 casos HTTP com `--fresh`, `--arg`, `--pace`; Prettier: 3,080 casos de CLI com `--inputs`; CLIs em `tests/app.rs` |
| Porta de entrada (`front-door`), `wire` | Verificado (servidor e CLI em TS) | Prettier: uma unidade ligada, 3,080/3,080 com 57.8% de fallback, a cola a diminuir caso a caso com `original::run_with_stdin`; json-server: primeiro as leituras ligadas (22.6% de fallback), depois todas as rotas (0%); servidores em oito linguagens em `tests/http.rs` |
| `lift` | Verificado (servidor TS), Testado (CLIs) | json-server: binário em Rust puro, 514/514; CLIs nas oito linguagens em `tests/app.rs` |
| Servidor MCP (`mcp`) | Testado | teste de protocolo em `src/mcp.rs` |

## Onde os testes são executados

A CI instala Node, Python, Go, Ruby e Java em macOS e Windows. Não instala PHP nem
.NET, por isso os testes de PHP e C# são ignorados aí e só são executados numa máquina que os tenha.
