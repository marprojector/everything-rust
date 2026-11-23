---
title: Халәт
description: everything-rust'та бүген нәрсә эшли һәм һәр өлеш соңгы тапкыр ничек тикшерелде.
---

# Халәт

Нәрсә эшли һәм һәр өлеш соңгы тапкыр ничек тикшерелде. Яңартылды: 2026-11-18.

**Тикшерелгән**: башка кеше язган, түбәндә аталган чын кодта башыннан ахырына кадәр җибәрелде (bridge, record, port, diff, gate).
**Тестланган**: бу репозиторийның тестлары белән алар өчен язылган fixture'ларда колачланган, әлегә
чын проектта җибәрелмәгән. **Preview**: агымның бер өлеше өчен эшли, тасвирланганча.

## Берәмлекләрне күчерү

| Тел | Халәт | Соңгы чын җибәрү | Нәтиҗә |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Тикшерелгән | [Prettier](https://github.com/marprojector/everything-rust/blob/main/docs/real-world.md#prettier) (2026-11-18), [json-server](/docs/real-world/json-server) (2026-11-18), [Fuse.js](/docs/real-world/fuse-js) (2026-11-18), [vercel/ms](/docs/real-world/vercel-ms) (2026-11-18), x18n'ның `packages/core` | Prettier: `doc-printer-rs` 2,848/2,848 һәм `js-format-rs` 1,279/1,279, икесендә дә gate уза; json-server: алты берәмлек, 3,399 очрак, һәрберсендә gate уза; Fuse.js, vercel/ms, x18n: 3,414/3,414, 975/975 һәм 9,000+ очрак туры килә |
| Python | Тикшерелгән | CPython 3.13'тән `colorsys.py` (2026-11-16) | 2,400/2,400; gate уза |
| Go | Тикшерелгән | Go 1.24'тән `path` (2026-11-16) | 2,642/2,642; gate уза |
| Ruby | Тикшерелгән | Ruby 3.3'тән `shellwords.rb` (2026-11-16) | 1,617/1,617; gate уза |
| Java | Тикшерелгән | фасад артында JDK 21 (2026-11-16) | 4,492/4,492; gate уза |
| C | Тикшерелгән | фасад артында glibc (2026-11-16) | билгеләнмәгән тәртипле бер очракны алып ташлаганнан соң 1,859/1,859; gate уза |
| PHP | Preview | юк | `analyze` һәм бөтен кушымталар гына; берәмлекләр өчен bridge әлегә юк |
| C# | Preview | юк | `analyze` һәм бөтен кушымталар гына; берәмлекләр өчен bridge әлегә юк |

Һәр җибәрүнең тулы мәгълүматы: [real-world.md](/docs/real-world/overview) (TS/JS) һәм
[real-world-languages.md](/docs/real-world/six-languages) (калганнары).

## Командалар

| Команда | Халәт | Искәрмәләр |
| --- | --- | --- |
| `analyze` | Тикшерелгән | өстәге һәр тел, x18n'ның 142 берәмлеге |
| `bridge`, `record`, `diff`, `gate`, `build` | Тикшерелгән | өстәге җибәрүләр; json-server'ның `paginate` функциясендә `bridge --as-json` |
| `bench` | Тикшерелгән | өстәге җибәрүләр; саннар [real-world.md](/docs/real-world/overview) эчендә |
| `record --from-tests` | Тикшерелгән (TS/JS), Тестланган (Python) | vercel/ms'ның Jest җыелмасы |
| `record` һәм `gate` эчендә оригиналның колачлавы | Тикшерелгән (TS/JS) | Fuse.js: очраклар ул импортлаган юлларның 81% өлешен җибәрә; башка телләр аны әлегә үлчәми |
| `--batch` | Тикшерелгән | Fuse.js, vercel/ms, x18n эзләве |
| wasm bridge (`--kind wasm`) | Тикшерелгән | x18n эзләве |
| `shadow` | Тикшерелгән | x18n'ның `packages/core` |
| `profile` | Тестланган | |
| `app record`, `app replay` | Тикшерелгән (серверлар һәм CLI) | json-server: `--fresh`, `--arg`, `--pace` белән 514 HTTP очрак; Prettier: `--inputs` белән 3,080 CLI очрагы; `tests/app.rs` эчендә CLI'лар |
| Front door (`front-door`), `wire` | Тикшерелгән (TS сервер һәм CLI) | Prettier: бер берәмлек тоташтырылган, 57.8% fallback белән 3,080/3,080, glue `original::run_with_stdin` белән очрак саен баш тарта; json-server: башта уку юллары тоташтырылды (22.6% fallback), аннары һәр маршрут (0%); `tests/http.rs` эчендә сигез телдә серверлар |
| `lift` | Тикшерелгән (TS сервер), Тестланган (CLI) | json-server: саф Rust бинар файлы, 514/514; `tests/app.rs` эчендә барлык сигез телдә CLI'лар |
| MCP сервер (`mcp`) | Тестланган | `src/mcp.rs` эчендә протокол тесты |

## Тестлар кайда җибәрелә

CI macOS һәм Windows'ка Node, Python, Go, Ruby һәм Java урнаштыра. Ул PHP яки
.NET урнаштырмый, шуңа күрә PHP һәм C# тестлары анда төшереп калдырыла һәм бары алар булган машинада гына җибәрелә.
