---
title: PHP（预览）
description: PHP 处于预览阶段。analyze 能为 PHP 应用制定计划，PHP CLI 或服务端可以被录制、回放，并放到 Rust 前门之后。
---

# PHP（预览）

| | |
| --- | --- |
| 单元 | 包含函数和 `public static` 方法的文件夹 |
| 桥接层 | 即将推出：基于 C ABI 的 PHP FFI |
| 目前可用 | 整个应用：`app record`、`app replay` 和前门，适用于 CLI 与服务端（`php -S`） |
| 状态 | 预览：仅支持 `analyze` 和整个应用；单元暂时还不能桥接 |

`analyze` 能为 PHP 应用制定计划：Composer 包和 PSR-4 自动加载，松散比较、整数溢出变成浮点数等风险，以及 Composer 包到 crate 的映射。类型来自 PHP 类型和 phpdoc。

PHP CLI 或服务端可以被录制、回放，并放到 Rust 前门之后：

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

通过 PHP 的 FFI 把它的单元迁移到 Rust 是下一步（[issue #2](https://github.com/marprojector/everything-rust/issues/2)）。在此之前，`bridge` 会拒绝 PHP 单元，并给出明确的提示。

CI 不安装 PHP，所以 PHP 测试只会在装有 PHP 的机器上运行。
