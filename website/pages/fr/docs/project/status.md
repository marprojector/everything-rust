---
title: État
description: Ce qui fonctionne aujourd'hui dans everything-rust, et comment chaque partie a été vérifiée pour la dernière fois.
---

# État

Ce qui fonctionne, et comment chaque partie a été vérifiée pour la dernière fois. Mis à jour le 2026-11-18.

**Vérifié** : exécuté de bout en bout (bridge, record, port, diff, gate) sur du vrai code écrit par quelqu'un d'autre,
nommé ci-dessous. **Testé** : couvert par les tests de ce dépôt sur des fixtures écrites pour eux, pas encore
exécuté sur un vrai projet. **Préversion** : fonctionne pour une partie du flux, comme décrit.

## Porter des unités

| Langage | État | Dernière exécution réelle | Résultat |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Vérifié | [Prettier](https://github.com/marprojector/everything-rust/blob/main/docs/real-world.md#prettier) (2026-11-18), [json-server](/docs/real-world/json-server) (2026-11-18), [Fuse.js](/docs/real-world/fuse-js) (2026-11-18), [vercel/ms](/docs/real-world/vercel-ms) (2026-11-18), `packages/core` de x18n | Prettier : `doc-printer-rs` 2,848/2,848 et `js-format-rs` 1,279/1,279, le gate passe sur les deux ; json-server : six unités, 3,399 cas, le gate passe sur chacune ; Fuse.js, vercel/ms, x18n : 3,414/3,414, 975/975 et plus de 9,000 cas correspondent |
| Python | Vérifié | `colorsys.py` de CPython 3.13 (2026-11-16) | 2,400/2,400 ; le gate passe |
| Go | Vérifié | `path` de Go 1.24 (2026-11-16) | 2,642/2,642 ; le gate passe |
| Ruby | Vérifié | `shellwords.rb` de Ruby 3.3 (2026-11-16) | 1,617/1,617 ; le gate passe |
| Java | Vérifié | JDK 21 derrière une façade (2026-11-16) | 4,492/4,492 ; le gate passe |
| C | Vérifié | glibc derrière une façade (2026-11-16) | 1,859/1,859 après retrait d'un cas de comportement indéfini ; le gate passe |
| PHP | Préversion | aucune | `analyze` et applications entières uniquement ; les unités ne peuvent pas encore passer par un bridge |
| C# | Préversion | aucune | `analyze` et applications entières uniquement ; les unités ne peuvent pas encore passer par un bridge |

Détails de chaque exécution : [real-world.md](/docs/real-world/overview) (TS/JS) et
[real-world-languages.md](/docs/real-world/six-languages) (les autres).

## Commandes

| Commande | État | Notes |
| --- | --- | --- |
| `analyze` | Vérifié | chaque langage ci-dessus, les 142 unités de x18n |
| `bridge`, `record`, `diff`, `gate`, `build` | Vérifié | les exécutions ci-dessus ; `bridge --as-json` sur le `paginate` de json-server |
| `bench` | Vérifié | les exécutions ci-dessus ; chiffres dans [real-world.md](/docs/real-world/overview) |
| `record --from-tests` | Vérifié (TS/JS), Testé (Python) | la suite Jest de vercel/ms |
| Couverture de l'original dans `record` et `gate` | Vérifié (TS/JS) | Fuse.js : les cas exécutent 81% des lignes qu'il importe ; les autres langages ne la mesurent pas encore |
| `--batch` | Vérifié | Fuse.js, vercel/ms, recherche de x18n |
| bridge wasm (`--kind wasm`) | Vérifié | recherche de x18n |
| `shadow` | Vérifié | `packages/core` de x18n |
| `profile` | Testé | |
| `app record`, `app replay` | Vérifié (serveurs et CLI) | json-server : 514 cas HTTP avec `--fresh`, `--arg`, `--pace` ; Prettier : 3,080 cas CLI avec `--inputs` ; CLI dans `tests/app.rs` |
| Porte d'entrée (`front-door`), `wire` | Vérifié (serveur et CLI TS) | Prettier : une unité câblée, 3,080/3,080 à 57.8% de fallback, la glue déclinant cas par cas avec `original::run_with_stdin` ; json-server : les lectures câblées d'abord (22.6% de fallback), puis toutes les routes (0%) ; serveurs dans huit langages dans `tests/http.rs` |
| `lift` | Vérifié (serveur TS), Testé (CLI) | json-server : binaire 100 % Rust, 514/514 ; CLI dans les huit langages dans `tests/app.rs` |
| Serveur MCP (`mcp`) | Testé | test de protocole dans `src/mcp.rs` |

## Où tournent les tests

La CI installe Node, Python, Go, Ruby et Java sur macOS et Windows. Elle n'installe ni PHP ni
.NET, donc les tests PHP et C# y sont ignorés et ne tournent que sur une machine qui les a.
