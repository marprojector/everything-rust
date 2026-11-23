---
title: Batches and hot paths
description: A port called once per item loses to a JIT. Batches move the whole loop into Rust, on all cores, and win even where per-call ports lose.
---

# Batches and hot paths

```sh
everything-rust bridge <repo> <unit> --batch fn:param    # or: everything-rust refresh <crate> --batch fn:param
```

`--batch` generates a class that takes the `param` values once and keeps them in Rust: `new FnBatch(items)` (TS, Python, Java), `Mod::FnBatch.new(items)` (Ruby), `NewFnBatch(items)` (Go). Per query, `run(<other args>)` returns every result as one array, and `rank(<other args>, min)` returns the item indexes, best first. It runs on all cores, and errors are the original's. Every language except C gets batches.

<!-- include: docs/how-it-works.md#hot-paths-keep-the-data-in-rust -->

## Writing a fast index

- Make the caller build the batch once, not per query. Map returned indexes back to items with a plain loop.
- To prepare items once (lowercase, masks, position tables), define `pub struct FnIndex` in `port.rs` with `new(items)` and `run(<other args>)`. It must give the same result as the function, item by item.
- When ranking can skip work, give the index its own `rank(<other args>, min)`. It must match ranking the function's results: score high to low, ties in item order.
- A batch multiplies the port's per-item speed by the cores. Against compiled originals (Go, Java) the port itself must be fast: handle ASCII without Unicode tables, build each result in one pre-sized `String`, avoid intermediate copies.
- Close the batch when done in Go and Java (finalizers are only a fallback).
