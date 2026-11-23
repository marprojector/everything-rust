---
title: 入口與線路協定
description: app front-door 會在原始版本前面放一個 Rust 執行檔；app wire 會在單元通過 gate 後，把指令交給該單元的移植版本。
---

# 入口與線路協定

先把進入點搬到 Rust，再一次一個單元地把指令交給它：

```sh
everything-rust app front-door .                            # rust/app：每個指令都執行原始版本
everything-rust app wire . textkit --command upper          # 在 textkit-rs 通過 gate 之後
# 在 rust/app/src/units/textkit.rs 中撰寫膠合程式碼，然後：
everything-rust app wire . textkit                          # 再次 100%，回退率降低
```

## 入口

`app front-door` 會建立 `rust/app` 的骨架，這是一個以應用程式命名的 Rust 執行檔：一個針對已錄製案例中的指令與旗標的 clap 解析器，其中每個指令都會以相同的 argv、stdin 與環境執行原始版本，並原樣傳遞它的輸出與結束代碼。每次回退都會被計數，所以重播剛建立的入口會得到 100% 通過、100% 回退（它會建置並重播來證明這一點）。它從不覆寫 `rust/app`：從此以後程式碼歸你所有。

## 接上單元

`app wire <repo> <unit> --command <name>`：

1. 對單元 crate 執行 `gate`（如果它剛通過，可以用 `--skip-gate`）；
2. 將它以函式庫的形式加入 `rust/app`；
3. 寫入一次 `src/units/<unit>.rs`，讓你或你的 AI 代理填寫：解析參數、呼叫 `<crate>::port`、印出原始版本會印出的內容；
4. 將指令路由到它，然後建置並重播。

撰寫完膠合程式碼後再執行一次：100% 通過，且回退率降低。`EVERYTHING_RUST_UNIT_<NAME>=original` 會在提升之前，把已接上的單元送回原始版本。

把應用程式自己的名稱接為一個指令（`--command mytool`），用來處理沒有指定指令的 argv：沒有參數、未知的指令、用法錯誤。

## 沒有任何東西會回頭呼叫原始版本

入口會以獨立行程執行原始版本（或將 HTTP 轉發給它），已橋接的單元由原始版本呼叫，絕不反過來，而回呼則留在來源語言中。

伺服器使用路由而不是指令：請參閱[伺服器](/docs/whole-apps/servers)。函式庫使用它們的公開 API：請參閱[函式庫](/docs/whole-apps/libraries)。
