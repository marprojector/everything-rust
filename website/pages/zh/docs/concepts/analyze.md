---
title: 性能分析与规划
description: 用 profile 找出时间花在哪里，再用 analyze 规划单元、迁移顺序、风险和 crate。
---

# 性能分析与规划

## Profile：找出值得迁移的部分

当目标是速度时，先测量，再做任何选择：

```sh
everything-rust profile <repo> -- node bench.mjs      # 或：python3 bench.py、ruby bench.rb、
                                                    #     java -cp out Main、go test -run X -bench . ./pkg
```

`profile` 在对应语言的性能分析器（node、cProfile、`go test` pprof、JFR、Ruby tracer）下运行真实工作负载，按包含子调用的时间列出热点函数、它们的调用方，以及哪些循环适合批处理。Go 只能通过 `go test` 做性能分析；C 不支持。

- 每条数据调用一次的小函数，单独迁移**不**值得：一次调用 Rust 的开销比它本身的工作还大，而 JIT（V8、JVM、Go）只需纳秒级时间就能运行它。请把它和它的调用方一起迁移，或者[批处理](/docs/concepts/batches)。建议行会告诉你该选哪种，并给出确切的命令。
- 已经编译过的代码（numba `@njit`、Cython、`@jax.jit`）以机器码运行：Rust 移植版几乎换不来速度提升。`profile` 会把它列在“Already compiled”下。

## Analyze：规划迁移

```sh
everything-rust analyze <repo> -o plan.md        # 给人看
everything-rust analyze <repo> --json            # 给智能体：schema everything-rust/analyze@1
everything-rust analyze <repo> --whole-app       # 另外包括入口点、应用形态、依赖和决策项
```

`analyze` 会列出单元、迁移顺序、带 `file:line` 的风险、每个单元的桥接层、每个依赖映射到 crate 的匹配程度，以及 `start_here`：最适合先迁移的单元。`--whole-app` 还会加上应用形态（CLI、服务端或程序库）和入口点、每个依赖（导入的或声明的）及其对应 crate、迁移块（相互导入的单元一起迁移），以及带推荐答案的编号决策项。见[迁移整个应用](/docs/whole-apps/overview)。

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
## 依赖对应的 crate

`analyze` 会为代码导入的每个库给出对应的 Rust crate，匹配度为 `good`、`partial` 或 `none`。不在映射表中的库会报告为 `unknown`；映射表从不猜测 crate。如何添加映射，见[参与贡献](/docs/project/contributing#add-a-crate-mapping)。
