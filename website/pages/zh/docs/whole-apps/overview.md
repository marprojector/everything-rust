---
title: 迁移整个应用
description: 从第一次录制到纯 Rust 二进制。目标是一个行为与原实现一致的 Rust 应用，同时保留原实现作为参照。
---

# 迁移整个应用

目标是一个行为与原实现一致的纯 Rust 应用，同时保留原实现作为参照。Rust 代码由你的智能体编写；everything-rust 负责排定工作顺序、搭建脚手架，并验证每一步。适用于 CLI、HTTP 服务端和程序库。

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in    pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down proved by
 decisions to make      outside         original answers                                           replay
```

## 1. 规划

```sh
everything-rust analyze . --whole-app
```

`--whole-app` 会加上应用形态（CLI、服务端或程序库）和入口点、每个外部依赖（导入的或声明的）及其对应的 Rust crate 和匹配度、按迁移块排列的迁移顺序（相互导入的单元一起迁移），以及带推荐答案的编号决策项：保留的单元、受阻的单元、没有 crate 或只有部分匹配 crate 的依赖。

## 2. 录制整个应用，然后迁移入口点

1. [`app record` 和 `app replay`](/docs/whole-apps/record-replay) 把程序当作黑盒，从外部验证它。
2. [`app front-door`](/docs/whole-apps/front-door) 在原实现前面放一个 Rust 二进制；每条命令仍然由原实现运行。
3. 每个单元走一遍[核心闭环](/docs/concepts/the-loop)，然后 `app wire` 把它的命令交给 Rust。
4. 当回放在回退率为 0 的情况下通过、且每个单元都通过门禁后，[`app lift`](/docs/whole-apps/lift) 会移除原实现。

[服务端](/docs/whole-apps/servers)和[程序库](/docs/whole-apps/libraries)使用相同的命令。

## 智能体如何推进

[整应用技能](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md)会引导智能体完成全过程，首先从只有你能做的决策开始：

- **深度访谈**：智能体分轮提问，每个问题都附带推荐答案，直到迁移所依赖的每项决策都确定下来，并把每项决策以 ADR 形式写入 `rust/DECISIONS.md`，同时在 `rust/GLOSSARY.md` 中把原项目的术语映射为 Rust 命名。
- **调研**：后台智能体阅读每个依赖的一手资料，并编写 `rust/research/<topic>.md`，列出它们发现的每一个语义陷阱。
- **跨会话**：`rust/MAP.md` 列出每个单元及其状态；每个会话开始前先认领一个单元，每个检查点都会留下 `rust/HANDOFF.md` 给下一个会话。同一迁移波次中的单元可以由子智能体并行迁移。
- **切换上线**：智能体完成它能做的部分，并编写 `rust/cutover.sh`，这是一个交互式脚本，引导你完成只有你才能执行的步骤（部署、DNS、密钥），每一步都有回退方案。

深度访谈、调研以及这些实践借鉴自 [Matt Pocock's skills](https://github.com/mattpocock/skills)。

## 这会不会把 Rust 锁死在旧设计里？

只在两者并行运行期间才会。录制结果检查的是应用边界上的行为（CLI 的输出、服务端的响应、模块导出的函数），而不是内部实现。在单元仍需跨越桥接层时，沿用旧的结构；一旦 `app lift` 移除了原实现，你就可以自由重构 Rust 代码，`app replay` 依然能验证行为。

## 一个真实案例

[json-server](/docs/real-world/json-server) 走完了每一步：514 条录制的 HTTP 用例，回退率 99.8% 的前门，先接入读取操作（回退率 22.6%），再接入所有路由（0%），最后收口为通过 514/514 的纯 Rust 二进制。
