---
title: Go
description: 將 Go 套件移植到 Rust，透過具型別的 cgo 呼叫與自動產生的二進位線路協定，放在 API 與原始版本相同的套件中。
---

# Go

| | |
| --- | --- |
| 單元 | 一個套件 |
| 橋接 | 具型別的 cgo 呼叫與自動產生的二進位線路協定，放在 API 與原始版本相同的套件中 |
| 錄製與重播方式 | 自動產生的 Go 驅動程式，建置為獨立的 module |
| 需求 | Go 1.21+ |
| 狀態 | 已在 Go 1.24 的 `path` 上驗證：2,642/2,642（其中 542 個來自 Go 自己的測試表），gate 通過 |

`profile` 透過 `go test` 運作：`everything-rust profile <repo> -- go test -run X -bench . ./pkg`。批次為 `NewFnBatch(items)`；使用完畢後請關閉。

## 移植版本必須做對的地方

- `path.Clean`：Go 的 `lazybuf` 在寫入索引往回移動後，讀取的是該索引處的位元組，而不是最後保留的位元組（9 個案例，來自 Go 自己的測試表）。
- `strings.ToLower` 會逐一轉換每個 rune：`ΣΊΣΥΦΟΣ` 會變成 `σίσυφοσ`，而 Rust 的 `to_lowercase` 會套用詞尾 sigma 規則。Go 也會把 `İ` 對應到 `i`，而 Rust 會得到 `i̇`。
- Go 以位元組切割字串；切斷 rune 的結果在 JSON 中會變成每個位元組一個 U+FFFD。

## 速度

Go 執行末端函式只需要幾奈秒，而光是一次 cgo 呼叫就要約 80 ns。在 `path` 上，`bench` 量測到移植版本單次呼叫為 0.04x：請移植更大的單元。改用批次時，工作會搬到 Rust 並使用所有核心：`Slugify` 處理 50,000 個字串在 Go 中需要 49 ms，而批次**快了 4x**（單次呼叫移植：0.93x）。批次 crate 使用 mimalloc 配置記憶體：使用系統配置器時，Go 的批次甚至比單一執行緒還慢。

詳情：[六種語言](/docs/real-world/six-languages)。
