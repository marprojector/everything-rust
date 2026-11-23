---
title: Doctor 健康檢查
description: everything-rust doctor 會檢查語言執行環境、AI 代理設定與 MCP 伺服器，並列出修正每個問題的指令。
---

# Doctor 健康檢查

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor` 會檢查語言執行環境、AI 代理設定與 MCP 伺服器，並印出修正每個問題的指令。當缺少必要項目時，它會以 1 結束。

當某個步驟因為執行環境、AI 代理設定或 MCP 伺服器缺少或版本過舊而失敗時，請執行它。工作流程技能也會要求 AI 代理這樣做。

## 每種語言需要什麼

`record`、`diff` 與 `bench` 會在原始程式自己的執行環境中執行它：TS 需要 Node 22.6+（它直接執行 `.ts` 原始檔），Python 與 C（ctypes）需要 Python 3.9+，Go 需要 1.21+，Ruby 需要 3+，Java 需要 JDK 17+（`javac` 與 `java`）。在 Node 上執行 `profile` 需要 Node 24+，才能在 `NODE_OPTIONS` 中使用 `--cpu-prof`。C 還需要 C 編譯器。wasm 橋接還需要 `rustup target add wasm32-unknown-unknown` 與對應版本的 `wasm-bindgen-cli`；`build` 會指出確切的版本。

## 給貢獻者

`cargo run -- doctor` 會列出你有哪些執行環境、它們的版本，以及安裝或更新每一個的指令。請參閱[參與貢獻](/docs/project/contributing)。
