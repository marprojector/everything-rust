---
title: 記録とリプレイ
description: app record はアプリ全体をドキュメント記載の例、あなたの入力、トラップで実行し、app replay は任意のビルドをケースに対して実行します。
---

# 記録とリプレイ

`record` と `diff` は 1 つのユニットを証明し、`app record` と `app replay` はアプリを証明します。6 つの言語いずれの CLI とサーバーにも対応しています。アプリはブラックボックスとして扱われます。

```sh
everything-rust app record . -- python3 cli.py          # 元の実装を実行するコマンド
everything-rust app replay .                            # 元の実装は 100% 合格しなければならない
everything-rust app replay . --build "target/release/mytool"
```

## ケースの出どころ

`app record` は次の入力で元の実装を実行します:

- アプリ自身のドキュメントにある例: Markdown やシェルファイル内の `$ mytool ...` の行と、そこで指定されたファイル。
- `--inputs` の JSONL ファイル。1 行に 1 つの `{"name", "args", "stdin", "env", "files"}`。
- それらのケースから派生したセマンティックトラップ: 非 ASCII や絵文字の文字列、負数の除算と剰余、オーバーフロー、非常に大きい・小さい浮動小数点数、マップの順序、存在しないファイル。

各ケースは 2 回実行され、2 回の実行で異なった部分は `[..]` ワイルドカードになります。`--normalize timestamps|uuids|key-order` や `--normalize name=regex` でノーマライザーを追加でき、ケースと一緒に保存されます。`app record` は異なった部分を種類別に報告します: 値（ワイルドカードになったもの）、順序（順序を問わず比較される行）、行（何にでも一致する `...`）です。すべて確認してください。

## ケースは普通のファイル

ケースは `rust/app-cases/` 内の [trycmd](https://docs.rs/trycmd) ファイルです:

| ファイル | 内容 |
| --- | --- |
| `<case>.toml` | argv、環境変数、終了コード |
| `<case>.stdin` | 標準入力 |
| `<case>.stdout`, `<case>.stderr` | 元の実装が出力したもの |
| `<case>.in/` | 入力ファイル |
| `<case>.out/` | 書き込まれたファイル |

手で読んだり編集したりできます。移植版がバイナリを持てば、Rust のテストから `trycmd` で実行できます。

## リプレイ

- `app replay` は任意のビルドをケースに対して実行し、`passed/total` を表示し、失敗したケースごとの差分を含む `rust/app-cases/replay-report.md` を書き出し、不一致があれば 0 以外で終了します。
- [フロントドア](/docs/whole-apps/front-door)のように、`EVERYTHING_RUST_FALLBACK_LOG` で指定されたファイルに追記する（元の実装へのフォールバックごとに 1 行）ビルドは、フォールバック率も報告されます。
- `app replay --case <name>` は、レポートが付けた名前で 1 つのケースを再実行します: すべてを再実行する前に不一致を直すための、速くて再現性のあるループです。

## CI でのリプレイ

レポートには実行時間や一時パスが含まれません: 同じビルドを 2 回リプレイすると同じファイルが書き出されます。それをコミットし、CI で次を実行してください:

```sh
everything-rust app replay .
git diff --exit-code rust/app-cases/replay-report.md
```

こうすると、移植途中のアプリは、合格するものに変化があれば（良くなっても悪くなっても）、レポートを更新するまで CI で失敗します。

## 数値の出力

`app record --float-tolerance 1e-9` を使うと、数値は大きい方の値に対するその割合の範囲内で一致とみなされます。[浮動小数点の許容誤差](/docs/concepts/float-tolerance)を参照してください。
