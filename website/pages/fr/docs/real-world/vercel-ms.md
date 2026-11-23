---
title: vercel/ms
description: vercel/ms porté de bout en bout. Correct sur chaque cas enregistré, plus lent par appel, plus rapide seulement quand un programme analyse de nombreuses durées d'un coup.
---

# vercel/ms

[vercel/ms](https://github.com/vercel/ms) (5.5k étoiles, commit `4ff48ce`) transforme `"2 days"` en
millisecondes et inversement ; la plupart des projets Node l'embarquent via une dépendance ou une autre. Porté de bout en bout
avec everything-rust 1.5.1 ; la crate se trouve dans
[`demo/real-world/vercel-ms-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/vercel-ms-rs).

| Étape | Résultat |
| --- | --- |
| `analyze` | 1 unité (`src/index.ts`), prête, maturité 96 |
| `bridge --kind napi` | `ms`, `parse`, `parseStrict` et `format` passés par bridge, aucun gardé en TS |
| `record --from-tests -- jest` | 975 cas : les cas générés, les pièges, et 340 appels capturés depuis les 167 tests Jest de vercel/ms (le premier portage utilisait 149 entrées recopiées à la main depuis ces tests) |
| portage | 200 lignes de Rust pour 184 de TS, dont un `FormatIndex` qui lit `options` une fois par lot |
| `diff` | 784/784 correspondaient dès le premier build ; 975/975 avec les appels capturés |
| `gate` | passe (fmt, clippy, tests, chemins d'erreur, taille, comportement) |
| `bench`, par appel | 0.56x sur les cas enregistrés (0.39x au départ) : `parse` sur une chaîne normale prend 79 ns, à égalité avec JS, mais les cas limites et les erreurs levées coûtent encore plus cher à travers le bridge |
| lot, `parse` x100,000 | JS 11.8 ms, Rust 0.65 ms : **18x** ; 3.4x en comptant le transfert unique des chaînes (2.1x avant que les chaînes ne passent jointes) |
| lot, `format` x100,000 | JS 2.1 ms, Rust 2.5 ms avec le transfert : **0.8x** (0.2x au départ) ; `format(x, { long: true })` 1.2x |

Ce que le portage devait respecter, tout cela détecté par les cas enregistrés ou les pièges :

- `/.../i` sans le drapeau `u` n'ignore la casse que pour les lettres ASCII, alors que la crate `regex` de Rust le fait pour tout Unicode, donc
  `"1 ſ"` (s long) et le signe Kelvin auraient été analysés comme des secondes. Le portage reconnaît les unités
  à la main, en ASCII uniquement.
- `Math.round` arrondit les moitiés vers le haut (`-2.5` donne `-2`) ; le `round` de Rust s'éloigne de zéro.
- `${n}` affiche `1e+21`, `5e-324` et `0` pour `-0` ; le portage reproduit la conversion nombre-texte de JS.
  Sa première version avait aussi un bug d'égalité (`25.728225708007812` affiché `...813`) que les
  sorties entières de ms n'atteignent jamais.
- `str.length` compte les unités UTF-16, donc la limite de 100 caractères compte un emoji pour deux.

Trouvé dans l'outil, et corrigé : une méthode de lot napi avec un paramètre structuré (les `options` de `format`)
ne compilait pas, car le binding généré prenait un `serde_json::Value` que napi
ne peut pas convertir sans sa feature `serde-json`. Les crates avec des classes de lot l'activent désormais.

Repris dans l'outil à partir de ce portage :

- Le `js.rs` généré fournit `js::number_to_string` et `js::math_round`, vérifiés face à Node sur
  20,000 doubles aléatoires ; le portage les utilise au lieu de ses propres copies. `number_to_string` prend
  ses chiffres dans l'afficheur de serde_json : 5x plus rapide que `format!` pour les entiers, 3x pour les fractions.
- Les lots napi envoient les éléments chaînes sous forme d'une seule chaîne jointe avec les positions de fin en UTF-16, les éléments nombres sous forme
  d'un seul `Float64Array`, et renvoient les résultats chaînes joints (`index.cjs` les découpe avec `slice`) :
  2x plus rapide pour les chaînes et 20x pour les nombres qu'une valeur JS par élément. `new Batch(items)` et
  `run()` gardent leur forme.
- `analyze` signale `Math.round`, `toFixed` et les regex `/i` sans `u` qui mentionnent `s` ou `k`,
  avec des conseils pour chacun.
- Les appels coûtent moins : sous napi, chaque fonction synchrone passe par le protocole binaire sans argument napi
  (un argument coûte environ 55 ns, l'appel entier 22 ns), un seul wrapper JS par fonction au lieu de
  deux, les paramètres `serde_json::Value` passent sous forme de valeurs étiquetées de n'importe quelle forme, et les erreurs reviennent
  dans le tampon du protocole. `parse` par appel est passé de 121 à 79 ns.
- `record --from-tests` remplace la copie manuelle des entrées de test : il enregistre chaque appel que fait la suite
  pendant l'exécution des tests, le code de capture étant exclu de la couverture (ms impose 100%).

Le verdict : `ms` est un bon portage et une mauvaise cible de vitesse. Correct sur chaque cas enregistré, plus rapide
seulement quand un programme analyse de nombreuses durées d'un coup.
