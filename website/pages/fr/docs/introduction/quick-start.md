---
title: Démarrage rapide
description: Installez everything-rust, enregistrez-le auprès de votre agent de code et lancez la boucle sur une première unité.
---

# Démarrage rapide

## Laissez votre agent l'installer

Collez ceci dans Claude Code, Codex, Cursor ou Gemini CLI :

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## Ou installez-le vous-même

::: tabs
== cargo
```sh
cargo install everything-rust   # depuis crates.io
everything-rust setup           # l'enregistre auprès de Claude Code, Codex, Cursor et Gemini CLI
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

Pas encore de toolchain Rust ? Des binaires précompilés pour macOS, Linux et Windows sont joints à chaque [release GitHub](https://github.com/marprojector/everything-rust/releases), et pour les projets JavaScript, npm installe le même binaire.

`setup` ajoute le serveur MCP à chaque agent qu'il trouve et installe le skill de workflow là où l'agent lit ses skills. Voir [Configurer votre agent](/docs/introduction/agents) pour savoir ce qu'il modifie.

## Demandez à votre agent

- *« Réécris cette CLI en Rust avec everything-rust, de bout en bout, et prouve qu'elle se comporte pareil. »*
- *« Cette recherche paraît lente. Profile-la avec everything-rust et déplace la boucle chaude en Rust. »*
- *« Planifie une migration de `src/parser/` vers Rust avec everything-rust ; porte la première unité et prouve-la. »*
- *« Lance un bench du portage Rust dans `rust/text-rs` contre l'original et dis-moi s'il faut le livrer. »*

## Ou lancez les étapes vous-même

Les étapes sur le `fuzzyScore` de x18n : `profile` trouve la boucle chaude, `bridge --batch` génère la crate, `gate` passe, et le benchmark la compare au TypeScript.

```sh
everything-rust profile . -- node bench.mjs              # ce qui est chaud, et qui l'appelle
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# écrivez src/port.rs (ou laissez votre agent le faire), puis :
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # chaque cas enregistré doit correspondre
everything-rust bench rust/fuzzy-rs                      # plus rapide ? le verdict le dit
everything-rust gate rust/fuzzy-rs                       # terminé signifie que ceci passe
```

Chaque étape est expliquée dans [La boucle](/docs/concepts/the-loop).

::: tip Quand il manque quelque chose
Si une étape échoue parce qu'un runtime, la configuration de l'agent ou le serveur MCP est absent ou obsolète, lancez [`everything-rust doctor`](/docs/introduction/doctor).
:::
