---
title: PHP (préversion)
description: PHP est en préversion. analyze planifie une application PHP, et une CLI ou un serveur PHP peut être enregistré, rejoué et placé derrière une porte d'entrée Rust.
---

# PHP (préversion)

| | |
| --- | --- |
| Unité | un dossier de fonctions et de méthodes `public static` |
| Bridge | à venir : PHP FFI via une ABI C |
| Aujourd'hui | applications entières : `app record`, `app replay` et la porte d'entrée, pour les CLI et les serveurs (`php -S`) |
| État | Préversion : `analyze` et applications entières uniquement ; les unités ne peuvent pas encore passer par un bridge |

`analyze` planifie une application PHP : packages Composer et autoload PSR-4, risques comme la comparaison souple et le dépassement d'entier vers les flottants, et packages Composer mis en correspondance avec des crates. Les types viennent des types PHP et de phpdoc.

Une CLI ou un serveur PHP peut être enregistré, rejoué et placé derrière une porte d'entrée Rust :

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

Porter ses unités en Rust via le FFI de PHP est la prochaine étape ([issue #2](https://github.com/marprojector/everything-rust/issues/2)). D'ici là, `bridge` refuse les unités PHP avec un message clair.

La CI n'installe pas PHP, donc les tests PHP ne tournent que sur une machine qui l'a.
