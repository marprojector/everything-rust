---
title: Күйі
description: everything-rust-те бүгін не жұмыс істейді және әр бөлік соңғы рет қалай тексерілді.
---

# Күйі

Не жұмыс істейді және әр бөлік соңғы рет қалай тексерілді. Жаңартылған күні: 2026-11-18.

**Тексерілген (Verified)**: төменде аталған, басқа біреу жазған нақты кодта басынан аяғына дейін
(bridge, record, port, diff, gate) іске қосылған. **Тесттелген (Tested)**: осы репозиторийдің тесттерімен,
олар үшін жазылған fixture-лерде қамтылған, бірақ әлі нақты жобада іске қосылмаған. **Preview**: сипатталғандай,
процестің бір бөлігі үшін жұмыс істейді.

## Бірліктерді порттау

| Тіл | Күйі | Соңғы нақты іске қосу | Нәтиже |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Тексерілген | [Prettier](https://github.com/marprojector/everything-rust/blob/main/docs/real-world.md#prettier) (2026-11-18), [json-server](/docs/real-world/json-server) (2026-11-18), [Fuse.js](/docs/real-world/fuse-js) (2026-11-18), [vercel/ms](/docs/real-world/vercel-ms) (2026-11-18), x18n-нің `packages/core` бумасы | Prettier: `doc-printer-rs` 2,848/2,848 және `js-format-rs` 1,279/1,279, gate екеуінде де өтеді; json-server: алты бірлік, 3,399 кейс, gate әрқайсысында өтеді; Fuse.js, vercel/ms, x18n: 3,414/3,414, 975/975 және 9,000+ кейс сәйкес келеді |
| Python | Тексерілген | CPython 3.13-тегі `colorsys.py` (2026-11-16) | 2,400/2,400; gate өтеді |
| Go | Тексерілген | Go 1.24-тегі `path` (2026-11-16) | 2,642/2,642; gate өтеді |
| Ruby | Тексерілген | Ruby 3.3-тегі `shellwords.rb` (2026-11-16) | 1,617/1,617; gate өтеді |
| Java | Тексерілген | Facade артындағы JDK 21 (2026-11-16) | 4,492/4,492; gate өтеді |
| C | Тексерілген | Facade артындағы glibc (2026-11-16) | Undefined behavior бар бір кейсті алып тастағаннан кейін 1,859/1,859; gate өтеді |
| PHP | Preview | жоқ | Тек `analyze` және бүкіл қолданбалар; бірліктерге әлі bridge жасау мүмкін емес |
| C# | Preview | жоқ | Тек `analyze` және бүкіл қолданбалар; бірліктерге әлі bridge жасау мүмкін емес |

Әр іске қосудың толығы: [real-world.md](/docs/real-world/overview) (TS/JS) және
[real-world-languages.md](/docs/real-world/six-languages) (қалғандары).

## Командалар

| Команда | Күйі | Ескертпелер |
| --- | --- | --- |
| `analyze` | Тексерілген | жоғарыдағы әр тіл, x18n-нің 142 бірлігі |
| `bridge`, `record`, `diff`, `gate`, `build` | Тексерілген | жоғарыдағы іске қосулар; json-server-дің `paginate` функциясында `bridge --as-json` |
| `bench` | Тексерілген | жоғарыдағы іске қосулар; сандар [real-world.md](/docs/real-world/overview) ішінде |
| `record --from-tests` | Тексерілген (TS/JS), Тесттелген (Python) | vercel/ms-тің Jest жиынтығы |
| `record` және `gate` ішіндегі түпнұсқаның қамтуы | Тексерілген (TS/JS) | Fuse.js: кейстер ол импорттайтын жолдардың 81%-ын іске қосады; басқа тілдер оны әлі өлшемейді |
| `--batch` | Тексерілген | Fuse.js, vercel/ms, x18n іздеуі |
| wasm bridge (`--kind wasm`) | Тексерілген | x18n іздеуі |
| `shadow` | Тексерілген | x18n-нің `packages/core` бумасы |
| `profile` | Тесттелген | |
| `app record`, `app replay` | Тексерілген (серверлер мен CLI-лар) | json-server: `--fresh`, `--arg`, `--pace` бар 514 HTTP кейсі; Prettier: `--inputs` бар 3,080 CLI кейсі; `tests/app.rs` ішіндегі CLI-лар |
| Front door (`front-door`), `wire` | Тексерілген (TS сервері және CLI) | Prettier: бір бірлік қосылған, 57.8% fallback кезінде 3,080/3,080, glue `original::run_with_stdin` арқылы кейс бойынша бас тартады; json-server: алдымен оқулар қосылды (22.6% fallback), содан кейін әр маршрут (0%); `tests/http.rs` ішіндегі сегіз тілдегі серверлер |
| `lift` | Тексерілген (TS сервері), Тесттелген (CLI-лар) | json-server: таза Rust бинарлық файлы, 514/514; `tests/app.rs` ішіндегі барлық сегіз тілдегі CLI-лар |
| MCP сервер (`mcp`) | Тесттелген | `src/mcp.rs` ішіндегі протокол тесті |

## Тесттер қайда іске қосылады

CI macOS пен Windows-та Node, Python, Go, Ruby және Java орнатады. Ол PHP немесе
.NET орнатпайды, сондықтан PHP және C# тесттері онда өткізіліп жіберіледі және тек олар орнатылған машинада іске қосылады.
