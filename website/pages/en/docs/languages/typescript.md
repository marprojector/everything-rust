---
title: TypeScript/JavaScript
description: Port Node.js code, TypeScript or JavaScript, to Rust through napi-rs or wasm-bindgen, with the original API kept.
---

# TypeScript/JavaScript

| | |
| --- | --- |
| Unit | a folder or a file |
| Bridge | napi-rs or wasm-bindgen with a generated binary wire; classes and async included |
| Recorded and replayed by | Node, running the `.ts` sources |
| Needs | Node 22.6+ (`profile` needs Node 24+) |
| Status | Verified |

## Choosing the bridge

`analyze` picks napi-rs when the unit or anything it imports needs Node, wasm-bindgen for portable code (it also runs in browsers), and keeps UI and editor code in JS. `bridge --kind napi|wasm` overrides it.

Exports that cannot cross a bridge stay in TS, with the reason, and the shadow module re-exports the original: functions that take or return functions, generators and iterators, RegExp, `instanceof` checks, generics (`--as-json T` bridges one the app only fills with data), and values whose types hold functions. See [Bridge](/docs/concepts/bridge).

## Recording TS as it is

`record` runs TS sources directly: a Node resolve hook follows bundler-style imports, `--resolve-from` supplies npm packages when the repo has none installed, and TS that needs transforming (enums, parameter properties) is retried with `--experimental-transform-types`. `record --from-tests -- npx jest` records every call the original's own test suite makes, with any runner. Coverage of the original is measured for TS/JS ([Coverage](/docs/concepts/coverage)).

## What a port has to get right

Found by recorded cases on real code:

- `str.length`, `indexOf` and `slice` count UTF-16 code units, so an emoji is two positions. Use the generated `utf16` helpers.
- `/.../i` without the `u` flag folds only ASCII letters, while Rust's `regex` folds Unicode, so `"1 ſ"` (long s) and the Kelvin sign would have parsed as seconds in vercel/ms.
- `Math.round` rounds halves up (`-2.5` gives `-2`); Rust's `round` goes away from zero. The generated `js.rs` has `js::math_round`.
- `${n}` prints `1e+21`, `5e-324` and `0` for `-0`. `js::number_to_string` reproduces it.
- `String.prototype.trim` uses JS whitespace, not Rust's `char::is_whitespace`.

## Speed

Per call, a port pays the bridge every time, and V8 runs small functions in nanoseconds. vercel/ms ran at 0.56x per call over its recorded cases; x18n's core ports are slower than the TS as called from JS. As batches, the loop moves to Rust:

| Workload | Original | With everything-rust |
| --- | ---: | ---: |
| x18n's sidebar search, 50,000 keys (Node) | 10 ms per query | 0.5 ms: 20x faster |
| The same search in the browser (wasm, one thread) | 10 ms | 0.95 ms: 11.5x |
| Fuse.js, 50,000 strings | 91.2 ms | 2.8 ms: 32x |
| vercel/ms `parse` x100,000 | 11.8 ms | 0.65 ms: 18x (3.4x counting the handover) |
| vercel/ms `format` x100,000 | 2.1 ms | 2.5 ms with the handover: 0.8x |

Case studies: [json-server](/docs/real-world/json-server), [Fuse.js](/docs/real-world/fuse-js), [vercel/ms](/docs/real-world/vercel-ms), [x18n](/docs/real-world/x18n).
