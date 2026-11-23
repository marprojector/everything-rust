---
title: 錄製與重播
description: app record 會以應用程式文件中的範例、你的輸入與陷阱案例執行整個應用程式；app replay 會拿任何建置版本來比對這些案例。
---

# 錄製與重播

`record` 與 `diff` 證明單一單元；`app record` 與 `app replay` 證明整個應用程式，適用於六種語言中任一種的 CLI 與伺服器。應用程式被當成黑盒子。

```sh
everything-rust app record . -- python3 cli.py          # 執行原始版本的指令
everything-rust app replay .                            # 原始版本必須 100% 通過
everything-rust app replay . --build "target/release/mytool"
```

## 案例從哪裡來

`app record` 會以下列內容執行原始版本：

- 應用程式自己文件中的範例：Markdown 與 shell 檔案中的 `$ mytool ...` 行，以及它們提到的檔案；
- 一個 `--inputs` JSONL 檔案，每行一個 `{"name", "args", "stdin", "env", "files"}`；
- 從這些案例衍生的語意陷阱：非 ASCII 與 emoji 字串、負數除法與取餘數、溢位、極大與極小的浮點數、map 順序、缺少的檔案。

每個案例都會執行兩次；兩次執行之間的差異會變成 `[..]` 萬用字元。`--normalize timestamps|uuids|key-order` 或 `--normalize name=regex` 會加入正規化器，並與案例一起儲存。`app record` 會依類型回報差異：值（現在是萬用字元）、順序（以任意順序比較各行）與行數（可以符合任何內容的 `...`）。請逐一檢查。

## 案例是一般檔案

案例是位於 `rust/app-cases/` 中的 [trycmd](https://docs.rs/trycmd) 檔案：

| 檔案 | 內容 |
| --- | --- |
| `<case>.toml` | argv、環境變數、結束代碼 |
| `<case>.stdin` | 標準輸入 |
| `<case>.stdout`、`<case>.stderr` | 原始版本印出的內容 |
| `<case>.in/` | 輸入的檔案 |
| `<case>.out/` | 寫出的檔案 |

你可以手動閱讀與編輯它們；一旦移植版本有了執行檔，Rust 測試就能用 `trycmd` 執行它們。

## 重播

- `app replay` 會拿任何建置版本來比對這些案例，印出 `passed/total`，寫入 `rust/app-cases/replay-report.md` 並為每個失敗的案例附上 diff，且在出現任何不一致時以非零值結束。
- 像[入口](/docs/whole-apps/front-door)那樣，會附加寫入 `EVERYTHING_RUST_FALLBACK_LOG` 所指定檔案的建置版本（每次回退到原始版本寫一行），也會回報它的回退率。
- `app replay --case <name>` 會以報告給出的名稱重新執行單一案例：這是在重新執行全部案例之前，修正不一致的快速、可重複的循環。

## 在 CI 中重播

報告中沒有計時或暫存路徑：同一個建置版本的兩次重播會寫出相同的檔案。請提交它，然後在 CI 中執行：

```sh
everything-rust app replay .
git diff --exit-code rust/app-cases/replay-report.md
```

這樣一來，移植到一半的應用程式只要通過的內容有任何變化，無論變好或變差，CI 都會失敗，直到報告隨之更新為止。

## 數值輸出

`app record --float-tolerance 1e-9` 讓數字在較大值的該比例內即視為一致。請參閱[浮點容差](/docs/concepts/float-tolerance)。
