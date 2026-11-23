---
title: Doctor
description: everything-rust doctor vérifie les runtimes des langages, la configuration de l'agent et le serveur MCP, avec la commande qui corrige chaque problème.
---

# Doctor

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor` vérifie les runtimes des langages, la configuration de l'agent et le serveur MCP, et affiche la commande qui corrige chaque problème. Il se termine avec le code 1 quand un élément requis manque.

Lancez-le quand une étape échoue parce qu'un runtime, la configuration de l'agent ou le serveur MCP est absent ou obsolète. Le skill de workflow demande aux agents de faire de même.

## Ce dont chaque langage a besoin

`record`, `diff` et `bench` exécutent l'original dans son propre runtime : Node 22.6+ pour TS (il exécute directement les sources `.ts`), Python 3.9+ pour Python et C (ctypes), Go 1.21+, Ruby 3+, JDK 17+ (`javac` et `java`). `profile` sous Node exige Node 24+, qui autorise `--cpu-prof` dans `NODE_OPTIONS`. C nécessite aussi un compilateur C. Les bridges wasm nécessitent en plus `rustup target add wasm32-unknown-unknown` et le `wasm-bindgen-cli` correspondant ; `build` indique la version exacte.

## Pour les contributeurs

`cargo run -- doctor` liste les runtimes dont vous disposez, leurs versions, et la commande qui installe ou met à jour chacun d'eux. Voir [Contribuer](/docs/project/contributing).
