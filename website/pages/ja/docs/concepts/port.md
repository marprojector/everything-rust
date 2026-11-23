---
title: 移植
description: あなたかエージェントが書く唯一のファイル port.rs の書き方と、元の実装との一致を保つためのルール。
---

# 移植

移植版は、あなたかエージェントが `src/port.rs` に書きます。クレートのそれ以外はすべて生成されます。`port.rs` のシグネチャを変更したら、`everything-rust refresh <crate>` を実行してください。

## diff の手戻りを 1 回減らすルール

- コードを書く前に、計画がそのユニットについて挙げている各リスクを、その `file:line` で読んでください。
- **文字列のオフセット**: JS と Java は UTF-16 コードユニットで、Rust は UTF-8 バイトでインデックスします。生成された `src/utf16.rs` を使ってください。
- **エラー**: `Err("TypeName: message")` を返します。Python、Ruby、Java ではその例外に、Go では `error` のテキストになります。Rust の panic は記録されたエラーに決して一致しません。
- **Python と Ruby の数値**: 元の実装がそのまま受け渡す int は int として返さなければなりません。Python の移植では、int または float には `serde_json::Value` ではなく `crate::pynum::Num` を使ってください。
- **C**: `crash:` として記録された入力は未定義動作です。どんな結果でも合格します。
- **順序付きマップとオブジェクトのキー**: 順序が出力に影響する場合は `indexmap` を使います。
- **正規表現の先読み・後読みや後方参照**: `regex` ではなく `fancy-regex` を使います。いずれにしても、`\w`、`\b`、`\d` や大文字小文字を区別しないマッチは、非 ASCII テキストを元の言語のエンジンとは異なる扱いをします。元の実装の `\w` が ASCII のみの場合（`u` なしの JS、Java）は、`unicode(false)` を設定するか ASCII のクラス（`[A-Za-z0-9_]`）を明示的に書いてください。
- **`unsafe` は使わない。** 本当に必要な場合は最小限にとどめ、隔離し、`// SAFETY:` コメントを付けてください。
- **ヘルパーは非公開にする。** `pub` にするのはユニットのエクスポートだけです。`pub` のヘルパーはブリッジされる API の一部になってしまいます。
- 検討したケースについては `port.rs` に Rust のユニットテストを追加してください。

生成される `js.rs` には、Rust の挙動が異なる部分で JavaScript を再現するヘルパーがあります。例えば `js::number_to_string`（JS の `String(n)`）や `js::math_round`（`Math.round`）です。

## 例: 位置が 1 つずれた絵文字

[x18n](https://github.com/x18n-dev/x18n) の `packages/core/src/fuzzy.ts` を、
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo) で 2 通りの方法で移植しました:

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: ready, wasm-bindgen (core runs in browsers)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# src/port.rs を書く
everything-rust gate demo/fuzzy-rs
```

Rust の `char` でインデックスした最初の移植版は、簡単なテストには合格しましたが `diff` で失敗しました:

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

JS の `indexOf` は UTF-16 コードユニットで数えるので、絵文字は 2 つの位置になります。
`demo/*/src/port.rs` の移植版は生成された `utf16` ヘルパーを使い、2000/2000 ケースとゲートに合格しています。
napi-rs 版（`demo/fuzzy-napi`）と wasm-bindgen 版（`demo/fuzzy-rs`）の両方です。

everything-rust が x18n 全体について書いた計画: [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md)。
## どのくらいの大きさにすべきか

`gate` は移植版の行数を元の実装の行数と比較し、3 倍（インポートとシグネチャのために 50 行の余裕あり）を超えると失敗します。それを超えるのは、たいていクレートがすでにやっていることを手書きしているか、ケースを 1 つずつ処理しているからです。使うべきクレートは `analyze` が一覧表示します。
