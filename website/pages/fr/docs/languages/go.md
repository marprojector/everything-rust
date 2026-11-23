---
title: Go
description: Portez un package Go en Rust via des appels cgo typés et un protocole binaire généré, dans un package qui garde l'API d'origine.
---

# Go

| | |
| --- | --- |
| Unité | un package |
| Bridge | appels cgo typés et un protocole binaire généré, dans un package avec l'API d'origine |
| Enregistré et rejoué par | un pilote Go généré, compilé comme module à part |
| Prérequis | Go 1.21+ |
| État | Vérifié sur `path` de Go 1.24 : 2,642/2,642 (542 issus des tables de tests de Go), le gate passe |

`profile` fonctionne via `go test` : `everything-rust profile <repo> -- go test -run X -bench . ./pkg`. Les lots s'écrivent `NewFnBatch(items)` ; fermez-les une fois terminé.

## Ce qu'un portage doit respecter

- `path.Clean` : le `lazybuf` de Go lit l'octet à l'index d'écriture après son recul, pas le dernier octet conservé (9 cas, issus de la propre table de tests de Go).
- `strings.ToLower` passe chaque rune en minuscule isolément : `ΣΊΣΥΦΟΣ` devient `σίσυφοσ`, là où le `to_lowercase` de Rust applique la règle du sigma final. Go convertit aussi `İ` en `i`, là où Rust donne `i̇`.
- Go découpe les chaînes par octets ; une coupure au milieu d'une rune arrive dans le JSON sous forme d'un U+FFFD par octet.

## Vitesse

Go exécute les fonctions feuilles en quelques nanosecondes, et un seul appel cgo coûte à lui seul environ 80 ns. Sur `path`, `bench` a mesuré le portage à 0.04x par appel : portez de plus grandes unités. En lot, le travail passe en Rust sur tous les cœurs : `Slugify` sur 50,000 chaînes prenait 49 ms en Go, et le lot a tourné **4x plus vite** (le portage par appel : 0.93x). Les crates de lot allouent avec mimalloc : avec l'allocateur système, le lot Go était plus lent qu'un seul thread.

Détails : [Six langages](/docs/real-world/six-languages).
