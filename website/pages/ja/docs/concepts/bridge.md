---
title: ブリッジ
description: bridge は 1 つのユニットのための Rust クレートを、バインディングとラッパー付きで生成します。呼び出し元は元の API をそのまま使えます。
---

# ブリッジ

```sh
everything-rust bridge <repo> <unit-id>          # <repo>/rust/<name> にクレートを作成。上書きはしない
everything-rust refresh <crate>                  # port.rs のシグネチャを変更した後に実行
```

`bridge` は、バインディング（`lib.rs` と、元の言語が読み込むラッパー）と移植用のスタブ（`port.rs`）を持つクレートを作成します。移植版は `src/port.rs` にだけ書きます。`src/lib.rs` とラッパーは生成されるもので、`refresh` が `port.rs` のシグネチャから再生成し、新しいバッチクラスも追加します。`port.rs` には一切触れません。

オプション:

- `--batch fn:param` は、多数の `param` の値を Rust 側に保持するクラスを追加します。[バッチ](/docs/concepts/batches)を参照してください。
- `--as-json T` は、アプリがデータでしか埋めない型パラメータ（`paginate<T>(items: T[])` の `T`）を JSON 値としてブリッジします。
- `--keep "name: reason"` は、さらに多くのエクスポートを元の言語に残します。例えば `Intl` のデータに依存するコードです。
- `--kind` は計画が選んだブリッジを上書きします（TypeScript では napi-rs または wasm）。

## 言語ごとのブリッジ

| 言語 | ブリッジ |
| --- | --- |
| TypeScript/JavaScript | napi-rs または wasm-bindgen と自動生成のバイナリワイヤ。クラスと async にも対応 |
| Python | PyO3（abi3）、直接変換、型付き例外 |
| C | 元の名前を持つ `extern "C"` 関数とヘッダ |
| Go | 型付き cgo 呼び出しと自動生成のバイナリワイヤ。元の API を持つパッケージとして提供 |
| Ruby | 元のモジュール配下に置かれる magnus 拡張 |
| Java | 型付き JNI と自動生成のバイナリワイヤ。元のメソッドを持つラッパークラスとして提供 |

PHP と C# のユニットはまだブリッジできません。[PHP](/docs/languages/php) と [C#](/docs/languages/csharp) を参照してください。

## bridge が判断すること

- ブリッジを越えられないエクスポートは理由付きで元の言語に残り、シャドウ
  モジュールは元の実装を再エクスポートします。TS の場合:
  関数を受け取る・返す関数、ジェネレーターとイテレーター、RegExp、`instanceof` チェック、
  ジェネリクス（アプリがデータでしか埋めないものは `--as-json T` でブリッジできます）、そして型（インポート、再エクスポート、`extends` をたどったもの）が
  関数を含む値です。`--keep "name: reason"` でさらに残せます。例えば `Intl` のデータに依存するコードです。
- TS の型は Rust の型に対応付けられ、型エイリアスはインポートをたどって解決され、引数の型は
  デフォルト値から推論されます。構造化された値は自動生成のバイナリワイヤで境界を越えます（ワイヤが扱わない形の場合のみ
  フォールバックとして JSON）。
- JSON は `undefined`、NaN、Infinity、-0、孤立サロゲートを表現できません: 構造化された値は
  マーカー（`{"$undefined": true}`、`{"$num": "NaN"}`、`{"$utf16": [...]}`）を持ち、生成された
  `index.cjs` がそれを変換します。ヘルパーはクレートの `js.rs` にあります。

`record` は TS のソースをそのまま実行します: Node の resolve フックがバンドラー形式のインポート
（`./errors` から `./errors.ts`）をたどり、リポジトリに何もインストールされていない場合は `--resolve-from` が npm パッケージを提供し、
変換が必要な TS（enum、パラメータプロパティ）は
`--experimental-transform-types` 付きで再試行されます。
