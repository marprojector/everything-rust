---
title: 批次與熱路徑
description: 每個項目呼叫一次的移植版本會輸給 JIT。批次會把整個迴圈搬進 Rust 並使用所有核心，即使在單次呼叫移植落敗的地方也能勝出。
---

# 批次與熱路徑

```sh
everything-rust bridge <repo> <unit> --batch fn:param    # 或：everything-rust refresh <crate> --batch fn:param
```

`--batch` 會產生一個類別，一次接收 `param` 的值並將它們保留在 Rust 中：`new FnBatch(items)`（TS、Python、Java）、`Mod::FnBatch.new(items)`（Ruby）、`NewFnBatch(items)`（Go）。每次查詢時，`run(<other args>)` 會以一個陣列回傳所有結果，而 `rank(<other args>, min)` 會回傳項目索引，最好的排在前面。它使用所有核心執行，錯誤則與原始版本相同。除了 C 以外，每種語言都支援批次。

每個項目呼叫一次的移植版本，每個項目都要付出一次橋接成本，因此會輸給 JIT。勝出的方法是
搬移迴圈：`profile` 會找出它，而 `--batch` 會產生一個類別，一次接收所有項目，
並在每次呼叫時對所有項目執行該函式，使用所有核心，以 typed array 回傳結果。

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore：佔執行時間的四分之一，由 `matched` 對每個 key 呼叫：做批次處理
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

`port.rs` 中的 `FuzzyScoreIndex` 結構會以另一個迴圈取代預設迴圈，先一次準備好所有
項目，而且能自行排名（`rank`）：它會保留轉為小寫的 key 以及每個 key 的字元遮罩、
一張記錄每個 key 中每個常見字元首次出現位置的表格（單一字母的查詢就是一欄），以及上一次查詢的符合結果（在 `s` 之後輸入 `se` 只會重新計分 `s` 符合的項目）。在
x18n 側邊欄對 50,000 個 key 的搜尋上，逐字母輸入，結果完全相同：

| | 每次查詢 | 相對於 TS |
| --- | ---: | ---: |
| TS：`map(fuzzyScore)`、filter、sort | 10 ms | 1x |
| 使用與索引相同縮減方式的 TS | 4.1-5.4 ms | 2x |
| 相同的移植版本，對每個 key 呼叫 | - | 0.2-0.4x |
| `FuzzyScoreBatch`，預設索引（4 核心） | 5.1 ms | 1.7x |
| `FuzzyScoreBatch` 搭配 `FuzzyScoreIndex`（napi，4 核心） | 0.45-0.56 ms | **18-22x** |
| 相同但使用 wasm（單一執行緒） | 0.95 ms | **11.5x** |

與做相同縮減的 TS 相比，Rust 仍然快 7-12x（wasm 為 5x）。請用一般的迴圈將回傳的
索引轉回 key：`Array.from(order, i => keys[i])` 的成本高了 20x，0.4 ms。

每種語言都支援批次，而且即使在單次呼叫移植落敗的地方，批次也能勝過原始版本。
在 4 核心上執行相同的工作，結果與錯誤完全相同（基準測試位於各個 demo 資料夾）：

| 語言 | 工作負載 | 原始版本 | 單次呼叫移植相對於原始版本 | 批次相對於原始版本 |
| --- | --- | ---: | ---: | ---: |
| TS | x18n 搜尋，50,000 個 key | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`，20,000 個字串 | 98 ms | - | **17x** |
| Ruby | `slugify`，20,000 個字串 | 110 ms | - | **12.7x** |
| Ruby | `shellescape`（標準函式庫），50,000 個混合值 | 88 ms | - | **11.8x** |
| Go | `Slugify`，50,000 個字串 | 49 ms | 0.93x | **4x** |
| Java | `slugify`，50,000 個字串 | 43 ms | 0.68x | **3x** |

編譯式的原始版本受益較少：每個項目的工作本來就很快，所以批次勝出的幅度約等於
核心數。批次 crate 使用 mimalloc 配置記憶體：使用系統配置器時，Go 的批次甚至比
單一執行緒還慢。Demo：[`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md)。
## 撰寫快速的索引

- 讓呼叫端只建立一次批次，而不是每次查詢都建立。用一般的迴圈將回傳的索引對應回項目。
- 若要一次準備好所有項目（轉小寫、遮罩、位置表），請在 `port.rs` 中定義 `pub struct FnIndex`，並提供 `new(items)` 與 `run(<other args>)`。它必須逐項給出與該函式相同的結果。
- 當排名可以省略部分工作時，為索引提供自己的 `rank(<other args>, min)`。它必須與對函式結果排名的方式一致：分數由高到低，同分時依項目順序。
- 批次會讓移植版本的單項速度乘上核心數。面對編譯式的原始版本（Go、Java），移植版本本身必須夠快：處理 ASCII 時不要查 Unicode 表、將每個結果建立在一個預先配置大小的 `String` 中、避免中間副本。
- 在 Go 與 Java 中，使用完畢後要關閉批次（finalizer 只是備援）。
