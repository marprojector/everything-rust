---
title: Bridge
description: bridge crée une crate Rust pour une unité, avec les bindings et un wrapper pour que les appelants gardent l'API d'origine.
---

# Bridge

```sh
everything-rust bridge <repo> <unit-id>          # crate dans <repo>/rust/<name> ; n'écrase jamais rien
everything-rust refresh <crate>                  # après avoir changé une signature dans port.rs
```

`bridge` crée une crate avec les bindings (`lib.rs`, plus le wrapper que charge le langage source) et des stubs à porter (`port.rs`). Vous écrivez le portage uniquement dans `src/port.rs` ; `src/lib.rs` et les wrappers sont générés, et `refresh` les régénère à partir des signatures de `port.rs`, en ajoutant les nouvelles classes de lot. Il ne touche jamais à `port.rs`.

Options :

- `--batch fn:param` ajoute une classe qui garde de nombreuses valeurs de `param` en Rust. Voir [Lots](/docs/concepts/batches).
- `--as-json T` fait passer sous forme de valeurs JSON un paramètre de type que l'application ne remplit qu'avec des données (`T` dans `paginate<T>(items: T[])`).
- `--keep "name: reason"` garde d'autres exports dans le langage source, par exemple du code qui dépend des données `Intl`.
- `--kind` remplace le bridge choisi par le plan (napi-rs ou wasm pour TypeScript).

## Le bridge par langage

| Langage | Bridge |
| --- | --- |
| TypeScript/JavaScript | napi-rs ou wasm-bindgen avec un protocole binaire généré ; classes et async compris |
| Python | PyO3 (abi3), conversion directe, exceptions typées |
| C | fonctions `extern "C"` avec les noms d'origine et un en-tête |
| Go | appels cgo typés et un protocole binaire généré, dans un package avec l'API d'origine |
| Ruby | une extension magnus sous les modules d'origine |
| Java | JNI typé et un protocole binaire généré, dans des classes wrapper avec les méthodes d'origine |

Les unités PHP et C# ne peuvent pas encore passer par un bridge ; voir [PHP](/docs/languages/php) et [C#](/docs/languages/csharp).

## Ce que bridge décide

- Les exports qui ne peuvent pas traverser un bridge restent dans le langage source, avec la raison, et le module shadow
  réexporte l'original. Pour TS :
  les fonctions qui prennent ou renvoient des fonctions, les générateurs et itérateurs, RegExp, les vérifications `instanceof`,
  les génériques (`--as-json T` fait passer celui que l'application ne remplit qu'avec des données), et les valeurs dont les types (suivis à travers les imports, les réexports et `extends`) contiennent
  des fonctions. `--keep "name: reason"` en garde davantage, par exemple du code qui dépend des données `Intl`.
- Les types TS sont convertis en types Rust, avec les alias de type résolus à travers les imports et les types de paramètres
  déduits des valeurs par défaut ; les valeurs structurées passent par un protocole binaire généré (JSON seulement
  en repli pour les formes que le protocole ne modélise pas).
- JSON ne peut pas exprimer `undefined`, NaN, Infinity, -0 ni les surrogates isolés : les valeurs structurées portent
  des marqueurs (`{"$undefined": true}`, `{"$num": "NaN"}`, `{"$utf16": [...]}`) que le
  `index.cjs` généré traduit, avec des helpers dans le `js.rs` de la crate.

`record` exécute les sources TS telles quelles : un hook de résolution Node suit les imports de style bundler
(`./errors` vers `./errors.ts`), `--resolve-from` fournit les packages npm quand le dépôt n'en a aucun
d'installé, et le TS qui doit être transformé (enums, parameter properties) est réessayé avec
`--experimental-transform-types`.
