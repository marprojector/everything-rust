---
title: json-server
description: 第一個端對端移植的整個應用程式。一個能回應 Node 伺服器所回應內容的 Rust 執行檔，514/514 個已錄製的 HTTP 案例。
---

# json-server

[json-server](https://github.com/typicode/json-server)（75k 顆星，MIT，commit `89a34a4`，
1.0.0-beta.15）會從 JSON 檔案提供 REST API：使用 Node、tinyhttp 與 lowdb。它是第一個
以整個應用程式流程（`app record`、`front-door`、`wire`、`lift`）端對端移植的應用程式：
結果是一個能回應 Node 伺服器所回應內容的 Rust 執行檔。所有內容都在
[`demo/real-world/json-server-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/json-server-rs)。

| 步驟 | 結果 |
| --- | --- |
| `analyze --whole-app` | 一個伺服器，9 個單元（7 個就緒，2 個需注意），13 個需要取代的相依套件（tinyhttp、lowdb、sirv、eta、milliparsec、dot-prop、sort-on、inflection……） |
| `app record --http` | 514 個案例：以 `--inputs` 提供的 268 個請求（每個路由與方法、where 運算子、排序、分頁、嵌入、CORS preflight、靜態檔案與範圍請求、29 個格式錯誤的 JSON 主體、`/__proto__` 與其他 prototype 名稱、錯誤的跳脫字元）以及 245 個陷阱案例；每個都錄製兩次，隨機 id 保留為 `[..]` |
| `front-door` | 513/514，回退率 99.8%；未通過的是 URL 中未編碼的 `ü`，Node 會回應 400，而代理會重新編碼它 |
| 單元 | `where-operators` 207、`parse-where` 289、`matches-where` 1,346、`paginate` 853 個案例（包含來自 json-server 自己測試的呼叫）；`service` 461 與 `app` 243 則透過兩個執行 Service 呼叫與真實 HTTP 請求的小型轉接器；六個單元都通過 `gate` |
| `wire` | 先接上讀取：514/514，回退率 22.6%，由 Node 回應寫入，Rust 重新載入 Node 寫入的檔案；接著是每個路由：514/514，回退率 0% |
| `lift` | 一個純 Rust 執行檔（5.3 MB），514/514；`bin.ts` 的命令列部分手動移植（help、version、檔案檢查、啟動橫幅） |

提升之後單元不再有綁定，所以每個 crate 的 `tests/recorded.rs` 會在移植版本上重播它的
已錄製案例：全部 3,399 個都透過 `cargo test` 執行。

移植版本必須做對的地方，全部由已錄製的案例抓到：

- tinyhttp 的路由器不只分派一次：每次 `next()` 之後，它都會在最後一個中介軟體改寫過的 URL 上
  重新比對所有中介軟體與路由，並對 `HEAD` 執行每個路由。所以
  `HEAD /posts` 會先執行 GET 的處理器，再執行 POST 的，並回應 POST 的 400；`//posts` 會略過 CORS
  標頭與主體解析器。移植版本照原樣執行這個迴圈。
- milliparsec 會把每個 POST、PUT、PATCH 與 DELETE 的主體都當成 JSON 解析，不論其類型，而格式錯誤的
  主體會得到附帶 V8 訊息的 500：`Expected double-quoted property name in JSON at position 7
  (line 1 column 8)`、`Unexpected token 'x', ..."bbbbbbb": x}" is not valid JSON`。移植版本
  重現了 V8 解析器的錯誤，包括以 UTF-16 單元計算的位置；已在
  3,806 個格式錯誤的輸入上與 Node 比對。
- 排序使用 `localeCompare`（ICU），以 ICU4X 的 collator 實作，並在 3,000 組配對上檢查過；
  `Number()`、`JSON.stringify`（`1e+21`、整數鍵優先）、Node 的兩個查詢字串解析器
  （對 `é%E9` 的解碼方式與 WHATWG 不同）、dot-prop 及其錯誤訊息，以及用於嵌入的 inflection
  （`person` 與 `people`），每一項都在 `json-server-js` 中與 Node 比對過。
- `db.data[name]` 會讀取 `Object.prototype`：`/__proto__` 回應 `{}`，`/constructor` 回應空的
  200，而 `PATCH /__proto__` 會取代資料庫的 prototype，之後的查找都會看到這個變化。
- Node 的 HTTP 解析器對於未編碼的非 ASCII 目標與未知的方法，會回應 400 並關閉連線。

在 json-server 中發現的問題：每次它自己寫入之後，它的檔案監看器都會重新載入檔案。當
下一個請求在重新載入結束前寫入時，重新載入可能會把那次寫入抹除：第一次
錄製以伺服器回應的最快速度送出請求，其中有一個 POST 回應了 201，之後的讀取卻沒有
出現它。`app record --pace 50` 會在請求之間等待，而案例就是以這種方式錄製的。移植版本
只在有其他人修改檔案時才重新載入，所以不會遺失那次寫入：這是一個刻意的
差異。

基準測試：Apple M4（10 核心，16 GB），macOS 27.3，Node 24.15.0 執行由
`tsc` 為 npm 建置的 `lib/bin.js`，Rust 執行檔以 `--release` 建置。兩者都在 loopback 上提供同一個
資料庫的副本；`ab -k` 送出相同的請求，暖機之後每列 20,000 個（大型資料庫上較慢的嵌入
為 40 個）。執行兩次，每次都在 1 分鐘平均負載低於 3 時開始；
儲存格中為第 1 次 / 第 2 次執行的每秒請求數。[`bench/bench.py`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/json-server-rs/bench/bench.py)
是所用的指令稿。

fixture 資料庫（6 篇文章，3 則留言）：

| 請求 | Node，1 個連線 | Rust，1 個連線 | Node，50 個連線 | Rust，50 個連線 |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts` | 11.1k / 10.8k | 27.3k / 23.9k | 19.7k / 20.4k | 115.0k / 110.7k |
| `GET /posts/1` | 11.0k / 11.3k | 27.7k / 25.4k | 21.9k / 22.6k | 119.9k / 99.0k |
| 篩選、排序、分頁 | 10.5k / 10.0k | 25.2k / 21.5k | 18.3k / 16.4k | 107.5k / 96.3k |
| `?_embed=comments` | 8.7k / 6.6k | 22.5k / 21.8k | 13.8k / 11.0k | 98.3k / 100.6k |
| `GET /`（索引頁面） | 7.3k / 6.3k | 27.6k / 21.7k | 10.2k / 9.5k | 135.1k / 119.3k |
| `GET /test.html`（靜態） | 12.0k / 11.0k | 25.8k / 25.9k | 30.3k / 26.1k | 103.9k / 104.4k |
| `POST /comments`，10 個連線 | | | 2,973 / 2,467 | 5,821 / 5,976 |

自動產生的資料庫（10,000 篇文章，20,000 則留言，3.6 MB）：

| 請求 | Node，1 個連線 | Rust，1 個連線 | Node，50（10）個連線 | Rust，50（10）個連線 |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts/5000` | 8.4k / 9.2k | 14.6k / 14.8k | 15.5k / 16.1k | 69.5k / 67.4k |
| `?_page=3&_per_page=10` | 4.6k / 4.2k | 8.8k / 8.4k | 6.5k / 6.8k | 47.4k / 46.2k |
| `title:contains`，排序 | 431 / 396 | 600 / 560 | 465 / 453 | 2,516 / 2,329 |
| `?_embed=comments`，一頁 5 筆（10 個連線） | 2.4 / 2.4 | 81 / 103 | 2.4 / 2.2 | 253 / 216 |
| `GET /posts`，完整列表（10 個連線） | 328 / 319 | 399 / 407 | 381 / 387 | 1,514 / 1,657 |
| `POST /comments`（10 個連線） | | | 236 / 241 | 292 / 264 |

| | Node | Rust |
| --- | ---: | ---: |
| 啟動到第一個回應，fixture 資料庫 | 85 / 79 ms | 9.9 / 7.9 ms |
| 啟動到第一個回應，10,000 篇文章 | 95 / 91 ms | 48 / 47 ms |
| 閒置記憶體（RSS），fixture 資料庫 | 74 MB | 3.5 MB |
| 閒置記憶體（RSS），10,000 篇文章 | 94 MB | 41 MB |
| 執行期間的記憶體（RSS）峰值，fixture 資料庫 | 424 / 390 MB | 18 MB |
| 執行期間的記憶體（RSS）峰值，10,000 篇文章 | 462 / 485 MB | 289 / 291 MB |

差距較小的情況：單一連線讀取完整的 10,000 篇文章列表（1.2x：兩者的時間都花在
輸出 1.5 MB 的 JSON 上）、在大型資料庫上寫入（1.1 到 1.2x：兩者每次寫入都會重寫整個
3.6 MB 的檔案，而 Rust 會像 lowdb 一樣合併排隊中的寫入），以及單一連線上排序後的標題
搜尋（1.4x）。大幅的提升來自並行處理，這是單一 Node 行程所沒有的，以及來自
嵌入，json-server 會為每篇文章篩選每則留言（移植版本每個請求只為留言建立一次索引，結果相同）。

第一個提升後的執行檔在大型資料庫上比 Node 還慢（完整列表每秒 33 對 298 個請求，
寫入為 39 對 217）。效能剖析依序找到：沒有設定
`TCP_NODELAY`（每個大型回應都要等待約 40 ms 的延遲 ACK）、每次寫入前都複製整個資料庫
來加入 `$schema`、輸出前複製每個列出的項目、所有請求共用一把鎖，
以及對 10,000 個項目的每一個都以 SipHash 雜湊 `item.id`。全部已修正；每次變更後，3,399 個
單元案例與 514 個 HTTP 案例都通過。

在工具中發現並已修正的問題：

- 會寫入自身狀態的伺服器，無法從相同的起點錄製兩次：
  `--fresh <path>` 現在會在每次啟動時，從與案例一起保存的快照，把資料庫（以及 `public/`）
  複製到新的工作目錄；`--arg` 會給每個建置版本原始版本的參數；
  `--pace` 如上所述。
- `bridge` 把 `paginate<T>(items: T[])` 留在 TS 中（對任何東西泛型化，包括函式）；
  `--as-json T` 會以 JSON 值橋接它。
- 入口中帶參數的路由對 `/posts/%E0` 回應了 axum 自己的 400，在單元
  看到請求之前；參數現在改從 URI 讀取。
- `lift` 只暫存了已接上的單元，而沒有暫存它以路徑相依的 crate，並寫出了一個
  會被 `cargo fmt --check` 拒絕的 `lib.rs`；兩者都已修正。
- 它產生的伺服器沒有設定 `TCP_NODELAY`：分成兩段寫出的回應要等待
  用戶端的延遲 ACK，約 40 ms。現在會設定它。

它沒有涵蓋的部分：JSON5 資料庫（執行檔會說明並結束）、100 KiB 的主體限制
（Node 的限制取決於主體如何以 64 KiB 的讀取到達；移植版本假設是完整讀取）、含有
孤立代理字元的字串（它們會變成 U+FFFD），以及資料庫中值為 `1e400` 的情況（Node 會在記憶體中保留 `Infinity`
直到下次重新載入；移植版本儲存的是 `null`，也就是檔案中寫入的值）。
