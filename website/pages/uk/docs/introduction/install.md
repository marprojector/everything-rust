---
title: Встановлення
description: Встановіть everything-rust з crates.io, npm або з бінарника релізу, а також середовища виконання, потрібні кожній мові.
---

# Встановлення

## З crates.io, npm або релізу

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

npm встановлює той самий бінарник, для проєктів на JavaScript.

Готові бінарники для macOS, Linux і Windows додано до кожного [релізу на GitHub](https://github.com/marprojector/everything-rust/releases). У Claude Code його також можна встановити як плагін (сервер MCP і навичка; CLI має бути у вашому `PATH`):

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## Із вихідного коду

```sh
cargo install --path .
```

`record`, `diff` і `bench` запускають оригінал у його власному середовищі: Node 22.6+ для TS (він запускає вихідні
файли `.ts` напряму), Python 3.9+ для Python і C (ctypes), Go 1.21+, Ruby 3+, JDK 17+
(`javac` і `java`). `profile` на Node потребує Node 24+, який дозволяє `--cpu-prof` у `NODE_OPTIONS`. Для C також потрібен компілятор C. Для wasm-мостів також потрібні `rustup target add wasm32-unknown-unknown` і
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked`; `build` називає
точну версію.

## Windows

Нативний Windows CI використовує ціль Rust `x86_64-pc-windows-msvc`. Встановіть стабільний тулчейн MSVC
і Visual Studio Build Tools з робочим навантаженням **Desktop development with C++**; компілятор C
і лінкер потрібні для мостів C і нативних розширень Rust. Встановіть середовище виконання
мови, з якої ви мігруєте. Python може бути доступний як `python3` або як
`python`; everything-rust розпізнає обидва варіанти. GNU-ціль Windows у CI не перевіряється.

Запускайте набір інтеграційних тестів нативних мостів із Developer PowerShell:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

Набір збирає і завантажує справжні розширення Node і Python на основі DLL, а також підтримувані нативні
мости для кожного середовища виконання, наявного в `PATH`. Відсутні необов'язкові середовища позначаються як пропущені;
Cargo, активна стандартна бібліотека Rust і лінкер MSVC обов'язкові. Якщо Cargo повідомляє, що
не може знайти `std`, перевстановіть активну ціль командою
`rustup target remove x86_64-pc-windows-msvc`, а потім
`rustup target add x86_64-pc-windows-msvc`. Якщо лінкування падає, запускайте з Visual Studio Developer
PowerShell і перевірте, що `where.exe link` знаходить лінкер MSVC. Тести використовують тимчасові теки,
зокрема шлях із пробілами та символами Unicode, і прибирають згенеровані артефакти після
успішного прогону.

Для агентів `everything-rust setup` реєструє все (див. Швидкий старт). Щоб натомість додати сервер MCP
вручну до одного проєкту (`.mcp.json`):

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

Усі інструменти, їхні параметри і те, що вони можуть змінювати: [docs/tools.md](/docs/reference/mcp-tools). Сервер також
пропонує два промпти, `port_unit` і `port_whole_app`, складені з навички.
## Оновлення або видалення

```sh
cargo install everything-rust                                    # оновити (Cargo)
npm install -g everything-rust@latest                            # оновити (npm)
everything-rust setup                                            # оновити налаштування агентів
everything-rust setup --remove && cargo uninstall everything-rust  # видалити (Cargo)
everything-rust setup --remove && npm uninstall -g everything-rust # видалити (npm)
```

`setup --remove` прибирає лише записи everything-rust; збережені ним копії
(`<file>.everything-rust.bak`) лишаються, доки ви їх не видалите.
