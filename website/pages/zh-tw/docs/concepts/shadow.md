---
title: 影子模式
description: shadow 會產生一個可直接替換的模組，將原始版本與 Rust 並行執行，並回報不一致之處。
---

# 影子模式

```sh
everything-rust shadow <crate>
```

`shadow` 會產生一個與原始版本 API 相同、可直接替換的模組：TS 或 Python 模組、Go 套件、Ruby 檔案、Java 類別或 C 檔案。在應用程式原本載入原始版本的地方載入它。它會同時執行兩者，回傳原始版本的結果，並回報 Rust 的不一致之處，讓你在正式環境中逐步建立信心。

| `EVERYTHING_RUST_MODE` | 行為 |
| --- | --- |
| `shadow`（預設） | 同時執行兩者，回傳原始版本的結果，回報不一致之處 |
| `new` | 切換為 Rust |
| `old` | 關閉 Rust |

公開 API 不會改變。呼叫端匯入相同的函式，或改為匯入影子模組。
