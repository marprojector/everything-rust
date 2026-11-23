---
title: Bench
description: bench chronomètre les cas enregistrés sur l'original et sur le Rust, bridge compris, et rend un verdict. Rust n'est pas toujours plus rapide.
---

# Bench

```sh
everything-rust bench <crate>
```

`bench` chronomètre les cas enregistrés sur l'original et sur le module Rust compilé, bridge compris, et renvoie les lignes de mesure et un verdict avec un conseil : livrer, garder les fonctions les plus lentes dans le langage source, ou porter une unité plus grande. Un gain inférieur à 1 signifie que le portage est plus lent tel que ses appelants l'appellent. `--min-ms` fixe le temps de mesure minimal par fonction et par côté (200 par défaut).

Suivez le verdict. Ne livrez pas un portage qui ralentit les appelants sans le dire.

## Pourquoi un portage peut être plus lent

Un portage appelé une fois par élément paie le bridge une fois par élément. Ce qui tranche :

- **Originaux interprétés (Python, Ruby)** : l'appel vers Rust coûte environ 100-300 ns et le code Rust est bien plus rapide que l'interpréteur, donc même les petites fonctions y gagnent.
- **Originaux compilés (Go, Java après le JIT)** : la fonction feuille d'origine prend 4-90 ns ; un seul appel cgo coûte à lui seul environ 80 ns, un appel JNI avec des chaînes davantage. Ces langages ne gagnent que lorsqu'un appel fait beaucoup de travail : portez une fonction avec ses appelants.
- **C** : il n'y a pas de traversée ; le code Rust est comparé directement au code C.

Mesuré par appel sur du code réel ([six langages](/docs/real-world/six-languages)) :

| Langage | Rust face à l'original (`bench`, moyenne géométrique) | Verdict |
| --- | --- | --- |
| Ruby | **4.4x plus rapide** (3.6-7x selon la fonction) | livrer |
| Python | **1.35x plus rapide** (1.5-1.9x sur la plupart des appels) | livrer |
| C | 0.87x : ctype 1.1-1.4x plus rapide, `strtod`, `strspn`, `strcmp` optimisés de la glibc 0.5-0.7x | optimiser ou garder les plus lentes |
| Java | 0.18x | porter de plus grandes unités |
| Go | 0.04x | porter de plus grandes unités |

Les 29 fonctions du cœur de x18n qui peuvent être déplacées sans changer l'API ont été portées : chaque gate passe et 7,925 cas enregistrés correspondent exactement, et `bench` a montré que les portages sont plus lents que le TS lorsqu'ils sont appelés depuis JS, car le bridge coûte plus que le travail ([rapport](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-core/README.md)). vercel/ms tournait à 0.56x par appel sur ses cas enregistrés ([étude de cas](/docs/real-world/vercel-ms)).

## Ce qui gagne : déplacer la boucle

`profile` trouve la boucle qui entoure une fonction chaude, et `--batch` déplace toute la boucle en Rust, sur tous les cœurs. Voir [Lots et chemins chauds](/docs/concepts/batches).

::: tip Mesurez sur une machine au repos
Lancez `bench` sur une machine au repos, lancez-le deux fois, et indiquez le matériel et le nombre de cœurs à côté de chaque chiffre publié.
:::
