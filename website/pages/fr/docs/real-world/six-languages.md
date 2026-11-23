---
title: Six langages
description: everything-rust exécuté de bout en bout sur du vrai code en Go, Ruby, Python, Java et C, avec la vitesse par appel et en lot, et les bugs que ces exécutions ont trouvés.
---

# Du vrai code dans six langages

everything-rust a été exécuté de bout en bout (analyze, bridge, record, port, diff, gate, bench) sur du vrai code dans
chaque langage qu'il prend en charge. Le code devait déjà se trouver sur cette machine : le code téléchargé depuis
Internet n'a pas été exécuté. Les cibles sont donc des modules de bibliothèque standard livrés avec les
toolchains installées, et pour Java et C, où aucune source de bibliothèque adaptée n'était sur le disque, le JDK
et la glibc eux-mêmes servent d'oracle derrière une fine façade.

Les portages, les cas enregistrés et les entrées supplémentaires se trouvent dans [`demo/real-world`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/README.md).

| Langage | Code | Fonctions | Cas enregistrés | Résultat |
| --- | --- | ---: | ---: | --- |
| Go | `path` de la bibliothèque standard de Go 1.24 | 7 (+1 gardée) | 2,642 (542 issus des propres tables de tests de Go) | 2,642/2,642, le gate passe |
| Ruby | `shellwords.rb` de Ruby 3.3 | 3 | 1,617 (446 lèvent une exception) | 1,617/1,617, le gate passe |
| Python | `colorsys.py` de CPython 3.13 | 6 | 2,400 | 2,400/2,400, le gate passe |
| Java | JDK 21 derrière une façade (`Integer`, `Long`, `Math`, `Double.toString`, `URLEncoder`, `Base64`, `toUpperCase`) | 11 | 4,492 (226 lèvent une exception) | 4,492/4,492, le gate passe |
| C | glibc derrière une façade (`strtol`, `atoi`, `strtod`, `strspn`, `strcmp`, ctype) | 9 | 1,860 (6 plantent) | 1,859/1,859 après retrait d'un cas de comportement indéfini, le gate passe |
| C | aklomp/base64 (embarqué dans le runtime de mypyc) | 0 sur 7 | - | plan uniquement : chaque fonction prend des tampons de sortie ou une struct d'état |

## Vitesse

Les premiers bridges pour Python, Go, Ruby et Java faisaient passer chaque argument et chaque résultat par du texte
JSON, ce qui coûtait 10-50x plus que le travail de ces fonctions. Ils ont été reconstruits pour convertir
les valeurs directement. `bench` ne chronomètre désormais que les appels (arguments décodés une seule fois, dans le processus) et dit
s'il faut livrer.

| Langage | Par appel avant | Par appel maintenant | Rust face à l'original (`bench`, moyenne géométrique) | Verdict |
| --- | ---: | ---: | --- | --- |
| Ruby | 6.9-9.5 µs (Fiddle + JSON) | 0.36-1.15 µs (extension magnus) | **4.4x plus rapide** (3.6-7x selon la fonction) | livrer |
| Python | 11.7 µs (JSON) | environ 0.3 µs (conversion PyO3 directe) | **1.35x plus rapide** (1.5-1.9x sur la plupart des appels) | livrer |
| C | - | pas de traversée : lié directement | 0.87x : ctype 1.1-1.4x plus rapide, `strtod`, `strspn`, `strcmp` optimisés de la glibc 0.5-0.7x | optimiser ou garder les plus lentes |
| Java | environ 1.7 µs (JNI + JSON) | 30-800 ns (JNI typé et protocole binaire) | 0.18x | porter de plus grandes unités |
| Go | 2.6 µs (cgo + JSON) | 0.11-0.42 µs (cgo typé et protocole binaire) | 0.04x | porter de plus grandes unités |

Les listes, maps, structs et résultats multiples en Go et en Java passent par un protocole binaire généré pour chaque
type : `WordFreq` en Go (une map) est passé de 5.1 à 1.0 µs, `Midpoint` (une struct) de 7.2 à 1.0 µs ;
`frequencies` en Java (une `Map`) de 8.4 à 1.0 µs, et `words` (une `List<String>`) de 6.0 µs à
0.57 µs, 1.39x plus rapide que Java lui-même.

TypeScript (cœur de x18n, neuf unités, 27 fonctions sur les mêmes cas) : le protocole binaire a réduit le
coût du bridge d'un tiers en moyenne (`rangeContains` 3.6 -> 1.1 µs, `applyTextEdits` 10.4 -> 5.7
µs, `toWireRange` 5.0 -> 2.6 µs), et `buildTree` est passé de l'égalité à 2.0x plus rapide que le TS. Les
autres fonctions sont des feuilles que V8 exécute en 15-600 ns : toujours plus lentes en Rust, comme pour Go et Java.

Ce qui tranche :

- **Originaux interprétés (Python, Ruby)** : l'appel vers Rust coûte environ 100-300 ns et le
  code Rust est bien plus rapide que l'interpréteur, donc même les petites fonctions y gagnent.
- **Originaux compilés (Go, Java après le JIT)** : la fonction feuille d'origine prend 4-90 ns ; un seul
  appel cgo coûte à lui seul environ 80 ns, un appel JNI avec des chaînes davantage. Aucun bridge ne peut rendre plus rapide une fonction
  de 5 ns. Ces langages ne gagnent que lorsqu'un appel fait beaucoup de travail : portez une fonction
  avec ses appelants. `bench` le dit.
- **C** : il n'y a pas de traversée ; le code Rust est comparé directement au code C, et le résultat
  est une propriété du portage.

### Lots : la recherche de x18n

Les portages de fonctions feuilles ci-dessus sont mesurés par appel. Sur la recherche de la barre latérale de x18n (50,000 clés, tapée lettre
par lettre), `everything-rust profile` a montré que `fuzzyScore` occupait environ un quart de l'exécution, appelée une fois par clé depuis
`matched`, et a suggéré `--batch fuzzyScore:text`. Le `FuzzyScoreBatch` généré garde les
clés en Rust ; un `FuzzyScoreIndex` dans `port.rs` les passe une seule fois en minuscules UTF-16 et garde un masque 64 bits
des unités de code de chaque clé, de sorte que la plupart des clés qui ne peuvent pas correspondre sont ignorées sans être lues. Les résultats
sont identiques à ceux du pipeline TS pour chaque requête :

| | ms par requête | face au TS |
| --- | ---: | ---: |
| Pipeline TS | 10 | 1x |
| TS avec le même filtrage | 4.1-5.4 | 2x |
| Lot, index par défaut (portage par élément, 4 cœurs) | 5.1 | 1.7x |
| Lot avec `FuzzyScoreIndex` (masques), napi, 4 cœurs | 1.2 | 7x |
| Lot avec `FuzzyScoreIndex` qui classe lui-même, napi, 4 cœurs | 0.45-0.56 | 18-22x |
| Le même, wasm, 1 thread | 0.95 | 11.5x |

Le passage de 7x à 20x : sur les 1.2 ms, 0.4 ms venait du propre `Array.from(order, i =>
keys[i])` du benchmark (une simple boucle coûte 0.02 ms). Le reste est venu du fait de laisser l'index classer lui-même :
il ne recalcule le score que des correspondances de la dernière requête quand la nouvelle requête la prolonge (une clé qui ne
correspond pas à `s` ne peut pas correspondre à `se`), lit les requêtes d'une lettre dans une table de premières positions, note les
clés ASCII comme des octets, et ordonne les scores entiers avec un tri par comptage. Callgrind a montré que les
appels de bibliothèque (`memchr`, `memcmp`) coûtaient plus que de simples boucles sur des clés de 30 octets. Une partie du gain
vient de l'algorithme, donc le benchmark exécute aussi le TS avec le même filtrage : Rust est 7-12x plus rapide
que celui-ci (wasm 5x).

Le même schéma dans les autres langages, face aux originaux (4 cœurs, résultats et
erreurs identiques) :

| Langage | Charge | Original | Portage par appel face à l'original | Lot face à l'original |
| --- | --- | ---: | ---: | ---: |
| Python | `slugify` de fixture, 20,000 chaînes | 98 ms | - | 17x |
| Ruby | `slugify` de fixture, 20,000 chaînes | 110 ms | - | 12.7x |
| Ruby | `Shellwords.shellescape`, 50,000 valeurs mixtes | 88 ms | - | 11.8x |
| Go | `Slugify` de fixture, 50,000 chaînes | 49 ms | 0.93x | 3.3-4.1x |
| Java | `slugify` de fixture, 50,000 chaînes | 43 ms | 0.68x | 3.0x |

Le lot Go a d'abord tourné plus lentement qu'un seul thread : le portage alloue pour chaque élément et l'allocateur de la glibc
sérialisait les threads à l'intérieur du processus Go. Les crates de lot utilisent désormais mimalloc. Le lot shellescape
a aussi trouvé un bug du portage que l'enregistrement par appel avait manqué : les symboles Ruby (`shellescape(:sym)`) revenaient
sous forme de leur marqueur JSON. Corrigé dans le portage.

Go et Java sont ensuite passés de 2.5x et 1.7x à 4x et 3x. Dans le lot, la moitié du temps n'était pas
le travail : les résultats étaient joints et libérés sur un seul thread, Go les copiait une deuxième fois, et Java
faisait un appel JNI par chaîne en passant par le modified UTF-8. Les résultats chaînes reviennent désormais en bloc, écrits
sur tous les cœurs dans le propre tampon de Go ou dans un seul `char[]` Java. L'autre moitié venait des portages : sur un seul
thread, le `slugify` Rust était à peine plus rapide que celui de Go et plus lent que le JIT de Java. Passer l'ASCII en minuscules
sans recherche dans une table et sans chaîne intermédiaire a réglé cela ; le portage Java garde la
règle du sigma final en ne passant en minuscules d'un bloc que le morceau délimité par des espaces qui contient un `Σ`.
Par appel, le portage Go est passé de 0.73x à 0.93x, le portage Java de 0.46x à 0.68x.

## Ce que les exécutions ont trouvé

### Bugs dans everything-rust, corrigés

- **Des flottants modifiés d'un ULP** (exécution Java). L'analyseur de flottants par défaut de `serde_json` n'arrondit pas toujours
  correctement. everything-rust altérait les entrées enregistrées en les lisant, et les crates générées
  les altéraient à nouveau. 43 cas `Double.toString` échouaient pour un portage correct. Corrigé avec la
  feature `float_roundtrip` de `serde_json` dans everything-rust et dans chaque crate qu'il génère (napi-rs,
  wasm et le dispatcher JSON).
- **Les plantages C tuaient l'enregistrement** (exécution C). Des entrées hors du contrat d'une fonction C (`isalpha(INT_MAX)`)
  font planter la glibc en segfault. Le pilote C exécute désormais chaque cas dans un processus enfant forké et enregistre
  `crash: SIGSEGV` ; `diff` accepte alors n'importe quel comportement du portage, puisque celui du C est indéfini.
- **`module_function :name` non détecté** (exécution Ruby). `shellwords` déclare ses fonctions de module
  avec des arguments symboles après les méthodes ; l'exporteur ne connaissait que la forme nue
  `module_function`. Les deux formes sont désormais trouvées.
- **Le code non typé recevait des entrées absurdes** (exécution Python). `colorsys` n'a pas d'annotations de type, donc 1,709 des
  1,800 cas générés étaient des `TypeError`. Les paramètres non annotés utilisés comme des nombres (`r - g`,
  `max(r, g, b)`, `x < 1`) sont désormais typés `number`, en Python et en Ruby : il reste 94 erreurs, toutes réelles
  (`ZeroDivisionError`).
- **Les entiers passés à la place de flottants n'étaient jamais enregistrés** (exécution Python). Les appelants Python passent `1` aussi souvent que
  `1.0`, et `colorsys` transmet les entiers tels quels (`hsv_to_rgb(0, 0, 7)` vaut `(7, 7, 7)`). L'enregistrement
  passe désormais les nombres entiers sous forme d'entiers dans un cas Python sur quatre ; cela a révélé 134 écarts dans un
  portage typé `f64`.
- **Des messages hérités de l'époque TypeScript** : « must stay in JS », « kept in TS », « characterization tests
  against the TS code » nomment désormais le langage source.
- **Les classes de base rouvertes** (`class String` dans `shellwords.rb`) sont désormais signalées comme un risque de monkeypatching.

### Comportements qu'un portage doit reproduire, trouvés par `diff`

Chaque ligne était un écart dans un premier portage, détecté par les cas enregistrés. Les lignes marquées *fixture*
viennent des petits projets de test de `tests/fixtures/`, écrits pour chaque langage ; les autres des
exécutions réelles ci-dessus.

- Go `path.Clean` : le `lazybuf` de Go lit l'octet à l'index d'écriture après son recul, pas le
  dernier octet conservé (9 cas, issus de la propre table de tests de Go).
- *fixture* `strings.ToLower` en Go et `downcase` en Ruby passent chaque rune en minuscule isolément : `ΣΊΣΥΦΟΣ` devient `σίσυφοσ`,
  là où le `to_lowercase` de Rust applique la règle du sigma final. Go convertit aussi `İ` en `i`, là où Rust
  donne `i̇`.
- *fixture* Go découpe les chaînes par octets ; une coupure au milieu d'une rune arrive dans le JSON sous forme d'un U+FFFD par octet.
- *fixture* `slugify` en Ruby levait `NoMethodError` pour un `max` négatif (`slug[0, -1]`
  vaut `nil`) : un bug de l'original que le portage doit conserver, à moins de corriger d'abord les appelants.
- *fixture* `Array#sum` en Ruby utilise la sommation de Kahan-Babuska et part de l'entier 0, donc `[-0.0].sum` vaut `0.0`.
- `Float#to_s` en Ruby passe en notation exponentielle à partir de 1e15 (`1e15.to_s` donne `1.0e+15`, `1e14.to_s` donne
  `100000000000000.0`) ; `nil.inspect` à l'intérieur d'un tableau donne `nil`.
- *fixture* En Java, `\s` est ASCII uniquement ; le `split_whitespace` de Rust coupe aussi sur U+00A0.
- Les résultats entier-ou-flottant de Python : `colorsys` garde les entiers à travers `+`, `-` et `*`, donc le portage fidèle
  calcule avec un petit type entier-ou-flottant au lieu de `f64` (134 cas une fois les entiers enregistrés).
- `strtod("-")` de la glibc ne convertit rien et renvoie `+0.0`, pas `-0.0`.

Traités dès le premier portage, et confirmés par les cas enregistrés :

- `Integer.parseInt` en Java accepte n'importe quel chiffre décimal Unicode (`١٢`, `２`) via `Character.digit`.
- `Double.toString` en Java (JDK 19+) affiche la plus courte représentation décimale, mais jamais moins de deux chiffres :
  `4.9E-324`, là où la forme la plus courte de Rust est `5e-324`.
- En Python, `max(nan, 1, 2)` vaut `nan` (la première valeur gagne sauf si une suivante est strictement plus grande) ;
  `-0.0 % 1.0` vaut `0.0`.
- `toupper(-128)` de la glibc vaut `128` (ses tables acceptent les valeurs `char` signées) ; `strtol` en base 0
  lit `077` comme de l'octal et `0x` sans chiffre hexadécimal comme `0`.

### Limites révélées par ces exécutions

- **Les API C qui passent des tampons.** Les fonctions publiques d'aklomp/base64 prennent `char *out, size_t *outlen`
  ou un `struct base64_state *`. Le bridge C ne transporte que des scalaires et des `const char *`, donc il les garde
  toutes en C, avec la raison. Prendre en charge le motif du tampon de sortie est la prochaine étape pour C.
- **Les macros C.** 43 des 51 fichiers de base64 définissent des fonctions par des macros
  (`BASE64_ENC_FUNCTION(arch)`) ; l'analyseur y signale des erreurs et le plan marque ces unités comme
  partielles. Les intrinsèques SIMD ne sont pas encore signalés comme un risque.
- **Le comportement indéfini qui ne plante pas.** `ispunct(1000)` lit au-delà de la table de la glibc et
  renvoie 1. Aucun portage ne devrait copier cela ; le cas en question a été retiré de `cases.jsonl` à la main. Il n'existe
  pas encore de moyen intégré de marquer un cas comme indéfini.
- **Le typage dynamique coûte du code.** Le portage de `shellwords` réimplémente le `to_s` et le `inspect` de Ruby
  pour les nombres, nil, les tableaux et les hashes, car `shellescape` accepte n'importe quoi. Le portage de `colorsys`
  a besoin de la tour numérique de Python. Un portage typé est plus court, mais change le comportement pour certains
  appelants ; `diff` montre exactement lesquels.
- **Les façades.** Les exécutions Java et C testent le vrai comportement du JDK et de la glibc, mais à travers des façades
  écrites pour l'occasion, pas à travers l'API propre d'une bibliothèque.

## Comment chaque langage s'exécute

| | Record | Diff | Bridge | Shadow |
| --- | --- | --- | --- | --- |
| Go | pilote généré, compilé comme module à part avec un `replace` vers l'original | le même pilote sur le wrapper cgo | dispatcher JSON + package cgo avec l'API d'origine | package avec l'API d'origine, exécute les deux |
| Ruby | le pilote généré charge les fichiers d'origine | le même pilote charge le wrapper Fiddle | dispatcher JSON + wrapper Fiddle qui définit les mêmes modules | fichier à charger avec require après l'original : fait un `prepend` des deux |
| Java | pilote par réflexion compilé avec les sources d'origine | le même pilote compilé avec les classes wrapper JNI | dispatcher JSON via JNI + classes wrapper avec les méthodes statiques d'origine | classes `<Class>Shadow` |
| Python | pilote dans le processus qui importe l'original | le même pilote importe le package PyO3 | PyO3 | module avec l'API d'origine |
| C | `cc -shared` de l'original, appelé via ctypes, un fork par cas | ctypes sur le `cdylib` Rust | fonctions `extern "C"` avec les noms d'origine et un en-tête | `.c` shadow avec des renommages `-D` ; la feature `shadow` de la crate n'exporte que des noms `everything_rust_` |
