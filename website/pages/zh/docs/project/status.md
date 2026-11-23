---
title: 项目状态
description: everything-rust 目前哪些功能可用，以及每一部分最近一次是如何检验的。
---

# 项目状态

哪些功能可用，以及每一部分最近一次是如何检验的。更新于 2026-11-18。

**已验证**：在下面列出的、由他人编写的真实代码上端到端运行过（bridge、record、port、diff、gate）。
**已测试**：由本仓库中专门编写的测试夹具覆盖，尚未
在真实项目上运行过。**预览**：只适用于流程的一部分，如下所述。

## 迁移单元

| 语言 | 状态 | 最近一次真实运行 | 结果 |
| --- | --- | --- | --- |
| TypeScript/JavaScript | 已验证 | [Prettier](https://github.com/marprojector/everything-rust/blob/main/docs/real-world.md#prettier)（2026-11-18）、[json-server](/docs/real-world/json-server)（2026-11-18）、[Fuse.js](/docs/real-world/fuse-js)（2026-11-18）、[vercel/ms](/docs/real-world/vercel-ms)（2026-11-18）、x18n 的 `packages/core` | Prettier：`doc-printer-rs` 2,848/2,848，`js-format-rs` 1,279/1,279，两者均通过门禁；json-server：六个单元，3,399 条用例，每个都通过门禁；Fuse.js、vercel/ms、x18n：3,414/3,414、975/975 以及 9,000+ 条用例一致 |
| Python | 已验证 | CPython 3.13 的 `colorsys.py`（2026-11-16） | 2,400/2,400；门禁通过 |
| Go | 已验证 | Go 1.24 的 `path`（2026-11-16） | 2,642/2,642；门禁通过 |
| Ruby | 已验证 | Ruby 3.3 的 `shellwords.rb`（2026-11-16） | 1,617/1,617；门禁通过 |
| Java | 已验证 | 门面后的 JDK 21（2026-11-16） | 4,492/4,492；门禁通过 |
| C | 已验证 | 门面后的 glibc（2026-11-16） | 去掉一条未定义行为用例后 1,859/1,859；门禁通过 |
| PHP | 预览 | 无 | 仅支持 `analyze` 和整个应用；单元暂时还不能桥接 |
| C# | 预览 | 无 | 仅支持 `analyze` 和整个应用；单元暂时还不能桥接 |

每次运行的详情：[real-world.md](/docs/real-world/overview)（TS/JS）和
[real-world-languages.md](/docs/real-world/six-languages)（其他语言）。

## 命令

| 命令 | 状态 | 备注 |
| --- | --- | --- |
| `analyze` | 已验证 | 上面的每种语言，x18n 的 142 个单元 |
| `bridge`、`record`、`diff`、`gate`、`build` | 已验证 | 上面的各次运行；在 json-server 的 `paginate` 上使用 `bridge --as-json` |
| `bench` | 已验证 | 上面的各次运行；数字见 [real-world.md](/docs/real-world/overview) |
| `record --from-tests` | 已验证（TS/JS），已测试（Python） | vercel/ms 的 Jest 测试套件 |
| `record` 和 `gate` 中原实现的覆盖率 | 已验证（TS/JS） | Fuse.js：用例运行了它导入代码中 81% 的行；其他语言目前还不测量 |
| `--batch` | 已验证 | Fuse.js、vercel/ms、x18n 搜索 |
| wasm 桥接层（`--kind wasm`） | 已验证 | x18n 搜索 |
| `shadow` | 已验证 | x18n 的 `packages/core` |
| `profile` | 已测试 | |
| `app record`、`app replay` | 已验证（服务端和 CLI） | json-server：514 条 HTTP 用例，使用 `--fresh`、`--arg`、`--pace`；Prettier：3,080 条 CLI 用例，使用 `--inputs`；`tests/app.rs` 中的 CLI |
| 前门（`front-door`）、`wire` | 已验证（TS 服务端和 CLI） | Prettier：接入一个单元，3,080/3,080，回退率 57.8%，胶水代码通过 `original::run_with_stdin` 按用例放行；json-server：先接入读取操作（回退率 22.6%），再接入所有路由（0%）；`tests/http.rs` 中八种语言的服务端 |
| `lift` | 已验证（TS 服务端），已测试（CLI） | json-server：纯 Rust 二进制，514/514；`tests/app.rs` 中全部八种语言的 CLI |
| MCP 服务器（`mcp`） | 已测试 | `src/mcp.rs` 中的协议测试 |

## 测试在哪里运行

CI 在 macOS 和 Windows 上安装 Node、Python、Go、Ruby 和 Java。它不安装 PHP 或
.NET，所以 PHP 和 C# 测试在 CI 中会跳过，只会在装有它们的机器上运行。
