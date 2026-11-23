---
title: Shadow
description: shadow génère un module de remplacement direct qui exécute l'original et le Rust côte à côte et signale les écarts.
---

# Shadow

```sh
everything-rust shadow <crate>
```

`shadow` génère un module de remplacement direct avec l'API de l'original : un module TS ou Python, un package Go, un fichier Ruby, des classes Java ou un fichier C. Chargez-le là où l'application chargeait l'original. Il exécute les deux, renvoie le résultat de l'original et signale les écarts du Rust pendant que vous gagnez en confiance en production.

| `EVERYTHING_RUST_MODE` | Comportement |
| --- | --- |
| `shadow` (par défaut) | exécute les deux, renvoie le résultat de l'original, signale les écarts |
| `new` | bascule sur Rust |
| `old` | désactive Rust |

L'API publique ne change pas. Les appelants importent les mêmes fonctions, ou le module shadow à leur place.
