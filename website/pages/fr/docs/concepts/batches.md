---
title: Lots et chemins chauds
description: Un portage appelé une fois par élément perd face à un JIT. Les lots déplacent toute la boucle en Rust, sur tous les cœurs, et gagnent même là où les portages appel par appel perdent.
---

# Lots et chemins chauds

```sh
everything-rust bridge <repo> <unit> --batch fn:param    # ou : everything-rust refresh <crate> --batch fn:param
```

`--batch` génère une classe qui reçoit les valeurs de `param` une seule fois et les garde en Rust : `new FnBatch(items)` (TS, Python, Java), `Mod::FnBatch.new(items)` (Ruby), `NewFnBatch(items)` (Go). Par requête, `run(<other args>)` renvoie tous les résultats dans un seul tableau, et `rank(<other args>, min)` renvoie les index des éléments, les meilleurs d'abord. Il s'exécute sur tous les cœurs, et les erreurs sont celles de l'original. Tous les langages sauf C ont les lots.

Un portage appelé une fois par élément paie le bridge une fois par élément, et perd face à un JIT. Ce qui gagne, c'est
de déplacer la boucle : `profile` la trouve, et `--batch` génère une classe qui reçoit les éléments une seule fois
et exécute la fonction sur tous à chaque appel, sur tous les cœurs, avec des résultats en tableaux typés.

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore : un quart de l'exécution, appelée par clé depuis `matched` : la mettre en lot
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

Une struct `FuzzyScoreIndex` dans `port.rs` remplace la boucle par défaut par une boucle qui prépare les
éléments une seule fois, et peut classer d'elle-même (`rank`) : elle garde les clés en minuscules, chacune avec un masque de caractères,
une table de la première position de chaque caractère courant dans chaque clé (une requête d'une lettre correspond à une
colonne), et les correspondances de la dernière requête (taper `se` après `s` ne recalcule le score que de ce que `s` a trouvé). Sur
la recherche de la barre latérale de x18n parmi 50,000 clés, tapée lettre par lettre, avec des résultats identiques :

| | Par requête | face au TS |
| --- | ---: | ---: |
| TS : `map(fuzzyScore)`, filtre, tri | 10 ms | 1x |
| TS avec le même filtrage que l'index | 4.1-5.4 ms | 2x |
| Le même portage, appelé par clé | - | 0.2-0.4x |
| `FuzzyScoreBatch`, index par défaut (4 cœurs) | 5.1 ms | 1.7x |
| `FuzzyScoreBatch` avec `FuzzyScoreIndex` (napi, 4 cœurs) | 0.45-0.56 ms | **18-22x** |
| Le même en wasm (un thread) | 0.95 ms | **11.5x** |

Face au TS qui fait le même filtrage, Rust reste 7-12x plus rapide (wasm 5x). Transformez les index
renvoyés en clés avec une simple boucle : `Array.from(order, i => keys[i])` coûtait 20x plus, 0.4 ms.

Tous les langages ont les lots, et ils battent l'original même là où les portages appel par appel perdent. Le
même travail sur 4 cœurs, résultats et erreurs identiques (benchmarks dans chaque dossier de démo) :

| Langage | Charge | Original | Portage par appel face à l'original | Lot face à l'original |
| --- | --- | ---: | ---: | ---: |
| TS | recherche x18n, 50,000 clés | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`, 20,000 chaînes | 98 ms | - | **17x** |
| Ruby | `slugify`, 20,000 chaînes | 110 ms | - | **12.7x** |
| Ruby | `shellescape` (stdlib), 50,000 valeurs mixtes | 88 ms | - | **11.8x** |
| Go | `Slugify`, 50,000 chaînes | 49 ms | 0.93x | **4x** |
| Java | `slugify`, 50,000 chaînes | 43 ms | 0.68x | **3x** |

Les originaux compilés gagnent moins : le travail par élément est déjà rapide, donc le lot gagne à hauteur du nombre de
cœurs. Les crates de lot allouent avec mimalloc : avec l'allocateur système, le lot Go était plus lent qu'un
seul thread. Démo : [`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md).
## Écrire un index rapide

- Faites construire le lot une seule fois par l'appelant, pas à chaque requête. Reconvertissez les index renvoyés en éléments avec une simple boucle.
- Pour préparer les éléments une seule fois (minuscules, masques, tables de positions), définissez `pub struct FnIndex` dans `port.rs` avec `new(items)` et `run(<other args>)`. Il doit donner le même résultat que la fonction, élément par élément.
- Quand le classement peut éviter du travail, donnez à l'index son propre `rank(<other args>, min)`. Il doit correspondre au classement des résultats de la fonction : score décroissant, égalités dans l'ordre des éléments.
- Un lot multiplie la vitesse par élément du portage par le nombre de cœurs. Face à des originaux compilés (Go, Java), le portage lui-même doit être rapide : traitez l'ASCII sans tables Unicode, construisez chaque résultat dans une seule `String` pré-dimensionnée, évitez les copies intermédiaires.
- Fermez le lot une fois terminé en Go et en Java (les finaliseurs ne sont qu'un filet de sécurité).
