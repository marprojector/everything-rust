---
title: 移植
description: 撰寫 port.rs，也就是唯一由你或你的 AI 代理撰寫的檔案，以及讓它與原始版本保持一致的規則。
---

# 移植

移植版本由你或你的 AI 代理在 `src/port.rs` 中撰寫。crate 中的其他所有東西都是自動產生的。修改 `port.rs` 中的簽章之後，請執行 `everything-rust refresh <crate>`。

## 能省下一輪 diff 的規則

- 在撰寫程式碼之前，先到計畫為該單元列出的每個風險所在的 `file:line` 閱讀它。
- **字串位移**：JS 與 Java 以 UTF-16 程式碼單元為索引，Rust 則以 UTF-8 位元組為索引。請使用自動產生的 `src/utf16.rs`。
- **錯誤**：回傳 `Err("TypeName: message")`。它在 Python、Ruby 與 Java 中會變成該例外，在 Go 中則是 `error` 文字。Rust 的 panic 永遠不會與已錄製的錯誤一致。
- **Python 與 Ruby 中的數字**：原始版本原樣傳遞的整數，回傳時也必須是整數。在 Python 移植版本中，對於可能是整數或浮點數的值，請使用 `crate::pynum::Num`，而不是 `serde_json::Value`。
- **C**：錄製為 `crash:` 的輸入屬於未定義行為；任何結果都算通過。
- **有序的 map 與物件鍵**：當順序會影響輸出時使用 `indexmap`。
- **正規表示式的 lookaround 或反向參照**：使用 `fancy-regex`，而不是 `regex`。無論哪一個，`\w`、`\b`、`\d` 與不分大小寫比對處理非 ASCII 文字的方式，都與來源語言的引擎不同。當原始版本的 `\w` 只涵蓋 ASCII 時（沒有 `u` 的 JS、Java），請設定 `unicode(false)` 或明確寫出 ASCII 字元類別（`[A-Za-z0-9_]`）。
- **不要用 `unsafe`。** 如果真的需要：盡量精簡、隔離，並加上 `// SAFETY:` 註解。
- **輔助函式保持私有。** 只有單元的匯出是 `pub`；`pub` 的輔助函式會變成橋接 API 的一部分。
- 在 `port.rs` 中為你推敲過的案例加入 Rust 單元測試。

自動產生的 `js.rs` 帶有在 Rust 行為不同之處重現 JavaScript 行為的輔助函式，例如 `js::number_to_string`（JS 的 `String(n)`）與 `js::math_round`（`Math.round`）。

## 範例：移動了一個位置的 emoji

來自 [x18n](https://github.com/x18n-dev/x18n) 的 `packages/core/src/fuzzy.ts`，以兩種方式移植於
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo)：

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts：就緒，wasm-bindgen（core 在瀏覽器中執行）
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# 撰寫 src/port.rs
everything-rust gate demo/fuzzy-rs
```

第一版以 Rust `char` 為索引的移植通過了簡單的測試，卻沒通過 `diff`：

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

JS 的 `indexOf` 以 UTF-16 程式碼單元計算，所以一個 emoji 佔兩個位置。
`demo/*/src/port.rs` 中的移植版本使用自動產生的 `utf16` 輔助函式，通過 2000/2000 個案例與 gate，
無論是以 napi-rs（`demo/fuzzy-napi`）還是 wasm-bindgen（`demo/fuzzy-rs`）的形式。

everything-rust 為整個 x18n 撰寫的計畫：[`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md)。
## 應該寫多大？

`gate` 會比較移植版本與原始版本的行數，超過 3 倍（另加 50 行給匯入與簽章）就會失敗。超過通常代表你手寫了某個 crate 已經做好的事，或是逐一處理個別案例。`analyze` 會列出應該使用的 crate。
