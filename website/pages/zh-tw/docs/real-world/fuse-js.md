---
title: Fuse.js
description: 以 Rust 實作的 Fuse.js 字串列表搜尋。3,414/3,414 個已錄製案例一致，結果逐位元組相同，在 50,000 個字串上快 32x。
---

# Fuse.js

[Fuse.js](https://github.com/krisk/Fuse)（20k 顆星，每週約 2,000 萬次 npm 下載，
commit `edf2fb6`）是無數搜尋框背後的模糊搜尋函式庫。以
everything-rust 1.6（尚未發布）移植；crate 位於 [`demo/real-world/fuse-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/fuse-rs)。

`analyze` 把 `src/` 的每個資料夾都歸在同一個循環匯入中，所以 Fuse 無法逐檔移植。
因此移植版本改為處理搜尋時花費時間的那條路徑：一個 54 行的轉接器
（`adapter/relevance.ts`）匯出 Fuse 自己對字串列表中單一字串的分數，以
Fuse 的 `BitapSearch`、欄位正規化與 `computeScoreSingle` 建構而成。對
50,000 個字串進行搜尋時，執行 50,000 次的就是這個函式。`bridge --batch relevance:text` 會把字串保留在 Rust 中，
並使用所有核心對全部字串執行搜尋。

| 步驟 | 結果 |
| --- | --- |
| 轉接器 | 在撰寫任何 Rust 之前，轉接器的排名在 81 組查詢與選項上與 `new Fuse(list).search(q)` 相同 |
| `record --inputs` | 3,414 個案例：來自 Fuse 自己 fixture 的標題、非 ASCII、emoji、超過 32 個單元的模式、每個支援的選項，以及型別錯誤的選項（被拒絕，149 個案例） |
| 移植 | 以 673 行 Rust 取代 839 行 Fuse |
| `diff` | 第一次建置就 3,414/3,414 一致 |
| `gate` | 通過 |
| 直接替換 | `withRust(Fuse)` 以 Rust 回應字串列表搜尋，其餘全部交給 Fuse；它在 360 次搜尋（12 組選項、15 個查詢，有無 `limit`）上回傳與 Fuse 逐位元組相同的結果 |

每次查詢的搜尋時間，Apple M4（10 核心），Node 24，計時前已確認結果相同：

| 列表 | Fuse.js | Rust | 加速比 |
| ---: | ---: | ---: | ---: |
| 1,000 個字串 | 1.9 ms | 0.1 ms | 18x |
| 10,000 個字串 | 19.1 ms | 0.7 ms | 27 到 30x |
| 50,000 個字串 | 91.2 ms | 2.8 ms | **32x** |
| 50,000 個字串，單一核心 | 92.0 ms | 17.5 ms | 5.3x |

移植版本必須做對的地方：

- Bitap 在 32 位元的 JS 整數上執行：`1 << 31` 是負數，而 `(1 << 31) - 1` 是一個 double，
  在下一次位移時會變成 `2147483647`。移植版本會依照 JS 的方式保留這些值。
- 文字以 UTF-16 程式碼單元為索引，所以一個 emoji 佔兩個位置，與 Fuse 相同。
- `ignoreDiacritics` 是 NFD 加上一份組合符號範圍清單，以及幾個明確列出的字母
  （`ł`、`ß`、`ı`）；移植版本使用相同的清單。
- `String.prototype.trim` 決定 Fuse 會略過哪些字串：使用 JS 的空白定義，而不是 Rust 的
  `char::is_whitespace`（它包含 U+0085 且不包含 U+FEFF）。

在工具中發現並已修正的問題：項目為字串但結果不是字串的批次 crate，
會產生一個未使用的輔助函式而無法通過 `clippy -D warnings`；而 `gate` 的大小檢查只計算了
轉接器（54 行，因此「太大」了 12.5x），而不是它所匯入的 Fuse 檔案。

它沒有涵蓋的部分：使用 `keys` 的物件列表、延伸搜尋與 token 搜尋、邏輯查詢、
`includeMatches`、自訂的 `sortFn` 或 `getFn`。直接替換版本會把這些原封不動地交給 Fuse。
加速需要列表在每次搜尋之間都留在 Rust 中；建立它的成本與
Fuse 自己的索引差不多（50,000 個字串為 3.4 ms 對 2.8 ms）。
