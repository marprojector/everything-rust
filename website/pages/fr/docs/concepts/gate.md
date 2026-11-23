---
title: Gate
description: gate est la définition de « terminé » pour un portage. Chaque vérification qu'il exécute, et comment corriger celle qui échoue.
---

# Gate

```sh
everything-rust gate <crate>
```

Terminé signifie que `gate` passe. Il vérifie :

- `cargo fmt`, clippy avec `-D warnings`, et les tests cargo ;
- `unsafe` uniquement avec des commentaires `// SAFETY:`, et aucun `unimplemented!` ;
- un cas d'échec enregistré pour chaque endroit où l'original échoue volontairement (`throw`, `raise`, un résultat `error`). Ajoutez-en un avec `record --inputs` quand gate le nomme ;
- aucun `unwrap`, `expect` ou `panic!` dans une fonction dont l'original renvoie une erreur. Renvoyez `Err` ; pour un état que l'original considère aussi comme impossible, un commentaire `// INFALLIBLE: <why>` ;
- un portage d'au plus 3x le nombre de lignes de l'original (plus 50 lignes pour les imports et les signatures) ;
- le module compile, et chaque export est présent ;
- chaque cas enregistré correspond, [différences connues](/docs/concepts/diff#known-diffs) comprises, et l'enregistrement n'est pas [périmé](/docs/concepts/diff#stale-recordings) ;
- pour TS/JS, une ligne de [couverture](/docs/concepts/coverage) : la part de l'original exécutée par les cas.

Chaque vérification qui échoue porte un `fix` : lancez-le en premier.

## Ensuite, faire un rapport et attendre

Quand le gate passe, le skill de workflow demande à l'agent de rapporter ce qui a changé (fichiers, fonctions, bridge, lots), la sortie du gate, les comptes du diff et les chiffres du bench, les différences de comportement ou les risques restants, et ce qu'il reste dans le plan, puis d'attendre le feu vert avant l'unité suivante.
