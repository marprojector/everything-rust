---
title: Tolérance des flottants
description: Le code numérique donne rarement des flottants identiques au bit près une fois porté. --float-tolerance laisse les flottants correspondre à une fraction près de la plus grande valeur.
---

# Tolérance des flottants

```sh
everything-rust record <crate> --float-tolerance 1e-9
everything-rust app record . --float-tolerance 1e-9 -- python3 cli.py
```

Le code numérique (physique, statistiques, graphisme) donne rarement des flottants identiques au bit près une fois porté : l'ordre des opérations, le fused multiply-add et la bibliothèque mathématique diffèrent d'un compilateur à l'autre. Si vous l'acceptez, enregistrez avec `--float-tolerance` et votre valeur. Les flottants correspondent alors à cette fraction près de la plus grande valeur (en absolu sous 1), et `diff` et `gate` l'utilisent. `0` la supprime.

Ce qui doit toujours correspondre exactement :

- les entiers, le texte et la structure ;
- la même valeur écrite différemment (`3` et `3.0`) ;
- le formatage. La conversion de nombre en texte diffère selon les langages (`1e21`, `-0`, nombre de chiffres) ; les pièges incluent ces valeurs, et `--float-tolerance` ne pardonne que l'arrondi, jamais le formatage.

N'élargissez jamais une tolérance pour faire disparaître un écart : une différence au-delà est un bug du portage.
