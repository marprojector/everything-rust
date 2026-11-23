---
title: Быстрый старт
description: Установите everything-rust, зарегистрируйте его в своём агенте разработки и пройдите цикл на первом модуле.
---

# Быстрый старт

## Пусть агент установит его сам

Вставьте это в Claude Code, Codex, Cursor или Gemini CLI:

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## Или установите сами

::: tabs
== cargo
```sh
cargo install everything-rust   # из crates.io
everything-rust setup           # регистрирует его в Claude Code, Codex, Cursor и Gemini CLI
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

Ещё нет тулчейна Rust? Готовые бинарники для macOS, Linux и Windows прикреплены к каждому [релизу на GitHub](https://github.com/marprojector/everything-rust/releases), а для JavaScript-проектов npm устанавливает тот же бинарник.

`setup` добавляет MCP-сервер в каждый найденный агент и устанавливает навык рабочего процесса туда, откуда агент читает навыки. Что именно он меняет, описано в разделе [Настройка агента](/docs/introduction/agents).

## Попросите агента

- *«Перепиши этот CLI на Rust с помощью everything-rust от начала до конца и докажи, что он ведёт себя так же.»*
- *«Этот поиск ощущается медленным. Профилируй его через everything-rust и перенеси горячий цикл в Rust.»*
- *«Спланируй миграцию `src/parser/` на Rust с everything-rust; портируй первый модуль и докажи, что он работает.»*
- *«Сравни производительность Rust-порта в `rust/text-rs` с оригиналом и скажи, стоит ли его выкатывать.»*

## Или выполните шаги сами

Шаги на `fuzzyScore` из x18n: `profile` находит горячий цикл, `bridge --batch` генерирует крейт, `gate` проходит, а бенчмарк сравнивает результат с TypeScript.

```sh
everything-rust profile . -- node bench.mjs              # что тормозит и кто это вызывает
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# напишите src/port.rs (или поручите агенту), затем:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # каждый записанный кейс должен совпасть
everything-rust bench rust/fuzzy-rs                      # стало быстрее? вердикт покажет
everything-rust gate rust/fuzzy-rs                       # готово, когда эта проверка пройдена
```

Каждый шаг объясняется в разделе [Цикл работы](/docs/concepts/the-loop).

::: tip Если чего-то не хватает
Если шаг падает из-за того, что среда выполнения, настройка агента или MCP-сервер отсутствуют или устарели, запустите [`everything-rust doctor`](/docs/introduction/doctor).
:::
