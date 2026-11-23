---
title: Java
description: 通过类型化 JNI 和生成的二进制传输协议，把由 public static 方法组成的包迁移到 Rust，放在保持原方法的包装类中。
---

# Java

| | |
| --- | --- |
| 单元 | 包含 `public static` 方法的包 |
| 桥接层 | 类型化 JNI 与生成的二进制传输协议，放在保持原方法的包装类中 |
| 录制与回放方式 | 生成的反射驱动 |
| 依赖 | JDK 17+（`javac` 和 `java`） |
| 状态 | 已在门面后的 JDK 21 上验证：4,492/4,492（226 条抛出异常），门禁通过 |

批处理通过 `new FnBatch(items)` 创建；用完后需要关闭。`profile` 使用 JFR。

## 移植版必须做对的地方

- `Integer.parseInt` 通过 `Character.digit` 接受任何 Unicode 十进制数字（`١٢`、`２`）。
- `Double.toString`（JDK 19+）输出最短的十进制表示，但至少两位数字：`4.9E-324`，而 Rust 的最短形式是 `5e-324`。
- `\s` 只匹配 ASCII；Rust 的 `split_whitespace` 还会在 U+00A0 处切分。
- 浮点数必须能无损往返：`serde_json` 默认的浮点解析器并不总是正确舍入，所以 everything-rust 及其生成的每个 crate 都启用了它的 `float_roundtrip` 特性。

## 速度

经过 JIT 之后，叶子函数只需几纳秒，而一次带字符串的 JNI 调用开销比这更大。在 JDK 门面上，`bench` 测得移植版逐次调用为 0.18x：请迁移更大的单元。改成 4 核批处理后，`slugify` 处理 50,000 个字符串（Java 中 43 ms）**快 3 倍**（逐次调用的移植版：0.68x）。

详情：[六种语言](/docs/real-world/six-languages)。
