---
title: 伺服器
description: 相同的指令也能證明 HTTP 伺服器。--http、--fresh、--arg 與 --pace，以及一個會代理所有尚無單元回應之路由的 axum 入口。
---

# 伺服器

相同的指令也能證明 HTTP 伺服器。`--http` 表示該指令會啟動一個伺服器；它會從
`PORT` 或指令中的 `{port}` 取得連接埠（對於總是監聽同一個連接埠的伺服器，請用 `--port`）：

```sh
everything-rust app record . --http -- node server.js
everything-rust app replay .
everything-rust app front-door .                                    # axum：代理每個路由
everything-rust app wire . api --route 'GET /users/{id}' --route '*'
everything-rust app lift .
```

- 請求來自應用程式文件中的 `curl` 行（`curl -X POST -d ... http://localhost:8080/x`）、
  `--inputs`（`{"name", "method", "path", "headers", "body"}`）以及陷阱案例：路徑區段與查詢
  值會替換成與 CLI 參數相同的陷阱，主體則替換成文字與 JSON 陷阱，另外還有一個
  未知的路由。每次執行都會重新啟動伺服器，並依名稱順序送出請求，所以請求之間的
  狀態會被保留。
- 將狀態存放在檔案中的伺服器，每次啟動都會拿到一份全新的副本：`--fresh db.json`（一個檔案
  或資料夾，可重複指定）會被快照到 `http/files/`，並在每次原始版本、入口或某個建置版本啟動時
  複製到新的工作目錄中；`--arg db.json` 是每次啟動時接在指令之後的參數，
  讓 Rust 執行檔以與原始版本相同的方式啟動。
  `--pace 50` 會在每個回應之後等待 50 ms，用於伺服器在回應之後才進行的工作（檔案
  監看器重新載入它剛寫入的內容）。
- 案例是 `rust/app-cases/http/<case>.toml`（請求、狀態碼以及重要的標頭）
  與 `<case>.body`（回應主體，兩次執行有差異之處以 `[..]` 表示）。Date、server 與
  其他每次回應都不同的標頭會被略過；你不在意的標頭行可以直接刪除。
- 入口是一個 axum 伺服器，它會在自己的連接埠上啟動原始版本，並代理
  所有尚無單元回應的路由。`wire --route` 接受 axum 路由：`METHOD /path/{param}`、代表任何方法的 `/path`，
  以及代表其他所有路由的 `*`；膠合程式碼會取得路由、它的參數以及
  請求。`lift` 會移除代理；之後 HTTP 案例會在 CI 中以 `app replay` 執行。
## 一個真實案例

[json-server](/docs/real-world/json-server) 需要 `--fresh`、`--arg` 與 `--pace`：它在每次寫入時都會重寫資料庫，而它的檔案監看器可能會撤銷緊接在後的寫入。
