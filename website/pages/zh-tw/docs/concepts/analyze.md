---
title: 效能剖析與分析
description: 用 profile 找出時間花在哪裡，再用 analyze 規劃單元、移植順序、風險與 crate。
---

# 效能剖析與分析

## 效能剖析：找出值得搬移的部分

當目標是速度時，在做任何選擇之前先量測：

```sh
everything-rust profile <repo> -- node bench.mjs      # 或：python3 bench.py、ruby bench.rb、
                                                    #     java -cp out Main、go test -run X -bench . ./pkg
```

`profile` 會在該語言的效能剖析工具（node、cProfile、`go test` pprof、JFR、Ruby tracer）下執行真實的工作負載，並依包含時間列出熱點函式、它們的呼叫端，以及哪些迴圈該做批次處理。Go 只能透過 `go test` 做效能剖析；C 不支援。

- 每個項目呼叫一次的小函式，**不值得**單獨移植：一次呼叫 Rust 的成本比它的工作量還高，而 JIT（V8、JVM、Go）執行它只需要幾奈秒。請將它連同呼叫端一起移植，或[做批次處理](/docs/concepts/batches)。建議那一行會指出該怎麼做，並附上確切的指令。
- 已經編譯過的程式碼（numba `@njit`、Cython、`@jax.jit`）是以機器碼執行：Rust 移植版本幾乎不會提升速度。`profile` 會把它列在「Already compiled」底下。

## 分析：規劃移植

```sh
everything-rust analyze <repo> -o plan.md        # 給人看
everything-rust analyze <repo> --json            # 給 AI 代理：schema everything-rust/analyze@1
everything-rust analyze <repo> --whole-app       # 另外加上進入點、型態、相依套件、決策
```

`analyze` 會列出單元、移植順序、附 `file:line` 的風險、每個單元的橋接方式、每個相依套件對應到 crate 的程度，以及 `start_here`：最適合先移植的部分。`--whole-app` 會加上應用程式的型態（CLI、伺服器或函式庫）與進入點、每個相依套件（已匯入或已宣告）及其 crate、區塊（互相匯入的單元會一起搬移），以及附有建議答案的編號決策。請參閱[移植整個應用程式](/docs/whole-apps/overview)。

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
## 相依套件對應的 crate

`analyze` 會為程式碼匯入的每個函式庫指出對應的 Rust crate，並標示適配程度為 `good`、`partial` 或 `none`。對照表中沒有的函式庫會回報為 `unknown`；對照表從不猜測 crate。若要新增一筆，請參閱[參與貢獻](/docs/project/contributing#add-a-crate-mapping)。
