---
title: json-server
description: 第一个端到端迁移的整个应用。一个与 Node 服务端应答完全一致的 Rust 二进制，514/514 条录制 HTTP 用例。
---

# json-server

[json-server](https://github.com/typicode/json-server)（75k stars，MIT，commit `89a34a4`，
1.0.0-beta.15）基于一个 JSON 文件提供 REST API：Node、tinyhttp 和 lowdb。它是第一个
用整应用流程（`app record`、`front-door`、`wire`、`lift`）端到端迁移的应用：
结果是一个与 Node 服务端应答完全一致的 Rust 二进制。全部内容位于
[`demo/real-world/json-server-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/json-server-rs)。

| 步骤 | 结果 |
| --- | --- |
| `analyze --whole-app` | 一个服务端，9 个单元（7 个就绪，2 个需注意），13 个需要替换的依赖（tinyhttp、lowdb、sirv、eta、milliparsec、dot-prop、sort-on、inflection……） |
| `app record --http` | 514 条用例：通过 `--inputs` 提供的 268 个请求（每个路由和方法、where 运算符、排序、分页、嵌入、CORS 预检、静态文件和范围请求、29 个格式错误的 JSON 请求体、`/__proto__` 等原型名称、错误的转义）以及 245 个语义陷阱；每条都录制两次，随机 ID 保留为 `[..]` |
| `front-door` | 513/514，回退率 99.8%；未通过的一条是 URL 中的原始 `ü`，Node 对它返回 400，而代理会重新编码它 |
| 单元 | `where-operators` 207 条、`parse-where` 289 条、`matches-where` 1,346 条、`paginate` 853 条用例（包括来自 json-server 自身测试的调用）；`service` 461 条和 `app` 243 条，通过两个小型适配器运行 Service 调用和真实的 HTTP 请求；六个单元都通过 `gate` |
| `wire` | 先接入读取操作：514/514，回退率 22.6%，写入由 Node 应答，Rust 重新加载 Node 写入的文件；然后接入所有路由：514/514，回退率 0% |
| `lift` | 一个纯 Rust 二进制（5.3 MB），514/514；`bin.ts` 的命令行部分手工移植（帮助、版本、文件检查、启动横幅） |

收口之后，各个单元不再有绑定代码，所以每个 crate 的 `tests/recorded.rs` 会在移植版上回放
它的录制用例：全部 3,399 条都通过 `cargo test` 运行。

移植版必须做对的地方，全部由录制用例发现：

- tinyhttp 的路由器不是只分发一次：每次 `next()` 之后，它都会在上一个中间件改写过的 URL 上
  重新匹配所有中间件和路由，并且对 `HEAD` 运行每一个路由。所以
  `HEAD /posts` 会先运行 GET 处理函数，再运行 POST 的，并返回 POST 的 400；`//posts` 会跳过 CORS
  头和请求体解析器。移植版按原样运行这个循环。
- milliparsec 会把每个 POST、PUT、PATCH 和 DELETE 请求体都当作 JSON 解析，无论其类型如何，而错误的
  请求体会得到一个带 V8 错误消息的 500：`Expected double-quoted property name in JSON at position 7
  (line 1 column 8)`、`Unexpected token 'x', ..."bbbbbbb": x}" is not valid JSON`。移植版
  复现了 V8 解析器的错误，包括以 UTF-16 码元计的位置；已在
  3,806 个格式错误的输入上与 Node 对照检查。
- 排序使用 `localeCompare`（ICU），用 ICU4X 的排序器实现，并在 3,000 对字符串上检查过；
  `Number()`、`JSON.stringify`（`1e+21`、整数键优先）、Node 的两种查询字符串解析器
  （它们对 `é%E9` 的解码方式与 WHATWG 不同）、带错误消息的 dot-prop，以及用于嵌入的 inflection
  （`person` 和 `people`），都在 `json-server-js` 中与 Node 逐一对照检查过。
- `db.data[name]` 会读到 `Object.prototype`：`/__proto__` 返回 `{}`，`/constructor` 返回一个空的
  200，而 `PATCH /__proto__` 会替换数据库的原型，之后的查找都会看到这一变化。
- 对于原始的非 ASCII 请求目标和未知方法，Node 的 HTTP 解析器会返回 400 并关闭连接。

在 json-server 中发现的问题：每次它自己写入之后，它的文件监视器都会重新加载文件。如果
下一个请求在重新加载结束之前写入，重新加载可能会抹掉那次写入：第一次
录制以服务端能应答的最快速度发送请求，结果有一个 POST 返回了 201，之后的读取却
看不到它。`app record --pace 50` 会在请求之间等待，用例就是这样录制的。移植版
只在文件被其他人修改时才重新加载，所以不会丢失那次写入：这是一个有意为之的
差异。

基准测试：Apple M4（10 核，16 GB），macOS 27.3，Node 24.15.0 运行由
`tsc` 为 npm 构建的 `lib/bin.js`，Rust 二进制使用 `--release` 构建。两者都在回环地址上提供同一个
数据库的副本；`ab -k` 在预热之后发送相同的请求，每行 20,000 个（大数据库上较慢的嵌入请求为 40 个）。
共运行两次，每次都在 1 分钟平均负载低于 3 时开始；
单元格给出第 1 次 / 第 2 次运行的每秒请求数。[`bench/bench.py`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/json-server-rs/bench/bench.py)
是所用的脚本。

测试夹具数据库（6 篇帖子，3 条评论）：

| 请求 | Node，1 个连接 | Rust，1 个连接 | Node，50 个连接 | Rust，50 个连接 |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts` | 11.1k / 10.8k | 27.3k / 23.9k | 19.7k / 20.4k | 115.0k / 110.7k |
| `GET /posts/1` | 11.0k / 11.3k | 27.7k / 25.4k | 21.9k / 22.6k | 119.9k / 99.0k |
| 过滤、排序、分页 | 10.5k / 10.0k | 25.2k / 21.5k | 18.3k / 16.4k | 107.5k / 96.3k |
| `?_embed=comments` | 8.7k / 6.6k | 22.5k / 21.8k | 13.8k / 11.0k | 98.3k / 100.6k |
| `GET /`（首页） | 7.3k / 6.3k | 27.6k / 21.7k | 10.2k / 9.5k | 135.1k / 119.3k |
| `GET /test.html`（静态） | 12.0k / 11.0k | 25.8k / 25.9k | 30.3k / 26.1k | 103.9k / 104.4k |
| `POST /comments`，10 个连接 | | | 2,973 / 2,467 | 5,821 / 5,976 |

一个生成的数据库（10,000 篇帖子，20,000 条评论，3.6 MB）：

| 请求 | Node，1 个连接 | Rust，1 个连接 | Node，50（10）个连接 | Rust，50（10）个连接 |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts/5000` | 8.4k / 9.2k | 14.6k / 14.8k | 15.5k / 16.1k | 69.5k / 67.4k |
| `?_page=3&_per_page=10` | 4.6k / 4.2k | 8.8k / 8.4k | 6.5k / 6.8k | 47.4k / 46.2k |
| `title:contains`，排序 | 431 / 396 | 600 / 560 | 465 / 453 | 2,516 / 2,329 |
| `?_embed=comments`，每页 5 条（10 个连接） | 2.4 / 2.4 | 81 / 103 | 2.4 / 2.2 | 253 / 216 |
| `GET /posts`，完整列表（10 个连接） | 328 / 319 | 399 / 407 | 381 / 387 | 1,514 / 1,657 |
| `POST /comments`（10 个连接） | | | 236 / 241 | 292 / 264 |

| | Node | Rust |
| --- | ---: | ---: |
| 从启动到首个响应，测试夹具数据库 | 85 / 79 ms | 9.9 / 7.9 ms |
| 从启动到首个响应，10,000 篇帖子 | 95 / 91 ms | 48 / 47 ms |
| 空闲内存（RSS），测试夹具数据库 | 74 MB | 3.5 MB |
| 空闲内存（RSS），10,000 篇帖子 | 94 MB | 41 MB |
| 运行期间峰值内存（RSS），测试夹具数据库 | 424 / 390 MB | 18 MB |
| 运行期间峰值内存（RSS），10,000 篇帖子 | 462 / 485 MB | 289 / 291 MB |

差距较小的地方：单个连接读取完整的 10,000 篇帖子列表（1.2x：两者的时间都花在
输出 1.5 MB 的 JSON 上），大数据库上的写入（1.1 到 1.2x：两者每次写入都会重写整个
3.6 MB 的文件，Rust 像 lowdb 一样合并排队的写入），以及单个连接上排序的标题
搜索（1.4x）。大幅提升来自并发（Node 进程没有这种能力），
以及嵌入请求：json-server 会为每篇帖子过滤所有评论（移植版
每个请求只为评论建一次索引，结果相同）。

第一个收口后的二进制在大数据库上比 Node 慢（完整列表每秒 33 个请求对 298 个，
写入 39 个对 217 个）。性能分析依次发现：没有
`TCP_NODELAY`（每个大响应都要等约 40 ms 的延迟 ACK），每次写入前都克隆整个数据库来添加 `$schema`，
输出前克隆列表中的每一项，所有请求共用一把锁，
以及对 10,000 个数据项中的每一个都用 SipHash 计算 `item.id` 的哈希。全部已修复；每次修改之后，3,399
条单元用例和 514 条 HTTP 用例都通过。

在工具中发现并已修复的问题：

- 会写入自身状态的服务端无法从同一个起点录制两次：
  现在 `--fresh <path>` 会在每次启动时，从与用例一起保存的快照中，把数据库（以及 `public/`）复制到一个全新的工作目录；
  `--arg` 会把原实现的参数传给每个构建产物；
  `--pace` 见上文。
- `bridge` 把 `paginate<T>(items: T[])` 保留在了 TS 中（对任何类型泛型，包括函数）；
  `--as-json T` 把它作为 JSON 值桥接。
- 前门中带参数的路由在单元看到请求之前，就对 `/posts/%E0` 返回了 axum 自己的 400；
  现在参数从 URI 中读取。
- `lift` 只暂存了已接入的单元，而没有暂存它通过路径依赖的 crate，并且写出了一个
  `cargo fmt --check` 会拒绝的 `lib.rs`；两者都已修复。
- 它生成的服务端没有设置 `TCP_NODELAY`：分两部分写出的响应要等待
  客户端的延迟 ACK，约 40 ms。现在会设置它。

它不覆盖的内容：JSON5 数据库（二进制会提示并退出）、100 KiB 的请求体上限
（Node 的上限取决于请求体如何以 64 KiB 为单位读入；移植版假设完整读取）、带孤立代理项的
字符串（它们会变成 U+FFFD），以及值为 `1e400` 的数据库（Node 在下次重新加载之前会在内存中保留 `Infinity`；
移植版存储 `null`，也就是文件中实际得到的值）。
