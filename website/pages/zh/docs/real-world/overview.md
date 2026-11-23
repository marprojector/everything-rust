---
title: 真实项目迁移
description: everything-rust 遇到真实代码时发生了什么，慢的结果与快的结果并列，以及工具因此做了哪些改变。
---

# 真实项目迁移

everything-rust 遇到真实代码时发生了什么，以及工具因此做了哪些改变。下面的每个数字都来自一个会先校验结果完全一致的基准测试，慢的情况也与快的情况并列列出。

| 迁移 | 是什么 | 结果 |
| --- | --- | --- |
| [json-server](/docs/real-world/json-server) | 一个完整的 Node 服务端，用 `app record`、`front-door`、`wire` 和 `lift` 端到端迁移 | 一个通过全部 514/514 条录制 HTTP 用例的纯 Rust 二进制；六个单元通过 `gate` |
| [Fuse.js](/docs/real-world/fuse-js) | 模糊搜索，搜索耗时所在的路径 | 3,414/3,414 条用例一致；在 50,000 个字符串上快 32 倍（单核为 5.3x） |
| [vercel/ms](/docs/real-world/vercel-ms) | 一个很小的时长解析器，端到端迁移 | 975/975 条用例一致；逐次调用 0.56x，`parse` 批处理 18x，`format` 批处理 0.8x |
| [x18n](/docs/real-world/x18n) | 案例研究仓库 | `packages/core` 上每个门禁都通过，经过桥接层后比 TS 慢；它的搜索以批处理方式快 18-22 倍 |
| [六种语言](/docs/real-world/six-languages) | Go 的 `path`、Ruby 的 `shellwords`、Python 的 `colorsys`，以及放在小型门面后的 JDK 和 glibc 行为 | 每个移植版都通过门禁，共 13,010 条录制用例 |
| [测试夹具与静态运行](/docs/real-world/fixtures) | 为给工具施压而编写的测试夹具，以及只做了分析而没有运行的库 | |

在真实项目上做过迁移？欢迎写一篇总结：见[参与贡献](/docs/project/contributing#share-a-real-port)。
