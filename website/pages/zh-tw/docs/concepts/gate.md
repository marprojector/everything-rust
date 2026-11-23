---
title: 閘門
description: gate 是一次移植的完成定義。它執行的每項檢查，以及如何修正失敗的檢查。
---

# 閘門

```sh
everything-rust gate <crate>
```

`gate` 通過才算完成。它會檢查：

- `cargo fmt`、加上 `-D warnings` 的 clippy，以及 cargo 測試；
- `unsafe` 只能搭配 `// SAFETY:` 註解使用，且沒有任何 `unimplemented!`；
- 原始版本每個刻意失敗的地方（`throw`、`raise`、`error` 結果）都有一個已錄製的失敗案例。當 gate 指出缺少時，用 `record --inputs` 新增一個；
- 在原始版本會回傳錯誤的函式中，不得使用 `unwrap`、`expect` 或 `panic!`。請回傳 `Err`；對於原始版本同樣視為不可能發生的狀態，加上 `// INFALLIBLE: <why>` 註解；
- 移植版本最多為原始版本行數的 3 倍（另加 50 行給匯入與簽章）；
- 模組可以建置，且每個匯出都存在；
- 每個已錄製的案例都一致，包括[已知差異](/docs/concepts/diff#known-diffs)，且錄製內容沒有[過期](/docs/concepts/diff#stale-recordings)；
- 對於 TS/JS，還有一行[覆蓋率](/docs/concepts/coverage)：案例執行了原始版本的多少程式碼。

每項失敗的檢查都附有一個 `fix`：請先執行它。

## 然後回報並等待

閘門通過後，工作流程技能會要求 AI 代理回報變更內容（檔案、函式、橋接、批次）、gate 的輸出、diff 的數量與 bench 的數字、剩下的行為差異或風險，以及計畫中尚未完成的部分，並在進行下一個單元之前等待你的同意。
