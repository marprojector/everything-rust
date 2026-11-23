---
title: Bibliothèques
description: Une bibliothèque n'a pas de processus à rejouer. Son API publique est la porte d'entrée, et les cas enregistrés de chaque unité en sont la preuve.
---

# Bibliothèques

Une bibliothèque n'a pas de processus à rejouer : son API publique est la porte d'entrée, et les cas enregistrés
de chaque unité en sont la preuve.

```sh
everything-rust app front-door .        # rust/app : une crate Rust avec l'API publique
# chaque unité passe par la boucle : bridge, record, port.rs, build, diff, gate ; puis :
everything-rust app wire . textkit-rs   # réexportée depuis rust/app ; l'original importe son shadow
everything-rust app replay .            # les cas de chaque unité câblée ; fallback = exports encore dans l'original
everything-rust app lift .
```

- `app front-door` trouve l'API publique (les exports de chaque unité) quand le dépôt n'a ni `main`
  ni serveur (`--library` sinon), et crée `rust/app` sous forme de crate bibliothèque. Chaque unité
  câblée en devient un module : `pub use textkit_rs::port as textkit;`.
- `app wire` prend une crate créée par `bridge`, exécute `gate`, et écrit le module shadow de remplacement de l'unité :
  le package d'origine l'importe à la place de l'unité, donc ses appelants gardent l'API d'origine et
  atteignent le Rust. `EVERYTHING_RUST_UNIT_<NAME>=original` renvoie l'unité vers l'original (C : compiler sans les
  options du shadow).
- `app replay` exécute les cas enregistrés de chaque unité câblée à travers ses bindings. Le taux de fallback est
  la part des exports publics auxquels Rust ne répond pas encore : unités non câblées, et exports que `bridge` garde
  dans le langage d'origine.
- `app lift` exige un fallback de 0 et que chaque unité passe `gate` ; il retire des unités les bindings, les wrappers et
  les shadows, et laisse `rust/app` comme une seule bibliothèque 100 % Rust. Il ne touche à aucun
  `port.rs`, donc les cas qui correspondaient à travers les bindings le décrivent toujours ; la checklist
  avertit des fichiers de l'original qui importent encore les shadows.

Les cinq sont aussi des outils MCP : `app_record`, `app_replay`, `app_front_door`, `app_wire` et
`app_lift`.
