---
title: Тиз башлау
description: everything-rust'ны урнаштыру, аны программалау агентыгызда теркәү һәм беренче берәмлектә циклны җибәрү.
---

# Тиз башлау

## Агентыгыз урнаштырсын

Моны Claude Code, Codex, Cursor яки Gemini CLI эченә куегыз:

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## Яки үзегез урнаштырыгыз

::: tabs
== cargo
```sh
cargo install everything-rust   # crates.io'дан
everything-rust setup           # аны Claude Code, Codex, Cursor һәм Gemini CLI'да теркәп куя
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

Әле Rust toolchain юкмы? macOS, Linux һәм Windows өчен әзер бинар файллар һәр [GitHub релизына](https://github.com/marprojector/everything-rust/releases) беркетелгән, ә JavaScript проектлары өчен npm шул ук бинар файлны урнаштыра.

`setup` тапкан һәр агентка MCP серверны өсти һәм workflow skill'ны агент skill'ларны укыган урынга урнаштыра. Ул нәрсәне үзгәртүен [Агентны көйләү](/docs/introduction/agents) битеннән карагыз.

## Агентыгыздан сорагыз

- *"Бу CLI'ны everything-rust ярдәмендә башыннан ахырына кадәр Rust телендә яңадан яз һәм аның шулай ук эшләвен исбатла."*
- *"Бу эзләү әкрен кебек. Аны everything-rust белән профильлә һәм кайнар циклны Rust'ка күчер."*
- *"everything-rust белән `src/parser/` папкасын Rust'ка күчерүне планлаштыр; беренче берәмлекне күчер һәм исбатла."*
- *"`rust/text-rs` эчендәге Rust портын оригинал белән бенчмаркта чагыштыр һәм аны чыгарыргамы икәнен әйт."*

## Яки адымнарны үзегез башкарыгыз

x18n'ның `fuzzyScore` функциясендә адымнар: `profile` кайнар циклны таба, `bridge --batch` crate ясый, `gate` уза, ә бенчмарк аны TypeScript белән чагыштыра.

```sh
everything-rust profile . -- node bench.mjs              # нәрсә кайнар һәм аны кем чакыра
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs языгыз (яки агентка тапшырыгыз), аннары:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # язып алынган һәр очрак туры килергә тиеш
everything-rust bench rust/fuzzy-rs                      # тизрәкме? нәтиҗә әйтә
everything-rust gate rust/fuzzy-rs                       # бу узса, эш бетте
```

Һәр адым [Цикл](/docs/concepts/the-loop) битендә аңлатыла.

::: tip Нәрсәдер җитмәгәндә
Runtime, агент көйләнмәләре яки MCP сервер булмаганга яки искергәнгә күрә берәр адым уңышсыз тәмамланса, [`everything-rust doctor`](/docs/introduction/doctor) җибәрегез.
:::
