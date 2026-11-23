---
title: 提升（lift）
description: 當重播以 0 回退率通過且每個單元都通過 gate 時，app lift 會讓 rust/app 成為純 Rust 應用程式。
---

# 提升（lift）

當重播以 0 回退率通過且每個單元都通過 gate 時，就把它變成純 Rust 應用程式：

```sh
everything-rust app lift .
```

- 在兩個條件都成立之前，`app lift` 會拒絕執行，並說明阻擋它的原因：仍在回退的案例、仍是空殼的膠合程式碼、未通過 gate 的單元。
- 它會先在副本上移除回退路徑與緊急開關，將每個單元 crate 變成 `rust/app` 的一般函式庫（沒有 `cdylib`、綁定或自動產生的包裝層），並加入 `rust/app/tests/app_cases.rs`，在 `cargo test` 時以 trycmd 執行 `rust/app-cases`，因此 CI 只需要 cargo。
- 接著它會執行提升檢查清單：對每個單元與 `rust/app` 執行 fmt、clippy `-D warnings` 與測試，在已安裝的地方建置或檢查 `rust/DECISIONS.md` 中列出的每個目標，確認執行期讀取的檔案與資源資料夾沒有被內嵌，並在純執行檔上重播。只有全部通過時，變更才會寫回。原始版本完全不會被更動。

## 提升之後

單元不再有任何綁定。在 json-server 上，每個 crate 的 `tests/recorded.rs` 會在移植版本上重播它的已錄製案例：全部 3,399 個都透過 `cargo test` 執行，與 514 個 HTTP 案例並列。

原始版本移除後，就可以自由地重構 Rust；`app replay` 仍然會證明行為正確。

## 然後切換

工作流程技能會讓 AI 代理完成它能做的部分，並撰寫 `rust/cutover.sh`，這是一個互動式指令稿，用於只有你能執行的步驟（部署、DNS、機密資訊），每一步都提供回復的方法。
