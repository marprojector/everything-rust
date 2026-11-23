---
title: 狀態
description: everything-rust 目前有哪些功能可用，以及每個部分最近一次是如何檢查的。
---

# 狀態

哪些功能可用，以及每個部分最近一次是如何檢查的。更新於 2026-11-18。

**已驗證**：在下方列出、由他人撰寫的真實程式碼上端對端執行過（bridge、record、port、diff、gate）。
**已測試**：由本儲存庫的測試在專為它們撰寫的 fixture 上涵蓋，尚未
在真實專案上執行。**預覽**：可用於流程的一部分，如說明所述。

## 移植單元

| 語言 | 狀態 | 最近一次實際執行 | 結果 |
| --- | --- | --- | --- |
| TypeScript/JavaScript | 已驗證 | [Prettier](https://github.com/marprojector/everything-rust/blob/main/docs/real-world.md#prettier)（2026-11-18）、[json-server](/docs/real-world/json-server)（2026-11-18）、[Fuse.js](/docs/real-world/fuse-js)（2026-11-18）、[vercel/ms](/docs/real-world/vercel-ms)（2026-11-18）、x18n 的 `packages/core` | Prettier：`doc-printer-rs` 2,848/2,848 與 `js-format-rs` 1,279/1,279，兩者 gate 都通過；json-server：六個單元、3,399 個案例，每個 gate 都通過；Fuse.js、vercel/ms、x18n：3,414/3,414、975/975 與 9,000+ 個案例一致 |
| Python | 已驗證 | CPython 3.13 的 `colorsys.py`（2026-11-16） | 2,400/2,400；gate 通過 |
| Go | 已驗證 | Go 1.24 的 `path`（2026-11-16） | 2,642/2,642；gate 通過 |
| Ruby | 已驗證 | Ruby 3.3 的 `shellwords.rb`（2026-11-16） | 1,617/1,617；gate 通過 |
| Java | 已驗證 | 外觀層後的 JDK 21（2026-11-16） | 4,492/4,492；gate 通過 |
| C | 已驗證 | 外觀層後的 glibc（2026-11-16） | 移除一個未定義行為的案例後為 1,859/1,859；gate 通過 |
| PHP | 預覽 | 無 | 僅支援 `analyze` 與整個應用程式；單元目前還無法橋接 |
| C# | 預覽 | 無 | 僅支援 `analyze` 與整個應用程式；單元目前還無法橋接 |

每次執行的詳細內容：[real-world.md](/docs/real-world/overview)（TS/JS）與
[real-world-languages.md](/docs/real-world/six-languages)（其他語言）。

## 指令

| 指令 | 狀態 | 備註 |
| --- | --- | --- |
| `analyze` | 已驗證 | 上述每種語言，x18n 的 142 個單元 |
| `bridge`、`record`、`diff`、`gate`、`build` | 已驗證 | 上述各次執行；`bridge --as-json` 用於 json-server 的 `paginate` |
| `bench` | 已驗證 | 上述各次執行；數字見 [real-world.md](/docs/real-world/overview) |
| `record --from-tests` | 已驗證（TS/JS），已測試（Python） | vercel/ms 的 Jest 測試套件 |
| `record` 與 `gate` 中原始版本的覆蓋率 | 已驗證（TS/JS） | Fuse.js：案例執行了它所匯入程式碼的 81%；其他語言目前還不會量測 |
| `--batch` | 已驗證 | Fuse.js、vercel/ms、x18n 搜尋 |
| wasm 橋接（`--kind wasm`） | 已驗證 | x18n 搜尋 |
| `shadow` | 已驗證 | x18n 的 `packages/core` |
| `profile` | 已測試 | |
| `app record`、`app replay` | 已驗證（伺服器與 CLI） | json-server：514 個 HTTP 案例，使用 `--fresh`、`--arg`、`--pace`；Prettier：3,080 個 CLI 案例，使用 `--inputs`；`tests/app.rs` 中的 CLI |
| 入口（`front-door`）、`wire` | 已驗證（TS 伺服器與 CLI） | Prettier：接上一個單元，3,080/3,080，回退率 57.8%，膠合程式碼以 `original::run_with_stdin` 逐案例退回；json-server：先接上讀取（22.6% 回退率），接著是每個路由（0%）；`tests/http.rs` 中八種語言的伺服器 |
| `lift` | 已驗證（TS 伺服器），已測試（CLI） | json-server：純 Rust 執行檔，514/514；`tests/app.rs` 中全部八種語言的 CLI |
| MCP 伺服器（`mcp`） | 已測試 | `src/mcp.rs` 中的協定測試 |

## 測試在哪裡執行

CI 在 macOS 與 Windows 上安裝 Node、Python、Go、Ruby 與 Java。它沒有安裝 PHP 或
.NET，所以 PHP 與 C# 的測試在那裡會被略過，只會在有安裝它們的機器上執行。
