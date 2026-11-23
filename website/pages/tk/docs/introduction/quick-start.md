---
title: Çalt başlangyç
description: everything-rust-i gurnaň, ony kodlaýyş agentiňiz bilen hasaba alyň we aýlawy ilkinji birlikde işlediň.
---

# Çalt başlangyç

## Gurnamagy agentiňize tabşyryň

Muny Claude Code, Codex, Cursor ýa-da Gemini CLI içine goýuň:

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## Ýa-da özüňiz gurnaň

::: tabs
== cargo
```sh
cargo install everything-rust   # crates.io-dan
everything-rust setup           # ony Claude Code, Codex, Cursor we Gemini CLI bilen hasaba alýar
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

Entek Rust toolchain ýokmy? macOS, Linux we Windows üçin taýýar binar faýllar her [GitHub release](https://github.com/marprojector/everything-rust/releases) sahypasyna goşulan, JavaScript taslamalary üçin bolsa npm şol bir binar faýly gurnaýar.

`setup` tapan her agentine MCP serweri goşýar we workflow skill-ini agentiň skill-leri okaýan ýerine gurnaýar. Onuň nämeleri üýtgedýändigi üçin [Agentiňizi sazlaň](/docs/introduction/agents) sahypasyna serediň.

## Agentiňizden soraň

- *"Bu CLI-ni everything-rust bilen başdan-aýak Rust-da täzeden ýaz we onuň edil şonuň ýaly işleýändigini subut et."*
- *"Bu gözleg haýal ýaly. Ony everything-rust bilen profille we hot loop-y Rust-a göçür."*
- *"everything-rust bilen `src/parser/` bukjasyny Rust-a geçirmegiň meýilnamasyny düz; birinji birligi geçir we subut et."*
- *"`rust/text-rs` içindäki Rust portuny asyl nusga bilen benchmark arkaly deňeşdir we ony çykarmalymy, aýt."*

## Ýa-da ädimleri özüňiz ýerine ýetiriň

x18n-iň `fuzzyScore` funksiýasyndaky ädimler: `profile` hot loop-y tapýar, `bridge --batch` crate-i döredýär, `gate` geçýär, benchmark bolsa ony TypeScript bilen deňeşdirýär.

```sh
everything-rust profile . -- node bench.mjs              # näme gyzgyn we ony kim çagyrýar
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs ýazyň (ýa-da agentiňize tabşyryň), soňra:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # ýazga alnan her ýagdaý gabat gelmeli
everything-rust bench rust/fuzzy-rs                      # has çaltmy? netije aýdýar
everything-rust gate rust/fuzzy-rs                       # taýýar diýmek şu geçýär diýmek
```

Her ädim [Aýlaw](/docs/concepts/the-loop) sahypasynda düşündirilýär.

::: tip Bir zat ýetmezçilik edende
Runtime, agent sazlamasy ýa-da MCP serwer ýok ýa-da köne bolany sebäpli bir ädim şowsuz bolsa, [`everything-rust doctor`](/docs/introduction/doctor) işlediň.
:::
