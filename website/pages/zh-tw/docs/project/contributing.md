---
title: 參與貢獻
description: 環境設定、測試放在哪裡、如何新增 crate 對應或語言，以及如何分享真實的移植案例。
---

# 參與貢獻

感謝你的協助。錯誤回報、修正、新語言支援以及真實移植案例的心得都非常歡迎，
而且修正錯誤的 pull request 不需要事先詢問就可以直接開。

## 開始之前

- **錯誤：** 開一個 [issue](https://github.com/marprojector/everything-rust/issues/new/choose)，附上
  你執行的指令、它的輸出、你的平台以及 everything-rust 的版本
  （`everything-rust --version`）。一個很小的重現專案最有幫助。
- **較大的變更**（新語言、新指令、改變單元或錄製內容的定義）：
  請先開一個 issue，讓我們在你動手撰寫之前先對方向達成共識。過去各項
  選擇背後的理由記錄在 [DECISIONS.md](https://github.com/marprojector/everything-rust/blob/main/DECISIONS.md)。
- **安全性問題：** 請不要開公開的 issue；請參閱 [SECURITY.md](https://github.com/marprojector/everything-rust/blob/main/SECURITY.md)。

## 環境設定

你需要 Rust（stable，1.97 或更新版本）。測試也會執行每個部分所處理的語言，並
略過執行環境缺少的部分，所以請安裝你會動到的那些：

| 用途 | 安裝 |
| --- | --- |
| TypeScript/JavaScript | Node 24（22.6+ 可用，`profile` 除外） |
| Python | Python 3.9+ |
| C | C 編譯器（`cc`） |
| Go | Go 1.22+ |
| Ruby | Ruby 3.x |
| Java | JDK（17+） |
| PHP | PHP 8 |
| C# | .NET SDK |
| wasm 橋接 | `rustup target add wasm32-unknown-unknown` 與對應版本的 `wasm-bindgen-cli` |

```sh
git clone https://github.com/marprojector/everything-rust
cd everything-rust
git config core.hooksPath .githooks
cargo test
```

`cargo run -- doctor` 會列出你有哪些執行環境、它們的版本，以及安裝或更新每一個
的指令。若要讓缺少的執行環境導致測試失敗而不是略過
（CI 就是這樣做的），請將它列在 `EVERYTHING_RUST_REQUIRE` 中，例如
`EVERYTHING_RUST_REQUIRE=node,python3,go,ruby,java cargo test`。

這個 hook 會執行 `scripts/check-secrets.sh`，拒絕你提交的內容中出現金鑰、權杖與個人家目錄
路徑。CI 會對每個檔案執行它。

## 進行變更

- **每個修正都要附上一個沒有該修正就會失敗的測試。** 請先對舊程式碼執行一次該測試，
  確認它確實會失敗。
- **測試放在哪裡：**
  - `tests/analyze.rs`：語言偵測、單元、風險、相依套件、`--whole-app`。
  - `tests/loop.rs`：各語言的 bridge、record、diff 與 shadow，以替身取代實際建置。
  - `tests/native.rs`：實際建置的橋接，然後載入並重播。
  - `tests/app.rs` 與 `tests/http.rs`：整個應用程式的錄製、重播、入口與提升，適用於 CLI 與
    伺服器。
  - `tests/library.rs`：整個函式庫的移植。
  - 單元測試放在程式碼旁邊。
- **Fixture** 是位於 `tests/fixtures/` 底下的小型應用程式。CLI 與 HTTP fixture 在每種語言中都實作了相同的
  指令與路由（`tcli upper|len|div|words|copy|now`、`/hello`、`/upper`、
  `/div/{a}/{b}`、`/now`），所以新增的 fixture 行為必須與其他的完全相同。
- **自動產生的程式碼**（everything-rust 撰寫的橋接、驅動程式與入口）也必須通過
  `clippy -D warnings`，因為 `gate` 會在使用者的 crate 上執行它。
- **風格：** 與周圍的程式碼保持一致。註解只用來說明程式碼本身無法表達的事，
  而變更的理由請寫在 pull request 中。

## 新增 crate 對應

`analyze` 會根據 `src/crates.toml`，為程式碼匯入的每個函式庫指出對應的 Rust crate。若要新增
一筆，請在對應語言底下附加一個項目：`names`（確切的套件名稱）或 `prefixes`（名稱的
開頭，用於 Java 與 C#）、`crates`、`fit`（`good`、`partial` 或 `none`），以及當適配程度
需要說明時加上 `note`。第一個符合的項目勝出，所以請把具體的項目放在廣泛的前綴之前。
對照表中沒有的函式庫會回報為 `unknown`；絕對不要猜測 crate。

## 新增語言

PHP 與 C# 是在兩個值得先閱讀的 commit 中以預覽形式加入的：
[e8ac243](https://github.com/marprojector/everything-rust/commit/e8ac243)（PHP）與
[caea59a](https://github.com/marprojector/everything-rust/commit/caea59a)（C#）。新語言的開始
方式相同：

1. 在 `src/lang/<name>.rs` 中撰寫外掛：探索、掃描（匯入與風險）、解析與匯出，
   並將該語言函式庫對應的 crate 以 `[[<name>]]` 項目加入 `src/crates.toml`。
2. 在 `src/lang/mod.rs` 中加入偵測與分派。
3. 在 `src/whole_app.rs` 中加入整個應用程式的支援：進入點、伺服器、已宣告的相依套件。
4. CLI 與 HTTP fixture、一個函式庫 fixture，以及 `tests/analyze.rs`、`tests/app.rs`
   與 `tests/http.rs` 中的測試。
5. 在橋接存在之前，`bridge` 會以清楚的訊息拒絕該語言的單元。

## 翻譯 README

歡迎翻譯，翻譯由社群維護。新的翻譯：

1. 放在 `docs/README_<CODE>.md`（例如 `docs/README_PT.md`）；根目錄的 `README.md` 維持
   英文，絕不重新命名或取代。
2. 使用在 `docs/` 中有效的路徑：`assets/logo.svg`、`../LICENSE`、`../README.md`。
3. 以與其他翻譯相同的語言列開頭，其中自己的語言以純文字表示，
   接著是說明該翻譯由社群維護、可能落後於英文 README 的註記
   （從 `docs/README_DE.md` 複製並翻譯）。
4. 加在語言列的**最後面**，包括 `README.md` 與每一份翻譯。已列出的語言
   順序不變。

## 文件網站

文件網站位於 `website/`（Void、`@void/md`、React）。它的許多頁面會在建置時引入
`README.md`、`CONTRIBUTING.md` 與 `docs/*.md` 的段落，所以只要編輯這些檔案，
網站就會跟著更新。用 `cd website && npm ci --ignore-scripts && npm run dev` 執行它，並用
`npm run build` 建置。CLI 參考來自 `website/content/cli-help.json` 中的 `--help`
快照；修改 CLI 之後，請在 `website/` 中執行 `cargo build --release` 與
`npm run cli-help` 更新它。

## 開 pull request

推送之前：

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

- 說明哪裡有問題、如何重現，以及這個變更如何修正它。
- 連結 issue（`Fixes #123`）。
- 註明任何你無法測試的部分（其他作業系統、你沒有的執行環境）。
- 一次只做一個變更：兩個修正分成兩個 pull request 會比較容易審查。

CI 在安裝了所有執行環境的 Ubuntu 與 macOS 上執行。對於第一次貢獻的人，GitHub
會等待維護者核准 CI 執行，所以在檢查開始之前預期會有短暫的延遲。

Pull request 會保留其 commit 進行合併，所以你的歷史紀錄仍然屬於你。如果變更需要
小幅調整，維護者可能會直接在上面加一個 commit，而不是要求你再改一輪。

## 分享真實的移植案例

真實專案的移植是證明什麼可行、什麼會壞掉的最佳證據。如果你曾在真實的專案上使用
everything-rust，非常歡迎開一個 issue 或 pull request，把它加入
[docs/real-world.md](/docs/real-world/overview)。請附上專案、你執行的
指令、案例數量，以及任何讓你意外的地方。

## 行為準則

所有參與者都應遵守[行為準則](https://github.com/marprojector/everything-rust/blob/main/CODE_OF_CONDUCT.md)。
