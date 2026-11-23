---
title: 錄製
description: record 會以自動產生的邊界案例、語意陷阱、原始版本自己的測試以及你的輸入執行原始程式，並將結果儲存為 cases.jsonl。
---

# 錄製

```sh
everything-rust record <crate>
everything-rust record <crate> --from-tests -- npx jest      # 或：-- pytest
everything-rust record <crate> --inputs extra.jsonl
```

`record` 會執行原始的函式，並將每個函式回傳的值或擲出的例外儲存在 crate 旁的 `cases.jsonl` 中：這就是移植版本必須符合的行為。之後出現的不一致就是行為差異：請修正移植版本，絕對不要修改錄製內容。

## 案例從哪裡來

- **自動產生的邊界案例**：非 ASCII、emoji、`-0`、`NaN`、預期浮點數的地方傳入整數等等（`--count` 設定每個函式的數量）。
- 每個參數上的**語意陷阱**：字串長度與大小寫、有號除法、溢位邊界、浮點數輸出、map 與排序的順序。
- 對使用正規表示式的檔案中的每個函式加入**正規表示式陷阱**：土耳其文無點的 `ı`、克耳文符號、長 s、阿拉伯－印度數字、行分隔符、詞尾 sigma、零寬連接符。
- **原始版本自己的測試**：`--from-tests -- <test command>` 會錄製測試套件對該單元匯出函式所做的每一次呼叫（TS/JS 支援任何測試執行器，Python 支援 pytest 或 unittest）。原始碼會在之後還原。在 vercel/ms 上，它從 167 個 Jest 測試中擷取了 340 次呼叫。
- **你的輸入**：`--inputs cases.jsonl`，每行一個 `{"fn": name, "args": [...]}`。請加入真實的輸入（正式環境的樣本、錯誤回報）；讓正式環境壞掉的使用者輸入，很少是產生器想得到的。

Python 的呼叫端傳入 `1` 的頻率和 `1.0` 一樣高，所以錄製時每四個 Python 案例中就有一個會將整數值以 int 傳入。

## cases.jsonl 中的標記

JSON 無法表示 `undefined`、NaN、Infinity、-0 或孤立代理字元。結構化的值會帶有標記，由自動產生的 `index.cjs` 轉換，輔助函式則位於 crate 的 `js.rs` 中：

| 標記 | 意義 |
| --- | --- |
| `{"$undefined": true}` | JavaScript 的 `undefined` |
| `{"$num": "NaN"}` | JSON 無法容納的數字（`NaN`、`Infinity`、`-0`） |
| `{"$utf16": [...]}` | 以 UTF-16 程式碼單元表示的字串，用於孤立代理字元 |

超出函式契約範圍的 C 輸入可能讓原始程式當掉。C 驅動程式會在 fork 出的子行程中執行每個案例，並錄製 `crash: SIGSEGV`；由於 C 在此的行為未定義，`diff` 會接受移植版本在這裡的任何行為。

## 兩次執行，以及差異之處

`record` 會執行原始版本兩次，並整理出兩次執行之間的差異：改變的值（時間、id）會變成萬用字元；順序不同的相同行（雜湊表、並行處理）會以任意順序比較；行數不同則會變成 `...`，可以符合任何內容。`record` 會列出每一種。請逐一檢查，因為雜湊順序或並行處理的差異，正是移植後真的可能改變的地方。

## 錄製內容會記住它的原始版本

`record` 會儲存單元檔案的指紋（`recorded-from.json`）。當原始版本之後有所變更，`diff` 會發出警告，而 `gate` 會失敗，直到你重新錄製為止，而不是拿原始版本過去的行為來判定通過。請參閱[比對](/docs/concepts/diff#stale-recordings)。

## TypeScript 的特別之處

`record` 會直接執行 TS 原始檔：Node 的 resolve hook 會跟隨打包工具風格的匯入（`./errors` 對應到 `./errors.ts`），當儲存庫沒有安裝 npm 套件時，`--resolve-from` 會提供它們，而需要轉換的 TS（enum、參數屬性）會以 `--experimental-transform-types` 重試。無法直接在 Node 中執行的 TS，可以用 `--module dist/x.mjs` 從建置輸出錄製，或使用 `--node-arg=--import=tsx`。

## 數值程式碼

對於浮點數很少能逐位元一致的程式碼，請以 `--float-tolerance` 錄製。請參閱[浮點容差](/docs/concepts/float-tolerance)。
