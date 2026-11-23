---
title: 核心闭环
description: profile、analyze、bridge、record、port、diff、bench、shadow 和 gate。每个单元都要经过的步骤。
---

# 核心闭环

每一次迁移，从一个热点函数到整个应用，都要一次一个单元地经过相同的步骤。只有 `gate` 通过，迁移才算完成。

everything-rust 如何校验一次迁移、它的各个命令做出哪些决定，以及它的局限在哪里。返回 [README](/)。

## 工作原理

```
profile ──> analyze ──> bridge ──> record ──> port.rs ──> diff ──> bench ──> shadow ──> gate
 where the    what to     the Rust   what the    you or      every     faster?    both side   done means
 time goes    port first  crate and  original    your agent  recorded  the        by side in  this passes
                          wrapper    does        write it    case      verdict    production
```

原实现始终是唯一的事实来源：`record` 在边界用例（以及你自己的输入）上运行它，
并保存它的返回值或抛出的异常；`diff` 在 Rust 实现上回放每一条用例；在全部一致且
`gate` 通过之前，什么都不会上线。整个过程中，调用方始终使用原有的 API。

整个应用也会一个单元一个单元地走同一个闭环，只是外面多了一层用来验证整个
程序的框架（[迁移整个应用](/docs/whole-apps/overview)）：

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop above, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in         pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down      proved by
 decisions to make      outside         original answers                                                replay
```

## 核心闭环

| 步骤 | 命令 | 作用 |
| --- | --- | --- |
| 0. 性能分析 | `everything-rust profile <repo> -- node bench.mjs` | 在对应语言的性能分析器（node、cProfile、`go test` pprof、JFR、Ruby tracer）下运行真实工作负载：按包含子调用的时间列出热点函数、它们的调用方，以及哪些循环适合批处理；标出已经编译过的代码（numba、Cython、JAX），Rust 在这些地方获益很少 |
| 1. 规划 | `everything-rust analyze <repo>` | 单元、迁移顺序、带 `file:line` 的风险、每个单元的桥接层、依赖与 crate 的匹配度、最适合先迁移的单元；`--whole-app` 还会加上入口点和应用形态、每个依赖（导入的或声明的）及其对应 crate、迁移块（相互导入的单元一起迁移），以及带推荐答案的编号决策项 |
| 2. 桥接 | `everything-rust bridge <repo> <unit>` | 生成带绑定代码的 crate（`lib.rs`，以及源语言加载的包装层）和待移植的桩函数（`port.rs`）；`--batch fn:param` 会添加一个把大量 `param` 值留在 Rust 中的类；`--as-json T` 把应用只用来装数据的类型参数作为 JSON 值桥接 |
| 3. 录制 | `everything-rust record <crate>` | 在生成的边界用例（非 ASCII、emoji、`-0`、`NaN`、期望浮点数的地方传整数……）以及每个参数上的每个语义陷阱（字符串长度与大小写、有符号除法、溢出边界、浮点输出、Map 与排序顺序）上运行原实现，并保存 `cases.jsonl`；`--from-tests -- npx jest`（或 `-- pytest`）还会录制原实现自身测试套件发出的每一次调用；`--float-tolerance 1e-9` 让数值代码中的浮点数在该比例内即可视为一致 |
| 3. 构建 | `everything-rust build <crate>` | 构建 Rust 侧：`.node`、wasm 包、Python 扩展或共享库 |
| 3. Diff | `everything-rust diff <crate>` | 以原实现调用方的调用方式，在 Rust 上回放录制用例；列出不一致之处 |
| 3. 基准测试 | `everything-rust bench <crate>` | 在原实现和 Rust 上为录制用例计时，包含桥接开销 |
| 4. 影子运行 | `everything-rust shadow <crate>` | 可直接替换的模块，同时运行两者，返回原实现的结果，并报告不一致；`EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. 门禁 | `everything-rust gate <crate>` | fmt、clippy `-D warnings`、测试、`unsafe` 审计、没有未移植的内容、原实现的每条错误路径都有一条录制的失败用例、在原实现返回错误的地方没有 `unwrap`/`expect`/`panic!`（除非标注了 `// INFALLIBLE: <why>`）、构建、API、行为 |
| 智能体 | `everything-rust setup` | 把 MCP 服务器（`everything-rust mcp`，每个步骤都是一个工具）和工作流技能（[`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)）注册到你的编程智能体 |

## 热点路径：把数据留在 Rust 侧

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

## `bridge` 如何做决定

- 无法跨越桥接层的导出项会保留在源语言中并注明原因，影子
  模块会重新导出原实现。对于 TS：
  接收或返回函数的函数、生成器和迭代器、RegExp、`instanceof` 检查、
  泛型（`--as-json T` 可以桥接一个应用只用来装数据的泛型），以及类型中（沿导入、重新导出和 `extends` 追踪）包含
  函数的值。`--keep "name: reason"` 可以保留更多内容，例如依赖 `Intl` 数据的代码。
- TS 类型会映射为 Rust 类型，类型别名会沿导入解析，参数类型
  会根据默认值推断；结构化的值通过生成的二进制传输协议跨边界（只有传输协议无法描述的结构
  才回退到 JSON）。
- JSON 无法表达 `undefined`、NaN、Infinity、-0 或孤立代理项：结构化的值会携带
  标记（`{"$undefined": true}`、`{"$num": "NaN"}`、`{"$utf16": [...]}`），由生成的
  `index.cjs` 转换，辅助函数位于 crate 的 `js.rs` 中。

`record` 按原样运行 TS 源码：Node 的 resolve 钩子会处理打包工具风格的导入
（`./errors` 解析为 `./errors.ts`），仓库中没有安装 npm 包时可以用 `--resolve-from`
提供，需要转换的 TS（枚举、参数属性）会用
`--experimental-transform-types` 重试。

## `analyze` 检查哪些内容

- 包（`package.json`）和源文件；跳过 `node_modules`、构建产物、测试夹具、
  playground 和工具配置。Vue/Svelte/Astro 的 `<script>` 块会被解析以提取导入。
- **单元**：一个包含源文件的文件夹，或包源码根目录下的一个文件。
- **迁移波次**：一个单元导入的内容会排在更早的波次中；存在循环导入的单元会一起迁移。
- **桥接层**：当单元或它导入的任何内容需要 Node 时用 napi-rs；可移植的
  代码用 wasm-bindgen；UI 和编辑器代码保留在 JS 中。
- **就绪度**（有多安全）和**优先级**（既安全又值得：循环和字符串处理、测试、规模）。

| 风险 | 严重程度 | 示例 |
| --- | --- | --- |
| 字符串偏移 | 高 | `slice`、`indexOf`、`matchAll`、`d` 标志：UTF-16 与 UTF-8 |
| 正则语法 | 高 | 环视、反向引用、`y`：`regex` crate 不支持 |
| 动态正则 | 高 | `new RegExp(userPattern)`：运行时的 JS 语法 |
| 动态代码 | 高 | `eval`、`new Function`、计算出的 `import()` |
| 动态类型 | 中 | `any`、`'key' in value` |
| 有序集合 | 中 | `Map`、`Set`、`Object.keys`：插入顺序 |
| 异步、生成器 | 中 | Promise 和生成器的边界 |
| 类继承、反射 | 中 | `extends`、`Proxy`、`Reflect`、`Symbol` |
| 回调式 API | 中 | 函数类型的参数和方法：桥接调用频繁 |
| 异常、数字、JSON | 低 | `throw`、位运算、`JSON.stringify` 的顺序 |

## 它捕获不到什么

通过只代表 Rust 实现与已录制的每条用例一致，仅此而已。重写中真正会出问题的地方
通常在录制范围之外，所以要清楚边界在哪里：

- **没人录制过的输入。** 生成的用例、语义陷阱、原实现自身的测试套件
  （`record --from-tests`）以及你自己的 `--inputs` 就是上限。请补充真实输入（生产环境
  样本、bug 报告）；把生产环境搞崩的用户输入，很少是生成器能想到的。
- **已经变化的原实现。** `record` 会保存单元文件的指纹
  （`recorded-from.json`）；如果原实现之后发生变化，`diff` 会发出警告，`gate` 会失败，直到你
  重新录制，而不是对照原实现过去的行为通过。
- **正则表达式。** `regex` crate 不支持环视和反向引用，而且即使是能编译通过的模式，在非 ASCII 文本上
  处理 `\w`、`\b`、`\d` 和忽略大小写匹配的方式也可能与
  JS、Python 或 Java 不同。换成 `fancy-regex` 或改写模式后，可能通过所有
  录制用例，却仍然在土耳其语无点 `ı` 或零宽连接符上表现不同。`record` 会给使用正则的文件中的
  每个函数加入正则陷阱（上述字符、开尔文符号、长 s、阿拉伯-印度数字、行分隔符、词尾西格玛），
  但你仍然应该用真实文本对你的真实模式做模糊测试。
- **两次运行之间会变化的内容。** `record` 会运行原实现两次，并归类出不同之处：
  变化的值（时间、ID）会变成通配符；行相同但顺序不同的（哈希表、
  并发）会按任意顺序比较，所以移植版仍然必须产出完全相同的这些行；
  行数不同的会变成 `...`，可以匹配任何内容。`record` 会列出每一类：
  请逐一查看，因为哈希顺序或并发上的差异，恰恰是移植版在真实场景中
  可能发生变化的地方。
- **浮点数。** 不同语言把数字格式化为文本的方式不同（`1e21`、`-0`、数字位数）。
  语义陷阱中包含这些值，而且它们必须完全一致；`--float-tolerance` 只容忍
  舍入误差，从不容忍格式差异。
- **输出之外的一切。** 耗时、内存、负载下的并发，以及 stdout、stderr、退出码、
  写入的文件和 HTTP 响应之外的副作用（网络调用、数据库、
  信号）都不会被比较。
- **其他平台。** 录制结果只能证明运行录制的那台机器上的行为。结果
  依赖类型大小、字节序或编译器的 C 代码，在目标平台上可能表现不同；正因如此，`analyze`
  会把贴近硬件的 C 代码保留在 C 中。
## 每一个步骤

| 步骤 | 命令 | 作用 |
| --- | --- | --- |
| 0. 性能分析 | `everything-rust profile <repo> -- node bench.mjs` | 在对应语言的性能分析器（node、cProfile、`go test` pprof、JFR、Ruby tracer）下运行真实工作负载：按包含子调用的时间列出热点函数、它们的调用方，以及哪些循环适合批处理；标出已经编译过的代码（numba、Cython、JAX），Rust 在这些地方获益很少 |
| 1. 规划 | `everything-rust analyze <repo>` | 单元、迁移顺序、带 `file:line` 的风险、每个单元的桥接层、依赖与 crate 的匹配度、最适合先迁移的单元；`--whole-app` 还会加上入口点和应用形态、每个依赖（导入的或声明的）及其对应 crate、迁移块（相互导入的单元一起迁移），以及带推荐答案的编号决策项 |
| 2. 桥接 | `everything-rust bridge <repo> <unit>` | 生成带绑定代码的 crate（`lib.rs`，以及源语言加载的包装层）和待移植的桩函数（`port.rs`）；`--batch fn:param` 会添加一个把大量 `param` 值留在 Rust 中的类；`--as-json T` 把应用只用来装数据的类型参数作为 JSON 值桥接 |
| 3. 录制 | `everything-rust record <crate>` | 在生成的边界用例（非 ASCII、emoji、`-0`、`NaN`、期望浮点数的地方传整数……）以及每个参数上的每个语义陷阱（字符串长度与大小写、有符号除法、溢出边界、浮点输出、Map 与排序顺序）上运行原实现，并保存 `cases.jsonl`；`--from-tests -- npx jest`（或 `-- pytest`）还会录制原实现自身测试套件发出的每一次调用；`--float-tolerance 1e-9` 让数值代码中的浮点数在该比例内即可视为一致 |
| 3. 构建 | `everything-rust build <crate>` | 构建 Rust 侧：`.node`、wasm 包、Python 扩展或共享库 |
| 3. Diff | `everything-rust diff <crate>` | 以原实现调用方的调用方式，在 Rust 上回放录制用例；列出不一致之处 |
| 3. 基准测试 | `everything-rust bench <crate>` | 在原实现和 Rust 上为录制用例计时，包含桥接开销 |
| 4. 影子运行 | `everything-rust shadow <crate>` | 可直接替换的模块，同时运行两者，返回原实现的结果，并报告不一致；`EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. 门禁 | `everything-rust gate <crate>` | fmt、clippy `-D warnings`、测试、`unsafe` 审计、没有未移植的内容、原实现的每条错误路径都有一条录制的失败用例、在原实现返回错误的地方没有 `unwrap`/`expect`/`panic!`（除非标注了 `// INFALLIBLE: <why>`）、构建、API、行为 |
| 智能体 | `everything-rust setup` | 把 MCP 服务器（`everything-rust mcp`，每个步骤都是一个工具）和工作流技能（[`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)）注册到你的编程智能体 |
## 下一步

每个步骤都有单独的页面：[性能分析与规划](/docs/concepts/analyze)、[桥接层](/docs/concepts/bridge)、[录制](/docs/concepts/record)、[移植](/docs/concepts/port)、[Diff](/docs/concepts/diff)、[基准测试](/docs/concepts/bench)、[影子运行](/docs/concepts/shadow)和[门禁](/docs/concepts/gate)。
