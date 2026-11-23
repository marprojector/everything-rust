---
title: C#（預覽）
description: C# 目前為預覽。analyze 會讀取 .csproj 專案，而 C# CLI 或 ASP.NET 伺服器可以被錄製、重播，並放到 Rust 入口之後。
---

# C#（預覽）

| | |
| --- | --- |
| 單元 | 一個包含 `public static` 方法的資料夾 |
| 橋接 | 即將推出：透過 C ABI 的 P/Invoke |
| 目前 | 整個應用程式：`app record`、`app replay` 與入口，適用於 CLI 與 ASP.NET 伺服器（`dotnet <app>.dll`） |
| 狀態 | 預覽：僅支援 `analyze` 與整個應用程式；單元目前還無法橋接 |

`analyze` 會讀取 `.csproj` 專案及其 NuGet 套件（對應到 crate），將 `using` 指示詞解析到資料夾，並標出 C# 行為不同之處：`decimal`、`dynamic`、LINQ 的延遲查詢、`Task`、`lock`、反射。

錄製一個已建置的應用程式：

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

透過 P/Invoke 移植單元是下一步。在那之前，`bridge` 會以清楚的訊息拒絕 C# 單元。

CI 沒有安裝 .NET，所以 C# 的測試只會在有安裝它的機器上執行。Kotlin 與 Swift 目前還不支援；請為你需要的語言開一個 issue（或為現有的 issue 按讚）。
