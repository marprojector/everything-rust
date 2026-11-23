---
title: PHP (preview)
description: PHP preview рәвешендә. analyze PHP кушымтасын планлаштыра, ә PHP CLI яки серверны язып алып, кабат уйнатып, Rust front door артына куеп була.
---

# PHP (preview)

| | |
| --- | --- |
| Берәмлек | функцияләр һәм `public static` методлар папкасы |
| Bridge | киләсе адым: C ABI өстендә PHP FFI |
| Бүген | бөтен кушымталар: `app record`, `app replay` һәм front door, CLI һәм серверлар өчен (`php -S`) |
| Халәт | Preview: `analyze` һәм бөтен кушымталар гына; берәмлекләр өчен bridge әлегә юк |

`analyze` PHP кушымтасын планлаштыра: Composer пакетлары һәм PSR-4 autoloading, катгый булмаган чагыштыру һәм бөтен саннарның float'ка ташып чыгуы кебек куркынычлар, Composer пакетларына туры килгән crate'лар. Типлар PHP типларыннан һәм phpdoc'тан алына.

PHP CLI яки серверны язып алып, кабат уйнатып, Rust front door артына куеп була:

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

Аның берәмлекләрен PHP FFI аша Rust'ка күчерү киләсе адым ([issue #2](https://github.com/marprojector/everything-rust/issues/2)). Ул вакытка кадәр `bridge` PHP берәмлекләрен аңлаешлы хәбәр белән кире кага.

CI PHP урнаштырмый, шуңа күрә PHP тестлары бары ул булган машинада гына җибәрелә.
