---
title: 什麼是 everything-rust
description: everything-rust 會錄製你的應用程式目前的行為，並將 Rust 重寫的每個部分與該錄製內容比對，直到應用程式成為純 Rust。
---

# 什麼是 everything-rust

<p class="lead">一次一個部分，用 Rust 重寫你的整個應用程式。everything-rust 會錄製你的 Node.js（TypeScript/JavaScript）、Python、C、Go、Ruby 或 Java 應用程式目前的行為，然後在繼續下一步之前，將 Rust 重寫的每個部分與該錄製內容比對，直到應用程式成為純 Rust。</p>

它適用於 CLI、HTTP 伺服器與函式庫。Rust 程式碼由你的程式開發 AI 代理（Claude Code、Codex、Cursor、Gemini CLI）或你自己撰寫。PHP 與 C# 目前為[預覽](/docs/languages/php)。

## 它做什麼，不做什麼

everything-rust 不翻譯程式碼。它提供的是 AI 代理無法可靠猜到的東西：

- **時間花在哪裡**：`profile` 執行真實的工作負載，並為熱點函式排名；
- **先移植什麼、按什麼順序**：`analyze` 規劃單元、波次、風險與 crate；
- **一座橋接**，讓其餘程式碼以不變的 API 呼叫 Rust（`bridge`）；
- **移植版本必須符合的確切行為**：`record` 以邊界案例執行原始程式，並儲存它回傳的值或擲出的例外；
- **安全的出貨方式**：`shadow` 將兩者並行執行；
- **一道判定移植何時完成的閘門**：`gate`。

原始版本始終是唯一可信來源。`diff` 會將每個已錄製的案例拿來重播比對 Rust 版本，在全部一致且 `gate` 通過之前，什麼都不會出貨。

## 四個步驟

| 步驟 | 發生什麼事 |
| --- | --- |
| **錄製** | `record` 會以自動產生的邊界案例、語意陷阱以及你自己的輸入執行原始程式，並儲存它回傳的值或擲出的例外。 |
| **移植** | `bridge` 會產生 Rust crate 與包裝層，讓呼叫端繼續使用原本的 API。Rust 程式碼由你或你的 AI 代理撰寫。 |
| **檢查** | `diff` 會將每個已錄製的案例拿來重播比對 Rust 版本，在全部一致且 `gate` 通過之前，什麼都不會出貨。 |
| **提升** | 當重播在沒有回退的情況下通過，且每個單元都通過 gate，`app lift` 就會把它變成純 Rust 應用程式。原始版本完全不會被更動。 |

## Rust 一定比較快嗎？

不一定，而 everything-rust 會老實告訴你。每個項目呼叫一次的小函式在 Rust 中往往比較慢，因為每次呼叫都要跨越橋接；V8、JVM 與 Go 執行這類函式只需要幾奈秒。`profile` 會找出包住它的迴圈，`--batch` 會把整個迴圈搬過去，而 `bench` 會在你出貨前給出結論。實測數字（包括較慢的結果）請見[批次與熱路徑](/docs/concepts/batches)。

## 它只能抓到錄製內容涵蓋的部分

通過代表 Rust 與每個已錄製的案例一致，僅此而已。在信任綠燈的 gate 之前，請先閱讀[它抓不到的問題](/docs/concepts/limits)。

::: warning 請在副本上執行
`record` 與 `app record` 會執行你的原始程式碼，以大量自動產生的輸入呼叫每個函式（或整個應用程式），而 `app record` 會把每個案例執行兩次。`diff`、`gate` 與 `app replay` 會在移植版本或原始版本上再次執行相同的案例。會寫入檔案、存取資料庫或呼叫網路的程式碼，每個案例都會做一次這些事。請在專案的副本上執行它們，使用測試資料庫，且環境中不要有正式環境的憑證。
:::

## 下一步

- [快速開始](/docs/introduction/quick-start)：安裝它，並請你的 AI 代理進行第一次移植。
- [工作循環](/docs/concepts/the-loop)：單一單元的每個步驟。
- [移植整個應用程式](/docs/whole-apps/overview)：從第一次錄製到純 Rust 執行檔。
- [實際移植案例](/docs/real-world/overview)：json-server、Fuse.js、vercel/ms 等，較慢的結果與較快的結果並列。
