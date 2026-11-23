---
title: 橋接
description: bridge 會為單一單元建立 Rust crate 的骨架，包含綁定與包裝層，讓呼叫端繼續使用原本的 API。
---

# 橋接

```sh
everything-rust bridge <repo> <unit-id>          # crate 放在 <repo>/rust/<name>；絕不覆寫
everything-rust refresh <crate>                  # 在修改 port.rs 中的簽章之後
```

`bridge` 會建立一個 crate，包含綁定（`lib.rs`，以及來源語言載入的包裝層）與待移植的空殼（`port.rs`）。你只需要在 `src/port.rs` 中撰寫移植版本；`src/lib.rs` 與包裝層都是自動產生的，而 `refresh` 會根據 `port.rs` 中的簽章重新產生它們，並加入任何新的批次類別。它從不更動 `port.rs`。

選項：

- `--batch fn:param` 會新增一個類別，將大量 `param` 值保留在 Rust 中。請參閱[批次](/docs/concepts/batches)。
- `--as-json T` 會把應用程式只用資料填入的型別參數（`paginate<T>(items: T[])` 中的 `T`）以 JSON 值的形式橋接。
- `--keep "name: reason"` 會把更多匯出保留在來源語言中，例如依賴 `Intl` 資料的程式碼。
- `--kind` 會覆寫計畫所選的橋接方式（TypeScript 可選 napi-rs 或 wasm）。

## 各語言的橋接方式

| 語言 | 橋接 |
| --- | --- |
| TypeScript/JavaScript | napi-rs 或 wasm-bindgen，搭配自動產生的二進位線路協定；支援類別與 async |
| Python | PyO3（abi3），直接轉換，具型別的例外 |
| C | 使用原始名稱的 `extern "C"` 函式，以及標頭檔 |
| Go | 具型別的 cgo 呼叫與自動產生的二進位線路協定，放在 API 與原始版本相同的套件中 |
| Ruby | 位於原始模組底下的 magnus 擴充 |
| Java | 具型別的 JNI 與自動產生的二進位線路協定，放在具有原始方法的包裝類別中 |

PHP 與 C# 的單元目前還無法橋接；請參閱 [PHP](/docs/languages/php) 與 [C#](/docs/languages/csharp)。

## bridge 會決定什麼

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
