---
title: 录制与回放
description: app record 在应用文档中的示例、你的输入和语义陷阱上运行整个应用；app replay 让任意构建产物跑一遍这些用例。
---

# 录制与回放

`record` 和 `diff` 验证一个单元；`app record` 和 `app replay` 验证整个应用，适用于六种语言中任意一种的 CLI 和服务端。应用被当作黑盒。

```sh
everything-rust app record . -- python3 cli.py          # 运行原实现的命令
everything-rust app replay .                            # 原实现必须 100% 通过
everything-rust app replay . --build "target/release/mytool"
```

## 用例从哪里来

`app record` 会用以下输入运行原实现：

- 应用自己文档中的示例：Markdown 和 shell 文件中的 `$ mytool ...` 命令行，以及它们引用的文件；
- 一个 `--inputs` JSONL 文件，每行一个 `{"name", "args", "stdin", "env", "files"}`；
- 从这些用例衍生出的语义陷阱：非 ASCII 和 emoji 字符串、负数除法与取模、溢出、极大和极小的浮点数、Map 顺序、缺失的文件。

每条用例运行两次；两次运行之间不同的部分会变成 `[..]` 通配符。`--normalize timestamps|uuids|key-order` 或 `--normalize name=regex` 可以添加归一化规则，并与用例一起保存。`app record` 会按类别报告不同之处：值（现在是通配符）、顺序（行按任意顺序比较）和行数（一个可以匹配任何内容的 `...`）。请逐一查看。

## 用例是普通文件

用例是 `rust/app-cases/` 中的 [trycmd](https://docs.rs/trycmd) 文件：

| 文件 | 内容 |
| --- | --- |
| `<case>.toml` | argv、环境变量、退出码 |
| `<case>.stdin` | 标准输入 |
| `<case>.stdout`、`<case>.stderr` | 原实现打印的内容 |
| `<case>.in/` | 输入文件 |
| `<case>.out/` | 写出的文件 |

你可以手动阅读和编辑它们；一旦移植版有了二进制文件，Rust 测试就可以用 `trycmd` 运行它们。

## 回放

- `app replay` 让任意构建产物跑一遍这些用例，打印 `passed/total`，写出 `rust/app-cases/replay-report.md`（每条失败用例附一份 diff），并在出现任何不一致时以非零状态码退出。
- 如果构建产物会向 `EVERYTHING_RUST_FALLBACK_LOG` 指定的文件追加内容（每次回退到原实现记一行），就像[前门](/docs/whole-apps/front-door)那样，那么它的回退率也会被报告。
- `app replay --case <name>` 按报告中给出的名称重跑单条用例：这是在全部重跑之前修复某个不一致的快速、可重复的循环。

## 在 CI 中回放

报告中没有耗时或临时路径：同一个构建产物回放两次，写出的文件完全相同。提交它，然后在 CI 中运行：

```sh
everything-rust app replay .
git diff --exit-code rust/app-cases/replay-report.md
```

这样，一个迁移了一半的应用，只要通过情况有任何变化（无论变好还是变差），CI 就会失败，直到报告随之更新。

## 数值输出

`app record --float-tolerance 1e-9` 让数字在较大值的该比例之内即可视为一致。见[浮点容差](/docs/concepts/float-tolerance)。
