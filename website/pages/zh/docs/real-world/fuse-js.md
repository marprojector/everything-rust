---
title: Fuse.js
description: 用 Rust 实现的 Fuse.js 字符串列表搜索。3,414/3,414 条录制用例一致，结果字节级一致，在 50,000 个字符串上快 32 倍。
---

# Fuse.js

[Fuse.js](https://github.com/krisk/Fuse)（20k stars，每周约 2000 万次 npm 下载，
commit `edf2fb6`）是无数搜索框背后的模糊搜索库。使用
everything-rust 1.6（尚未发布）迁移；crate 位于 [`demo/real-world/fuse-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/fuse-rs)。

`analyze` 把 `src/` 的每个文件夹都放进了同一个导入循环，所以 Fuse 无法逐个文件迁移。
这次迁移改为针对搜索耗时所在的路径：一个 54 行的适配器
（`adapter/relevance.ts`）导出 Fuse 自己对字符串列表中单个字符串的评分，它由
Fuse 的 `BitapSearch`、字段归一化和 `computeScoreSingle` 构成。在
50,000 个字符串上搜索一次，就要运行这个函数 50,000 次。`bridge --batch relevance:text` 把字符串留在 Rust 中，
并在全部核心上对它们执行搜索。

| 步骤 | 结果 |
| --- | --- |
| 适配器 | 在写任何 Rust 之前，适配器的排序结果在 81 组查询和选项组合上都等于 `new Fuse(list).search(q)` |
| `record --inputs` | 3,414 条用例：来自 Fuse 自身测试夹具的标题、非 ASCII、emoji、长于 32 个单位的模式、每个支持的选项，以及类型错误的选项（被拒绝，149 条用例） |
| 移植 | 673 行 Rust，替代 839 行 Fuse 代码 |
| `diff` | 首次构建即 3,414/3,414 一致 |
| `gate` | 通过 |
| 直接替换 | `withRust(Fuse)` 用 Rust 处理字符串列表搜索，其他一切交给 Fuse；在 360 次搜索中（12 组选项、15 个查询，带和不带 `limit`），它返回的结果与 Fuse 字节级一致 |

每次查询的搜索时间，Apple M4（10 核），Node 24，计时前已校验结果一致：

| 列表 | Fuse.js | Rust | 加速比 |
| ---: | ---: | ---: | ---: |
| 1,000 个字符串 | 1.9 ms | 0.1 ms | 18x |
| 10,000 个字符串 | 19.1 ms | 0.7 ms | 27 到 30x |
| 50,000 个字符串 | 91.2 ms | 2.8 ms | **32x** |
| 50,000 个字符串，单核 | 92.0 ms | 17.5 ms | 5.3x |

移植版必须做对的地方：

- Bitap 运行在 32 位 JS 整数上：`1 << 31` 是负数，而 `(1 << 31) - 1` 是一个 double，在下一次移位时
  变成 `2147483647`。移植版按 JS 的方式保留这些值。
- 文本按 UTF-16 码元索引，所以一个 emoji 占两个位置，与 Fuse 一致。
- `ignoreDiacritics` 是 NFD 加上一组组合标记范围，以及少数单独列出的字母
  （`ł`、`ß`、`ı`）；移植版使用相同的列表。
- `String.prototype.trim` 决定 Fuse 跳过哪些字符串：使用的是 JS 的空白字符定义，而不是 Rust 的
  `char::is_whitespace`（后者包含 U+0085，但不包含 U+FEFF）。

在工具中发现并已修复的问题：数据项是字符串而结果不是字符串的批处理 crate，
会生成一个未使用的辅助函数，导致 `clippy -D warnings` 失败；`gate` 的大小检查只统计了
适配器（54 行，因此被判为“过大” 12.5 倍），而没有统计它导入的 Fuse 文件。

它不覆盖的内容：带 `keys` 的对象列表、扩展搜索和 token 搜索、逻辑查询、
`includeMatches`、自定义的 `sortFn` 或 `getFn`。直接替换模块会把这些原样交给 Fuse。
要获得加速，列表需要在多次搜索之间留在 Rust 中；构建它的开销与
Fuse 自己的索引相当（50,000 个字符串为 3.4 ms，而 Fuse 为 2.8 ms）。
