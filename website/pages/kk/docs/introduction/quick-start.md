---
title: Жылдам бастау
description: everything-rust-ті орнатыңыз, оны кодинг-агентіңізде тіркеңіз және алғашқы бірлікте циклды іске қосыңыз.
---

# Жылдам бастау

## Орнатуды агентке тапсырыңыз

Мынаны Claude Code, Codex, Cursor немесе Gemini CLI ішіне қойыңыз:

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## Немесе өзіңіз орнатыңыз

::: tabs
== cargo
```sh
cargo install everything-rust   # crates.io-дан
everything-rust setup           # Claude Code, Codex, Cursor және Gemini CLI ішінде тіркейді
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

Rust toolchain әлі жоқ па? macOS, Linux және Windows үшін дайын бинарлық файлдар әр [GitHub release](https://github.com/marprojector/everything-rust/releases) бетіне тіркелген, ал JavaScript жобалары үшін npm сол бинарлық файлды орнатады.

`setup` тапқан әр агентке MCP серверді қосады және workflow skill-ін агент skill-дерді оқитын жерге орнатады. Оның нені өзгертетінін [Агентті баптау](/docs/introduction/agents) бетінен қараңыз.

## Агентіңізден сұраңыз

- *"Осы CLI-ды everything-rust көмегімен басынан аяғына дейін Rust тілінде қайта жаз және оның дәл солай жұмыс істейтінін дәлелде."*
- *"Бұл іздеу баяу сияқты. Оны everything-rust көмегімен профайлда және hot loop-ты Rust-қа көшір."*
- *"everything-rust көмегімен `src/parser/` бумасын Rust-қа көшіру жоспарын жаса; бірінші бірлікті порттап, дәлелде."*
- *"`rust/text-rs` ішіндегі Rust портын түпнұсқамен бенчмарк арқылы салыстыр да, оны шығару керек пе, айт."*

## Немесе қадамдарды өзіңіз орындаңыз

x18n-нің `fuzzyScore` функциясындағы қадамдар: `profile` hot loop-ты табады, `bridge --batch` crate-ті генерациялайды, `gate` өтеді, ал бенчмарк оны TypeScript-пен салыстырады.

```sh
everything-rust profile . -- node bench.mjs              # не ыстық және оны кім шақырады
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs жазыңыз (немесе агентіңізге тапсырыңыз), содан кейін:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # әр жазылған кейс сәйкес келуі керек
everything-rust bench rust/fuzzy-rs                      # жылдамырақ па? вердикт айтады
everything-rust gate rust/fuzzy-rs                       # дайын дегеніміз осының өтуі
```

Әр қадам [Цикл](/docs/concepts/the-loop) бетінде түсіндірілген.

::: tip Бірдеңе жетіспесе
Runtime, агент баптауы немесе MCP сервер жоқ не ескірген болғандықтан бір қадам құласа, [`everything-rust doctor`](/docs/introduction/doctor) іске қосыңыз.
:::
