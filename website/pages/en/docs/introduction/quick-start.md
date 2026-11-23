---
title: Quick start
description: Install everything-rust, register it with your coding agent, and run the loop on a first unit.
---

# Quick start

## Let your agent install it

Paste this into Claude Code, Codex, Cursor or Gemini CLI:

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## Or install it yourself

::: tabs
== cargo
```sh
cargo install everything-rust   # from crates.io
everything-rust setup           # registers it with Claude Code, Codex, Cursor and Gemini CLI
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

No Rust toolchain yet? Prebuilt binaries for macOS, Linux and Windows are attached to each [GitHub release](https://github.com/marprojector/everything-rust/releases), and for JavaScript projects npm installs the same binary.

`setup` adds the MCP server to each agent it finds and installs the workflow skill where the agent reads skills. See [Set up your agent](/docs/introduction/agents) for what it changes.

## Ask your agent

- *"Rewrite this CLI in Rust with everything-rust, end to end, and prove it behaves the same."*
- *"This search feels slow. Profile it with everything-rust and move the hot loop to Rust."*
- *"Plan a migration of `src/parser/` to Rust with everything-rust; port the first unit and prove it."*
- *"Bench the Rust port in `rust/text-rs` against the original and tell me whether to ship it."*

## Or run the steps yourself

The steps on x18n's `fuzzyScore`: `profile` finds the hot loop, `bridge --batch` generates the crate, `gate` passes, and the benchmark compares it with the TypeScript.

```sh
everything-rust profile . -- node bench.mjs              # what is hot, and who calls it
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# write src/port.rs (or let your agent), then:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # every recorded case must match
everything-rust bench rust/fuzzy-rs                      # faster? the verdict says
everything-rust gate rust/fuzzy-rs                       # done means this passes
```

Each step is explained in [The loop](/docs/concepts/the-loop).

::: tip When something is missing
If a step fails because a runtime, the agent setup or the MCP server is missing or out of date, run [`everything-rust doctor`](/docs/introduction/doctor).
:::
