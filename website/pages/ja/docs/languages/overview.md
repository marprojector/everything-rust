---
title: 対応言語
description: TypeScript/JavaScript、Python、C、Go、Ruby、Java、そしてプレビュー版の PHP と C#。それぞれのユニット、ブリッジ、記録方法。
---

# 対応言語

6 つの言語がループ全体を通ります: Node.js（TypeScript/JavaScript）、Python、C、Go、Ruby、Java です。PHP と C# はプレビュー版です: 現時点では `analyze` とアプリ全体に対応し、ユニットのブリッジは次に対応予定です。

| 言語 | ユニット | ブリッジ | 記録とリプレイの実行方法 |
| --- | --- | --- | --- |
| TypeScript/JavaScript | フォルダまたはファイル | napi-rs または wasm-bindgen と自動生成のバイナリワイヤ。クラスと async にも対応 | Node で `.ts` ソースを実行 |
| Python | パッケージまたはモジュール | PyO3（abi3）、直接変換、型付き例外 | Python で元のモジュールをインポート |
| C | `.c` ファイル | 元の名前を持つ `extern "C"` 関数とヘッダ | 元のコードを共有ライブラリにコンパイルし、ctypes 経由でケースごとに fork して呼び出し |
| Go | パッケージ | 型付き cgo 呼び出しと自動生成のバイナリワイヤ。元の API を持つパッケージとして提供 | 独立したモジュールとしてビルドされる自動生成の Go ドライバー |
| Ruby | モジュール関数を含むファイルまたはフォルダ | 元のモジュール配下に置かれる magnus 拡張 | 自動生成の Ruby ドライバー |
| Java | `public static` メソッドのパッケージ | 型付き JNI と自動生成のバイナリワイヤ。元のメソッドを持つラッパークラスとして提供 | 自動生成のリフレクションドライバー |
| PHP（プレビュー） | 関数および `public static` メソッドのフォルダ | 次に対応予定: C ABI 越しの PHP FFI | 現時点でアプリ全体に対応: CLI およびサーバー（`php -S`）向けの `app record`、`app replay`、フロントドア |
| C#（プレビュー） | `public static` メソッドのフォルダ | 次に対応予定: C ABI 越しの P/Invoke | 現時点でアプリ全体に対応: CLI および ASP.NET サーバー（`dotnet <app>.dll`）向けの `app record`、`app replay`、フロントドア |

C 以外のすべての言語でバッチクラス（`--batch`）も生成されます: 要素は一度だけ境界を越えて
Rust 側に保持され、呼び出しのたびに全コアで全要素を処理します。

言語はリポジトリから検出されます（`go.mod`、`pom.xml`、`.csproj` または `.sln`、
`pyproject.toml`、`Gemfile`、`composer.json`、`package.json`、該当しなければ最も多いソースファイル）。
`--lang` で上書きできます。型はソースから取得します: TS の型、Python の型ヒント、C、Go、Java、C# の
シグネチャ、Ruby の YARD タグ、PHP の型と phpdoc。型のない引数で数値として使われているものは
数値と推論されます。

PHP はプレビュー版です: `analyze` は PHP アプリを計画し（Composer パッケージと PSR-4 オートローディング、
緩い比較や整数オーバーフローによる float 化などのリスク、Composer パッケージからクレートへの対応付け）、
PHP の CLI やサーバーは記録・リプレイして Rust のフロントドアの背後に置けます。PHP の FFI を通じて
ユニットを Rust に移植するのが次のステップです（[issue #2](https://github.com/marprojector/everything-rust/issues/2)）。

C# も同様にプレビュー版です: `analyze` は `.csproj` プロジェクトとその NuGet パッケージ
（クレートに対応付け）を読み取り、`using` ディレクティブをフォルダに解決し、C# の挙動が異なる点
（`decimal`、`dynamic`、LINQ の遅延クエリ、`Task`、`lock`、リフレクション）を指摘します。C# の CLI や ASP.NET
サーバーは記録・リプレイして Rust のフロントドアの背後に置けます。P/Invoke を通じたユニットの移植は
次に対応予定です。
## 言語ごとのステータス

| 言語 | ステータス | 最後の実プロジェクトでの実行 |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | 検証済み | json-server、Fuse.js、vercel/ms、x18n の `packages/core` |
| [Python](/docs/languages/python) | 検証済み | CPython 3.13 の `colorsys.py`: 2,400/2,400 |
| [Go](/docs/languages/go) | 検証済み | Go 1.24 の `path`: 2,642/2,642 |
| [Ruby](/docs/languages/ruby) | 検証済み | Ruby 3.3 の `shellwords.rb`: 1,617/1,617 |
| [Java](/docs/languages/java) | 検証済み | ファサード経由の JDK 21: 4,492/4,492 |
| [C](/docs/languages/c) | 検証済み | ファサード経由の glibc: 未定義動作のケースを 1 件除外して 1,859/1,859 |
| [PHP](/docs/languages/php) | プレビュー | `analyze` とアプリ全体のみ |
| [C#](/docs/languages/csharp) | プレビュー | `analyze` とアプリ全体のみ |

コマンドを含む完全な表は[ステータス](/docs/project/status)ページにあります。
