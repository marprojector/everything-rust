---
title: PHP（預覽）
description: PHP 目前為預覽。analyze 可以規劃 PHP 應用程式，而 PHP CLI 或伺服器可以被錄製、重播，並放到 Rust 入口之後。
---

# PHP（預覽）

| | |
| --- | --- |
| 單元 | 一個包含函式與 `public static` 方法的資料夾 |
| 橋接 | 即將推出：透過 C ABI 的 PHP FFI |
| 目前 | 整個應用程式：`app record`、`app replay` 與入口，適用於 CLI 與伺服器（`php -S`） |
| 狀態 | 預覽：僅支援 `analyze` 與整個應用程式；單元目前還無法橋接 |

`analyze` 可以規劃 PHP 應用程式：Composer 套件與 PSR-4 自動載入、寬鬆比較與整數溢位成浮點數等風險，以及 Composer 套件對應到 crate。型別來自 PHP 型別與 phpdoc。

PHP CLI 或伺服器可以被錄製、重播，並放到 Rust 入口之後：

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

透過 PHP 的 FFI 將其單元移植到 Rust 是下一步（[issue #2](https://github.com/marprojector/everything-rust/issues/2)）。在那之前，`bridge` 會以清楚的訊息拒絕 PHP 單元。

CI 沒有安裝 PHP，所以 PHP 的測試只會在有安裝它的機器上執行。
