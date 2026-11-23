---
title: Gate
description: gate is the definition of done for a port. Every check it runs, and how to fix a failing one.
---

# Gate

```sh
everything-rust gate <crate>
```

Done means `gate` passes. It checks:

- `cargo fmt`, clippy with `-D warnings`, and cargo tests;
- `unsafe` only with `// SAFETY:` comments, and nothing `unimplemented!`;
- a recorded failing case for every place the original fails on purpose (`throw`, `raise`, an `error` result). Add one with `record --inputs` when gate names it;
- no `unwrap`, `expect` or `panic!` in a function whose original returns an error. Return `Err`; for a state the original also treats as impossible, a `// INFALLIBLE: <why>` comment;
- the port at most 3x the original's lines (plus 50 lines for imports and signatures);
- the module builds, and every export is present;
- every recorded case matches, [known diffs](/docs/concepts/diff#known-diffs) included, and the recording is not [stale](/docs/concepts/diff#stale-recordings);
- for TS/JS, a [coverage](/docs/concepts/coverage) line: how much of the original the cases ran.

Each failed check carries a `fix`: run that first.

## Then report, and wait

When the gate passes, the workflow skill has the agent report what changed (files, functions, bridge, batches), the gate output, diff counts and bench numbers, behavior differences or risks left, and what is left in the plan, and wait for the go-ahead before the next unit.
