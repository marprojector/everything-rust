---
title: 實際移植案例
description: everything-rust 遇上真實程式碼時發生了什麼，較慢的結果與較快的結果並列，以及工具因此做了哪些改變。
---

# 實際移植案例

everything-rust 遇上真實程式碼時發生了什麼，以及工具因此做了哪些改變。下方的每個數字都來自一個會先確認結果完全相同的基準測試，而較慢的案例也與較快的案例並列。

| 移植 | 內容 | 結果 |
| --- | --- | --- |
| [json-server](/docs/real-world/json-server) | 一個完整的 Node 伺服器，以 `app record`、`front-door`、`wire` 與 `lift` 端對端移植 | 一個通過 514/514 個已錄製 HTTP 案例的純 Rust 執行檔；六個單元通過 `gate` |
| [Fuse.js](/docs/real-world/fuse-js) | 模糊搜尋，搜尋時花費時間的那條路徑 | 3,414/3,414 個案例一致；在 50,000 個字串上快 32x（單一核心為 5.3x） |
| [vercel/ms](/docs/real-world/vercel-ms) | 一個很小的時間長度解析器，端對端移植 | 975/975 個案例一致；單次呼叫 0.56x，`parse` 批次 18x，`format` 批次 0.8x |
| [x18n](/docs/real-world/x18n) | 案例研究用的儲存庫 | `packages/core` 上每個 gate 都通過，透過橋接時比 TS 慢；它的搜尋以批次方式達到 18-22x |
| [六種語言](/docs/real-world/six-languages) | Go 的 `path`、Ruby 的 `shellwords`、Python 的 `colorsys`、小型外觀層後的 JDK 與 glibc 行為 | 每個移植版本都通過 gate，總共 13,010 個已錄製案例 |
| [Fixture 與靜態執行](/docs/real-world/fixtures) | 為了對工具施壓而撰寫的 fixture，以及只分析但未執行的函式庫 | |

移植過真實的專案嗎？歡迎撰寫心得：請參閱[參與貢獻](/docs/project/contributing#share-a-real-port)。
