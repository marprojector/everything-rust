---
title: 工作循環
description: profile、analyze、bridge、record、port、diff、bench、shadow 與 gate。每個單元都會經過的步驟。
---

# 工作循環

每一次移植，從單一熱點函式到整個應用程式，都會一次一個單元地經過相同的步驟。只有在 `gate` 通過時，移植才算完成。

everything-rust 如何檢查移植版本、它的指令會決定什麼，以及它的極限在哪裡。回到 [README](/)。

## 運作方式

```
profile ──> analyze ──> bridge ──> record ──> port.rs ──> diff ──> bench ──> shadow ──> gate
 where the    what to     the Rust   what the    you or      every     faster?    both side   done means
 time goes    port first  crate and  original    your agent  recorded  the        by side in  this passes
                          wrapper    does        write it    case      verdict    production
```

原始版本始終是唯一可信來源：`record` 會以邊界案例（以及你自己的輸入）執行它，
並儲存它回傳的值或擲出的例外；`diff` 會將每個案例拿來重播比對 Rust 版本；在全部一致且
`gate` 通過之前，什麼都不會出貨。呼叫端自始至終都使用原本的 API。

整個應用程式也會逐一單元地經過相同的工作循環，並置於一個證明整個
程式正確的框架之中（[移植整個應用程式](/docs/whole-apps/overview)）：

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop above, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in         pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down      proved by
 decisions to make      outside         original answers                                                replay
```

## 工作循環

| 步驟 | 指令 | 作用 |
| --- | --- | --- |
| 0. 效能剖析 | `everything-rust profile <repo> -- node bench.mjs` | 在該語言的效能剖析工具（node、cProfile、`go test` pprof、JFR、Ruby tracer）下執行真實的工作負載：依包含時間列出熱點函式、它們的呼叫端，以及哪些迴圈該做批次處理；並標出已經編譯過的程式碼（numba、Cython、JAX），這類程式碼用 Rust 改寫幾乎沒有好處 |
| 1. 規劃 | `everything-rust analyze <repo>` | 單元、移植順序、附 `file:line` 的風險、每個單元的橋接方式、相依套件對應 crate 的適配程度、最適合先移植的部分；`--whole-app` 會加上進入點與型態、每個相依套件（已匯入或已宣告）及其 crate、區塊（互相匯入的單元會一起搬移），以及附有建議答案的編號決策 |
| 2. 橋接 | `everything-rust bridge <repo> <unit>` | 建立包含綁定（`lib.rs`，以及來源語言載入的包裝層）與待移植空殼（`port.rs`）的 crate；`--batch fn:param` 會新增一個將大量 `param` 值保留在 Rust 中的類別；`--as-json T` 會把應用程式只用資料填入的型別參數以 JSON 值的形式橋接 |
| 3. 錄製 | `everything-rust record <crate>` | 以自動產生的邊界案例（非 ASCII、emoji、`-0`、`NaN`、預期浮點數的地方傳入整數……）以及每個參數上的每個語意陷阱（字串長度與大小寫、有號除法、溢位邊界、浮點數輸出、map 與排序的順序）執行原始版本，並儲存 `cases.jsonl`；`--from-tests -- npx jest`（或 `-- pytest`）還會錄製原始版本自己的測試套件所做的每一次呼叫；`--float-tolerance 1e-9` 讓數值程式碼的浮點數在該比例內即視為一致 |
| 3. 建置 | `everything-rust build <crate>` | 建置 Rust 端：`.node`、wasm 套件、Python 擴充或共用函式庫 |
| 3. 比對 | `everything-rust diff <crate>` | 以原始版本呼叫端的呼叫方式，將已錄製的案例拿來重播比對 Rust；列出不一致之處 |
| 3. 基準測試 | `everything-rust bench <crate>` | 在原始版本與 Rust 上（包含橋接）為已錄製的案例計時 |
| 4. 影子模式 | `everything-rust shadow <crate>` | 可直接替換的模組，同時執行兩者，回傳原始版本的結果，回報不一致之處；`EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. 閘門 | `everything-rust gate <crate>` | fmt、clippy `-D warnings`、測試、`unsafe` 稽核、沒有未移植的部分、原始版本的每條錯誤路徑都有已錄製的失敗案例、在原始版本會回傳錯誤的地方不使用 `unwrap`/`expect`/`panic!`（除非標記了 `// INFALLIBLE: <why>`）、建置、API、行為 |
| AI 代理 | `everything-rust setup` | 在你的程式開發 AI 代理中註冊 MCP 伺服器（`everything-rust mcp`，每個步驟都是一個工具）與工作流程技能（[`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)） |

## 熱路徑：把資料留在 Rust 中

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

## `bridge` 會決定什麼

- 無法跨越橋接的匯出會留在來源語言中並附上原因，而影子
  模組會重新匯出原始版本。以 TS 為例：
  接收或回傳函式的函式、generator 與迭代器、RegExp、`instanceof` 檢查、
  泛型（`--as-json T` 可橋接應用程式只用資料填入的泛型），以及型別（沿著匯入、重新匯出與 `extends` 追蹤）中含有
  函式的值。`--keep "name: reason"` 可以保留更多，例如依賴 `Intl` 資料的程式碼。
- TS 型別會對應到 Rust 型別，型別別名會沿著匯入解析，而參數型別會
  從預設值推斷；結構化的值以自動產生的二進位線路協定跨越（只有線路協定無法建模的形狀
  才會回退使用 JSON）。
- JSON 無法表示 `undefined`、NaN、Infinity、-0 或孤立代理字元：結構化的值會帶有
  標記（`{"$undefined": true}`、`{"$num": "NaN"}`、`{"$utf16": [...]}`），由自動產生的
  `index.cjs` 轉換，輔助函式則位於 crate 的 `js.rs` 中。

`record` 會直接執行 TS 原始檔：Node 的 resolve hook 會跟隨打包工具風格的匯入
（`./errors` 對應到 `./errors.ts`），當儲存庫沒有安裝 npm 套件時，`--resolve-from` 會提供它們，
而需要轉換的 TS（enum、參數屬性）會以
`--experimental-transform-types` 重試。

## `analyze` 會檢查什麼

- 套件（`package.json`）與原始檔；會略過 `node_modules`、建置輸出、fixture、
  playground、工具設定。Vue/Svelte/Astro 的 `<script>` 區塊會被解析以找出匯入。
- **單元**：一個原始檔資料夾，或位於套件原始碼根目錄的一個檔案。
- **波次**：單元所匯入的東西會出現在較早的波次；有循環匯入的部分會一起移植。
- **橋接**：當單元或它匯入的任何東西需要 Node 時使用 napi-rs；可攜式程式碼使用 wasm-bindgen；
  UI 與編輯器程式碼則留在 JS。
- **就緒程度**（有多安全）與**優先順序**（安全且值得：迴圈與字串處理、測試、大小）。

| 風險 | 嚴重程度 | 範例 |
| --- | --- | --- |
| 字串位移 | 高 | `slice`、`indexOf`、`matchAll`、`d` 旗標：UTF-16 與 UTF-8 |
| 正規表示式語法 | 高 | lookaround、反向參照、`y`：`regex` crate 不支援 |
| 動態正規表示式 | 高 | `new RegExp(userPattern)`：執行期的 JS 語法 |
| 動態程式碼 | 高 | `eval`、`new Function`、計算式的 `import()` |
| 動態型別 | 中 | `any`、`'key' in value` |
| 有序集合 | 中 | `Map`、`Set`、`Object.keys`：插入順序 |
| 非同步、generator | 中 | promise 與 generator 的邊界 |
| 類別繼承、反射 | 中 | `extends`、`Proxy`、`Reflect`、`Symbol` |
| 回呼形式的 API | 中 | 函式型別的參數與方法：往返頻繁的橋接 |
| 例外、數字、JSON | 低 | `throw`、位元運算、`JSON.stringify` 的順序 |

## 它抓不到的問題

通過代表 Rust 與每個已錄製的案例一致，僅此而已。讓重寫出問題的東西
通常位在錄製內容之外，所以要清楚邊界在哪裡：

- **沒有人錄製過的輸入。** 自動產生的案例、陷阱案例、原始版本的測試套件
  （`record --from-tests`）以及你自己的 `--inputs` 就是上限。請加入真實的輸入（正式環境的
  樣本、錯誤回報）；讓正式環境壞掉的使用者輸入，很少是產生器想得到的。
- **已經改變的原始版本。** `record` 會儲存單元檔案的指紋
  （`recorded-from.json`）；當原始版本之後有所變更，`diff` 會發出警告，而 `gate` 會失敗，直到你
  重新錄製為止，而不是拿原始版本過去的行為來判定通過。
- **正規表示式。** `regex` crate 不支援 lookaround 或反向參照，而且即使是能
  編譯的模式，對非 ASCII 文字處理 `\w`、`\b`、`\d` 與不分大小寫比對的方式，也可能與
  JS、Python 或 Java 不同。改用 `fancy-regex` 或重寫模式，可能通過每個
  已錄製的案例，卻仍在土耳其文無點的 `ı` 或零寬連接符上出現差異。`record` 會為
  使用正規表示式的檔案中的每個函式加入正規表示式陷阱（上述那些，以及克耳文符號、長 s、阿拉伯－印度數字、行分隔符、詞尾 sigma），
  但也請用真實文字對你真正使用的模式做模糊測試。
- **每次執行都會變的東西。** `record` 會執行原始版本兩次，並整理出有差異的部分：
  改變的值（時間、id）會變成萬用字元；順序不同的相同行（雜湊表、
  並行處理）會以任意順序比較，所以移植版本仍然必須產生完全相同的那些行；
  行數不同則會變成 `...`，可以符合任何內容。`record` 會列出每一種：
  請逐一檢查，因為雜湊順序或並行處理的差異，正是移植後真的可能改變的地方。
- **浮點數。** 數字轉文字的格式在各語言之間有差異（`1e21`、`-0`、位數）。
  陷阱案例包含這些值，而且它們必須完全一致；`--float-tolerance` 只原諒
  捨入誤差，從不原諒格式差異。
- **輸出以外的任何東西。** 時間、記憶體、負載下的並行行為，以及
  stdout、stderr、結束代碼、寫入的檔案與 HTTP 回應以外的副作用（網路呼叫、資料庫、
  訊號）都不會被比較。
- **其他平台。** 錄製內容只證明執行它的那台機器上的行為。結果取決於
  型別大小、位元組順序或編譯器的 C 程式碼，在目標平台上的行為可能不同；`analyze`
  因此會把與硬體相關的 C 程式碼留在 C 中。
## 每個步驟

| 步驟 | 指令 | 作用 |
| --- | --- | --- |
| 0. 效能剖析 | `everything-rust profile <repo> -- node bench.mjs` | 在該語言的效能剖析工具（node、cProfile、`go test` pprof、JFR、Ruby tracer）下執行真實的工作負載：依包含時間列出熱點函式、它們的呼叫端，以及哪些迴圈該做批次處理；並標出已經編譯過的程式碼（numba、Cython、JAX），這類程式碼用 Rust 改寫幾乎沒有好處 |
| 1. 規劃 | `everything-rust analyze <repo>` | 單元、移植順序、附 `file:line` 的風險、每個單元的橋接方式、相依套件對應 crate 的適配程度、最適合先移植的部分；`--whole-app` 會加上進入點與型態、每個相依套件（已匯入或已宣告）及其 crate、區塊（互相匯入的單元會一起搬移），以及附有建議答案的編號決策 |
| 2. 橋接 | `everything-rust bridge <repo> <unit>` | 建立包含綁定（`lib.rs`，以及來源語言載入的包裝層）與待移植空殼（`port.rs`）的 crate；`--batch fn:param` 會新增一個將大量 `param` 值保留在 Rust 中的類別；`--as-json T` 會把應用程式只用資料填入的型別參數以 JSON 值的形式橋接 |
| 3. 錄製 | `everything-rust record <crate>` | 以自動產生的邊界案例（非 ASCII、emoji、`-0`、`NaN`、預期浮點數的地方傳入整數……）以及每個參數上的每個語意陷阱（字串長度與大小寫、有號除法、溢位邊界、浮點數輸出、map 與排序的順序）執行原始版本，並儲存 `cases.jsonl`；`--from-tests -- npx jest`（或 `-- pytest`）還會錄製原始版本自己的測試套件所做的每一次呼叫；`--float-tolerance 1e-9` 讓數值程式碼的浮點數在該比例內即視為一致 |
| 3. 建置 | `everything-rust build <crate>` | 建置 Rust 端：`.node`、wasm 套件、Python 擴充或共用函式庫 |
| 3. 比對 | `everything-rust diff <crate>` | 以原始版本呼叫端的呼叫方式，將已錄製的案例拿來重播比對 Rust；列出不一致之處 |
| 3. 基準測試 | `everything-rust bench <crate>` | 在原始版本與 Rust 上（包含橋接）為已錄製的案例計時 |
| 4. 影子模式 | `everything-rust shadow <crate>` | 可直接替換的模組，同時執行兩者，回傳原始版本的結果，回報不一致之處；`EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. 閘門 | `everything-rust gate <crate>` | fmt、clippy `-D warnings`、測試、`unsafe` 稽核、沒有未移植的部分、原始版本的每條錯誤路徑都有已錄製的失敗案例、在原始版本會回傳錯誤的地方不使用 `unwrap`/`expect`/`panic!`（除非標記了 `// INFALLIBLE: <why>`）、建置、API、行為 |
| AI 代理 | `everything-rust setup` | 在你的程式開發 AI 代理中註冊 MCP 伺服器（`everything-rust mcp`，每個步驟都是一個工具）與工作流程技能（[`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)） |
## 下一步

每個步驟都有自己的頁面：[效能剖析與分析](/docs/concepts/analyze)、[橋接](/docs/concepts/bridge)、[錄製](/docs/concepts/record)、[移植](/docs/concepts/port)、[比對](/docs/concepts/diff)、[基準測試](/docs/concepts/bench)、[影子模式](/docs/concepts/shadow)與[閘門](/docs/concepts/gate)。
