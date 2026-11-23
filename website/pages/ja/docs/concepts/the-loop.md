---
title: 移植ループ
description: profile、analyze、bridge、record、port、diff、bench、shadow、gate。すべてのユニットが通るステップ。
---

# 移植ループ

1 つのホットな関数からアプリ全体まで、すべての移植は同じステップを 1 ユニットずつ通ります。移植が完了するのは `gate` に合格したときだけです。

everything-rust が移植をどう検証するか、各コマンドが何を判断するか、そして限界がどこにあるか。[README](/) に戻る。

## 仕組み

```
profile ──> analyze ──> bridge ──> record ──> port.rs ──> diff ──> bench ──> shadow ──> gate
 where the    what to     the Rust   what the    you or      every     faster?    both side   done means
 time goes    port first  crate and  original    your agent  recorded  the        by side in  this passes
                          wrapper    does        write it    case      verdict    production
```

元の実装が常に正解の基準です: `record` はエッジケース（とあなた自身の入力）で元の実装を実行し、
返したもの、または投げたものを保存します。`diff` はすべてのケースを Rust に対してリプレイします。すべて一致して
`gate` に合格するまで何も出荷されません。呼び出し元はその間ずっと元の API を使い続けます。

アプリ全体も同じループをユニットごとに通りますが、それはプログラム全体を証明する枠組みの中で行われます
（[アプリ全体の移植](/docs/whole-apps/overview)）:

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop above, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in         pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down      proved by
 decisions to make      outside         original answers                                                replay
```

## 移植ループ

| ステップ | コマンド | 内容 |
| --- | --- | --- |
| 0. プロファイル | `everything-rust profile <repo> -- node bench.mjs` | 実際のワークロードをその言語のプロファイラー（node、cProfile、`go test` の pprof、JFR、Ruby のトレーサー）で実行: ホットな関数を包括時間順に、その呼び出し元、バッチ化すべきループとともに示す。すでにコンパイルされているコード（numba、Cython、JAX）は Rust で得るものが少ないと指摘する |
| 1. 計画 | `everything-rust analyze <repo>` | ユニット、移植順序、`file:line` 付きのリスク、ユニットごとのブリッジ、依存関係とクレートの適合度、最初に移植するのに最適なもの。`--whole-app` はエントリポイントと形態、すべての依存関係（インポートされたものと宣言されたもの）とそのクレート、ピース（互いにインポートし合うユニットはまとめて移動する）、推奨回答付きの番号付き決定事項を追加する |
| 2. ブリッジ | `everything-rust bridge <repo> <unit>` | バインディング（`lib.rs` と、元の言語が読み込むラッパー）と移植用のスタブ（`port.rs`）を持つクレート。`--batch fn:param` は多数の `param` の値を Rust 側に保持するクラスを追加する。`--as-json T` はアプリがデータでしか埋めない型パラメータを JSON 値としてブリッジする |
| 3. 記録 | `everything-rust record <crate>` | 生成したエッジケース（非 ASCII、絵文字、`-0`、`NaN`、float が期待される場所での int など）と、すべての引数に対するすべてのセマンティックトラップ（文字列の長さと大文字小文字、符号付き除算、オーバーフローの境界、浮動小数点数の表示、マップとソートの順序）で元の実装を実行し、`cases.jsonl` を保存する。`--from-tests -- npx jest`（または `-- pytest`）は元の実装自身のテストスイートが行うすべての呼び出しも記録する。`--float-tolerance 1e-9` は数値計算のコードで浮動小数点数をその割合の範囲内で一致とみなす |
| 3. ビルド | `everything-rust build <crate>` | Rust 側をビルドする: `.node`、wasm パッケージ、Python 拡張、または共有ライブラリ |
| 3. 差分 | `everything-rust diff <crate>` | 記録ケースを、元の実装の呼び出し元と同じ呼び方で Rust に対してリプレイし、不一致を一覧表示する |
| 3. ベンチマーク | `everything-rust bench <crate>` | 記録ケースを元の実装と Rust でブリッジ込みで計測する |
| 4. シャドウ | `everything-rust shadow <crate>` | 両方を実行し、元の実装の結果を返し、不一致を報告する差し替え用モジュール。`EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. ゲート | `everything-rust gate <crate>` | fmt、clippy `-D warnings`、テスト、`unsafe` の監査、未移植のものがないこと、元の実装のすべてのエラーパスについて失敗する記録ケースがあること、元の実装がエラーを返す箇所に `unwrap`/`expect`/`panic!` がないこと（`// INFALLIBLE: <why>` が付いている場合を除く）、ビルド、API、挙動 |
| エージェント | `everything-rust setup` | MCP サーバー（`everything-rust mcp`、すべてのステップをツールとして提供）とワークフロースキル（[`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)）をコーディングエージェントに登録する |

## ホットパス: データを Rust 側に保持する

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

## `bridge` が判断すること

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

## `analyze` が見ること

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

## 検出できないもの

合格とは、Rust が記録されたすべてのケースに一致するということであり、それ以上ではありません。書き直しで問題になるのは
たいてい記録の外にあるものなので、限界がどこにあるかを知っておいてください:

- **誰も記録していない入力。** 生成されたケース、トラップ、元の実装のテストスイート
  （`record --from-tests`）、そしてあなた自身の `--inputs` が上限です。実際の入力（本番環境の
  サンプル、バグ報告）を追加してください。本番を壊すユーザー入力は、ジェネレーターが思いつくものではまずありません。
- **先に進んでしまった元の実装。** `record` はユニットのファイルのフィンガープリントを保存します
  （`recorded-from.json`）。後で元の実装が変わると、元の実装の以前の挙動に対して合格させるのではなく、`diff` が警告し、
  再度記録するまで `gate` が失敗します。
- **正規表現。** `regex` クレートには先読み・後読みや後方参照がなく、コンパイルできるパターンでも、
  非 ASCII テキストに対する `\w`、`\b`、`\d` や大文字小文字を区別しないマッチの扱いが
  JS、Python、Java と異なることがあります。`fancy-regex` に差し替えたりパターンを書き直したりすると、記録された
  すべてのケースに合格しても、トルコ語のドットなし `ı` やゼロ幅接合子で異なる結果になることがあります。`record` は
  正規表現を使うファイル内のすべての関数に正規表現トラップ（上記のもの、ケルビン記号、長い s、アラビア・インド数字、行区切り文字、語末シグマ）
  を追加しますが、実際のパターンを実際のテキストでファズテストすることもしてください。
- **実行ごとに変わるもの。** `record` は元の実装を 2 回実行し、異なった部分を分類します:
  変わった値（時刻、ID）はワイルドカードになります。同じ行が別の順序で出る場合（ハッシュマップ、
  並行処理）は順序を問わず比較されるので、移植版はそれでもまさにその行を出力しなければなりません。
  行数が異なる場合は `...` になり、何にでも一致します。`record` はそれぞれの種類を一覧表示します:
  ハッシュ順や並行処理による違いは、移植版で実際に変わりうるものなので、すべて確認してください。
- **浮動小数点数。** 数値からテキストへの変換は言語によって異なります（`1e21`、`-0`、桁数）。
  トラップにはこれらの値が含まれており、完全に一致しなければなりません。`--float-tolerance` が許容するのは
  丸めだけで、フォーマットは決して許容しません。
- **出力に現れないもの。** 実行時間、メモリ、負荷時の並行処理、そして stdout、stderr、終了コード、
  書き込まれたファイル、HTTP レスポンス以外の副作用（ネットワーク呼び出し、データベース、
  シグナル）は比較されません。
- **他のプラットフォーム。** 記録が証明するのは、それを実行したマシン上での挙動です。結果が型のサイズ、
  エンディアン、コンパイラに依存する C は、ターゲット上で異なる挙動をすることがあります。そのため `analyze` は
  ハードウェアに近い C は C のまま残します。
## すべてのステップ

| ステップ | コマンド | 内容 |
| --- | --- | --- |
| 0. プロファイル | `everything-rust profile <repo> -- node bench.mjs` | 実際のワークロードをその言語のプロファイラー（node、cProfile、`go test` の pprof、JFR、Ruby のトレーサー）で実行: ホットな関数を包括時間順に、その呼び出し元、バッチ化すべきループとともに示す。すでにコンパイルされているコード（numba、Cython、JAX）は Rust で得るものが少ないと指摘する |
| 1. 計画 | `everything-rust analyze <repo>` | ユニット、移植順序、`file:line` 付きのリスク、ユニットごとのブリッジ、依存関係とクレートの適合度、最初に移植するのに最適なもの。`--whole-app` はエントリポイントと形態、すべての依存関係（インポートされたものと宣言されたもの）とそのクレート、ピース（互いにインポートし合うユニットはまとめて移動する）、推奨回答付きの番号付き決定事項を追加する |
| 2. ブリッジ | `everything-rust bridge <repo> <unit>` | バインディング（`lib.rs` と、元の言語が読み込むラッパー）と移植用のスタブ（`port.rs`）を持つクレート。`--batch fn:param` は多数の `param` の値を Rust 側に保持するクラスを追加する。`--as-json T` はアプリがデータでしか埋めない型パラメータを JSON 値としてブリッジする |
| 3. 記録 | `everything-rust record <crate>` | 生成したエッジケース（非 ASCII、絵文字、`-0`、`NaN`、float が期待される場所での int など）と、すべての引数に対するすべてのセマンティックトラップ（文字列の長さと大文字小文字、符号付き除算、オーバーフローの境界、浮動小数点数の表示、マップとソートの順序）で元の実装を実行し、`cases.jsonl` を保存する。`--from-tests -- npx jest`（または `-- pytest`）は元の実装自身のテストスイートが行うすべての呼び出しも記録する。`--float-tolerance 1e-9` は数値計算のコードで浮動小数点数をその割合の範囲内で一致とみなす |
| 3. ビルド | `everything-rust build <crate>` | Rust 側をビルドする: `.node`、wasm パッケージ、Python 拡張、または共有ライブラリ |
| 3. 差分 | `everything-rust diff <crate>` | 記録ケースを、元の実装の呼び出し元と同じ呼び方で Rust に対してリプレイし、不一致を一覧表示する |
| 3. ベンチマーク | `everything-rust bench <crate>` | 記録ケースを元の実装と Rust でブリッジ込みで計測する |
| 4. シャドウ | `everything-rust shadow <crate>` | 両方を実行し、元の実装の結果を返し、不一致を報告する差し替え用モジュール。`EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. ゲート | `everything-rust gate <crate>` | fmt、clippy `-D warnings`、テスト、`unsafe` の監査、未移植のものがないこと、元の実装のすべてのエラーパスについて失敗する記録ケースがあること、元の実装がエラーを返す箇所に `unwrap`/`expect`/`panic!` がないこと（`// INFALLIBLE: <why>` が付いている場合を除く）、ビルド、API、挙動 |
| エージェント | `everything-rust setup` | MCP サーバー（`everything-rust mcp`、すべてのステップをツールとして提供）とワークフロースキル（[`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)）をコーディングエージェントに登録する |
## 次へ

各ステップには専用のページがあります: [プロファイルと分析](/docs/concepts/analyze)、[ブリッジ](/docs/concepts/bridge)、[記録](/docs/concepts/record)、[移植](/docs/concepts/port)、[差分](/docs/concepts/diff)、[ベンチマーク](/docs/concepts/bench)、[シャドウ](/docs/concepts/shadow)、[ゲート](/docs/concepts/gate)。
