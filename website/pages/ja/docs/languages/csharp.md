---
title: C#（プレビュー）
description: C# はプレビュー版です。analyze は .csproj プロジェクトを読み取り、C# の CLI や ASP.NET サーバーは記録・リプレイして Rust のフロントドアの背後に置けます。
---

# C#（プレビュー）

| | |
| --- | --- |
| ユニット | `public static` メソッドのフォルダ |
| ブリッジ | 次に対応予定: C ABI 越しの P/Invoke |
| 現時点 | アプリ全体: CLI および ASP.NET サーバー（`dotnet <app>.dll`）向けの `app record`、`app replay`、フロントドア |
| ステータス | プレビュー: `analyze` とアプリ全体のみ。ユニットはまだブリッジできません |

`analyze` は `.csproj` プロジェクトとその NuGet パッケージ（クレートに対応付け）を読み取り、`using` ディレクティブをフォルダに解決し、C# の挙動が異なる点を指摘します: `decimal`、`dynamic`、LINQ の遅延クエリ、`Task`、`lock`、リフレクションです。

ビルドしたアプリを記録します:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

P/Invoke を通じたユニットの移植は次に対応予定です。それまでは、`bridge` は明確なメッセージとともに C# のユニットを拒否します。

CI は .NET をインストールしないので、C# のテストは .NET がインストールされたマシンでのみ実行されます。Kotlin と Swift はまだサポートしていません。必要な言語について Issue を開く（または既存のものに賛成票を入れる）ことができます。
