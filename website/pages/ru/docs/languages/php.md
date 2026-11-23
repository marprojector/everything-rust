---
title: PHP (превью)
description: PHP находится в превью. analyze планирует PHP-приложение, а PHP CLI или сервер можно записать, воспроизвести и поставить за входную точку на Rust.
---

# PHP (превью)

| | |
| --- | --- |
| Модуль | каталог функций и методов `public static` |
| Мост | далее: PHP FFI поверх C ABI |
| Уже сейчас | целые приложения: `app record`, `app replay` и входная точка, для CLI и серверов (`php -S`) |
| Статус | Превью: только `analyze` и целые приложения; модули пока нельзя подключить через мост |

`analyze` планирует PHP-приложение: пакеты Composer и автозагрузка PSR-4, риски вроде нестрогого сравнения и переполнения целых чисел во float, сопоставление пакетов Composer с крейтами. Типы берутся из типов PHP и phpdoc.

PHP CLI или сервер можно записать, воспроизвести и поставить за входную точку на Rust:

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

Перенос его модулей в Rust через PHP FFI станет следующим этапом ([issue #2](https://github.com/marprojector/everything-rust/issues/2)). До тех пор `bridge` отклоняет модули PHP с понятным сообщением.

CI не устанавливает PHP, поэтому тесты PHP запускаются только на машине, где он есть.
