---
title: PHP (Vorschau)
description: PHP ist in der Vorschau. analyze plant eine PHP-App, und eine PHP-CLI oder ein PHP-Server lässt sich aufzeichnen, abspielen und hinter eine Rust-Front-Door stellen.
---

# PHP (Vorschau)

| | |
| --- | --- |
| Unit | ein Ordner mit Funktionen und `public static`-Methoden |
| Bridge | folgt als Nächstes: PHP FFI über eine C-ABI |
| Heute | ganze Apps: `app record`, `app replay` und die Front Door, für CLIs und Server (`php -S`) |
| Status | Vorschau: nur `analyze` und ganze Apps; Units lassen sich noch nicht bridgen |

`analyze` plant eine PHP-App: Composer-Pakete und PSR-4-Autoloading, Risiken wie loser Vergleich und Ganzzahlüberlauf in Floats, und Composer-Pakete, die auf Crates abgebildet werden. Typen kommen aus PHP-Typen und phpdoc.

Eine PHP-CLI oder ein PHP-Server lässt sich aufzeichnen, abspielen und hinter eine Rust-Front-Door stellen:

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

Das Portieren der Units nach Rust über PHPs FFI ist der nächste Schritt ([Issue #2](https://github.com/marprojector/everything-rust/issues/2)). Bis dahin lehnt `bridge` PHP-Units mit einer klaren Meldung ab.

Die CI installiert kein PHP, daher laufen die PHP-Tests nur auf einem Rechner, auf dem es vorhanden ist.
