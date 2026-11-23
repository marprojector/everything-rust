---
title: Shadow
description: shadow generates a drop-in module that runs the original and the Rust side by side and reports mismatches.
---

# Shadow

```sh
everything-rust shadow <crate>
```

`shadow` generates a drop-in with the original's API: a TS or Python module, a Go package, a Ruby file, Java classes or a C file. Load it where the app loaded the original. It runs both, returns the original's result, and reports Rust mismatches while you build confidence in production.

| `EVERYTHING_RUST_MODE` | Behavior |
| --- | --- |
| `shadow` (default) | runs both, returns the original's result, reports mismatches |
| `new` | switches to Rust |
| `old` | turns Rust off |

The public API does not change. Callers import the same functions, or the shadow module in their place.
