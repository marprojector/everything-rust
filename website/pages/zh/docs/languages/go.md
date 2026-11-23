---
title: Go
description: 通过类型化 cgo 调用和生成的二进制传输协议，把 Go 包迁移到 Rust，放在保持原 API 的包中。
---

# Go

| | |
| --- | --- |
| 单元 | 一个包 |
| 桥接层 | 类型化 cgo 调用与生成的二进制传输协议，放在保持原 API 的包中 |
| 录制与回放方式 | 生成的 Go 驱动，作为独立模块构建 |
| 依赖 | Go 1.21+ |
| 状态 | 已在 Go 1.24 的 `path` 上验证：2,642/2,642（其中 542 条来自 Go 自身的测试表），门禁通过 |

`profile` 通过 `go test` 工作：`everything-rust profile <repo> -- go test -run X -bench . ./pkg`。批处理通过 `NewFnBatch(items)` 创建；用完后需要关闭。

## 移植版必须做对的地方

- `path.Clean`：Go 的 `lazybuf` 在写入索引回退后读取的是该索引处的字节，而不是最后保留的字节（9 条用例，来自 Go 自身的测试表）。
- `strings.ToLower` 逐个 rune 转小写：`ΣΊΣΥΦΟΣ` 变成 `σίσυφοσ`，而 Rust 的 `to_lowercase` 会应用词尾西格玛规则。Go 还会把 `İ` 映射为 `i`，而 Rust 得到的是 `i̇`。
- Go 按字节切分字符串；切断一个 rune 后，到 JSON 中每个字节都会变成一个 U+FFFD。

## 速度

Go 运行叶子函数只需几纳秒，而单次 cgo 调用本身就要约 80 ns。在 `path` 上，`bench` 测得移植版逐次调用为 0.04x：请迁移更大的单元。改成批处理后，工作被迁到 Rust 并在全部核心上运行：`Slugify` 处理 50,000 个字符串在 Go 中耗时 49 ms，批处理版**快 4 倍**（逐次调用的移植版：0.93x）。批处理 crate 使用 mimalloc 分配内存：使用系统分配器时，Go 批处理比单线程还慢。

详情：[六种语言](/docs/real-world/six-languages)。
