---
title: 語言
description: TypeScript/JavaScript、Python、C、Go、Ruby 與 Java，以及預覽中的 PHP 與 C#。每種語言的單元、橋接與錄製方式。
---

# 語言

有六種語言能走完整個工作循環：Node.js（TypeScript/JavaScript）、Python、C、Go、Ruby 與 Java。PHP 與 C# 目前為預覽：現在支援 `analyze` 與整個應用程式，單元橋接即將推出。

| 語言 | 單元 | 橋接 | 錄製與重播方式 |
| --- | --- | --- | --- |
| TypeScript/JavaScript | 資料夾或檔案 | napi-rs 或 wasm-bindgen，搭配自動產生的二進位線路協定；支援類別與 async | Node，直接執行 `.ts` 原始檔 |
| Python | 套件或 module | PyO3（abi3），直接轉換，具型別的例外 | Python，匯入原始版本 |
| C | `.c` 檔案 | 使用原始名稱的 `extern "C"` 函式，以及標頭檔 | 將原始程式編譯為共用函式庫，透過 ctypes 呼叫，每個案例一次 fork |
| Go | 套件 | 具型別的 cgo 呼叫與自動產生的二進位線路協定，放在 API 與原始版本相同的套件中 | 自動產生的 Go 驅動程式，建置為獨立的 module |
| Ruby | 包含 module 函式的檔案或資料夾 | 位於原始模組底下的 magnus 擴充 | 自動產生的 Ruby 驅動程式 |
| Java | 包含 `public static` 方法的套件 | 具型別的 JNI 與自動產生的二進位線路協定，放在具有原始方法的包裝類別中 | 自動產生的反射驅動程式 |
| PHP（預覽） | 包含函式與 `public static` 方法的資料夾 | 即將推出：透過 C ABI 的 PHP FFI | 目前支援整個應用程式：`app record`、`app replay` 與入口，適用於 CLI 與伺服器（`php -S`） |
| C#（預覽） | 包含 `public static` 方法的資料夾 | 即將推出：透過 C ABI 的 P/Invoke | 目前支援整個應用程式：`app record`、`app replay` 與入口，適用於 CLI 與 ASP.NET 伺服器（`dotnet <app>.dll`） |

除了 C 以外，每種語言也都有批次類別（`--batch`）：項目只跨越一次邊界並留在
Rust 中，每次呼叫都會使用所有核心處理全部項目。

語言會從儲存庫自動偵測（`go.mod`、`pom.xml`、`.csproj` 或 `.sln`、
`pyproject.toml`、`Gemfile`、`composer.json`、`package.json`，否則依最常見的原始檔判斷）；
`--lang` 可以覆寫它。型別來自原始碼：TS 型別、Python 型別提示、C、Go、Java 與 C# 的
簽章、Ruby 的 YARD 標籤、PHP 型別與 phpdoc；當作數字使用的未標型別參數會被推斷
為數字。

PHP 目前為預覽：`analyze` 可以規劃 PHP 應用程式（Composer 套件與 PSR-4 自動載入、
寬鬆比較與整數溢位成浮點數等風險、Composer 套件對應到 crate），
而 PHP CLI 或伺服器可以被錄製、重播，並放到 Rust 入口之後。透過 PHP 的 FFI 將其
單元移植到 Rust 是下一步（[issue #2](https://github.com/marprojector/everything-rust/issues/2)）。

C# 同樣處於預覽：`analyze` 會讀取 `.csproj` 專案及其 NuGet 套件
（對應到 crate），將 `using` 指示詞解析到資料夾，並標出 C# 行為不同之處
（`decimal`、`dynamic`、LINQ 的延遲查詢、`Task`、`lock`、反射）。C# CLI 或 ASP.NET
伺服器可以被錄製、重播，並放到 Rust 入口之後；透過 P/Invoke 移植單元
是下一步。
## 各語言的狀態

| 語言 | 狀態 | 最近一次實際執行 |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | 已驗證 | json-server、Fuse.js、vercel/ms、x18n 的 `packages/core` |
| [Python](/docs/languages/python) | 已驗證 | CPython 3.13 的 `colorsys.py`：2,400/2,400 |
| [Go](/docs/languages/go) | 已驗證 | Go 1.24 的 `path`：2,642/2,642 |
| [Ruby](/docs/languages/ruby) | 已驗證 | Ruby 3.3 的 `shellwords.rb`：1,617/1,617 |
| [Java](/docs/languages/java) | 已驗證 | 外觀層後的 JDK 21：4,492/4,492 |
| [C](/docs/languages/c) | 已驗證 | 外觀層後的 glibc：移除一個未定義行為的案例後為 1,859/1,859 |
| [PHP](/docs/languages/php) | 預覽 | 僅支援 `analyze` 與整個應用程式 |
| [C#](/docs/languages/csharp) | 預覽 | 僅支援 `analyze` 與整個應用程式 |

包含指令的完整表格位於[狀態](/docs/project/status)頁面。
