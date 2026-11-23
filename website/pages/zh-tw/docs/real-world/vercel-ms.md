---
title: vercel/ms
description: 端對端移植 vercel/ms。在每個已錄製的案例上都正確，單次呼叫較慢，只有在程式一次解析大量時間長度時才較快。
---

# vercel/ms

[vercel/ms](https://github.com/vercel/ms)（5.5k 顆星，commit `4ff48ce`）會把 `"2 days"` 轉成
毫秒，也能反向轉換；大多數 Node 專案都會透過某個相依套件引入它。以
everything-rust 1.5.1 端對端移植；crate 位於
[`demo/real-world/vercel-ms-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/vercel-ms-rs)。

| 步驟 | 結果 |
| --- | --- |
| `analyze` | 1 個單元（`src/index.ts`），就緒，就緒程度 96 |
| `bridge --kind napi` | `ms`、`parse`、`parseStrict` 與 `format` 已橋接，沒有任何一個留在 TS 中 |
| `record --from-tests -- jest` | 975 個案例：自動產生的案例、陷阱案例，以及從 vercel/ms 的 167 個 Jest 測試中擷取的 340 次呼叫（第一版移植使用的是從測試中手動複製的 149 個輸入） |
| 移植 | 以 200 行 Rust 取代 184 行 TS，包括一個每個批次只讀取一次 `options` 的 `FormatIndex` |
| `diff` | 第一次建置就 784/784 一致；加上擷取的呼叫後為 975/975 |
| `gate` | 通過（fmt、clippy、測試、錯誤路徑、大小、行為） |
| `bench`，單次呼叫 | 在已錄製的案例上為 0.56x（最初為 0.39x）：對一般字串執行 `parse` 需 79 ns，與 JS 持平，但邊界案例與擲出的錯誤透過橋接仍然成本較高 |
| 批次，`parse` x100,000 | JS 11.8 ms，Rust 0.65 ms：**18x**；計入一次性交接字串的成本為 3.4x（字串合併後跨越之前為 2.1x） |
| 批次，`format` x100,000 | JS 2.1 ms，計入交接成本的 Rust 2.5 ms：**0.8x**（最初為 0.2x）；`format(x, { long: true })` 為 1.2x |

移植版本必須做對的地方，全部由已錄製的案例或陷阱案例抓到：

- 沒有 `u` 旗標的 `/.../i` 只會對 ASCII 字母做大小寫折疊，而 Rust 的 `regex` 會折疊 Unicode，所以
  `"1 ſ"`（長 s）與克耳文符號原本會被解析為秒。移植版本以手動方式比對單位，
  只限 ASCII。
- `Math.round` 將 .5 往上捨入（`-2.5` 得到 `-2`）；Rust 的 `round` 則是遠離零捨入。
- `${n}` 會輸出 `1e+21`、`5e-324`，而 `-0` 會輸出 `0`；移植版本重現了 JS 的數字轉字串。
  它的第一版還有一個同值捨入的錯誤（`25.728225708007812` 被印成 `...813`），而 ms 的
  整數輸出永遠不會碰到它。
- `str.length` 以 UTF-16 單元計算，所以 100 字元的上限會把一個 emoji 算成兩個。

在工具中發現並已修正的問題：具有結構化參數（`format` 的
`options`）的 napi 批次方法無法編譯，因為自動產生的綁定接收的是 `serde_json::Value`，而 napi
在沒有啟用 `serde-json` 功能的情況下無法轉換它。具有批次類別的 crate 現在會啟用它。

從這次移植帶回工具中的改進：

- 自動產生的 `js.rs` 提供 `js::number_to_string` 與 `js::math_round`，已在
  20,000 個隨機 double 上與 Node 比對；移植版本使用它們，而不是自己的版本。`number_to_string` 的
  數字取自 serde_json 的輸出器：對整數比 `format!` 快 5x，對小數快 3x。
- napi 批次會把字串項目以一個合併的字串搭配 UTF-16 結束位移傳送，數字項目則以
  一個 `Float64Array` 傳送，而字串結果會合併後回傳（`index.cjs` 以 `slice` 分割它們）：
  與每個項目一個 JS 值相比，字串快 2x，數字快 20x。`new Batch(items)` 與
  `run()` 的形式保持不變。
- `analyze` 會標出 `Math.round`、`toFixed`，以及沒有 `u` 且提到 `s` 或 `k` 的 `/i` 正規表示式，
  並為每一項提供指引。
- 呼叫成本更低：在 napi 上，每個同步函式都透過二進位線路協定傳遞，不使用 napi 參數
  （一個參數約需 55 ns，整個呼叫 22 ns），每個函式只有一個 JS 包裝層而不是
  兩個，`serde_json::Value` 參數以任何形狀的帶標籤值跨越，而錯誤則
  透過線路協定的緩衝區回傳。`parse` 的單次呼叫從 121 ns 降到 79 ns。
- `record --from-tests` 取代了手動複製測試輸入：它會在測試執行時錄製測試套件所做的每一次呼叫，
  並讓擷取程式碼不計入覆蓋率（ms 要求 100%）。

結論：`ms` 是個好的移植，卻不是好的加速目標。在每個已錄製的案例上都正確，只有
在程式一次解析大量時間長度時才較快。
