---
title: 批处理与热点路径
description: 逐条数据调用一次的移植版会输给 JIT。批处理把整个循环迁到 Rust，在全部核心上运行，即使逐次调用的移植版输了，它也能胜出。
---

# 批处理与热点路径

```sh
everything-rust bridge <repo> <unit> --batch fn:param    # 或：everything-rust refresh <crate> --batch fn:param
```

`--batch` 生成一个类，它只接收一次 `param` 值并把它们留在 Rust 中：`new FnBatch(items)`（TS、Python、Java）、`Mod::FnBatch.new(items)`（Ruby）、`NewFnBatch(items)`（Go）。每次查询时，`run(<other args>)` 以一个数组返回所有结果，`rank(<other args>, min)` 返回按得分从高到低排列的数据项索引。它在全部核心上运行，错误与原实现一致。除 C 以外的每种语言都支持批处理。

逐条数据调用一次的移植版，每条数据都要跨一次桥接层，会输给 JIT。真正能赢的是
把循环迁过去：`profile` 找到它，`--batch` 生成一个类，只接收一次数据，
每次调用都在全部核心上对所有数据运行该函数，并返回类型化数组结果。

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore：占整个运行时间的四分之一，由 `matched` 按键逐个调用：将其批处理化
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

`port.rs` 中的 `FuzzyScoreIndex` 结构体用一个只预处理一次数据的循环替换了默认循环，
并且可以自行排序（`rank`）：它保存小写化后的键以及每个键的字符掩码，
一张记录每个键中每个常见字符首次出现位置的表（单字母查询就是查一列），
以及上一次查询的匹配结果（在 `s` 之后输入 `se`，只会重新给 `s` 匹配到的结果打分）。在
x18n 的侧边栏搜索中，对 50,000 个键逐字母输入，结果完全一致：

| | 每次查询 | 相比 TS |
| --- | ---: | ---: |
| TS：`map(fuzzyScore)`、过滤、排序 | 10 ms | 1x |
| 采用与索引相同缩小范围策略的 TS | 4.1-5.4 ms | 2x |
| 同一个移植版，按键逐个调用 | - | 0.2-0.4x |
| `FuzzyScoreBatch`，默认索引（4 核） | 5.1 ms | 1.7x |
| `FuzzyScoreBatch` 配合 `FuzzyScoreIndex`（napi，4 核） | 0.45-0.56 ms | **18-22x** |
| 同样的实现用 wasm（单线程） | 0.95 ms | **11.5x** |

即使与采用相同缩小范围策略的 TS 相比，Rust 仍然快 7-12 倍（wasm 为 5 倍）。把返回的
索引转换回键时请用普通循环：`Array.from(order, i => keys[i])` 的开销是它的 20 倍，0.4 ms。

每种语言都支持批处理，而且即使逐次调用的移植版输给原实现，批处理也能胜出。
在 4 核上做相同的工作，结果和错误完全一致（基准测试位于各个 demo 文件夹中）：

| 语言 | 工作负载 | 原实现 | 逐次调用移植版 vs 原实现 | 批处理 vs 原实现 |
| --- | --- | ---: | ---: | ---: |
| TS | x18n 搜索，50,000 个键 | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`，20,000 个字符串 | 98 ms | - | **17x** |
| Ruby | `slugify`，20,000 个字符串 | 110 ms | - | **12.7x** |
| Ruby | `shellescape`（标准库），50,000 个混合值 | 88 ms | - | **11.8x** |
| Go | `Slugify`，50,000 个字符串 | 49 ms | 0.93x | **4x** |
| Java | `slugify`，50,000 个字符串 | 43 ms | 0.68x | **3x** |

编译型原实现获益较少：逐条数据的处理本来就快，所以批处理的优势约等于核心
数。批处理 crate 使用 mimalloc 分配内存：使用系统分配器时，Go 批处理比
单线程还慢。示例：[`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md)。
## 编写快速索引

- 让调用方只构建一次批处理对象，而不是每次查询都构建。用普通循环把返回的索引映射回数据项。
- 要只预处理一次数据（小写化、掩码、位置表），请在 `port.rs` 中定义 `pub struct FnIndex`，带有 `new(items)` 和 `run(<other args>)`。它必须逐条给出与该函数相同的结果。
- 当排序可以跳过部分工作时，给索引定义自己的 `rank(<other args>, min)`。它必须与对函数结果排序的结果一致：得分从高到低，得分相同时按数据项顺序。
- 批处理会把移植版逐条处理的速度乘以核心数。面对编译型原实现（Go、Java）时，移植版本身必须足够快：处理 ASCII 时不要查 Unicode 表，每个结果在一个预分配大小的 `String` 中构建，避免中间拷贝。
- 在 Go 和 Java 中，用完后请关闭批处理对象（finalizer 只是兜底）。
