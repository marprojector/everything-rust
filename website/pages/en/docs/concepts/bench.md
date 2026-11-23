---
title: Bench
description: bench times the recorded cases on the original and on the Rust, bridge included, and gives a verdict. Rust is not always faster.
---

# Bench

```sh
everything-rust bench <crate>
```

`bench` times the recorded cases on the original and on the built Rust module, bridge included, and returns the rows and a verdict with advice: ship, keep the slower functions in the source language, or port a bigger unit. A speedup below 1 means the port is slower as its callers call it. `--min-ms` sets the minimum measured time per function and side (default 200).

Follow the verdict. Do not ship a port that makes callers slower without saying so.

## Why a port can be slower

A port called once per item pays the bridge once per item. What decides it:

- **Interpreted originals (Python, Ruby)**: the call into Rust costs about 100-300 ns and the Rust code is much faster than the interpreter, so even small functions win.
- **Compiled originals (Go, Java after the JIT)**: the original leaf function takes 4-90 ns; one cgo call alone costs about 80 ns, a JNI call with strings more. These languages gain only when one call does a lot of work: port a function together with its callers.
- **C**: there is no crossing; the Rust code is compared with the C code directly.

Measured per call on real code ([six languages](/docs/real-world/six-languages)):

| Language | Rust vs the original (`bench`, geometric mean) | Verdict |
| --- | --- | --- |
| Ruby | **4.4x faster** (3.6-7x per function) | ship |
| Python | **1.35x faster** (1.5-1.9x on most calls) | ship |
| C | 0.87x: ctype 1.1-1.4x faster, glibc's tuned `strtod`, `strspn`, `strcmp` 0.5-0.7x | optimize or keep the slower ones |
| Java | 0.18x | port bigger units |
| Go | 0.04x | port bigger units |

All 29 functions of x18n's core that can move without an API change were ported: every gate passes and 7,925 recorded cases match exactly, and `bench` showed the ports are slower than the TS as called from JS, because the bridge costs more than the work ([report](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-core/README.md)). vercel/ms ran at 0.56x per call over its recorded cases ([case study](/docs/real-world/vercel-ms)).

## What wins: moving the loop

`profile` finds the loop around a hot function, and `--batch` moves the whole loop into Rust, on all cores. See [Batches and hot paths](/docs/concepts/batches).

::: tip Measure on a quiet machine
Run `bench` on a quiet machine, run it twice, and name the hardware and core count next to every published number.
:::
