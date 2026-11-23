---
title: Profiler et analyser
description: Trouvez où passe le temps avec profile, puis planifiez les unités, l'ordre de portage, les risques et les crates avec analyze.
---

# Profiler et analyser

## Profile : trouver ce qui vaut la peine d'être déplacé

Quand l'objectif est la vitesse, mesurez avant de choisir quoi que ce soit :

```sh
everything-rust profile <repo> -- node bench.mjs      # ou : python3 bench.py, ruby bench.rb,
                                                    #      java -cp out Main, go test -run X -bench . ./pkg
```

`profile` exécute une charge réelle sous le profileur du langage (node, cProfile, pprof de `go test`, JFR, un traceur Ruby) et liste les fonctions chaudes par temps inclusif, leurs appelants, et les boucles à mettre en lot. Go ne se profile qu'à travers `go test` ; C n'est pas pris en charge.

- Une petite fonction appelée une fois par élément ne vaut **pas** la peine d'être portée seule : un appel vers Rust coûte plus que son travail, et les JIT (V8, la JVM, Go) l'exécutent en quelques nanosecondes. Portez-la avec ses appelants, ou [mettez-la en lot](/docs/concepts/batches). La ligne de conseil dit lequel des deux, avec la commande exacte.
- Le code déjà compilé (numba `@njit`, Cython, `@jax.jit`) s'exécute en code machine : un portage Rust y gagne peu en vitesse. `profile` le liste sous « Already compiled ».

## Analyze : planifier le portage

```sh
everything-rust analyze <repo> -o plan.md        # pour les humains
everything-rust analyze <repo> --json            # pour les agents : schéma everything-rust/analyze@1
everything-rust analyze <repo> --whole-app       # plus points d'entrée, forme, dépendances, décisions
```

`analyze` liste les unités, l'ordre de portage, les risques avec `file:line`, le bridge de chaque unité, la correspondance de chaque dépendance avec une crate, et `start_here` : les meilleurs premiers portages. `--whole-app` ajoute la forme de l'application (CLI, serveur ou bibliothèque) et ses points d'entrée, chaque dépendance (importée ou déclarée) avec sa crate, les morceaux (les unités qui s'importent mutuellement se déplacent ensemble) et des décisions numérotées avec une réponse recommandée. Voir [Porter une application entière](/docs/whole-apps/overview).

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
## Crates pour les dépendances

`analyze` nomme la crate Rust de chaque bibliothèque importée par le code, avec une adéquation `good`, `partial` ou `none`. Une bibliothèque absente de sa table est signalée comme `unknown` ; la table ne devine jamais une crate. Pour en ajouter une, voir [Contribuer](/docs/project/contributing#add-a-crate-mapping).
