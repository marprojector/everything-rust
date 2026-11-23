---
title: 收口
description: 当回放在回退率为 0 的情况下通过、且每个单元都通过门禁后，app lift 把 rust/app 变成纯 Rust 应用。
---

# 收口

当回放在回退率为 0 的情况下通过、且每个单元都通过门禁时，就可以把它变成纯 Rust 应用：

```sh
everything-rust app lift .
```

- 在这两个条件都满足之前，`app lift` 会拒绝执行，并说明卡在哪里：仍在回退的用例、仍是桩函数的胶水代码、未通过门禁的单元。
- 它会先在副本上操作：移除回退路径和应急开关，把每个单元 crate 变成 `rust/app` 的普通库（不再有 `cdylib`、绑定代码或生成的包装层），并添加 `rust/app/tests/app_cases.rs`，让 `cargo test` 通过 trycmd 运行 `rust/app-cases`，这样 CI 只需要 cargo。
- 然后它会运行收口检查清单：对每个单元和 `rust/app` 运行 fmt、clippy `-D warnings` 和测试，在已安装对应工具链的情况下构建或检查 `rust/DECISIONS.md` 中列出的每个目标平台，检查运行时读取的文件和未嵌入的资源文件夹，并在纯 Rust 二进制上回放。只有全部通过后，改动才会写回。原实现始终不会被改动。

## 收口之后

各个单元不再有绑定代码。在 json-server 上，每个 crate 的 `tests/recorded.rs` 会在移植版上回放它的录制用例：全部 3,399 条都通过 `cargo test` 运行，与 514 条 HTTP 用例并列。

原实现移除后，你可以自由重构 Rust 代码；`app replay` 依然能验证行为。

## 然后切换上线

工作流技能会让智能体完成它能做的部分，并编写 `rust/cutover.sh`，这是一个交互式脚本，引导你完成只有你才能执行的步骤（部署、DNS、密钥），每一步都有回退方案。
