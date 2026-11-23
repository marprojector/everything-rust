---
title: PHP (попередній перегляд)
description: PHP у стадії попереднього перегляду. analyze планує застосунок на PHP, а CLI чи сервер на PHP можна записати, відтворити і поставити за front door на Rust.
---

# PHP (попередній перегляд)

| | |
| --- | --- |
| Модуль | тека з функціями та методами `public static` |
| Міст | далі: PHP FFI через C ABI |
| Сьогодні | цілі застосунки: `app record`, `app replay` і front door, для CLI та серверів (`php -S`) |
| Статус | Попередній перегляд: лише `analyze` і цілі застосунки; модулі поки не можна з'єднати мостом |

`analyze` планує застосунок на PHP: пакети Composer і автозавантаження PSR-4, ризики на кшталт нестрогого порівняння і переповнення цілих чисел у float, пакети Composer, зіставлені з крейтами. Типи беруться з типів PHP і phpdoc.

CLI чи сервер на PHP можна записати, відтворити і поставити за front door на Rust:

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

Портування його модулів у Rust через FFI у PHP буде наступним кроком ([issue #2](https://github.com/marprojector/everything-rust/issues/2)). Доти `bridge` відхиляє модулі PHP із зрозумілим повідомленням.

CI не встановлює PHP, тож тести PHP запускаються лише на машині, де він є.
