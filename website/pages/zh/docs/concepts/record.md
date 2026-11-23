---
title: 录制
description: record 在生成的边界用例、语义陷阱、原实现自身的测试和你的输入上运行原实现，并把结果保存为 cases.jsonl。
---

# 录制

```sh
everything-rust record <crate>
everything-rust record <crate> --from-tests -- npx jest      # 或：-- pytest
everything-rust record <crate> --inputs extra.jsonl
```

`record` 运行原始函数，并把每个函数的返回值或抛出的异常保存到 crate 旁边的 `cases.jsonl` 中：这就是移植版必须匹配的行为。之后出现的不一致就是行为差异：要修的是移植版，而不是录制结果。

## 用例从哪里来

- **生成的边界用例**：非 ASCII、emoji、`-0`、`NaN`、期望浮点数的地方传整数，等等（`--count` 设置每个函数生成多少条）。
- 每个参数上的**语义陷阱**：字符串长度与大小写、有符号除法、溢出边界、浮点输出、Map 与排序顺序。
- 使用正则的文件中每个函数的**正则陷阱**：土耳其语无点 `ı`、开尔文符号、长 s、阿拉伯-印度数字、行分隔符、词尾西格玛、零宽连接符。
- **原实现自身的测试**：`--from-tests -- <test command>` 会录制测试套件对该单元导出函数发出的每一次调用（TS/JS 支持任何测试运行器，Python 支持 pytest 或 unittest）。之后源码会被恢复。在 vercel/ms 上，它从 167 个 Jest 测试中捕获了 340 次调用。
- **你的输入**：`--inputs cases.jsonl`，每行一个 `{"fn": name, "args": [...]}`。请补充真实输入（生产环境样本、bug 报告）；把生产环境搞崩的用户输入，很少是生成器能想到的。

Python 调用方传 `1` 和传 `1.0` 一样常见，所以在 Python 用例中，每四条就有一条把整数值作为 int 传入。

## cases.jsonl 中的标记

JSON 无法表达 `undefined`、NaN、Infinity、-0 或孤立代理项。结构化的值会携带标记，由生成的 `index.cjs` 转换，辅助函数位于 crate 的 `js.rs` 中：

| 标记 | 含义 |
| --- | --- |
| `{"$undefined": true}` | JavaScript 的 `undefined` |
| `{"$num": "NaN"}` | JSON 无法表示的数字（`NaN`、`Infinity`、`-0`） |
| `{"$utf16": [...]}` | 以 UTF-16 码元给出的字符串，用于孤立代理项 |

超出函数约定范围的 C 输入可能让原实现崩溃。C 驱动会在 fork 出的子进程中运行每条用例，并记录 `crash: SIGSEGV`；对这类用例，`diff` 接受移植版的任何行为，因为 C 的行为本身就是未定义的。

## 两次运行，以及不同之处

`record` 会运行原实现两次，并归类出两次运行之间的不同之处：变化的值（时间、ID）会变成通配符；行相同但顺序不同的（哈希表、并发）会按任意顺序比较；行数不同的会变成 `...`，可以匹配任何内容。`record` 会列出每一类。请逐一查看，因为哈希顺序或并发上的差异，恰恰是移植版在真实场景中可能发生变化的地方。

## 录制结果会记住它的原实现

`record` 会保存单元文件的指纹（`recorded-from.json`）。如果原实现之后发生变化，`diff` 会发出警告，`gate` 会失败，直到你重新录制，而不是对照原实现过去的行为通过。见 [Diff](/docs/concepts/diff#stale-recordings)。

## TypeScript 的特殊之处

`record` 按原样运行 TS 源码：Node 的 resolve 钩子会处理打包工具风格的导入（`./errors` 解析为 `./errors.ts`），仓库中没有安装 npm 包时可以用 `--resolve-from` 提供，需要转换的 TS（枚举、参数属性）会用 `--experimental-transform-types` 重试。无法直接在 Node 中运行的 TS，可以用 `--module dist/x.mjs` 从构建产物录制，或者使用 `--node-arg=--import=tsx`。

## 数值代码

对于浮点数很少能逐位一致的代码，请使用 `--float-tolerance` 录制。见[浮点容差](/docs/concepts/float-tolerance)。
