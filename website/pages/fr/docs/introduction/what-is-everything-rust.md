---
title: Qu'est-ce que everything-rust
description: everything-rust enregistre ce que fait votre application aujourd'hui et vérifie chaque morceau de la réécriture Rust face à cet enregistrement, jusqu'à ce que l'application soit 100 % Rust.
---

# Qu'est-ce que everything-rust

<p class="lead">Réécrivez toute votre application en Rust, un morceau à la fois. everything-rust enregistre ce que fait aujourd'hui votre application Node.js (TypeScript/JavaScript), Python, C, Go, Ruby ou Java, puis vérifie chaque morceau de la réécriture Rust face à cet enregistrement avant de passer au suivant, jusqu'à ce que l'application soit 100 % Rust.</p>

Il fonctionne sur les CLI, les serveurs HTTP et les bibliothèques. Votre agent de code (Claude Code, Codex, Cursor, Gemini CLI) ou vous-même écrivez le Rust. PHP et C# sont en [préversion](/docs/languages/php).

## Ce qu'il fait, et ce qu'il ne fait pas

everything-rust ne traduit pas le code. Il fournit ce qu'un agent ne peut pas deviner de façon fiable :

- **où passe le temps** : `profile` exécute une charge réelle et classe les fonctions chaudes ;
- **quoi porter en premier, et dans quel ordre** : `analyze` planifie les unités, les vagues, les risques et les crates ;
- **un bridge** pour que le reste du code appelle le Rust avec son API inchangée (`bridge`) ;
- **le comportement exact que le portage doit reproduire** : `record` exécute l'original sur des cas limites et sauvegarde ce qu'il renvoie ou lève ;
- **une façon sûre de le livrer** : `shadow` exécute les deux côte à côte ;
- **un gate qui dit quand un portage est terminé** : `gate`.

L'original reste la source de vérité. `diff` rejoue chaque cas enregistré contre le Rust, et rien n'est livré tant qu'ils ne correspondent pas tous et que `gate` ne passe pas.

## Quatre étapes

| Étape | Ce qui se passe |
| --- | --- |
| **Enregistrer** | `record` exécute l'original sur des cas limites générés, des pièges sémantiques et vos propres entrées, et sauvegarde ce qu'il renvoie ou lève. |
| **Porter** | `bridge` génère la crate Rust et un wrapper, pour que les appelants gardent l'API d'origine. Vous ou votre agent écrivez le Rust. |
| **Vérifier** | `diff` rejoue chaque cas enregistré contre le Rust, et rien n'est livré tant qu'ils ne correspondent pas tous et que `gate` ne passe pas. |
| **Lift** | Quand le replay passe sans fallback et que chaque unité passe le gate, `app lift` en fait une application 100 % Rust. L'original n'est jamais modifié. |

## Rust est-il toujours plus rapide ?

Non, et everything-rust le dit. Une petite fonction appelée une fois par élément est souvent plus lente en Rust, car chaque appel traverse un bridge ; V8, la JVM et Go exécutent ce genre de fonction en quelques nanosecondes. `profile` trouve la boucle qui l'entoure, `--batch` déplace toute la boucle, et `bench` rend un verdict avant la livraison. Voir [Lots et chemins chauds](/docs/concepts/batches) pour les chiffres mesurés, y compris les lents.

## Il ne détecte que ce que couvre l'enregistrement

Passer signifie que le Rust correspond à chaque cas enregistré, rien de plus. Lisez [Ce qu'il ne détecte pas](/docs/concepts/limits) avant de faire confiance à un gate vert.

::: warning Lancez-le sur une copie
`record` et `app record` exécutent votre code d'origine en appelant chaque fonction (ou toute l'application) sur de nombreuses entrées générées, et `app record` exécute chaque cas deux fois. `diff`, `gate` et `app replay` rejouent les mêmes cas, sur le portage ou sur l'original. Du code qui écrit des fichiers, parle à une base de données ou appelle le réseau le fait une fois par cas. Lancez-les sur une copie du projet, avec une base de test et aucun identifiant de production dans l'environnement.
:::

## Étapes suivantes

- [Démarrage rapide](/docs/introduction/quick-start) : installez-le et demandez un premier portage à votre agent.
- [La boucle](/docs/concepts/the-loop) : toutes les étapes pour une unité.
- [Porter une application entière](/docs/whole-apps/overview) : du premier enregistrement à un binaire 100 % Rust.
- [Portages réels](/docs/real-world/overview) : json-server, Fuse.js, vercel/ms et d'autres, avec les résultats lents à côté des rapides.
