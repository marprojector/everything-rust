---
title: Lift
description: app lift makes rust/app a pure Rust app once replay passes with fallback rate 0 and every unit passes gate.
---

# Lift

When replay passes with fallback rate 0 and every unit passes gate, make it a pure Rust app:

```sh
everything-rust app lift .
```

- `app lift` refuses until both hold, and says what blocks it: the cases still falling back, glue still a stub, units failing gate.
- On a copy first, it removes the fallback path and the kill switches, turns each unit crate into a plain library of `rust/app` (no `cdylib`, bindings or generated wrappers), and adds `rust/app/tests/app_cases.rs`, which runs `rust/app-cases` with trycmd on `cargo test`, so CI needs nothing but cargo.
- Then it runs the lift checklist: fmt, clippy `-D warnings` and tests on every unit and on `rust/app`, every target named in `rust/DECISIONS.md` built or checked where it is installed, files read at run time and asset folders not embedded, and replay on the pure binary. Only when all of it passes are the changes written back. The original is never touched.

## After the lift

The units have no bindings any more. On json-server, each crate's `tests/recorded.rs` replays its recorded cases on the port: all 3,399 run with `cargo test`, next to the 514 HTTP cases.

Once the original is gone, restructure the Rust freely; `app replay` still proves the behavior.

## Then cut over

The workflow skill has the agent do what it can and write `rust/cutover.sh`, an interactive script for the steps only you can do (deploys, DNS, secrets), with a way back at each step.
