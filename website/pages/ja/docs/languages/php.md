---
title: PHP（プレビュー）
description: PHP はプレビュー版です。analyze は PHP アプリを計画し、PHP の CLI やサーバーは記録・リプレイして Rust のフロントドアの背後に置けます。
---

# PHP（プレビュー）

| | |
| --- | --- |
| ユニット | 関数および `public static` メソッドのフォルダ |
| ブリッジ | 次に対応予定: C ABI 越しの PHP FFI |
| 現時点 | アプリ全体: CLI およびサーバー（`php -S`）向けの `app record`、`app replay`、フロントドア |
| ステータス | プレビュー: `analyze` とアプリ全体のみ。ユニットはまだブリッジできません |

`analyze` は PHP アプリを計画します: Composer パッケージと PSR-4 オートローディング、緩い比較や整数オーバーフローによる float 化などのリスク、Composer パッケージからクレートへの対応付けです。型は PHP の型と phpdoc から取得します。

PHP の CLI やサーバーは記録・リプレイして Rust のフロントドアの背後に置けます:

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

PHP の FFI を通じてユニットを Rust に移植するのが次のステップです（[issue #2](https://github.com/marprojector/everything-rust/issues/2)）。それまでは、`bridge` は明確なメッセージとともに PHP のユニットを拒否します。

CI は PHP をインストールしないので、PHP のテストは PHP がインストールされたマシンでのみ実行されます。
