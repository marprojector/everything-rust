---
title: Lift
description: app lift fait de rust/app une application 100 % Rust une fois que le replay passe avec un taux de fallback de 0 et que chaque unité passe le gate.
---

# Lift

Quand le replay passe avec un taux de fallback de 0 et que chaque unité passe le gate, faites-en une application 100 % Rust :

```sh
everything-rust app lift .
```

- `app lift` refuse tant que ces deux conditions ne sont pas réunies, et dit ce qui bloque : les cas qui retombent encore sur l'original, la glue encore à l'état de stub, les unités qui échouent au gate.
- D'abord sur une copie, il retire le chemin de fallback et les interrupteurs d'arrêt, transforme chaque crate d'unité en simple bibliothèque de `rust/app` (sans `cdylib`, bindings ni wrappers générés), et ajoute `rust/app/tests/app_cases.rs`, qui exécute `rust/app-cases` avec trycmd lors de `cargo test`, donc la CI n'a besoin que de cargo.
- Ensuite il déroule la checklist du lift : fmt, clippy `-D warnings` et les tests sur chaque unité et sur `rust/app`, chaque cible nommée dans `rust/DECISIONS.md` compilée ou vérifiée là où elle est installée, les fichiers lus à l'exécution et les dossiers d'assets non embarqués, et le replay sur le binaire 100 % Rust. Ce n'est que lorsque tout passe que les changements sont réécrits. L'original n'est jamais modifié.

## Après le lift

Les unités n'ont plus de bindings. Sur json-server, le `tests/recorded.rs` de chaque crate rejoue ses cas enregistrés sur le portage : les 3,399 tournent avec `cargo test`, à côté des 514 cas HTTP.

Une fois l'original disparu, restructurez librement le Rust ; `app replay` prouve toujours le comportement.

## Puis la bascule

Le skill de workflow demande à l'agent de faire ce qu'il peut et d'écrire `rust/cutover.sh`, un script interactif pour les étapes que vous seul pouvez faire (déploiements, DNS, secrets), avec un retour arrière possible à chaque étape.
