---
title: Porter une application entière
description: Du premier enregistrement à un binaire 100 % Rust. Le but est une application Rust qui se comporte comme l'original, l'original étant conservé comme référence.
---

# Porter une application entière

Le but est une application 100 % Rust qui se comporte comme l'original, l'original étant conservé comme référence. Votre agent écrit le Rust ; everything-rust ordonne le travail, crée la structure et prouve chaque étape. CLI, serveurs HTTP et bibliothèques.

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in    pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down proved by
 decisions to make      outside         original answers                                           replay
```

## 1. Planifier

```sh
everything-rust analyze . --whole-app
```

`--whole-app` ajoute la forme de l'application (CLI, serveur ou bibliothèque) et ses points d'entrée, chaque dépendance externe (importée ou déclarée) avec sa crate Rust et son adéquation, l'ordre de portage sous forme de morceaux (les unités qui s'importent mutuellement se déplacent ensemble), et des décisions numérotées avec une réponse recommandée : unités à garder, unités bloquées, dépendances sans crate ou avec une crate partielle.

## 2. Enregistrer toute l'application, puis déplacer le point d'entrée

1. [`app record` et `app replay`](/docs/whole-apps/record-replay) prouvent le programme de l'extérieur, comme une boîte noire.
2. [`app front-door`](/docs/whole-apps/front-door) place un binaire Rust devant l'original ; chaque commande exécute encore l'original.
3. Chaque unité passe par [la boucle](/docs/concepts/the-loop), puis `app wire` confie ses commandes à Rust.
4. [`app lift`](/docs/whole-apps/lift) retire l'original une fois que le replay passe avec un taux de fallback de 0 et que chaque unité passe le gate.

Les [serveurs](/docs/whole-apps/servers) et les [bibliothèques](/docs/whole-apps/libraries) utilisent les mêmes commandes.

## Comment l'agent s'y prend

Le [skill whole-app](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) guide l'agent pas à pas, en commençant par les décisions que vous seul pouvez prendre :

- **Interrogatoire** : l'agent pose ses questions par séries, chacune avec sa réponse recommandée, jusqu'à ce que chaque décision dont dépend le portage soit tranchée, et consigne chacune sous forme d'ADR dans `rust/DECISIONS.md`, avec `rust/GLOSSARY.md` qui fait correspondre les termes de l'original aux noms Rust.
- **Recherche** : des agents en arrière-plan lisent les sources primaires de chaque dépendance et écrivent `rust/research/<topic>.md`, avec chaque piège sémantique trouvé.
- **D'une session à l'autre** : `rust/MAP.md` liste chaque unité avec son état ; une session réserve une unité avant de commencer, et chaque point d'étape laisse un `rust/HANDOFF.md` pour la session suivante. Les unités d'une même vague peuvent être portées en parallèle par des sous-agents.
- **Bascule** : l'agent fait ce qu'il peut et écrit `rust/cutover.sh`, un script interactif pour les étapes que vous seul pouvez faire (déploiements, DNS, secrets), avec un retour arrière possible à chaque étape.

L'interrogatoire, la recherche et ces pratiques sont adaptés des [skills de Matt Pocock](https://github.com/mattpocock/skills).

## Le Rust ne restera-t-il pas prisonnier de l'ancienne conception ?

Seulement tant que les deux tournent côte à côte. Les enregistrements vérifient le comportement aux bords de l'application (la sortie d'une CLI, les réponses d'un serveur, les fonctions exportées d'un module), pas ses entrailles. Reproduisez l'ancienne structure tant que des unités traversent encore le bridge ; une fois que `app lift` a retiré l'original, restructurez librement le Rust, et `app replay` prouve toujours le comportement.

## Un cas réel

[json-server](/docs/real-world/json-server) est passé par toutes les étapes : 514 cas HTTP enregistrés, une porte d'entrée à 99.8% de fallback, les lectures câblées d'abord (22.6% de fallback), puis toutes les routes (0%), et un lift vers un binaire 100 % Rust qui passe 514/514.
