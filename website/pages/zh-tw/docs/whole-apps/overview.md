---
title: 移植整個應用程式
description: 從第一次錄製到純 Rust 執行檔。目標是一個行為與原始版本相同的 Rust 應用程式，並保留原始版本作為參考。
---

# 移植整個應用程式

目標是一個行為與原始版本相同的純 Rust 應用程式，並保留原始版本作為參考。Rust 程式碼由你的 AI 代理撰寫；everything-rust 負責安排工作順序、建立骨架，並證明每一步都正確。適用於 CLI、HTTP 伺服器與函式庫。

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in    pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down proved by
 decisions to make      outside         original answers                                           replay
```

## 1. 規劃

```sh
everything-rust analyze . --whole-app
```

`--whole-app` 會加上應用程式的型態（CLI、伺服器或函式庫）與進入點、每個外部相依套件（已匯入或已宣告）及其 Rust crate 與適配程度、以區塊表示的移植順序（互相匯入的單元會一起搬移），以及附有建議答案的編號決策：保留的單元、受阻的單元、沒有對應 crate 或只有部分對應的相依套件。

## 2. 錄製整個應用程式，再搬移進入點

1. [`app record` 與 `app replay`](/docs/whole-apps/record-replay) 會從外部把程式當成黑盒子來證明。
2. [`app front-door`](/docs/whole-apps/front-door) 會在原始版本前面放一個 Rust 執行檔；每個指令仍然執行原始版本。
3. 每個單元都走過[工作循環](/docs/concepts/the-loop)，然後 `app wire` 會把它的指令交給 Rust。
4. 當重播以 0 回退率通過且每個單元都通過 gate 時，[`app lift`](/docs/whole-apps/lift) 會移除原始版本。

[伺服器](/docs/whole-apps/servers)與[函式庫](/docs/whole-apps/libraries)使用相同的指令。

## AI 代理如何進行

[整個應用程式技能](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md)會引導 AI 代理完成整個流程，從只有你能做的決策開始：

- **連番提問**：AI 代理會分輪次提問，每個問題都附上建議答案，直到移植所依賴的每個決策都定案為止，並將每個決策以 ADR 的形式寫入 `rust/DECISIONS.md`，同時以 `rust/GLOSSARY.md` 將原始版本的術語對應到 Rust 名稱。
- **研究**：背景 AI 代理會閱讀每個相依套件的第一手資料，並撰寫 `rust/research/<topic>.md`，記錄它們找到的每個語意陷阱。
- **跨工作階段**：`rust/MAP.md` 列出每個單元及其狀態；一個工作階段在開始前會認領一個單元，而每個檢查點都會為下一個工作階段留下 `rust/HANDOFF.md`。同一波次中的單元可以由子代理並行移植。
- **切換**：AI 代理會完成它能做的部分，並撰寫 `rust/cutover.sh`，這是一個互動式指令稿，用於只有你能執行的步驟（部署、DNS、機密資訊），每一步都提供回復的方法。

連番提問、研究與這些做法改編自 [Matt Pocock 的技能](https://github.com/mattpocock/skills)。

## 這不會把 Rust 綁死在舊的設計上嗎？

只有在兩者並行執行的期間會。錄製內容檢查的是應用程式邊緣的行為（CLI 的輸出、伺服器的回應、模組匯出的函式），而不是它的內部。在單元仍然跨越橋接的期間，請沿用舊的結構；一旦 `app lift` 移除了原始版本，就可以自由地重構 Rust，而 `app replay` 仍然會證明行為正確。

## 一個真實案例

[json-server](/docs/real-world/json-server) 走過了每一步：514 個已錄製的 HTTP 案例、99.8% 回退率的入口、先接上讀取（22.6% 回退率），接著是每個路由（0%），最後提升為通過 514/514 的純 Rust 執行檔。
