---
title: Record
description: record exécute l'original sur des cas limites générés, des pièges sémantiques, ses propres tests et vos entrées, et sauvegarde les résultats dans cases.jsonl.
---

# Record

```sh
everything-rust record <crate>
everything-rust record <crate> --from-tests -- npx jest      # ou : -- pytest
everything-rust record <crate> --inputs extra.jsonl
```

`record` exécute les fonctions d'origine et sauvegarde ce que chacune renvoie ou lève dans `cases.jsonl`, à côté de la crate : le comportement que le portage doit reproduire. Un écart par la suite est une différence de comportement : corrigez le portage, jamais l'enregistrement.

## D'où viennent les cas

- **Cas limites générés** : non-ASCII, emoji, `-0`, `NaN`, des entiers là où des flottants sont attendus, et plus encore (`--count` fixe leur nombre par fonction).
- **Pièges sémantiques** dans chaque paramètre : longueur et casse des chaînes, division signée, limites de dépassement, affichage des flottants, ordre des maps et des tris.
- **Pièges regex** pour chaque fonction d'un fichier qui utilise des regex : le `ı` turc sans point, le signe Kelvin, le s long, les chiffres arabes-indiens, les séparateurs de ligne, le sigma final, les zero-width joiners.
- **Les propres tests de l'original** : `--from-tests -- <test command>` enregistre chaque appel que la suite de tests fait aux fonctions exportées de l'unité (TS/JS avec n'importe quel runner, Python avec pytest ou unittest). La source est restaurée ensuite. Sur vercel/ms, cela a capturé 340 appels issus de 167 tests Jest.
- **Vos entrées** : `--inputs cases.jsonl` avec un `{"fn": name, "args": [...]}` par ligne. Ajoutez de vraies entrées (échantillons de production, rapports de bugs) ; l'entrée utilisateur qui casse la prod est rarement celle à laquelle un générateur pense.

Les appelants Python passent `1` aussi souvent que `1.0`, donc l'enregistrement passe les nombres entiers sous forme d'entiers dans un cas Python sur quatre.

## Marqueurs dans cases.jsonl

JSON ne peut pas exprimer `undefined`, NaN, Infinity, -0 ni les surrogates isolés. Les valeurs structurées portent des marqueurs que le `index.cjs` généré traduit, avec des helpers dans le `js.rs` de la crate :

| Marqueur | Signification |
| --- | --- |
| `{"$undefined": true}` | `undefined` en JavaScript |
| `{"$num": "NaN"}` | un nombre que JSON ne peut pas contenir (`NaN`, `Infinity`, `-0`) |
| `{"$utf16": [...]}` | une chaîne donnée en unités de code UTF-16, pour les surrogates isolés |

Des entrées C hors du contrat d'une fonction peuvent faire planter l'original. Le pilote C exécute chaque cas dans un processus enfant forké et enregistre `crash: SIGSEGV` ; `diff` accepte alors n'importe quel comportement du portage, puisque celui du C est indéfini.

## Deux exécutions, et ce qui diffère

`record` exécute l'original deux fois et trie ce qui a différé entre les exécutions : les valeurs changeantes (heures, identifiants) deviennent des jokers ; les mêmes lignes dans un autre ordre (tables de hachage, concurrence) sont comparées dans n'importe quel ordre ; un nombre de lignes différent devient `...`, qui correspond à tout. `record` liste chaque type. Examinez-les tous, car une différence d'ordre de hachage ou de concurrence est justement ce qui peut changer pour de vrai dans un portage.

## L'enregistrement se souvient de son original

`record` sauvegarde une empreinte des fichiers de l'unité (`recorded-from.json`). Quand l'original change par la suite, `diff` avertit et `gate` échoue tant que vous n'avez pas réenregistré, au lieu de passer face à ce que faisait l'original auparavant. Voir [Diff](/docs/concepts/diff#stale-recordings).

## Spécificités de TypeScript

`record` exécute les sources TS telles quelles : un hook de résolution Node suit les imports de style bundler (`./errors` vers `./errors.ts`), `--resolve-from` fournit les packages npm quand le dépôt n'en a aucun d'installé, et le TS qui doit être transformé (enums, parameter properties) est réessayé avec `--experimental-transform-types`. Le TS qui ne peut pas s'exécuter directement dans Node peut être enregistré depuis la sortie de build avec `--module dist/x.mjs`, ou avec `--node-arg=--import=tsx`.

## Code numérique

Pour le code où les flottants correspondent rarement au bit près, enregistrez avec `--float-tolerance`. Voir [Tolérance des flottants](/docs/concepts/float-tolerance).
