---
title: TypeScript/JavaScript
description: napi-rs または wasm-bindgen を通じて、Node.js のコード（TypeScript または JavaScript）を Rust に移植します。元の API はそのままです。
---

# TypeScript/JavaScript

| | |
| --- | --- |
| ユニット | フォルダまたはファイル |
| ブリッジ | napi-rs または wasm-bindgen と自動生成のバイナリワイヤ。クラスと async にも対応 |
| 記録とリプレイの実行方法 | Node で `.ts` ソースを実行 |
| 必要なもの | Node 22.6+（`profile` には Node 24+ が必要） |
| ステータス | 検証済み |

## ブリッジの選び方

`analyze` は、ユニットまたはそのインポート先が Node を必要とする場合は napi-rs、ポータブルなコードには wasm-bindgen（ブラウザでも動作します）を選び、UI やエディターのコードは JS に残します。`bridge --kind napi|wasm` で上書きできます。

ブリッジを越えられないエクスポートは理由付きで TS に残り、シャドウモジュールは元の実装を再エクスポートします: 関数を受け取る・返す関数、ジェネレーターとイテレーター、RegExp、`instanceof` チェック、ジェネリクス（アプリがデータでしか埋めないものは `--as-json T` でブリッジできます）、そして型が関数を含む値です。[ブリッジ](/docs/concepts/bridge)を参照してください。

## TS をそのまま記録する

`record` は TS のソースを直接実行します: Node の resolve フックがバンドラー形式のインポートをたどり、リポジトリに何もインストールされていない場合は `--resolve-from` が npm パッケージを提供し、変換が必要な TS（enum、パラメータプロパティ）は `--experimental-transform-types` 付きで再試行されます。`record --from-tests -- npx jest` は、任意のテストランナーで、元の実装自身のテストスイートが行うすべての呼び出しを記録します。元の実装のカバレッジは TS/JS で計測されます（[カバレッジ](/docs/concepts/coverage)）。

## 移植版が正しく扱うべきこと

実際のコードで記録ケースによって見つかったもの:

- `str.length`、`indexOf`、`slice` は UTF-16 コードユニットで数えるので、絵文字は 2 つの位置になります。生成された `utf16` ヘルパーを使ってください。
- `u` フラグのない `/.../i` は ASCII 文字だけを大文字小文字同一視しますが、Rust の `regex` は Unicode で同一視するので、vercel/ms では `"1 ſ"`（長い s）やケルビン記号が秒としてパースされてしまうところでした。
- `Math.round` は 0.5 を切り上げます（`-2.5` は `-2`）。Rust の `round` はゼロから遠い方に丸めます。生成された `js.rs` には `js::math_round` があります。
- `${n}` は `1e+21`、`5e-324` を出力し、`-0` には `0` を出力します。`js::number_to_string` がこれを再現します。
- `String.prototype.trim` は Rust の `char::is_whitespace` ではなく、JS の空白文字を使います。

## 速度

呼び出しごとに、移植版は毎回ブリッジのコストを払い、V8 は小さな関数をナノ秒で実行します。vercel/ms は記録ケース全体で呼び出しごとに 0.56 倍でした。x18n のコアの移植版は、JS から呼ぶ場合 TS より遅くなります。バッチにすると、ループが Rust に移ります:

| ワークロード | 元の実装 | everything-rust 使用時 |
| --- | ---: | ---: |
| x18n のサイドバー検索、50,000 キー（Node） | 1 クエリあたり 10 ms | 0.5 ms: 20 倍高速 |
| 同じ検索をブラウザで実行（wasm、シングルスレッド） | 10 ms | 0.95 ms: 11.5 倍 |
| Fuse.js、50,000 文字列 | 91.2 ms | 2.8 ms: 32 倍 |
| vercel/ms `parse` x100,000 | 11.8 ms | 0.65 ms: 18 倍（受け渡しを含めると 3.4 倍） |
| vercel/ms `format` x100,000 | 2.1 ms | 受け渡しを含めて 2.5 ms: 0.8 倍 |

ケーススタディ: [json-server](/docs/real-world/json-server)、[Fuse.js](/docs/real-world/fuse-js)、[vercel/ms](/docs/real-world/vercel-ms)、[x18n](/docs/real-world/x18n)。
