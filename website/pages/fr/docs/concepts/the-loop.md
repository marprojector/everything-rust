---
title: La boucle
description: profile, analyze, bridge, record, port, diff, bench, shadow et gate. Les étapes par lesquelles passe chaque unité.
---

# La boucle

Chaque portage, d'une seule fonction chaude à une application entière, passe par les mêmes étapes, une unité à la fois. Un portage n'est terminé que lorsque `gate` passe.

Comment everything-rust vérifie un portage, ce que décident ses commandes, et où sont ses limites. Retour au [README](/).

## Comment ça marche

```
profile ──> analyze ──> bridge ──> record ──> port.rs ──> diff ──> bench ──> shadow ──> gate
 where the    what to     the Rust   what the    you or      every     faster?    both side   done means
 time goes    port first  crate and  original    your agent  recorded  the        by side in  this passes
                          wrapper    does        write it    case      verdict    production
```

L'original reste la source de vérité : `record` l'exécute sur des cas limites (et vos propres entrées) et
sauvegarde ce qu'il renvoie ou lève ; `diff` rejoue chaque cas contre le Rust ; rien n'est livré tant
qu'ils ne correspondent pas tous et que `gate` ne passe pas. Les appelants gardent l'API d'origine tout du long.

Une application entière passe par la même boucle, unité par unité, à l'intérieur d'un cadre qui prouve le
programme entier ([Porter une application entière](/docs/whole-apps/overview)) :

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop above, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in         pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down      proved by
 decisions to make      outside         original answers                                                replay
```

## La boucle

| Étape | Commande | Ce qu'elle fait |
| --- | --- | --- |
| 0. Profiler | `everything-rust profile <repo> -- node bench.mjs` | Exécute une charge réelle sous le profileur du langage (node, cProfile, pprof de `go test`, JFR, un traceur Ruby) : fonctions chaudes par temps inclusif, leurs appelants, et les boucles à mettre en lot ; signale le code déjà compilé (numba, Cython, JAX), où Rust gagne peu |
| 1. Planifier | `everything-rust analyze <repo>` | Unités, ordre de portage, risques avec `file:line`, bridge par unité, correspondance dépendance-crate, meilleurs premiers portages ; `--whole-app` ajoute les points d'entrée et la forme, chaque dépendance (importée ou déclarée) avec sa crate, les morceaux (les unités qui s'importent mutuellement se déplacent ensemble) et des décisions numérotées avec une réponse recommandée |
| 2. Bridge | `everything-rust bridge <repo> <unit>` | Crate avec les bindings (`lib.rs`, plus le wrapper que charge le langage source) et des stubs à porter (`port.rs`) ; `--batch fn:param` ajoute une classe qui garde de nombreuses valeurs de `param` en Rust ; `--as-json T` fait passer sous forme de valeurs JSON un paramètre de type que l'application ne remplit qu'avec des données |
| 3. Enregistrer | `everything-rust record <crate>` | Exécute l'original sur des cas limites générés (non-ASCII, emoji, `-0`, `NaN`, des entiers là où des flottants sont attendus, ...) et sur chaque piège sémantique de chaque paramètre (longueur et casse des chaînes, division signée, limites de dépassement, affichage des flottants, ordre des maps et des tris), et sauvegarde `cases.jsonl` ; `--from-tests -- npx jest` (ou `-- pytest`) enregistre aussi chaque appel que fait la propre suite de tests de l'original ; `--float-tolerance 1e-9` laisse les flottants correspondre à cette fraction près pour le code numérique |
| 3. Compiler | `everything-rust build <crate>` | Compile le côté Rust : `.node`, package wasm, extension Python ou bibliothèque partagée |
| 3. Diff | `everything-rust diff <crate>` | Rejoue les cas enregistrés contre le Rust, appelé comme l'appellent les appelants de l'original ; liste les écarts |
| 3. Bench | `everything-rust bench <crate>` | Chronomètre les cas enregistrés sur l'original et sur le Rust, bridge compris |
| 4. Shadow | `everything-rust shadow <crate>` | Module de remplacement direct qui exécute les deux, renvoie le résultat de l'original, signale les écarts ; `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. Gate | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, tests, audit des `unsafe`, rien de non porté, un cas d'échec enregistré pour chaque chemin d'erreur de l'original, aucun `unwrap`/`expect`/`panic!` là où l'original renvoie une erreur (sauf si marqué `// INFALLIBLE: <why>`), build, API, comportement |
| Agents | `everything-rust setup` | Enregistre le serveur MCP (`everything-rust mcp`, chaque étape comme outil) et le skill de workflow ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) auprès de vos agents de code |

## Chemins chauds : garder les données en Rust

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

## Ce que décide `bridge`

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

## Ce que regarde `analyze`

- Les packages (`package.json`) et les fichiers sources ; ignore `node_modules`, les sorties de build, les fixtures,
  les playgrounds, les configurations d'outils. Les blocs `<script>` Vue/Svelte/Astro sont analysés pour leurs imports.
- **Unités** : un dossier de fichiers sources, ou un fichier à la racine des sources d'un package.
- **Vagues** : les imports d'une unité arrivent dans des vagues antérieures ; les cycles d'import sont portés ensemble.
- **Bridge** : napi-rs quand l'unité ou l'une de ses dépendances importées a besoin de Node ; wasm-bindgen pour le code
  portable ; rester en JS pour le code d'interface et d'éditeur.
- **Maturité** (degré de sûreté) et **priorité** (sûr et rentable : boucles et traitement de chaînes, tests, taille).

| Risque | Gravité | Exemple |
| --- | --- | --- |
| Positions dans les chaînes | haute | `slice`, `indexOf`, `matchAll`, le drapeau `d` : UTF-16 contre UTF-8 |
| Syntaxe des regex | haute | lookaround, références arrière, `y` : absents de la crate `regex` |
| Regex dynamiques | haute | `new RegExp(userPattern)` : syntaxe JS à l'exécution |
| Code dynamique | haute | `eval`, `new Function`, `import()` calculé |
| Typage dynamique | moyenne | `any`, `'key' in value` |
| Collections ordonnées | moyenne | `Map`, `Set`, `Object.keys` : ordre d'insertion |
| Async, générateurs | moyenne | frontières des promesses et des générateurs |
| Héritage de classes, réflexion | moyenne | `extends`, `Proxy`, `Reflect`, `Symbol` |
| API à base de callbacks | moyenne | paramètres et méthodes de type fonction : bridges bavards |
| Exceptions, nombres, JSON | basse | `throw`, opérations bit à bit, ordre de `JSON.stringify` |

## Ce qu'il ne détecte pas

Passer signifie que le Rust correspond à chaque cas enregistré, rien de plus. Ce qui fait échouer les réécritures
se trouve en général hors de l'enregistrement, alors sachez où sont les limites :

- **Les entrées que personne n'a enregistrées.** Les cas générés, les pièges, la suite de tests de l'original
  (`record --from-tests`) et vos propres `--inputs` sont le plafond. Ajoutez de vraies entrées (échantillons
  de production, rapports de bugs) ; l'entrée utilisateur qui casse la prod est rarement celle à laquelle un générateur pense.
- **Un original qui a évolué.** `record` sauvegarde une empreinte des fichiers de l'unité
  (`recorded-from.json`) ; quand l'original change par la suite, `diff` avertit et `gate` échoue tant que vous
  n'avez pas réenregistré, au lieu de passer face à ce que faisait l'original auparavant.
- **Les regex.** La crate `regex` n'a ni lookaround ni références arrière, et même un motif qui
  compile peut traiter `\w`, `\b`, `\d` et la correspondance insensible à la casse sur du texte non ASCII autrement
  que JS, Python ou Java. Passer à `fancy-regex` ou réécrire le motif peut passer chaque
  cas enregistré et pourtant différer sur un `ı` turc sans point ou un zero-width joiner. `record` ajoute
  des pièges regex (ceux-là, le signe Kelvin, le s long, les chiffres arabes-indiens, les séparateurs de ligne, le sigma final)
  à chaque fonction d'un fichier qui utilise des regex, mais testez aussi vos vrais motifs par fuzzing sur du texte réel.
- **Ce qui change d'une exécution à l'autre.** `record` exécute l'original deux fois et trie ce qui a différé :
  les valeurs changeantes (heures, identifiants) deviennent des jokers ; les mêmes lignes dans un autre ordre (tables de hachage,
  concurrence) sont comparées dans n'importe quel ordre, donc un portage doit quand même produire exactement ces lignes ; un
  nombre de lignes différent devient `...`, qui correspond à tout. `record` liste chaque type :
  examinez-les tous, car une différence d'ordre de hachage ou de concurrence est justement ce qui peut changer
  pour de vrai dans un portage.
- **Les flottants.** La conversion de nombre en texte diffère selon les langages (`1e21`, `-0`, nombre de chiffres).
  Les pièges incluent ces valeurs, et elles doivent correspondre exactement ; `--float-tolerance` ne pardonne
  que l'arrondi, jamais le formatage.
- **Tout ce qui n'est pas dans la sortie.** Le temps d'exécution, la mémoire, la concurrence sous charge, et les effets de bord autres
  que stdout, stderr, les codes de sortie, les fichiers écrits et les réponses HTTP (appels réseau, bases de données,
  signaux) ne sont pas comparés.
- **Les autres plateformes.** Un enregistrement prouve le comportement sur la machine qui l'a exécuté. Du C dont les résultats
  dépendent de la taille des types, de l'endianness ou du compilateur peut se comporter autrement sur la cible ; `analyze`
  garde en C le code C proche du matériel pour cette raison.
## Toutes les étapes

| Étape | Commande | Ce qu'elle fait |
| --- | --- | --- |
| 0. Profiler | `everything-rust profile <repo> -- node bench.mjs` | Exécute une charge réelle sous le profileur du langage (node, cProfile, pprof de `go test`, JFR, un traceur Ruby) : fonctions chaudes par temps inclusif, leurs appelants, et les boucles à mettre en lot ; signale le code déjà compilé (numba, Cython, JAX), où Rust gagne peu |
| 1. Planifier | `everything-rust analyze <repo>` | Unités, ordre de portage, risques avec `file:line`, bridge par unité, correspondance dépendance-crate, meilleurs premiers portages ; `--whole-app` ajoute les points d'entrée et la forme, chaque dépendance (importée ou déclarée) avec sa crate, les morceaux (les unités qui s'importent mutuellement se déplacent ensemble) et des décisions numérotées avec une réponse recommandée |
| 2. Bridge | `everything-rust bridge <repo> <unit>` | Crate avec les bindings (`lib.rs`, plus le wrapper que charge le langage source) et des stubs à porter (`port.rs`) ; `--batch fn:param` ajoute une classe qui garde de nombreuses valeurs de `param` en Rust ; `--as-json T` fait passer sous forme de valeurs JSON un paramètre de type que l'application ne remplit qu'avec des données |
| 3. Enregistrer | `everything-rust record <crate>` | Exécute l'original sur des cas limites générés (non-ASCII, emoji, `-0`, `NaN`, des entiers là où des flottants sont attendus, ...) et sur chaque piège sémantique de chaque paramètre (longueur et casse des chaînes, division signée, limites de dépassement, affichage des flottants, ordre des maps et des tris), et sauvegarde `cases.jsonl` ; `--from-tests -- npx jest` (ou `-- pytest`) enregistre aussi chaque appel que fait la propre suite de tests de l'original ; `--float-tolerance 1e-9` laisse les flottants correspondre à cette fraction près pour le code numérique |
| 3. Compiler | `everything-rust build <crate>` | Compile le côté Rust : `.node`, package wasm, extension Python ou bibliothèque partagée |
| 3. Diff | `everything-rust diff <crate>` | Rejoue les cas enregistrés contre le Rust, appelé comme l'appellent les appelants de l'original ; liste les écarts |
| 3. Bench | `everything-rust bench <crate>` | Chronomètre les cas enregistrés sur l'original et sur le Rust, bridge compris |
| 4. Shadow | `everything-rust shadow <crate>` | Module de remplacement direct qui exécute les deux, renvoie le résultat de l'original, signale les écarts ; `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. Gate | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, tests, audit des `unsafe`, rien de non porté, un cas d'échec enregistré pour chaque chemin d'erreur de l'original, aucun `unwrap`/`expect`/`panic!` là où l'original renvoie une erreur (sauf si marqué `// INFALLIBLE: <why>`), build, API, comportement |
| Agents | `everything-rust setup` | Enregistre le serveur MCP (`everything-rust mcp`, chaque étape comme outil) et le skill de workflow ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) auprès de vos agents de code |
## Pour aller plus loin

Chaque étape a sa propre page : [Profiler et analyser](/docs/concepts/analyze), [Bridge](/docs/concepts/bridge), [Record](/docs/concepts/record), [Port](/docs/concepts/port), [Diff](/docs/concepts/diff), [Bench](/docs/concepts/bench), [Shadow](/docs/concepts/shadow) et [Gate](/docs/concepts/gate).
