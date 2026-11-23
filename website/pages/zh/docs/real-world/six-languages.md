---
title: 六种语言
description: 在 Go、Ruby、Python、Java 和 C 的真实代码上端到端运行 everything-rust，包括逐次调用和批处理的速度，以及这些运行发现的 bug。
---

# 六种语言的真实代码

everything-rust 在它支持的每种语言的真实代码上都端到端运行过（analyze、bridge、record、port、diff、gate、bench）。
代码必须已经在这台机器上：从互联网下载的代码不会被执行。所以目标是
随已安装工具链附带的标准库模块；对于 Java 和 C，由于磁盘上没有合适的库源码，
就让 JDK 和 glibc 本身放在一层很薄的门面之后充当参照基准。

移植版、录制用例和额外输入位于 [`demo/real-world`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/README.md)。

| 语言 | 代码 | 函数 | 录制用例 | 结果 |
| --- | --- | ---: | ---: | --- |
| Go | Go 1.24 标准库中的 `path` | 7（另有 1 个保留） | 2,642（542 条来自 Go 自身的测试表） | 2,642/2,642，门禁通过 |
| Ruby | Ruby 3.3 的 `shellwords.rb` | 3 | 1,617（446 条抛出异常） | 1,617/1,617，门禁通过 |
| Python | CPython 3.13 的 `colorsys.py` | 6 | 2,400 | 2,400/2,400，门禁通过 |
| Java | 门面后的 JDK 21（`Integer`、`Long`、`Math`、`Double.toString`、`URLEncoder`、`Base64`、`toUpperCase`） | 11 | 4,492（226 条抛出异常） | 4,492/4,492，门禁通过 |
| C | 门面后的 glibc（`strtol`、`atoi`、`strtod`、`strspn`、`strcmp`、ctype） | 9 | 1,860（6 条崩溃） | 去掉一条未定义行为用例后 1,859/1,859，门禁通过 |
| C | aklomp/base64（内置于 mypyc 的运行时中） | 7 个中的 0 个 | - | 仅规划：每个函数都接收输出缓冲区或状态结构体 |

## 速度

Python、Go、Ruby 和 Java 的第一版桥接层把每个参数和结果都以 JSON
文本传递，开销是这些函数实际工作的 10-50 倍。之后它们被重写为直接转换
值。现在 `bench` 只为调用计时（参数在进程内只解码一次），并给出
是否上线的结论。

| 语言 | 之前的逐次调用 | 现在的逐次调用 | Rust 相比原实现（`bench`，几何平均） | 结论 |
| --- | ---: | ---: | --- | --- |
| Ruby | 6.9-9.5 µs（Fiddle + JSON） | 0.36-1.15 µs（magnus 扩展） | **快 4.4 倍**（各函数为 3.6-7x） | 上线 |
| Python | 11.7 µs（JSON） | 约 0.3 µs（直接 PyO3 转换） | **快 1.35 倍**（大多数调用为 1.5-1.9x） | 上线 |
| C | - | 没有跨边界开销：直接链接 | 0.87x：ctype 快 1.1-1.4 倍，glibc 精心调优的 `strtod`、`strspn`、`strcmp` 为 0.5-0.7x | 优化，或保留较慢的函数 |
| Java | 约 1.7 µs（JNI + JSON） | 30-800 ns（类型化 JNI 和二进制传输协议） | 0.18x | 迁移更大的单元 |
| Go | 2.6 µs（cgo + JSON） | 0.11-0.42 µs（类型化 cgo 和二进制传输协议） | 0.04x | 迁移更大的单元 |

Go 和 Java 中的列表、Map、结构体和多返回值，通过按类型生成的二进制传输协议跨边界：
Go 的 `WordFreq`（一个 map）从 5.1 µs 降到 1.0 µs，`Midpoint`（一个结构体）从 7.2 µs 降到 1.0 µs；
Java 的 `frequencies`（一个 `Map`）从 8.4 µs 降到 1.0 µs，`words`（一个 `List<String>`）从 6.0 µs 降到
0.57 µs，比 Java 本身快 1.39 倍。

TypeScript（x18n core，九个单元，27 个函数，使用相同的用例）：二进制传输协议把
桥接开销平均降低了三分之一（`rangeContains` 3.6 -> 1.1 µs，`applyTextEdits` 10.4 -> 5.7
µs，`toWireRange` 5.0 -> 2.6 µs），`buildTree` 从与 TS 持平变为比 TS 快 2.0 倍。其他
函数都是 V8 只需 15-600 ns 就能运行的叶子函数：与 Go 和 Java 一样，在 Rust 中仍然更慢。

决定因素是：

- **解释型原实现（Python、Ruby）**：调用 Rust 的开销约为 100-300 ns，而
  Rust 代码比解释器快得多，所以即使是小函数也能获益。
- **编译型原实现（Go、经过 JIT 的 Java）**：原实现的叶子函数只需 4-90 ns；单次
  cgo 调用本身就要约 80 ns，带字符串的 JNI 调用更多。没有任何桥接层能让一个 5 ns 的
  函数更快。这些语言只有在一次调用完成大量工作时才能获益：把函数和它的
  调用方一起迁移。`bench` 会这样告诉你。
- **C**：没有跨边界开销；Rust 代码直接与 C 代码对比，结果
  反映的是移植版本身的特性。

### 批处理：x18n 的搜索

上面的叶子函数移植是按逐次调用测量的。在 x18n 的侧边栏搜索中（50,000 个键，逐字母
输入），`everything-rust profile` 显示 `fuzzyScore` 约占整个运行时间的四分之一，由
`matched` 按键逐个调用，并建议使用 `--batch fuzzyScore:text`。生成的 `FuzzyScoreBatch` 把
键留在 Rust 中；`port.rs` 中的 `FuzzyScoreIndex` 只做一次把它们小写化为 UTF-16 的处理，并为每个键保存一个 64 位
码元掩码，因此大多数不可能匹配的键无需读取即可跳过。对于每个查询，结果
都与 TS 流水线完全一致：

| | 每次查询 ms | 相比 TS |
| --- | ---: | ---: |
| TS 流水线 | 10 | 1x |
| 采用相同缩小范围策略的 TS | 4.1-5.4 | 2x |
| 批处理，默认索引（逐条移植，4 核） | 5.1 | 1.7x |
| 批处理配合 `FuzzyScoreIndex`（掩码），napi，4 核 | 1.2 | 7x |
| 批处理配合自行排序的 `FuzzyScoreIndex`，napi，4 核 | 0.45-0.56 | 18-22x |
| 同上，wasm，单线程 | 0.95 | 11.5x |

从 7x 到 20x 的提升：在 1.2 ms 中，有 0.4 ms 是基准测试自身的 `Array.from(order, i =>
keys[i])`（普通循环只需 0.02 ms）。其余的提升来自让索引自行排序：
当新查询是上一次查询的延伸时，它只重新给上一次查询的匹配结果打分（不匹配
`s` 的键不可能匹配 `se`），单字母查询从首次出现位置表中读取，ASCII 键按字节
打分，并用计数排序对整数分数排序。Callgrind 显示，在 30 字节的键上，
库函数调用（`memchr`、`memcmp`）比普通循环开销更大。部分提升来自
算法本身，所以基准测试也运行了采用相同缩小范围策略的 TS：Rust 比它快 7-12 倍
（wasm 为 5 倍）。

其他语言中的相同模式，与原实现对比（4 核，结果和错误完全一致）：

| 语言 | 工作负载 | 原实现 | 逐次调用移植版 vs 原实现 | 批处理 vs 原实现 |
| --- | --- | ---: | ---: | ---: |
| Python | 测试夹具 `slugify`，20,000 个字符串 | 98 ms | - | 17x |
| Ruby | 测试夹具 `slugify`，20,000 个字符串 | 110 ms | - | 12.7x |
| Ruby | `Shellwords.shellescape`，50,000 个混合值 | 88 ms | - | 11.8x |
| Go | 测试夹具 `Slugify`，50,000 个字符串 | 49 ms | 0.93x | 3.3-4.1x |
| Java | 测试夹具 `slugify`，50,000 个字符串 | 43 ms | 0.68x | 3.0x |

Go 批处理最初比单线程还慢：移植版为每个数据项分配内存，而 glibc 的分配器
在 Go 进程内让各线程串行执行。现在批处理 crate 使用 mimalloc。shellescape 批处理
还发现了一个逐次调用录制遗漏的移植版 bug：Ruby 符号（`shellescape(:sym)`）返回的是
它们的 JSON 标记。已在移植版中修复。

随后 Go 和 Java 从 2.5x 和 1.7x 提升到 4x 和 3x。在批处理内部，有一半时间并不是
实际工作：结果在单个线程上拼接和释放，Go 又复制了一次，Java
对每个字符串都通过 modified UTF-8 做一次 JNI 调用。现在字符串结果批量返回，在
全部核心上直接写入 Go 自己的缓冲区或一个 Java `char[]`。另一半来自移植版本身：在单
线程上，Rust 的 `slugify` 只比 Go 快一点点，比 Java 的 JIT 还慢。处理 ASCII 小写化时
不查表、不生成中间字符串，解决了这个问题；Java 的移植版为了保留
词尾西格玛规则，只把包含 `Σ` 的那个以空白分隔的片段作为整体转小写。
逐次调用时，Go 移植版从 0.73x 提升到 0.93x，Java 移植版从 0.46x 提升到 0.68x。

## 这些运行发现了什么

### everything-rust 中的 bug，已修复

- **浮点数变化了一个 ULP**（Java 运行）。`serde_json` 默认的浮点解析器并不总是
  正确舍入。everything-rust 在读取录制的输入时扰动了它们，生成的 crate
  又扰动了一次。43 条 `Double.toString` 用例在移植版正确的情况下失败。通过在 everything-rust 及其生成的每个 crate（napi-rs、
  wasm 和 JSON 分发器）中启用 `serde_json` 的 `float_roundtrip` 特性修复。
- **C 崩溃中断了录制**（C 运行）。超出 C 函数约定范围的输入（`isalpha(INT_MAX)`）
  会让 glibc 段错误。现在 C 驱动会在 fork 出的子进程中运行每条用例，并记录
  `crash: SIGSEGV`；对这类用例，`diff` 接受移植版的任何行为，因为 C 的行为本身就是未定义的。
- **没有识别 `module_function :name`**（Ruby 运行）。`shellwords` 在方法之后用符号参数
  声明它的模块函数；导出项识别器只认识不带参数的 `module_function`
  形式。现在两种形式都能识别。
- **无类型代码得到了无效输入**（Python 运行）。`colorsys` 没有类型注解，所以
  1,800 条生成用例中有 1,709 条是 `TypeError`。现在在 Python 和 Ruby 中，未标注但按数值使用的参数（`r - g`、
  `max(r, g, b)`、`x < 1`）会被标为 `number` 类型：剩下 94 个错误，全都是真实的
  （`ZeroDivisionError`）。
- **从未录制以浮点数位置传入的整数**（Python 运行）。Python 调用方传 `1` 和传
  `1.0` 一样常见，而 `colorsys` 会原样传递整数（`hsv_to_rgb(0, 0, 7)` 是 `(7, 7, 7)`）。现在录制时
  在 Python 用例中每四条就有一条把整数值作为 int 传入；这暴露了一个类型为 `f64` 的移植版中的
  134 处不一致。
- **TypeScript 时代遗留的消息**：“must stay in JS”、“kept in TS”、“characterization tests
  against the TS code” 现在都会写出实际的源语言。
- **重新打开的核心类**（`shellwords.rb` 中的 `class String`）现在会被标为猴子补丁风险。

### 移植版必须匹配的行为，由 `diff` 发现

每一行都是第一版移植中的一处不一致，由录制用例发现。标有 *fixture* 的行
来自 `tests/fixtures/` 中为每种语言编写的小型测试项目；其余的来自上面的
真实运行。

- Go `path.Clean`：Go 的 `lazybuf` 在写入索引回退后读取的是该索引处的字节，而不是
  最后保留的字节（9 条用例，来自 Go 自身的测试表）。
- *fixture* Go 的 `strings.ToLower` 和 Ruby 的 `downcase` 逐个 rune 转小写：`ΣΊΣΥΦΟΣ` 变成 `σίσυφοσ`，
  而 Rust 的 `to_lowercase` 会应用词尾西格玛规则。Go 还会把 `İ` 映射为 `i`，而 Rust
  得到的是 `i̇`。
- *fixture* Go 按字节切分字符串；切断一个 rune 后，到 JSON 中每个字节都会变成一个 U+FFFD。
- *fixture* Ruby 的 `slugify` 在 `max` 为负数时抛出 `NoMethodError`（`slug[0, -1]`
  是 `nil`）：这是原实现中的 bug，移植版要么保留它，要么先修复调用方。
- *fixture* Ruby 的 `Array#sum` 使用 Kahan-Babuska 求和，并从整数 0 开始，所以 `[-0.0].sum` 是 `0.0`。
- Ruby 的 `Float#to_s` 从 1e15 起使用指数形式（`1e15.to_s` 是 `1.0e+15`，`1e14.to_s` 是
  `100000000000000.0`）；数组中的 `nil.inspect` 是 `nil`。
- *fixture* Java 的 `\s` 只匹配 ASCII；Rust 的 `split_whitespace` 还会在 U+00A0 处切分。
- Python 的“整数或浮点数”结果：`colorsys` 在 `+`、`-` 和 `*` 运算中保留整数，所以忠实的移植版
  使用一个小型的“整数或浮点数”类型而不是 `f64` 来计算（录制了 int 之后发现 134 条用例）。
- glibc 的 `strtod("-")` 什么都不转换，返回 `+0.0`，而不是 `-0.0`。

在第一版移植中就已处理、并由录制用例确认的：

- Java 的 `Integer.parseInt` 通过 `Character.digit` 接受任何 Unicode 十进制数字（`١٢`、`２`）。
- Java 的 `Double.toString`（JDK 19+）输出最短的十进制表示，但至少两位数字：
  `4.9E-324`，而 Rust 的最短形式是 `5e-324`。
- Python 的 `max(nan, 1, 2)` 是 `nan`（除非后面的值比较结果更大，否则第一个值胜出）；
  `-0.0 % 1.0` 是 `0.0`。
- glibc 的 `toupper(-128)` 是 `128`（它的表接受有符号 `char` 值）；`strtol` 在 base 为 0 时
  把 `077` 读作八进制，把不带十六进制数字的 `0x` 读作 `0`。

### 这些运行暴露的限制

- **传递缓冲区的 C API。** aklomp/base64 的公共函数接收 `char *out, size_t *outlen`
  或 `struct base64_state *`。C 桥接层只支持标量和 `const char *`，所以把
  它们全部保留在 C 中，并注明原因。支持输出缓冲区模式是 C 的下一步。
- **C 宏。** base64 的 51 个文件中有 43 个通过宏定义函数
  （`BASE64_ENC_FUNCTION(arch)`）；解析器在这些地方报错，规划把这些单元标为
  部分迁移。SIMD 内建函数目前还不会被标为风险。
- **不会导致崩溃的未定义行为。** `ispunct(1000)` 越过 glibc 的表读取并
  返回 1。任何移植版都不应该照搬这一点；这条用例是手动从 `cases.jsonl` 中删除的。目前
  还没有内置的方式把用例标记为未定义。
- **动态类型会带来额外代码。** `shellwords` 移植版为数字、nil、数组和哈希重新实现了 Ruby 的 `to_s` 和 `inspect`，
  因为 `shellescape` 什么都接受。`colorsys`
  移植版需要 Python 的数值塔。带类型的移植版更短，但会改变某些
  调用方看到的行为；`diff` 会准确指出是哪些。
- **门面。** Java 和 C 的运行测试的是真实的 JDK 和 glibc 行为，但经过的是为本次运行
  编写的门面，而不是某个库自己的 API。

## 各语言如何运行

| | 录制 | Diff | 桥接层 | 影子运行 |
| --- | --- | --- | --- | --- |
| Go | 生成的驱动，作为独立模块构建，并用 `replace` 指向原实现 | 同一个驱动，运行在 cgo 包装之上 | JSON 分发器 + 保持原 API 的 cgo 包 | 保持原 API 的包，同时运行两者 |
| Ruby | 生成的驱动加载原始文件 | 同一个驱动加载 Fiddle 包装 | JSON 分发器 + 定义相同模块的 Fiddle 包装 | 在原实现之后 require 的文件：对两者都使用 `prepend` |
| Java | 与原始源码一起编译的反射驱动 | 同一个驱动，与 JNI 包装类一起编译 | 基于 JNI 的 JSON 分发器 + 保持原静态方法的包装类 | `<Class>Shadow` 类 |
| Python | 进程内驱动导入原实现 | 同一个驱动导入 PyO3 包 | PyO3 | 保持原 API 的模块 |
| C | 用 `cc -shared` 编译原实现，通过 ctypes 调用，每条用例一个 fork | 基于 Rust `cdylib` 的 ctypes | 与原函数同名的 `extern "C"` 函数及头文件 | 使用 `-D` 重命名的影子 `.c` 文件；crate 的 `shadow` 特性只导出 `everything_rust_` 开头的名称 |
