---
title: Diff et différences connues
description: diff rejoue chaque cas enregistré contre le Rust. known-diffs.jsonl consigne les différences voulues, et les enregistrements périmés font échouer le gate.
---

# Diff et différences connues

```sh
everything-rust build <crate>
everything-rust diff <crate>                     # doit être N/N
everything-rust diff <crate> --json
```

`diff` rejoue `cases.jsonl` contre le module Rust compilé, appelé comme l'appellent les appelants de l'original, et liste les écarts. Un écart est une différence de comportement : corrigez le portage, jamais l'enregistrement.

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

## Différences connues

Parfois le portage doit volontairement différer : l'utilisateur veut qu'il corrige un bug de l'original. Demandez d'abord, puis ajoutez une ligne par cas dans `<crate>/known-diffs.jsonl` :

**known-diffs.jsonl**
```json
{"fn": "...", "args": [...], "actual": ..., "reason": "..."}
```

Copiez `fn`, `args` et `actual` depuis `diff --json`. Ce cas ne passe alors qu'avec exactement ce résultat, et `gate` échoue dès qu'il ne diffère plus : supprimez alors la ligne.

## Enregistrements périmés

`record` sauvegarde une empreinte des fichiers de l'unité d'origine dans `recorded-from.json`. Quand l'original change par la suite, `diff` avertit et `gate` échoue tant que vous n'avez pas réenregistré, au lieu de passer face à ce que faisait l'original auparavant. Si `gate` dit que l'enregistrement est périmé, réenregistrez avant d'aller plus loin dans le portage.

## Quand le portage stagne

`diff` suit la progression d'un build à l'autre. Quand il indique que le portage stagne (son `trend.stalled` : trois modifications successives de `port.rs` n'ont fait gagner aucun cas), arrêtez de modifier. Les écarts restants demandent peut-être une règle que seul l'utilisateur connaît, ou ces fonctions peuvent rester dans l'original pour le moment.

## Tolérance des flottants

Si la crate a été enregistrée avec `--float-tolerance`, `diff` l'utilise ; `--float-tolerance` sur `diff` la remplace le temps d'une exécution. Voir [Tolérance des flottants](/docs/concepts/float-tolerance).
