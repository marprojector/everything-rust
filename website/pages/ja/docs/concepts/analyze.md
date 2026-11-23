---
title: プロファイルと分析
description: profile で時間がどこにかかっているかを見つけ、analyze でユニット、移植順序、リスク、クレートを計画します。
---

# プロファイルと分析

## プロファイル: 移す価値のあるものを見つける

目的が速度なら、何かを選ぶ前に計測してください:

```sh
everything-rust profile <repo> -- node bench.mjs      # または: python3 bench.py, ruby bench.rb,
                                                    #         java -cp out Main, go test -run X -bench . ./pkg
```

`profile` は実際のワークロードをその言語のプロファイラー（node、cProfile、`go test` の pprof、JFR、Ruby のトレーサー）で実行し、ホットな関数を包括時間順に、その呼び出し元、バッチ化すべきループとともに一覧表示します。Go は `go test` 経由でのみプロファイルでき、C には対応していません。

- 要素ごとに 1 回呼ばれる小さな関数は、それ単体で移植する価値は**ありません**: Rust への 1 回の呼び出しはその処理よりコストが高く、JIT（V8、JVM、Go）はそれをナノ秒で実行します。呼び出し元ごと移植するか、[バッチ化](/docs/concepts/batches)してください。どちらにすべきかはアドバイスの行が、正確なコマンド付きで示します。
- すでにコンパイルされているコード（numba の `@njit`、Cython、`@jax.jit`）はマシンコードとして実行されます: Rust に移植しても速度はほとんど上がりません。`profile` はこれを "Already compiled" の下に一覧表示します。

## 分析: 移植を計画する

```sh
everything-rust analyze <repo> -o plan.md        # 人間向け
everything-rust analyze <repo> --json            # エージェント向け: スキーマ everything-rust/analyze@1
everything-rust analyze <repo> --whole-app       # さらにエントリポイント、形態、依存関係、決定事項
```

`analyze` は、ユニット、移植順序、`file:line` 付きのリスク、ユニットごとのブリッジ、各依存関係がクレートにどの程度対応付くか、そして `start_here`（最初に移植するのに最適なもの）を一覧表示します。`--whole-app` は、アプリの形態（CLI、サーバー、ライブラリ）とエントリポイント、すべての依存関係（インポートされたものと宣言されたもの）とそのクレート、ピース（互いにインポートし合うユニットはまとめて移動する）、推奨回答付きの番号付き決定事項を追加します。[アプリ全体の移植](/docs/whole-apps/overview)を参照してください。

- パッケージ（`package.json`）とソースファイル。`node_modules`、ビルド出力、フィクスチャ、
  プレイグラウンド、ツールの設定はスキップします。Vue/Svelte/Astro の `<script>` ブロックはインポートを解析します。
- **ユニット**: ソースファイルのフォルダ、またはパッケージのソースルートにあるファイル。
- **ウェーブ**: ユニットのインポート先はそれより前のウェーブに入ります。インポートの循環はまとめて移植されます。
- **ブリッジ**: ユニットまたはそのインポート先が Node を必要とする場合は napi-rs、ポータブルな
  コードには wasm-bindgen、UI やエディターのコードは JS のまま残します。
- **準備度**（どれだけ安全か）と**優先度**（安全で価値があるか: ループと文字列処理、テスト、サイズ）。

| リスク | 深刻度 | 例 |
| --- | --- | --- |
| 文字列のオフセット | high | `slice`、`indexOf`、`matchAll`、`d` フラグ: UTF-16 と UTF-8 の違い |
| 正規表現の構文 | high | 先読み・後読み、後方参照、`y`: `regex` クレートにはない |
| 動的な正規表現 | high | `new RegExp(userPattern)`: 実行時の JS 構文 |
| 動的なコード | high | `eval`、`new Function`、計算された `import()` |
| 動的型付け | medium | `any`、`'key' in value` |
| 順序付きコレクション | medium | `Map`、`Set`、`Object.keys`: 挿入順 |
| 非同期、ジェネレーター | medium | Promise とジェネレーターの境界 |
| クラス継承、リフレクション | medium | `extends`、`Proxy`、`Reflect`、`Symbol` |
| コールバック形式の API | medium | 関数型の引数とメソッド: やり取りの多いブリッジ |
| 例外、数値、JSON | low | `throw`、ビット演算、`JSON.stringify` の順序 |
## 依存関係のクレート

`analyze` は、コードがインポートする各ライブラリに対応する Rust クレートを、`good`、`partial`、`none` の適合度付きで示します。テーブルにないライブラリは `unknown` と報告されます。テーブルがクレートを推測することはありません。追加するには[コントリビュート](/docs/project/contributing#add-a-crate-mapping)を参照してください。
