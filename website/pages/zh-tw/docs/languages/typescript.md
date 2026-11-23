---
title: TypeScript/JavaScript
description: 透過 napi-rs 或 wasm-bindgen 將 Node.js 程式碼（TypeScript 或 JavaScript）移植到 Rust，並保留原本的 API。
---

# TypeScript/JavaScript

| | |
| --- | --- |
| 單元 | 一個資料夾或檔案 |
| 橋接 | napi-rs 或 wasm-bindgen，搭配自動產生的二進位線路協定；支援類別與 async |
| 錄製與重播方式 | Node，直接執行 `.ts` 原始檔 |
| 需求 | Node 22.6+（`profile` 需要 Node 24+） |
| 狀態 | 已驗證 |

## 選擇橋接方式

當單元或它匯入的任何東西需要 Node 時，`analyze` 會選擇 napi-rs；可攜式程式碼使用 wasm-bindgen（也能在瀏覽器中執行）；UI 與編輯器程式碼則留在 JS。`bridge --kind napi|wasm` 可以覆寫它。

無法跨越橋接的匯出會留在 TS 中並附上原因，而影子模組會重新匯出原始版本：接收或回傳函式的函式、generator 與迭代器、RegExp、`instanceof` 檢查、泛型（`--as-json T` 可橋接應用程式只用資料填入的泛型），以及型別中含有函式的值。請參閱[橋接](/docs/concepts/bridge)。

## 直接錄製 TS

`record` 會直接執行 TS 原始檔：Node 的 resolve hook 會跟隨打包工具風格的匯入，當儲存庫沒有安裝 npm 套件時，`--resolve-from` 會提供它們，而需要轉換的 TS（enum、參數屬性）會以 `--experimental-transform-types` 重試。`record --from-tests -- npx jest` 會錄製原始版本自己的測試套件所做的每一次呼叫，支援任何測試執行器。TS/JS 會量測原始版本的覆蓋率（[覆蓋率](/docs/concepts/coverage)）。

## 移植版本必須做對的地方

在真實程式碼上由已錄製的案例找出的問題：

- `str.length`、`indexOf` 與 `slice` 以 UTF-16 程式碼單元計算，所以一個 emoji 佔兩個位置。請使用自動產生的 `utf16` 輔助函式。
- 沒有 `u` 旗標的 `/.../i` 只會對 ASCII 字母做大小寫折疊，而 Rust 的 `regex` 會折疊 Unicode，所以在 vercel/ms 中 `"1 ſ"`（長 s）與克耳文符號原本會被解析為秒。
- `Math.round` 將 .5 往上捨入（`-2.5` 得到 `-2`）；Rust 的 `round` 則是遠離零捨入。自動產生的 `js.rs` 提供 `js::math_round`。
- `${n}` 會輸出 `1e+21`、`5e-324`，而 `-0` 會輸出 `0`。`js::number_to_string` 會重現這個行為。
- `String.prototype.trim` 使用 JS 的空白定義，而不是 Rust 的 `char::is_whitespace`。

## 速度

以單次呼叫而言，移植版本每次都要付出橋接成本，而 V8 執行小函式只需要幾奈秒。vercel/ms 在其已錄製的案例上單次呼叫為 0.56x；從 JS 呼叫時，x18n 核心的移植版本比 TS 慢。改用批次時，迴圈會搬到 Rust：

| 工作負載 | 原始版本 | 使用 everything-rust |
| --- | ---: | ---: |
| x18n 側邊欄搜尋，50,000 個 key（Node） | 每次查詢 10 ms | 0.5 ms：快 20x |
| 相同的搜尋在瀏覽器中（wasm，單一執行緒） | 10 ms | 0.95 ms：11.5x |
| Fuse.js，50,000 個字串 | 91.2 ms | 2.8 ms：32x |
| vercel/ms `parse` x100,000 | 11.8 ms | 0.65 ms：18x（計入交接成本為 3.4x） |
| vercel/ms `format` x100,000 | 2.1 ms | 計入交接成本為 2.5 ms：0.8x |

案例研究：[json-server](/docs/real-world/json-server)、[Fuse.js](/docs/real-world/fuse-js)、[vercel/ms](/docs/real-world/vercel-ms)、[x18n](/docs/real-world/x18n)。
