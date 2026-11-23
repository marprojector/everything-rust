---
title: 函式庫
description: 函式庫沒有可以重播的行程。它的公開 API 就是入口，而每個單元已錄製的案例就是證明。
---

# 函式庫

函式庫沒有可以重播的行程：它的公開 API 就是入口，而每個單元已錄製的
案例就是證明。

```sh
everything-rust app front-door .        # rust/app：具有公開 API 的 Rust crate
# 每個單元都走過工作循環：bridge、record、port.rs、build、diff、gate；然後：
everything-rust app wire . textkit-rs   # 由 rust/app 重新匯出；原始版本匯入它的影子模組
everything-rust app replay .            # 每個已接上單元的案例；回退 = 仍在原始版本中的匯出
everything-rust app lift .
```

- 當儲存庫沒有 `main` 也沒有伺服器時（否則使用 `--library`），`app front-door` 會找出公開 API（每個單元的匯出），
  並將 `rust/app` 建立為函式庫 crate。每個已接上的
  單元都會成為它的一個模組：`pub use textkit_rs::port as textkit;`。
- `app wire` 接收由 `bridge` 建立的 crate，執行 `gate`，並寫入該單元可直接替換的影子模組：
  原始套件會以它取代該單元來匯入，所以它的呼叫端保留原本的 API，並且
  呼叫到 Rust。`EVERYTHING_RUST_UNIT_<NAME>=original` 會把該單元送回去（C：建置時不加上
  影子模組的旗標）。
- `app replay` 會透過綁定執行每個已接上單元的已錄製案例。回退率是
  Rust 尚未回應的公開匯出所佔的比例：尚未接上的單元，以及 `bridge` 保留
  在原始語言中的匯出。
- `app lift` 需要回退率為 0 且每個單元都通過 `gate`；它會從單元中移除綁定、包裝層與
  影子模組，讓 `rust/app` 成為單一的純 Rust 函式庫。它不會更動任何
  `port.rs`，所以透過綁定一致的案例仍然描述著它；檢查清單會
  對仍然匯入影子模組的原始檔案發出警告。

這五個也都是 MCP 工具：`app_record`、`app_replay`、`app_front_door`、`app_wire` 與
`app_lift`。
