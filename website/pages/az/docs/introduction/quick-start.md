---
title: Sürətli başlanğıc
description: everything-rust-i quraşdırın, onu kodlaşdırma agentinizdə qeydiyyatdan keçirin və dövrü ilk vahid üzərində işə salın.
---

# Sürətli başlanğıc

## Quraşdırmanı agentinizə həvalə edin

Bunu Claude Code, Codex, Cursor və ya Gemini CLI-a yapışdırın:

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## Və ya özünüz quraşdırın

::: tabs
== cargo
```sh
cargo install everything-rust   # crates.io-dan
everything-rust setup           # Claude Code, Codex, Cursor və Gemini CLI-da qeydiyyatdan keçirir
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

Hələ Rust toolchain-iniz yoxdur? macOS, Linux və Windows üçün hazır binary fayllar hər [GitHub release](https://github.com/marprojector/everything-rust/releases)-ə əlavə olunub, JavaScript layihələri üçün isə npm eyni binary-ni quraşdırır.

`setup` tapdığı hər agentə MCP serveri əlavə edir və iş axını skill-ini agentin skill-ləri oxuduğu yerə quraşdırır. Nəyi dəyişdiyi üçün bax [Agentinizi qurun](/docs/introduction/agents).

## Agentinizdən istəyin

- *"Bu CLI-ı everything-rust ilə başdan-başa Rust-da yenidən yaz və eyni davrandığını sübut et."*
- *"Bu axtarış yavaş görünür. everything-rust ilə profilini çıxar və isti dövrü Rust-a köçür."*
- *"`src/parser/`-in everything-rust ilə Rust-a miqrasiyasını planlaşdır; ilk vahidi port et və sübut et."*
- *"`rust/text-rs`-dəki Rust portunu orijinala qarşı benchmark et və onu buraxmağa dəyib-dəymədiyini de."*

## Və ya addımları özünüz icra edin

x18n-in `fuzzyScore` funksiyası üzərində addımlar: `profile` isti dövrü tapır, `bridge --batch` crate-i yaradır, `gate` keçir və benchmark onu TypeScript ilə müqayisə edir.

```sh
everything-rust profile . -- node bench.mjs              # isti olan nədir və onu kim çağırır
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs-i yazın (və ya agentinizə yazdırın), sonra:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # hər qeydə alınmış hal uyğun gəlməlidir
everything-rust bench rust/fuzzy-rs                      # daha sürətlidir? hökm deyəcək
everything-rust gate rust/fuzzy-rs                       # hazırdır, yəni bu keçir
```

Hər addım [Dövr](/docs/concepts/the-loop) səhifəsində izah olunur.

::: tip Nəsə çatışmayanda
Addım runtime, agent quraşdırması və ya MCP server çatışmadığı və ya köhnəldiyi üçün uğursuz olarsa, [`everything-rust doctor`](/docs/introduction/doctor) işə salın.
:::
