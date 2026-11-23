---
title: PHP (pré-visualização)
description: O PHP está em pré-visualização. O analyze planeia uma aplicação PHP, e uma CLI ou um servidor PHP pode ser registado, reproduzido e colocado atrás de uma front door em Rust.
---

# PHP (pré-visualização)

| | |
| --- | --- |
| Unidade | uma pasta de funções e métodos `public static` |
| Bridge | em breve: PHP FFI sobre uma ABI de C |
| Hoje | aplicações completas: `app record`, `app replay` e a front door, para CLIs e servidores (`php -S`) |
| Estado | Pré-visualização: apenas `analyze` e aplicações completas; ainda não é possível criar bridges para unidades |

O `analyze` planeia uma aplicação PHP: pacotes Composer e autoloading PSR-4, riscos como a comparação não estrita e o overflow de inteiros para floats, e pacotes Composer mapeados para crates. Os tipos vêm dos tipos de PHP e do phpdoc.

Uma CLI ou um servidor PHP pode ser registado, reproduzido e colocado atrás de uma front door em Rust:

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

Migrar as suas unidades para Rust através da FFI do PHP é o próximo passo ([issue #2](https://github.com/marprojector/everything-rust/issues/2)). Até lá, o `bridge` recusa unidades PHP com uma mensagem clara.

A CI não instala o PHP, por isso os testes de PHP só correm numa máquina que o tenha.
