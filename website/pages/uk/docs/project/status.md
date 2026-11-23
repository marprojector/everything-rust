---
title: Статус
description: Що працює в everything-rust сьогодні і як кожну частину перевіряли востаннє.
---

# Статус

Що працює і як кожну частину перевіряли востаннє. Оновлено 2026-11-18.

**Перевірено**: запущено від початку до кінця (bridge, record, port, diff, gate) на реальному коді, написаному кимось іншим,
названому нижче. **Протестовано**: покрито тестами цього репозиторію на фікстурах, написаних для них, але ще не
запускалося на реальному проєкті. **Попередній перегляд**: працює для частини процесу, як описано.

## Портування модулів

| Мова | Статус | Останній реальний прогін | Результат |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Перевірено | [Prettier](https://github.com/marprojector/everything-rust/blob/main/docs/real-world.md#prettier) (2026-11-18), [json-server](/docs/real-world/json-server) (2026-11-18), [Fuse.js](/docs/real-world/fuse-js) (2026-11-18), [vercel/ms](/docs/real-world/vercel-ms) (2026-11-18), `packages/core` з x18n | Prettier: `doc-printer-rs` 2,848/2,848 і `js-format-rs` 1,279/1,279, gate проходить на обох; json-server: шість модулів, 3,399 випадків, gate проходить на кожному; Fuse.js, vercel/ms, x18n: 3,414/3,414, 975/975 і 9,000+ випадків збігаються |
| Python | Перевірено | `colorsys.py` з CPython 3.13 (2026-11-16) | 2,400/2,400; gate проходить |
| Go | Перевірено | `path` з Go 1.24 (2026-11-16) | 2,642/2,642; gate проходить |
| Ruby | Перевірено | `shellwords.rb` з Ruby 3.3 (2026-11-16) | 1,617/1,617; gate проходить |
| Java | Перевірено | JDK 21 за фасадом (2026-11-16) | 4,492/4,492; gate проходить |
| C | Перевірено | glibc за фасадом (2026-11-16) | 1,859/1,859 після вилучення одного випадку з невизначеною поведінкою; gate проходить |
| PHP | Попередній перегляд | немає | лише `analyze` і цілі застосунки; модулі поки не можна з'єднати мостом |
| C# | Попередній перегляд | немає | лише `analyze` і цілі застосунки; модулі поки не можна з'єднати мостом |

Подробиці кожного прогону: [real-world.md](/docs/real-world/overview) (TS/JS) і
[real-world-languages.md](/docs/real-world/six-languages) (інші).

## Команди

| Команда | Статус | Примітки |
| --- | --- | --- |
| `analyze` | Перевірено | кожна мова вище, 142 модулі x18n |
| `bridge`, `record`, `diff`, `gate`, `build` | Перевірено | прогони вище; `bridge --as-json` на `paginate` з json-server |
| `bench` | Перевірено | прогони вище; числа в [real-world.md](/docs/real-world/overview) |
| `record --from-tests` | Перевірено (TS/JS), Протестовано (Python) | набір тестів Jest з vercel/ms |
| Покриття оригіналу в `record` і `gate` | Перевірено (TS/JS) | Fuse.js: випадки виконують 81% рядків, які він імпортує; інші мови поки цього не вимірюють |
| `--batch` | Перевірено | Fuse.js, vercel/ms, пошук x18n |
| wasm-міст (`--kind wasm`) | Перевірено | пошук x18n |
| `shadow` | Перевірено | `packages/core` з x18n |
| `profile` | Протестовано | |
| `app record`, `app replay` | Перевірено (сервери і CLI) | json-server: 514 HTTP-випадків з `--fresh`, `--arg`, `--pace`; Prettier: 3,080 випадків CLI з `--inputs`; CLI в `tests/app.rs` |
| Front door (`front-door`), `wire` | Перевірено (сервер і CLI на TS) | Prettier: один модуль підключено, 3,080/3,080 при 57.8% fallback, glue відмовляється від окремих випадків через `original::run_with_stdin`; json-server: спершу підключене читання (22.6% fallback), потім кожен маршрут (0%); сервери вісьмома мовами в `tests/http.rs` |
| `lift` | Перевірено (сервер на TS), Протестовано (CLI) | json-server: чистий бінарник Rust, 514/514; CLI усіма вісьмома мовами в `tests/app.rs` |
| Сервер MCP (`mcp`) | Протестовано | тест протоколу в `src/mcp.rs` |

## Де запускаються тести

CI встановлює Node, Python, Go, Ruby і Java на macOS і Windows. Він не встановлює PHP чи
.NET, тож тести PHP і C# там пропускаються і запускаються лише на машині, де вони є.
