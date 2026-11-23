---
title: Тез баштоо
description: everything-rust'ти орнотуп, коддоо агентиңиз менен каттаңыз жана биринчи бирдикте циклди иштетиңиз.
---

# Тез баштоо

## Орнотууну агентиңизге тапшырыңыз

Муну Claude Code, Codex, Cursor же Gemini CLI ичине чаптаңыз:

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## Же өзүңүз орнотуңуз

::: tabs
== cargo
```sh
cargo install everything-rust   # from crates.io
everything-rust setup           # registers it with Claude Code, Codex, Cursor and Gemini CLI
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

Rust toolchain азырынча жокпу? macOS, Linux жана Windows үчүн даяр бинардык файлдар ар бир [GitHub release](https://github.com/marprojector/everything-rust/releases) барагына тиркелген, ал эми JavaScript долбоорлору үчүн npm ошол эле бинардык файлды орнотот.

`setup` тапкан ар бир агентке MCP серверди кошот жана workflow skill'ин агент skill'дерди окуган жерге орнотот. Ал эмнени өзгөртөрүн [Агентиңизди жөндөө](/docs/introduction/agents) барагынан караңыз.

## Агентиңизден сураңыз

- *"Бул CLI'ды everything-rust менен башынан аягына чейин Rust тилинде кайра жаз жана анын так ошондой иштээрин далилде."*
- *"Бул издөө жай иштеп жаткандай. Аны everything-rust менен профилде жана hot loop'ту Rust'ка көчүр."*
- *"everything-rust менен `src/parser/` папкасын Rust'ка көчүрүү планын түз; биринчи бирдикти порттоп, далилде."*
- *"`rust/text-rs` ичиндеги Rust портун түп нуска менен бенчмарк аркылуу салыштыр да, аны чыгаруу керекпи, айт."*

## Же кадамдарды өзүңүз аткарыңыз

x18n'дин `fuzzyScore` функциясындагы кадамдар: `profile` hot loop'ту табат, `bridge --batch` crate'ти генерациялайт, `gate` өтөт, ал эми бенчмарк аны TypeScript менен салыштырат.

```sh
everything-rust profile . -- node bench.mjs              # what is hot, and who calls it
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# write src/port.rs (or let your agent), then:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # every recorded case must match
everything-rust bench rust/fuzzy-rs                      # faster? the verdict says
everything-rust gate rust/fuzzy-rs                       # done means this passes
```

Ар бир кадам [Цикл](/docs/concepts/the-loop) барагында түшүндүрүлөт.

::: tip Бир нерсе жетишпесе
Runtime, агенттин жөндөөлөрү же MCP сервер жок же эскирип калгандыктан кайсы бир кадам ишке ашпай калса, [`everything-rust doctor`](/docs/introduction/doctor) буйругун иштетиңиз.
:::
