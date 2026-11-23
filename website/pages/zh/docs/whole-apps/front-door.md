---
title: 前门与接入
description: app front-door 在原实现前面放一个 Rust 二进制；当某个单元的移植版通过门禁后，app wire 把命令交给它。
---

# 前门与接入

先把入口点迁到 Rust，然后一次一个单元地把命令交给它：

```sh
everything-rust app front-door .                            # rust/app：每条命令都运行原实现
everything-rust app wire . textkit --command upper          # 在 textkit-rs 通过 gate 之后
# 在 rust/app/src/units/textkit.rs 中编写胶水代码，然后：
everything-rust app wire . textkit                          # 再次 100% 通过，回退率降低
```

## 前门

`app front-door` 会搭建 `rust/app`，这是一个以应用命名的 Rust 二进制：基于 clap 为录制用例中的命令和参数生成解析器，每条命令都会用相同的 argv、stdin 和环境变量运行原实现，并透传它的输出和退出码。每次回退都会被计数，所以刚生成的前门回放结果是 100% 通过、100% 回退（它会构建并回放一遍来展示这一点）。它从不覆盖 `rust/app`：从那以后，代码就归你了。

## 接入一个单元

`app wire <repo> <unit> --command <name>`：

1. 对该单元的 crate 运行 `gate`（如果刚刚通过，可以加 `--skip-gate`）；
2. 把它作为库加入 `rust/app`；
3. 只生成一次 `src/units/<unit>.rs`，由你或你的智能体填写：解析参数，调用 `<crate>::port`，打印与原实现相同的内容；
4. 把命令路由过去，然后构建并回放。

写好胶水代码后再运行一次：100% 通过，回退率降低。在收口之前，`EVERYTHING_RUST_UNIT_<NAME>=original` 可以把已接入的单元切回原实现。

对于没有指定命令的 argv（没有参数、未知命令、用法错误），可以把应用自身的名称作为命令接入（`--command mytool`）。

## 没有任何东西会回调原实现

前门把原实现作为独立进程运行（或把 HTTP 请求转发给它），桥接的单元由原实现调用，而绝不会反过来，回调函数也始终留在源语言中。

服务端使用路由而不是命令：见[服务端](/docs/whole-apps/servers)。程序库使用它们的公共 API：见[程序库](/docs/whole-apps/libraries)。
