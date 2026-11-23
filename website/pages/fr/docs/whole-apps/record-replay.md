---
title: Enregistrer et rejouer
description: app record exécute toute l'application sur ses exemples documentés, vos entrées et des pièges ; app replay exécute n'importe quel build contre les cas.
---

# Enregistrer et rejouer

`record` et `diff` prouvent une unité ; `app record` et `app replay` prouvent l'application, pour les CLI et les serveurs dans n'importe lequel des six langages. L'application est une boîte noire.

```sh
everything-rust app record . -- python3 cli.py          # la commande qui exécute l'original
everything-rust app replay .                            # l'original doit passer à 100%
everything-rust app replay . --build "target/release/mytool"
```

## D'où viennent les cas

`app record` exécute l'original sur :

- les propres exemples documentés de l'application : les lignes `$ mytool ...` de ses fichiers Markdown et shell, avec les fichiers qu'elles nomment ;
- un fichier JSONL `--inputs`, avec un `{"name", "args", "stdin", "env", "files"}` par ligne ;
- des pièges sémantiques dérivés de ces cas : chaînes non ASCII et emoji, division et modulo négatifs, dépassement, flottants très grands et très petits, ordre des maps, fichiers manquants.

Chaque cas est exécuté deux fois ; ce qui diffère entre les deux exécutions devient un joker `[..]`. `--normalize timestamps|uuids|key-order` ou `--normalize name=regex` ajoute des normaliseurs, sauvegardés avec les cas. `app record` rapporte ce qui a différé par type : valeurs (désormais des jokers), ordre (lignes comparées dans n'importe quel ordre) et lignes (un `...` qui correspond à tout). Examinez-les tous.

## Les cas sont de simples fichiers

Les cas sont des fichiers [trycmd](https://docs.rs/trycmd) dans `rust/app-cases/` :

| Fichier | Contenu |
| --- | --- |
| `<case>.toml` | argv, env, code de sortie |
| `<case>.stdin` | entrée standard |
| `<case>.stdout`, `<case>.stderr` | ce qu'a affiché l'original |
| `<case>.in/` | fichiers en entrée |
| `<case>.out/` | fichiers écrits |

Lisez-les et modifiez-les à la main ; un test Rust peut les exécuter avec `trycmd` dès que le portage a un binaire.

## Replay

- `app replay` exécute n'importe quel build contre les cas, affiche `passed/total`, écrit `rust/app-cases/replay-report.md` avec un diff par cas en échec, et se termine avec un code non nul au moindre écart.
- Un build qui ajoute des lignes au fichier nommé par `EVERYTHING_RUST_FALLBACK_LOG` (une ligne par fallback vers l'original), comme le fait la [porte d'entrée](/docs/whole-apps/front-door), obtient aussi son taux de fallback dans le rapport.
- `app replay --case <name>` relance un seul cas par le nom que lui donne le rapport : la boucle rapide et reproductible pour corriger un écart avant de tout relancer.

## Replay en CI

Le rapport ne contient ni durées ni chemins temporaires : deux replays du même build écrivent le même fichier. Commitez-le, puis lancez en CI :

```sh
everything-rust app replay .
git diff --exit-code rust/app-cases/replay-report.md
```

Une application à moitié portée fait alors échouer la CI au moindre changement dans ce qui passe, en mieux ou en pire, jusqu'à ce que le rapport soit mis à jour en conséquence.

## Sortie numérique

`app record --float-tolerance 1e-9` laisse les nombres correspondre à cette fraction près de la plus grande valeur. Voir [Tolérance des flottants](/docs/concepts/float-tolerance).
