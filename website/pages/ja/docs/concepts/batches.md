---
title: バッチとホットパス
description: 要素ごとに 1 回呼ばれる移植版は JIT に負けます。バッチはループ全体を Rust に移して全コアで実行し、呼び出しごとの移植版が負ける場面でも勝ちます。
---

# バッチとホットパス

```sh
everything-rust bridge <repo> <unit> --batch fn:param    # または: everything-rust refresh <crate> --batch fn:param
```

`--batch` は、`param` の値を一度だけ受け取って Rust 側に保持するクラスを生成します: `new FnBatch(items)`（TS、Python、Java）、`Mod::FnBatch.new(items)`（Ruby）、`NewFnBatch(items)`（Go）。クエリごとに、`run(<other args>)` はすべての結果を 1 つの配列で返し、`rank(<other args>, min)` は要素のインデックスを良い順に返します。全コアで実行され、エラーは元の実装と同じです。C 以外のすべての言語でバッチが使えます。

要素ごとに 1 回呼ばれる移植版は要素ごとに 1 回ブリッジのコストを払い、JIT に負けます。勝つのは
ループを移すことです: `profile` がループを見つけ、`--batch` が、要素を一度だけ受け取り、
呼び出しのたびに全要素に対して関数を全コアで実行し、型付き配列で結果を返すクラスを生成します。

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore: 実行時間の 4 分の 1、`matched` からキーごとに呼ばれている: バッチ化する
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

`port.rs` の `FuzzyScoreIndex` 構造体は、デフォルトのループを、要素を一度だけ準備するものに置き換え、
自分でランク付け（`rank`）もできます: キーを小文字化してそれぞれに文字マスクを付けて保持し、
各キーについてよく使われる文字それぞれの最初の位置のテーブル（1 文字のクエリは 1
列）と、直前のクエリの一致（`s` の後に `se` と入力すると、`s` に一致したものだけを再スコアリング）を保持します。
x18n のサイドバー検索で 50,000 キーに対して 1 文字ずつ入力した場合、結果は同一で:

| | 1 クエリあたり | TS 比 |
| --- | ---: | ---: |
| TS: `map(fuzzyScore)`、filter、sort | 10 ms | 1x |
| インデックスと同じ絞り込みをした TS | 4.1-5.4 ms | 2x |
| 同じ移植版をキーごとに呼び出し | - | 0.2-0.4x |
| `FuzzyScoreBatch`、デフォルトのインデックス（4 コア） | 5.1 ms | 1.7x |
| `FuzzyScoreBatch` と `FuzzyScoreIndex`（napi、4 コア） | 0.45-0.56 ms | **18-22x** |
| 同じものを wasm で（シングルスレッド） | 0.95 ms | **11.5x** |

同じ絞り込みをした TS と比べても、Rust はなお 7-12 倍高速です（wasm は 5 倍）。返されたインデックスは
単純なループでキーに変換してください: `Array.from(order, i => keys[i])` は 20 倍のコスト、0.4 ms かかりました。

すべての言語でバッチが使え、呼び出しごとの移植版が負ける場面でも元の実装に勝ちます。
4 コアで同じ処理を行い、結果とエラーは同一です（ベンチマークは各デモフォルダにあります）:

| 言語 | ワークロード | 元の実装 | 呼び出しごとの移植版 vs 元の実装 | バッチ vs 元の実装 |
| --- | --- | ---: | ---: | ---: |
| TS | x18n の検索、50,000 キー | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`、20,000 文字列 | 98 ms | - | **17x** |
| Ruby | `slugify`、20,000 文字列 | 110 ms | - | **12.7x** |
| Ruby | `shellescape`（標準ライブラリ）、50,000 個の混在値 | 88 ms | - | **11.8x** |
| Go | `Slugify`、50,000 文字列 | 49 ms | 0.93x | **4x** |
| Java | `slugify`、50,000 文字列 | 43 ms | 0.68x | **3x** |

コンパイル型の元の実装では効果が小さくなります: 要素ごとの処理がすでに速いので、バッチの効果はほぼコア数分です。
バッチのクレートは mimalloc でメモリを確保します: システムのアロケーターでは、Go のバッチは
シングルスレッドより遅くなりました。デモ: [`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md)。
## 速いインデックスの書き方

- 呼び出し元にはバッチをクエリごとではなく一度だけ作らせてください。返されたインデックスは単純なループで要素に戻します。
- 要素を一度だけ準備する（小文字化、マスク、位置テーブル）には、`port.rs` に `new(items)` と `run(<other args>)` を持つ `pub struct FnIndex` を定義します。要素ごとに関数と同じ結果を返さなければなりません。
- ランク付けで処理を省ける場合は、インデックスに独自の `rank(<other args>, min)` を持たせます。関数の結果をランク付けしたものと一致しなければなりません: スコアの高い順、同点は要素の順です。
- バッチは移植版の要素ごとの速度をコア数倍にします。コンパイル型の元の実装（Go、Java）に対しては、移植版自体が速くなければなりません: ASCII は Unicode テーブルなしで処理し、各結果は事前にサイズを確保した 1 つの `String` で組み立て、中間コピーを避けてください。
- Go と Java では、使い終わったらバッチを閉じてください（ファイナライザーはフォールバックにすぎません）。
