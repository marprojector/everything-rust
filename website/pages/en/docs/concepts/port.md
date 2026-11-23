---
title: Port
description: Writing port.rs, the one file you or your agent write, and the rules that keep it matching the original.
---

# Port

You or your agent write the port in `src/port.rs`. Everything else in the crate is generated. After changing a signature in `port.rs`, run `everything-rust refresh <crate>`.

## Rules that save a round of diff

- Read each risk the plan lists for the unit, at its `file:line`, before writing code.
- **String offsets**: JS and Java index UTF-16 code units, Rust UTF-8 bytes. Use the generated `src/utf16.rs`.
- **Errors**: return `Err("TypeName: message")`. It becomes that exception in Python, Ruby and Java, and the `error` text in Go. A Rust panic never matches a recorded error.
- **Numbers in Python and Ruby**: ints that the original passes through must come back as ints. In Python ports use `crate::pynum::Num` for int-or-float, not `serde_json::Value`.
- **C**: inputs recorded as `crash:` are undefined behavior; any result passes.
- **Ordered maps and object keys**: `indexmap` where order reaches the output.
- **Regex lookaround or backreferences**: `fancy-regex`, not `regex`. Either way, `\w`, `\b`, `\d` and case-insensitive matching treat non-ASCII text differently from the source language's engine. Set `unicode(false)` or spell out ASCII classes (`[A-Za-z0-9_]`) where the original's `\w` is ASCII-only (JS without `u`, Java).
- **No `unsafe`.** If truly needed: minimal, isolated, with a `// SAFETY:` comment.
- **Keep helpers private.** Only the unit's exports are `pub`; a `pub` helper becomes part of the bridged API.
- Add Rust unit tests in `port.rs` for the cases you reason about.

The generated `js.rs` carries helpers that reproduce JavaScript where Rust differs, such as `js::number_to_string` (JS `String(n)`) and `js::math_round` (`Math.round`).

## An example: the emoji that moved a position

<!-- include: docs/examples.md#example-a-real-module-from-x18n -->

## How big should it be?

`gate` compares the port's lines with the original's and fails above 3x (plus 50 lines for imports and signatures). Over that usually means hand-writing what a crate already does, or handling cases one by one. `analyze` lists the crates to use.
