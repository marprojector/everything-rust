---
title: Fuse.js
description: La recherche dans une liste de chaînes de Fuse.js en Rust. 3,414/3,414 cas enregistrés correspondent, résultats identiques octet pour octet, 32x plus rapide sur 50,000 chaînes.
---

# Fuse.js

[Fuse.js](https://github.com/krisk/Fuse) (20k étoiles, environ 20 millions de téléchargements npm par semaine,
commit `edf2fb6`) est la bibliothèque de recherche floue derrière d'innombrables champs de recherche. Portée avec
everything-rust 1.6 (non publié) ; la crate se trouve dans [`demo/real-world/fuse-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/fuse-rs).

`analyze` a placé chaque dossier de `src/` dans un même cycle d'import, donc Fuse ne peut pas être porté fichier par fichier.
Le portage prend plutôt le chemin où une recherche passe son temps : un adaptateur de 54 lignes
(`adapter/relevance.ts`) exporte le score propre à Fuse pour une chaîne d'une liste de chaînes, construit à partir de
`BitapSearch`, de la norme de champ et de `computeScoreSingle` de Fuse. C'est cette fonction qu'une recherche sur
50,000 chaînes exécute 50,000 fois. `bridge --batch relevance:text` garde les chaînes en Rust et
lance une recherche sur toutes, sur tous les cœurs.

| Étape | Résultat |
| --- | --- |
| adaptateur | le classement de l'adaptateur est égal à `new Fuse(list).search(q)` sur 81 requêtes et jeux d'options avant toute ligne de Rust |
| `record --inputs` | 3,414 cas : titres issus des propres fixtures de Fuse, non-ASCII, emoji, motifs de plus de 32 unités, chaque option prise en charge, et options mal typées (refusées, 149 cas) |
| portage | 673 lignes de Rust pour les 839 lignes de Fuse qu'il remplace |
| `diff` | 3,414/3,414 correspondent dès le premier build |
| `gate` | passe |
| remplacement direct | `withRust(Fuse)` répond aux recherches dans les listes de chaînes en Rust et confie tout le reste à Fuse ; il renvoie des résultats identiques octet pour octet à Fuse sur 360 recherches (12 jeux d'options, 15 requêtes, avec et sans `limit`) |

Temps de recherche par requête, Apple M4 (10 cœurs), Node 24, résultats vérifiés identiques avant le chronométrage :

| Liste | Fuse.js | Rust | Gain |
| ---: | ---: | ---: | ---: |
| 1,000 chaînes | 1.9 ms | 0.1 ms | 18x |
| 10,000 chaînes | 19.1 ms | 0.7 ms | 27 à 30x |
| 50,000 chaînes | 91.2 ms | 2.8 ms | **32x** |
| 50,000 chaînes, un seul cœur | 92.0 ms | 17.5 ms | 5.3x |

Ce que le portage devait respecter :

- Bitap tourne sur des entiers JS 32 bits : `1 << 31` est négatif et `(1 << 31) - 1` est un double qui
  devient `2147483647` au décalage suivant. Le portage garde ces valeurs telles que JS les a.
- Le texte est indexé en unités de code UTF-16, donc un emoji occupe deux positions, comme dans Fuse.
- `ignoreDiacritics` correspond à NFD plus une liste de plages de signes combinants et quelques lettres écrites en toutes lettres
  (`ł`, `ß`, `ı`) ; le portage utilise la même liste.
- `String.prototype.trim` décide quelles chaînes Fuse ignore : les espaces blancs de JS, pas le
  `char::is_whitespace` de Rust (qui inclut U+0085 et exclut U+FEFF).

Trouvé dans l'outil, et corrigé : les crates de lot dont les éléments étaient des chaînes mais pas les résultats
recevaient un helper inutilisé et échouaient à `clippy -D warnings` ; et la vérification de taille de `gate` ne comptait que
l'adaptateur (54 lignes, donc 12.5x « trop gros ») au lieu des fichiers de Fuse qu'il importe.

Ce qu'il ne couvre pas : les listes d'objets avec `keys`, la recherche étendue et par tokens, les requêtes logiques,
`includeMatches`, un `sortFn` ou un `getFn` personnalisé. Le remplacement direct les confie à Fuse sans changement. Le
gain exige que la liste reste en Rust entre les recherches ; la construire coûte à peu près autant que
l'index propre à Fuse (3.4 ms contre 2.8 ms pour 50,000 chaînes).
