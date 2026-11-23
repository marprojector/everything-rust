---
title: PHP (preview)
description: PHP preview абалында. analyze PHP колдонмосун пландайт, ал эми PHP CLI же серверин жазып алууга, кайра ойнотууга жана Rust front door'дун артына коюуга болот.
---

# PHP (preview)

| | |
| --- | --- |
| Бирдик | функциялар жана `public static` методдорунан турган папка |
| Bridge | кийинки кезекте: C ABI аркылуу PHP FFI |
| Бүгүн | бүт колдонмолор: `app record`, `app replay` жана front door, CLI'лар жана серверлер үчүн (`php -S`) |
| Абалы | Preview: `analyze` жана бүт колдонмолор гана; бирдиктерди азырынча bridge аркылуу өткөрүүгө болбойт |

`analyze` PHP колдонмосун пландайт: Composer пакеттери жана PSR-4 autoloading, loose comparison жана integer'дин float'ка толуп кетиши сыяктуу тобокелдиктер, ошондой эле crate'терге шайкештелген Composer пакеттери. Типтер PHP типтеринен жана phpdoc'тон алынат.

PHP CLI же серверин жазып алууга, кайра ойнотууга жана Rust front door'дун артына коюуга болот:

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

Анын бирдиктерин PHP FFI аркылуу Rust'ка порттоо кийинки кадам ([issue #2](https://github.com/marprojector/everything-rust/issues/2)). Ага чейин `bridge` PHP бирдиктерин түшүнүктүү билдирүү менен четке кагат.

CI PHP орнотпойт, ошондуктан PHP тесттери ал орнотулган машинада гана иштейт.
