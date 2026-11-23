---
title: Front door and wire
description: app front-door puts a Rust binary in front of the original; app wire hands commands to a unit's port once it passes gate.
---

# Front door and wire

Move the entry point to Rust first, then hand it commands one unit at a time:

```sh
everything-rust app front-door .                            # rust/app: every command runs the original
everything-rust app wire . textkit --command upper          # after textkit-rs passes gate
# write the glue in rust/app/src/units/textkit.rs, then:
everything-rust app wire . textkit                          # 100% again, lower fallback rate
```

## The front door

`app front-door` scaffolds `rust/app`, a Rust binary named after the app: a clap parser for the commands and flags in the recorded cases, where every command runs the original with the same argv, stdin and environment and passes its output and exit code through. Each fallback is counted, so replaying the fresh front door gives 100% passed at 100% fallback (it builds and replays to show it). It never overwrites `rust/app`: the code is yours from then on.

## Wiring a unit

`app wire <repo> <unit> --command <name>`:

1. runs `gate` on the unit crate (`--skip-gate` if it just passed);
2. adds it to `rust/app` as a library;
3. writes `src/units/<unit>.rs` once, for you or your agent to fill in: parse the arguments, call `<crate>::port`, print what the original prints;
4. routes the commands to it, then builds and replays.

Run it again after writing the glue: 100% passed and a lower fallback rate. `EVERYTHING_RUST_UNIT_<NAME>=original` sends a wired unit back to the original until the lift.

Wire the app's own name as a command (`--command mytool`) for argv that names no command: no arguments, unknown commands, usage errors.

## Nothing calls back into the original

The front door runs the original as a separate process (or forwards HTTP to it), bridged units are called by the original and never the other way round, and callbacks stay in the source language.

Servers use routes instead of commands: see [Servers](/docs/whole-apps/servers). Libraries use their public API: see [Libraries](/docs/whole-apps/libraries).
