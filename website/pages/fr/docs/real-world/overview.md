---
title: Portages réels
description: Ce qui s'est passé quand everything-rust a rencontré du vrai code, avec les résultats lents à côté des rapides, et ce qui a changé dans l'outil en conséquence.
---

# Portages réels

Ce qui s'est passé quand everything-rust a rencontré du vrai code, et ce qui a changé dans l'outil en conséquence. Chaque chiffre ci-dessous vient d'un benchmark qui vérifie d'abord que les résultats sont identiques, et les cas lents sont listés à côté des rapides.

| Portage | Ce que c'est | Résultat |
| --- | --- | --- |
| [json-server](/docs/real-world/json-server) | un serveur Node entier, porté de bout en bout avec `app record`, `front-door`, `wire` et `lift` | un binaire 100 % Rust qui passe 514/514 cas HTTP enregistrés ; six unités passent `gate` |
| [Fuse.js](/docs/real-world/fuse-js) | recherche floue, le chemin où une recherche passe son temps | 3,414/3,414 cas correspondent ; 32x plus rapide sur 50,000 chaînes (5.3x sur un seul cœur) |
| [vercel/ms](/docs/real-world/vercel-ms) | un minuscule analyseur de durées, de bout en bout | 975/975 cas correspondent ; 0.56x par appel, 18x en lot `parse`, 0.8x en lot `format` |
| [x18n](/docs/real-world/x18n) | le dépôt de l'étude de cas | chaque gate passe sur `packages/core`, plus lent que le TS à travers un bridge ; sa recherche 18-22x en lot |
| [Six langages](/docs/real-world/six-languages) | `path` de Go, `shellwords` de Ruby, `colorsys` de Python, comportements du JDK et de la glibc derrière de petites façades | chaque portage passe son gate, 13,010 cas enregistrés au total |
| [Fixtures et exécutions statiques](/docs/real-world/fixtures) | des fixtures écrites pour mettre l'outil à l'épreuve, et des bibliothèques analysées mais non exécutées | |

Vous avez porté quelque chose de réel ? Un compte rendu est le bienvenu : voir [Contribuer](/docs/project/contributing#share-a-real-port).
