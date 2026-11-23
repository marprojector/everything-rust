---
title: Hızlı başlangıç
description: everything-rust'i kurun, kodlama ajanınıza kaydedin ve döngüyü ilk birim üzerinde çalıştırın.
---

# Hızlı başlangıç

## Kurulumu ajanınıza bırakın

Bunu Claude Code, Codex, Cursor veya Gemini CLI'a yapıştırın:

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## Ya da kendiniz kurun

::: tabs
== cargo
```sh
cargo install everything-rust   # crates.io'dan
everything-rust setup           # Claude Code, Codex, Cursor ve Gemini CLI'a kaydeder
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

Henüz Rust toolchain'iniz yok mu? macOS, Linux ve Windows için hazır derlenmiş binary'ler her [GitHub release](https://github.com/marprojector/everything-rust/releases)'e eklenmiştir; JavaScript projeleri için npm aynı binary'yi kurar.

`setup`, bulduğu her ajana MCP sunucusunu ekler ve iş akışı skill'ini ajanın skill'leri okuduğu yere kurar. Neleri değiştirdiği için bkz. [Ajanınızı kurun](/docs/introduction/agents).

## Ajanınıza sorun

- *"Bu CLI'ı everything-rust ile baştan sona Rust'ta yeniden yaz ve aynı davrandığını kanıtla."*
- *"Bu arama yavaş geliyor. everything-rust ile profilini çıkar ve sıcak döngüyü Rust'a taşı."*
- *"`src/parser/`'ın everything-rust ile Rust'a geçişini planla; ilk birimi port et ve kanıtla."*
- *"`rust/text-rs` içindeki Rust portunu orijinaline karşı benchmark'la ve yayınlamaya değer mi söyle."*

## Ya da adımları kendiniz çalıştırın

x18n'in `fuzzyScore` fonksiyonu üzerinde adımlar: `profile` sıcak döngüyü bulur, `bridge --batch` crate'i üretir, `gate` geçer ve benchmark onu TypeScript ile karşılaştırır.

```sh
everything-rust profile . -- node bench.mjs              # sıcak olan ne, onu kim çağırıyor
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs'i yazın (ya da ajanınıza yazdırın), ardından:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # kaydedilen her vaka eşleşmeli
everything-rust bench rust/fuzzy-rs                      # daha mı hızlı? karar söyler
everything-rust gate rust/fuzzy-rs                       # bitti demek, bunun geçmesi demek
```

Her adım [Döngü](/docs/concepts/the-loop) sayfasında açıklanır.

::: tip Bir şey eksik olduğunda
Bir runtime, ajan kurulumu ya da MCP sunucusu eksik veya güncel olmadığı için bir adım başarısız olursa [`everything-rust doctor`](/docs/introduction/doctor) çalıştırın.
:::
