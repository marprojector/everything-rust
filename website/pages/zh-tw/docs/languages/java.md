---
title: Java
description: 將包含 public static 方法的套件移植到 Rust，透過具型別的 JNI 與自動產生的二進位線路協定，放在具有原始方法的包裝類別中。
---

# Java

| | |
| --- | --- |
| 單元 | 一個包含 `public static` 方法的套件 |
| 橋接 | 具型別的 JNI 與自動產生的二進位線路協定，放在具有原始方法的包裝類別中 |
| 錄製與重播方式 | 自動產生的反射驅動程式 |
| 需求 | JDK 17+（`javac` 與 `java`） |
| 狀態 | 已在外觀層後的 JDK 21 上驗證：4,492/4,492（226 個擲出例外），gate 通過 |

批次為 `new FnBatch(items)`；使用完畢後請關閉。`profile` 使用 JFR。

## 移植版本必須做對的地方

- `Integer.parseInt` 透過 `Character.digit` 接受任何 Unicode 十進位數字（`١٢`、`２`）。
- `Double.toString`（JDK 19+）會輸出最短的十進位表示，但絕不少於兩位數：`4.9E-324`，而 Rust 的最短形式是 `5e-324`。
- `\s` 只涵蓋 ASCII；Rust 的 `split_whitespace` 也會在 U+00A0 處分割。
- 浮點數必須能完整往返轉換：`serde_json` 預設的浮點數解析器並不總是正確捨入，所以 everything-rust 以及它產生的每個 crate 都使用它的 `float_roundtrip` 功能。

## 速度

經過 JIT 後，末端函式只需要幾奈秒，而帶字串的 JNI 呼叫成本比這還高。在 JDK 外觀層上，`bench` 量測到移植版本單次呼叫為 0.18x：請移植更大的單元。改用 4 核心上的批次時，`slugify` 處理 50,000 個字串（Java 中需 43 ms）**快了 3x**（單次呼叫移植：0.68x）。

詳情：[六種語言](/docs/real-world/six-languages)。
