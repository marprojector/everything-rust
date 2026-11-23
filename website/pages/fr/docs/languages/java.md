---
title: Java
description: Portez un package de méthodes public static en Rust via du JNI typé et un protocole binaire généré, dans des classes wrapper qui gardent les méthodes d'origine.
---

# Java

| | |
| --- | --- |
| Unité | un package de méthodes `public static` |
| Bridge | JNI typé et un protocole binaire généré, dans des classes wrapper avec les méthodes d'origine |
| Enregistré et rejoué par | un pilote par réflexion généré |
| Prérequis | JDK 17+ (`javac` et `java`) |
| État | Vérifié sur JDK 21 derrière une façade : 4,492/4,492 (226 lèvent une exception), le gate passe |

Les lots s'écrivent `new FnBatch(items)` ; fermez-les une fois terminé. `profile` utilise JFR.

## Ce qu'un portage doit respecter

- `Integer.parseInt` accepte n'importe quel chiffre décimal Unicode (`١٢`, `２`) via `Character.digit`.
- `Double.toString` (JDK 19+) affiche la plus courte représentation décimale, mais jamais moins de deux chiffres : `4.9E-324`, là où la forme la plus courte de Rust est `5e-324`.
- `\s` est ASCII uniquement ; le `split_whitespace` de Rust coupe aussi sur U+00A0.
- Les flottants doivent survivre à l'aller-retour : l'analyseur de flottants par défaut de `serde_json` n'arrondit pas toujours correctement, donc everything-rust et chaque crate qu'il génère utilisent sa feature `float_roundtrip`.

## Vitesse

Après le JIT, les fonctions feuilles prennent quelques nanosecondes, et un appel JNI avec des chaînes coûte plus que cela. Sur la façade JDK, `bench` a mesuré le portage à 0.18x par appel : portez de plus grandes unités. En lot sur 4 cœurs, `slugify` sur 50,000 chaînes (43 ms en Java) a tourné **3x plus vite** (le portage par appel : 0.68x).

Détails : [Six langages](/docs/real-world/six-languages).
