---
title: Port
description: Écrire port.rs, le seul fichier que vous ou votre agent écrivez, et les règles qui le gardent fidèle à l'original.
---

# Port

Vous ou votre agent écrivez le portage dans `src/port.rs`. Tout le reste de la crate est généré. Après avoir changé une signature dans `port.rs`, lancez `everything-rust refresh <crate>`.

## Des règles qui épargnent un tour de diff

- Lisez chaque risque que le plan liste pour l'unité, à son `file:line`, avant d'écrire du code.
- **Positions dans les chaînes** : JS et Java indexent en unités de code UTF-16, Rust en octets UTF-8. Utilisez le `src/utf16.rs` généré.
- **Erreurs** : renvoyez `Err("TypeName: message")`. Cela devient cette exception en Python, Ruby et Java, et le texte de `error` en Go. Une panique Rust ne correspond jamais à une erreur enregistrée.
- **Nombres en Python et Ruby** : les entiers que l'original transmet tels quels doivent revenir sous forme d'entiers. Dans les portages Python, utilisez `crate::pynum::Num` pour un entier-ou-flottant, pas `serde_json::Value`.
- **C** : les entrées enregistrées comme `crash:` sont du comportement indéfini ; n'importe quel résultat passe.
- **Maps ordonnées et clés d'objet** : `indexmap` là où l'ordre se retrouve dans la sortie.
- **Lookaround ou références arrière dans les regex** : `fancy-regex`, pas `regex`. Dans les deux cas, `\w`, `\b`, `\d` et la correspondance insensible à la casse traitent le texte non ASCII autrement que le moteur du langage source. Mettez `unicode(false)` ou écrivez les classes ASCII en toutes lettres (`[A-Za-z0-9_]`) là où le `\w` de l'original est ASCII uniquement (JS sans `u`, Java).
- **Pas de `unsafe`.** Si c'est vraiment nécessaire : minimal, isolé, avec un commentaire `// SAFETY:`.
- **Gardez les helpers privés.** Seuls les exports de l'unité sont `pub` ; un helper `pub` fait partie de l'API exposée par le bridge.
- Ajoutez des tests unitaires Rust dans `port.rs` pour les cas sur lesquels vous raisonnez.

Le `js.rs` généré contient des helpers qui reproduisent JavaScript là où Rust diffère, comme `js::number_to_string` (`String(n)` en JS) et `js::math_round` (`Math.round`).

## Un exemple : l'emoji qui décalait une position

`packages/core/src/fuzzy.ts` de [x18n](https://github.com/x18n-dev/x18n), porté des deux façons dans
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo) :

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: ready, wasm-bindgen (core runs in browsers)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# écrivez src/port.rs
everything-rust gate demo/fuzzy-rs
```

Un premier portage qui indexait par `char` Rust passait des tests simples mais échouait au `diff` :

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

`indexOf` en JS compte les unités de code UTF-16, donc un emoji occupe deux positions. Le portage dans
`demo/*/src/port.rs` utilise les helpers `utf16` générés et passe 2000/2000 cas ainsi que le gate,
en napi-rs (`demo/fuzzy-napi`) et en wasm-bindgen (`demo/fuzzy-rs`).

Le plan que everything-rust écrit pour tout x18n : [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md).
## Quelle taille doit-il faire ?

`gate` compare les lignes du portage à celles de l'original et échoue au-delà de 3x (plus 50 lignes pour les imports et les signatures). Au-delà, cela signifie en général qu'on réécrit à la main ce qu'une crate fait déjà, ou qu'on traite les cas un par un. `analyze` liste les crates à utiliser.
