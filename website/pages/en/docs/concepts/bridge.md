---
title: Bridge
description: bridge scaffolds a Rust crate for one unit, with bindings and a wrapper so callers keep the original API.
---

# Bridge

```sh
everything-rust bridge <repo> <unit-id>          # crate in <repo>/rust/<name>; never overwrites
everything-rust refresh <crate>                  # after changing a signature in port.rs
```

`bridge` creates a crate with the bindings (`lib.rs`, plus the wrapper the source language loads) and stubs to port (`port.rs`). You write the port in `src/port.rs` only; `src/lib.rs` and the wrappers are generated, and `refresh` regenerates them from the signatures in `port.rs`, adding any new batch classes. It never touches `port.rs`.

Options:

- `--batch fn:param` adds a class that keeps many `param` values in Rust. See [Batches](/docs/concepts/batches).
- `--as-json T` bridges a type parameter the app only fills with data (`T` in `paginate<T>(items: T[])`) as JSON values.
- `--keep "name: reason"` keeps more exports in the source language, e.g. code that depends on `Intl` data.
- `--kind` overrides the bridge the plan chose (napi-rs or wasm for TypeScript).

## The bridge per language

| Language | Bridge |
| --- | --- |
| TypeScript/JavaScript | napi-rs or wasm-bindgen with a generated binary wire; classes and async included |
| Python | PyO3 (abi3), direct conversion, typed exceptions |
| C | `extern "C"` functions with the original names and a header |
| Go | typed cgo calls and a generated binary wire, in a package with the original API |
| Ruby | a magnus extension under the original modules |
| Java | typed JNI and a generated binary wire, in wrapper classes with the original methods |

PHP and C# units cannot be bridged yet; see [PHP](/docs/languages/php) and [C#](/docs/languages/csharp).

## What bridge decides

<!-- include: docs/how-it-works.md#what-bridge-decides -->
