---
title: PHP (preview)
description: PHP is in preview. analyze plans a PHP app, and a PHP CLI or server can be recorded, replayed and put behind a Rust front door.
---

# PHP (preview)

| | |
| --- | --- |
| Unit | a folder of functions and `public static` methods |
| Bridge | coming next: PHP FFI over a C ABI |
| Today | whole apps: `app record`, `app replay` and the front door, for CLIs and servers (`php -S`) |
| Status | Preview: `analyze` and whole apps only; units cannot be bridged yet |

`analyze` plans a PHP app: Composer packages and PSR-4 autoloading, risks such as loose comparison and integer overflow into floats, and Composer packages mapped to crates. Types come from PHP types and phpdoc.

A PHP CLI or server can be recorded, replayed and put behind a Rust front door:

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

Porting its units into Rust through PHP's FFI is the next step ([issue #2](https://github.com/marprojector/everything-rust/issues/2)). Until then, `bridge` refuses PHP units with a clear message.

CI does not install PHP, so the PHP tests run only on a machine that has it.
