---
title: TypeScript/JavaScript
description: Portez du code Node.js, TypeScript ou JavaScript, en Rust via napi-rs ou wasm-bindgen, avec l'API d'origine conservée.
---

# TypeScript/JavaScript

| | |
| --- | --- |
| Unité | un dossier ou un fichier |
| Bridge | napi-rs ou wasm-bindgen avec un protocole binaire généré ; classes et async compris |
| Enregistré et rejoué par | Node, qui exécute les sources `.ts` |
| Prérequis | Node 22.6+ (`profile` exige Node 24+) |
| État | Vérifié |

## Choisir le bridge

`analyze` choisit napi-rs quand l'unité ou l'une de ses dépendances importées a besoin de Node, wasm-bindgen pour le code portable (il tourne aussi dans les navigateurs), et garde le code d'interface et d'éditeur en JS. `bridge --kind napi|wasm` remplace ce choix.

Les exports qui ne peuvent pas traverser un bridge restent en TS, avec la raison, et le module shadow réexporte l'original : les fonctions qui prennent ou renvoient des fonctions, les générateurs et itérateurs, RegExp, les vérifications `instanceof`, les génériques (`--as-json T` fait passer celui que l'application ne remplit qu'avec des données), et les valeurs dont les types contiennent des fonctions. Voir [Bridge](/docs/concepts/bridge).

## Enregistrer le TS tel quel

`record` exécute directement les sources TS : un hook de résolution Node suit les imports de style bundler, `--resolve-from` fournit les packages npm quand le dépôt n'en a aucun d'installé, et le TS qui doit être transformé (enums, parameter properties) est réessayé avec `--experimental-transform-types`. `record --from-tests -- npx jest` enregistre chaque appel que fait la propre suite de tests de l'original, avec n'importe quel runner. La couverture de l'original est mesurée pour TS/JS ([Couverture](/docs/concepts/coverage)).

## Ce qu'un portage doit respecter

Trouvé par les cas enregistrés sur du code réel :

- `str.length`, `indexOf` et `slice` comptent en unités de code UTF-16, donc un emoji occupe deux positions. Utilisez les helpers `utf16` générés.
- `/.../i` sans le drapeau `u` n'ignore la casse que pour les lettres ASCII, alors que la crate `regex` de Rust le fait pour tout Unicode, donc `"1 ſ"` (s long) et le signe Kelvin auraient été analysés comme des secondes dans vercel/ms.
- `Math.round` arrondit les moitiés vers le haut (`-2.5` donne `-2`) ; le `round` de Rust s'éloigne de zéro. Le `js.rs` généré fournit `js::math_round`.
- `${n}` affiche `1e+21`, `5e-324` et `0` pour `-0`. `js::number_to_string` le reproduit.
- `String.prototype.trim` utilise les espaces blancs de JS, pas le `char::is_whitespace` de Rust.

## Vitesse

Par appel, un portage paie le bridge à chaque fois, et V8 exécute les petites fonctions en quelques nanosecondes. vercel/ms tournait à 0.56x par appel sur ses cas enregistrés ; les portages du cœur de x18n sont plus lents que le TS lorsqu'ils sont appelés depuis JS. En lot, la boucle passe en Rust :

| Charge | Original | Avec everything-rust |
| --- | ---: | ---: |
| Recherche de la barre latérale de x18n, 50,000 clés (Node) | 10 ms par requête | 0.5 ms : 20x plus rapide |
| La même recherche dans le navigateur (wasm, un thread) | 10 ms | 0.95 ms : 11.5x |
| Fuse.js, 50,000 chaînes | 91.2 ms | 2.8 ms : 32x |
| vercel/ms `parse` x100,000 | 11.8 ms | 0.65 ms : 18x (3.4x en comptant le transfert) |
| vercel/ms `format` x100,000 | 2.1 ms | 2.5 ms avec le transfert : 0.8x |

Études de cas : [json-server](/docs/real-world/json-server), [Fuse.js](/docs/real-world/fuse-js), [vercel/ms](/docs/real-world/vercel-ms), [x18n](/docs/real-world/x18n).
