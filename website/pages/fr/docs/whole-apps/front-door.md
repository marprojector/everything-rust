---
title: Porte d'entrée et câblage
description: app front-door place un binaire Rust devant l'original ; app wire confie des commandes au portage d'une unité une fois qu'il passe le gate.
---

# Porte d'entrée et câblage

Déplacez d'abord le point d'entrée en Rust, puis confiez-lui les commandes une unité à la fois :

```sh
everything-rust app front-door .                            # rust/app : chaque commande exécute l'original
everything-rust app wire . textkit --command upper          # après que textkit-rs passe le gate
# écrivez la glue dans rust/app/src/units/textkit.rs, puis :
everything-rust app wire . textkit                          # de nouveau 100%, taux de fallback plus bas
```

## La porte d'entrée

`app front-door` crée `rust/app`, un binaire Rust qui porte le nom de l'application : un analyseur clap pour les commandes et les options présentes dans les cas enregistrés, où chaque commande exécute l'original avec les mêmes argv, stdin et environnement et transmet sa sortie et son code de sortie. Chaque fallback est compté, donc rejouer la porte d'entrée toute neuve donne 100% de réussite à 100% de fallback (elle compile et rejoue pour le montrer). Il n'écrase jamais `rust/app` : à partir de là, le code est à vous.

## Câbler une unité

`app wire <repo> <unit> --command <name>` :

1. exécute `gate` sur la crate de l'unité (`--skip-gate` s'il vient de passer) ;
2. l'ajoute à `rust/app` comme bibliothèque ;
3. écrit `src/units/<unit>.rs` une seule fois, à remplir par vous ou votre agent : analyser les arguments, appeler `<crate>::port`, afficher ce qu'affiche l'original ;
4. lui route les commandes, puis compile et rejoue.

Relancez-le après avoir écrit la glue : 100% de réussite et un taux de fallback plus bas. `EVERYTHING_RUST_UNIT_<NAME>=original` renvoie une unité câblée vers l'original jusqu'au lift.

Câblez le nom propre de l'application comme commande (`--command mytool`) pour les argv qui ne nomment aucune commande : aucun argument, commandes inconnues, erreurs d'utilisation.

## Rien ne rappelle l'original

La porte d'entrée exécute l'original comme un processus séparé (ou lui transmet le HTTP), les unités passées par un bridge sont appelées par l'original et jamais l'inverse, et les callbacks restent dans le langage source.

Les serveurs utilisent des routes au lieu de commandes : voir [Serveurs](/docs/whole-apps/servers). Les bibliothèques utilisent leur API publique : voir [Bibliothèques](/docs/whole-apps/libraries).
