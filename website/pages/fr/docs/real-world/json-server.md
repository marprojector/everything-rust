---
title: json-server
description: La première application entière portée de bout en bout. Un binaire Rust qui répond ce que répond le serveur Node, 514/514 cas HTTP enregistrés.
---

# json-server

[json-server](https://github.com/typicode/json-server) (75k étoiles, MIT, commit `89a34a4`,
1.0.0-beta.15) sert une API REST à partir d'un fichier JSON : Node, tinyhttp et lowdb. C'est la première
application entière portée de bout en bout avec le flux application entière (`app record`, `front-door`, `wire`, `lift`) :
le résultat est un seul binaire Rust qui répond ce que répond le serveur Node. Tout se trouve dans
[`demo/real-world/json-server-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/json-server-rs).

| Étape | Résultat |
| --- | --- |
| `analyze --whole-app` | un serveur, 9 unités (7 prêtes, 2 à surveiller), 13 dépendances à remplacer (tinyhttp, lowdb, sirv, eta, milliparsec, dot-prop, sort-on, inflection...) |
| `app record --http` | 514 cas : 268 requêtes fournies avec `--inputs` (chaque route et méthode, les opérateurs where, le tri, la pagination, les embeds, le preflight CORS, les fichiers statiques et les ranges, 29 corps JSON malformés, `/__proto__` et d'autres noms de prototype, des échappements invalides) et 245 pièges ; chacun enregistré deux fois, les identifiants aléatoires laissés en `[..]` |
| `front-door` | 513/514 à 99.8% de fallback ; l'échec est un `ü` brut dans l'URL, auquel Node répond 400 et que le proxy réencode |
| unités | `where-operators` 207, `parse-where` 289, `matches-where` 1,346, `paginate` 853 cas (appels issus des propres tests de json-server compris) ; `service` 461 et `app` 243 via deux petits adaptateurs qui exécutent des appels Service et de vraies requêtes HTTP ; `gate` passe sur les six |
| `wire` | les lectures d'abord : 514/514 à 22.6% de fallback, Node répondant aux écritures et Rust rechargeant le fichier écrit par Node ; puis toutes les routes : 514/514 à 0% |
| `lift` | un binaire 100 % Rust (5.3 MB), 514/514 ; la ligne de commande de `bin.ts` portée à la main (aide, version, vérifications de fichiers, bannière de démarrage) |

Après le lift, les unités n'ont plus de bindings, donc le `tests/recorded.rs` de chaque crate rejoue ses
cas enregistrés sur le portage : les 3,399 tournent avec `cargo test`.

Ce que le portage devait respecter, tout cela détecté par les cas enregistrés :

- Le routeur de tinyhttp ne fait pas qu'un seul aiguillage : après chaque `next()`, il compare de nouveau tous les middlewares et
  toutes les routes, sur l'URL que le dernier middleware a réécrite, et exécute chaque route pour `HEAD`. Ainsi
  `HEAD /posts` exécute le handler GET, puis celui de POST, et répond le 400 de POST ; `//posts` saute les en-têtes CORS
  et l'analyseur de corps. Le portage exécute cette boucle telle quelle.
- milliparsec analyse chaque corps de POST, PUT, PATCH et DELETE comme du JSON, quel que soit son type, et un corps invalide
  donne un 500 avec le message de V8 : `Expected double-quoted property name in JSON at position 7
  (line 1 column 8)`, `Unexpected token 'x', ..."bbbbbbb": x}" is not valid JSON`. Le portage
  reproduit les erreurs de l'analyseur de V8, positions en unités UTF-16 comprises ; vérifié face à Node sur
  3,806 entrées malformées.
- Le tri utilise `localeCompare` (ICU), réalisé avec le collator d'ICU4X et vérifié sur 3,000 paires ;
  `Number()`, `JSON.stringify` (`1e+21`, clés entières en premier), les deux analyseurs de query string de Node
  (qui décodent `é%E9` autrement que WHATWG), dot-prop avec ses messages d'erreur, et inflection
  pour les embeds (`person` et `people`) sont chacun vérifiés face à Node dans `json-server-js`.
- `db.data[name]` lit `Object.prototype` : `/__proto__` répond `{}`, `/constructor` un 200 vide,
  et `PATCH /__proto__` remplace le prototype de la base de données, ce que voient les recherches suivantes.
- L'analyseur HTTP de Node répond 400 et ferme la connexion sur les cibles non ASCII brutes et les méthodes inconnues.

Trouvé dans json-server : après chacune de ses propres écritures, son observateur de fichiers recharge le fichier. Quand la
requête suivante écrit avant la fin de ce rechargement, le rechargement peut effacer cette écriture : le premier
enregistrement, envoyé aussi vite que le serveur répondait, contenait un POST répondu 201 que les lectures suivantes ne
montraient pas. `app record --pace 50` attend entre les requêtes, et les cas sont enregistrés ainsi. Le portage
ne recharge que lorsque quelqu'un d'autre a modifié le fichier, donc il ne perd pas cette écriture : une
différence voulue.

Benchmark : Apple M4 (10 cœurs, 16 GB), macOS 27.3, Node 24.15.0 exécutant `lib/bin.js` tel que
`tsc` le compile pour npm, le binaire Rust compilé avec `--release`. Les deux servent une copie de la même
base de données en loopback ; `ab -k` envoie les mêmes requêtes, 20,000 par ligne (40 pour l'embed lent sur
la grosse base) après une phase de chauffe. Deux exécutions, chacune lancée avec une charge moyenne sur 1 minute inférieure à 3 ;
les cellules donnent exécution 1 / exécution 2 en requêtes par seconde. [`bench/bench.py`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/json-server-rs/bench/bench.py)
est le script.

La base de fixtures (6 posts, 3 commentaires) :

| Requête | Node, 1 connexion | Rust, 1 connexion | Node, 50 connexions | Rust, 50 connexions |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts` | 11.1k / 10.8k | 27.3k / 23.9k | 19.7k / 20.4k | 115.0k / 110.7k |
| `GET /posts/1` | 11.0k / 11.3k | 27.7k / 25.4k | 21.9k / 22.6k | 119.9k / 99.0k |
| filtre, tri, page | 10.5k / 10.0k | 25.2k / 21.5k | 18.3k / 16.4k | 107.5k / 96.3k |
| `?_embed=comments` | 8.7k / 6.6k | 22.5k / 21.8k | 13.8k / 11.0k | 98.3k / 100.6k |
| `GET /` (page d'index) | 7.3k / 6.3k | 27.6k / 21.7k | 10.2k / 9.5k | 135.1k / 119.3k |
| `GET /test.html` (statique) | 12.0k / 11.0k | 25.8k / 25.9k | 30.3k / 26.1k | 103.9k / 104.4k |
| `POST /comments`, 10 connexions | | | 2,973 / 2,467 | 5,821 / 5,976 |

Une base générée (10,000 posts, 20,000 commentaires, 3.6 MB) :

| Requête | Node, 1 connexion | Rust, 1 connexion | Node, 50 (10) connexions | Rust, 50 (10) connexions |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts/5000` | 8.4k / 9.2k | 14.6k / 14.8k | 15.5k / 16.1k | 69.5k / 67.4k |
| `?_page=3&_per_page=10` | 4.6k / 4.2k | 8.8k / 8.4k | 6.5k / 6.8k | 47.4k / 46.2k |
| `title:contains`, trié | 431 / 396 | 600 / 560 | 465 / 453 | 2,516 / 2,329 |
| `?_embed=comments`, une page de 5 (10 connexions) | 2.4 / 2.4 | 81 / 103 | 2.4 / 2.2 | 253 / 216 |
| `GET /posts`, la liste complète (10 connexions) | 328 / 319 | 399 / 407 | 381 / 387 | 1,514 / 1,657 |
| `POST /comments` (10 connexions) | | | 236 / 241 | 292 / 264 |

| | Node | Rust |
| --- | ---: | ---: |
| Du démarrage à la première réponse, base de fixtures | 85 / 79 ms | 9.9 / 7.9 ms |
| Du démarrage à la première réponse, 10,000 posts | 95 / 91 ms | 48 / 47 ms |
| Mémoire (RSS) au repos, base de fixtures | 74 MB | 3.5 MB |
| Mémoire (RSS) au repos, 10,000 posts | 94 MB | 41 MB |
| Mémoire (RSS) maximale pendant les exécutions, base de fixtures | 424 / 390 MB | 18 MB |
| Mémoire (RSS) maximale pendant les exécutions, 10,000 posts | 462 / 485 MB | 289 / 291 MB |

Là où l'écart est faible : une connexion qui lit toute la liste de 10,000 posts (1.2x : les deux passent le
temps à écrire 1.5 MB de JSON), les écritures sur la grosse base (1.1 à 1.2x : les deux réécrivent le fichier entier de
3.6 MB à chaque écriture, Rust regroupant les écritures en attente comme le fait lowdb), et la recherche triée par titre
sur une connexion (1.4x). Les gros gains viennent de la concurrence, qu'un processus Node
n'a pas, et de l'embed, où json-server filtre chaque commentaire pour chaque post (le portage
indexe les commentaires une fois par requête, même résultat).

Le premier binaire issu du lift était plus lent que Node sur la grosse base (33 contre 298 requêtes par
seconde pour la liste complète, 39 contre 217 pour les écritures). Le profilage a trouvé, dans l'ordre : pas de
`TCP_NODELAY` (chaque grosse réponse attendait environ 40 ms un ACK retardé), la base entière
clonée pour ajouter `$schema` avant chaque écriture, chaque élément listé cloné avant l'affichage, un seul verrou
pour toutes les requêtes, et `item.id` haché avec SipHash pour chacun des 10,000 éléments. Tout est corrigé ; les 3,399
cas unitaires et les 514 cas HTTP passent après chaque changement.

Trouvé dans l'outil, et corrigé :

- Un serveur qui écrit son état ne pouvait pas être enregistré deux fois à partir du même point de départ :
  `--fresh <path>` copie désormais la base (et `public/`) dans un nouveau répertoire de travail à chaque
  démarrage, à partir d'un instantané conservé avec les cas ; `--arg` donne à chaque build les arguments de l'original ;
  `--pace` est décrit plus haut.
- `bridge` gardait `paginate<T>(items: T[])` en TS (générique sur n'importe quoi, fonctions comprises) ;
  `--as-json T` le fait passer sous forme de valeurs JSON.
- Les routes avec paramètres de la porte d'entrée répondaient le 400 propre à axum pour `/posts/%E0`, avant que
  l'unité ne voie la requête ; les paramètres sont désormais lus depuis l'URI.
- `lift` ne préparait que l'unité câblée, pas les crates dont elle dépend par chemin, et écrivait un `lib.rs`
  que `cargo fmt --check` rejetait ; les deux sont corrigés.
- Les serveurs qu'il génère ne définissaient pas `TCP_NODELAY` : une réponse écrite en deux parties attendait
  l'ACK retardé du client, environ 40 ms. Ils le définissent désormais.

Ce qu'il ne couvre pas : les bases JSON5 (le binaire le dit et s'arrête), la limite de corps de 100 KiB
(celle de Node dépend de la façon dont le corps arrive par lectures de 64 KiB ; le portage suppose des lectures complètes), les chaînes
avec des surrogates isolés (elles deviennent U+FFFD), et une valeur `1e400` dans la base (Node garde `Infinity`
en mémoire jusqu'au rechargement suivant ; le portage stocke `null`, ce que reçoit le fichier).
