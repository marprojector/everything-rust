---
title: Установка
description: Установка everything-rust из crates.io, npm или бинарника из релиза, а также среды выполнения, нужные каждому языку.
---

# Установка

## Из crates.io, npm или релиза

::: tabs
== cargo
```sh
cargo install everything-rust
everything-rust setup
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

npm устанавливает тот же бинарник, для JavaScript-проектов.

Готовые бинарники для macOS, Linux и Windows прикреплены к каждому [релизу на GitHub](https://github.com/marprojector/everything-rust/releases). В Claude Code его также можно установить как плагин (MCP-сервер и навык; CLI должен быть в вашем `PATH`):

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## Из исходников

```sh
cargo install --path .
```

`record`, `diff` и `bench` запускают оригинал в его собственной среде выполнения: Node 22.6+ для TS (он выполняет
исходники `.ts` напрямую), Python 3.9+ для Python и C (ctypes), Go 1.21+, Ruby 3+, JDK 17+
(`javac` и `java`). `profile` для Node требует Node 24+, где `--cpu-prof` разрешён в `NODE_OPTIONS`. Для C также нужен компилятор C. Мостам wasm также нужны `rustup target add wasm32-unknown-unknown` и
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked`; `build` называет
точную версию.

## Windows

Нативный CI под Windows использует Rust-таргет `x86_64-pc-windows-msvc`. Установите стабильный тулчейн MSVC
и Visual Studio Build Tools с рабочей нагрузкой **Desktop development with C++**; компилятор C
и компоновщик нужны для мостов C и нативных расширений на Rust. Установите среду выполнения
исходного языка, с которого вы мигрируете. Python может быть доступен как `python3` или как
`python`; everything-rust находит оба. GNU-таргет для Windows в CI не проверяется.

Запустите набор интеграционных тестов нативных мостов из Developer PowerShell:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

Набор собирает и загружает настоящие расширения Node и Python на основе DLL, а также поддерживаемые нативные
мосты для каждой среды выполнения, найденной в `PATH`. Отсутствующие необязательные среды выполнения отмечаются как пропущенные;
Cargo, активная стандартная библиотека Rust и компоновщик MSVC обязательны. Если Cargo сообщает, что
не может найти `std`, переустановите активный таргет командами
`rustup target remove x86_64-pc-windows-msvc` и затем
`rustup target add x86_64-pc-windows-msvc`. Если падает компоновка, запустите из Visual Studio Developer
PowerShell и убедитесь, что `where.exe link` находит компоновщик MSVC. Тесты используют временные каталоги,
включая путь с пробелами и символами Unicode, и удаляют сгенерированные артефакты после
успешного прогона.

Для агентов `everything-rust setup` регистрирует всё (см. Быстрый старт). Чтобы вместо этого добавить MCP-сервер
вручную в один проект (`.mcp.json`):

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

Все инструменты, их параметры и то, что они могут затронуть: [docs/tools.md](/docs/reference/mcp-tools). Сервер также
предлагает два промпта, `port_unit` и `port_whole_app`, собранных из навыка.
## Обновление и удаление

```sh
cargo install everything-rust                                    # обновить (Cargo)
npm install -g everything-rust@latest                            # обновить (npm)
everything-rust setup                                            # обновить настройки агентов
everything-rust setup --remove && cargo uninstall everything-rust  # удалить (Cargo)
everything-rust setup --remove && npm uninstall -g everything-rust # удалить (npm)
```

`setup --remove` удаляет только записи everything-rust; сохранённые копии
(`<file>.everything-rust.bak`) остаются, пока вы их не удалите.
