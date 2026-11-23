---
title: What is everything-rust
description: everything-rust records what your app does today and checks every piece of the Rust rewrite against that recording, until the app is pure Rust.
---

# What is everything-rust

<p class="lead">Rewrite your whole app in Rust, one piece at a time. everything-rust records what your Node.js (TypeScript/JavaScript), Python, C, Go, Ruby or Java app does today, then checks every piece of the Rust rewrite against that recording before it moves on, until the app is pure Rust.</p>

It works on CLIs, HTTP servers and libraries. Your coding agent (Claude Code, Codex, Cursor, Gemini CLI) or you write the Rust. PHP and C# are in [preview](/docs/languages/php).

## What it does, and what it doesn't

everything-rust does not translate code. It supplies what an agent cannot guess reliably:

- **where the time goes**: `profile` runs a real workload and ranks the hot functions;
- **what to port first, and in which order**: `analyze` plans units, waves, risks and crates;
- **a bridge** so the rest of the code calls the Rust with its API unchanged (`bridge`);
- **the exact behavior the port must match**: `record` runs the original on edge cases and saves what it returns or throws;
- **a safe way to ship it**: `shadow` runs both side by side;
- **a gate that says when a port is done**: `gate`.

The original stays the source of truth. `diff` replays every recorded case against the Rust, and nothing ships until they all match and `gate` passes.

## Four steps

| Step | What happens |
| --- | --- |
| **Record** | `record` runs the original on generated edge cases, semantic traps and your own inputs, and saves what it returns or throws. |
| **Port** | `bridge` generates the Rust crate and a wrapper, so callers keep the original API. You or your agent write the Rust. |
| **Check** | `diff` replays every recorded case against the Rust, and nothing ships until they all match and `gate` passes. |
| **Lift** | When replay passes with no fallback and every unit passes gate, `app lift` makes it a pure Rust app. The original is never touched. |

## Is Rust always faster?

No, and everything-rust says so. A small function called once per item is often slower in Rust, because each call crosses a bridge; V8, the JVM and Go run such functions in nanoseconds. `profile` finds the loop around it, `--batch` moves the whole loop, and `bench` gives a verdict before you ship. See [Batches and hot paths](/docs/concepts/batches) for measured numbers, the slow ones included.

## It can only catch what the recording covers

Passing means the Rust matches every case that was recorded, nothing more. Read [What it can't catch](/docs/concepts/limits) before you trust a green gate.

::: warning Run it on a copy
`record` and `app record` run your original code, calling each function (or the whole app) on many generated inputs, and `app record` runs every case twice. `diff`, `gate` and `app replay` run the same cases again, on the port or the original. Code that writes files, talks to a database or calls the network does that once per case. Run them on a copy of the project, with a test database and no production credentials in the environment.
:::

## Next steps

- [Quick start](/docs/introduction/quick-start): install it and ask your agent for a first port.
- [The loop](/docs/concepts/the-loop): every step for one unit.
- [Port a whole app](/docs/whole-apps/overview): from the first recording to a pure Rust binary.
- [Real-world ports](/docs/real-world/overview): json-server, Fuse.js, vercel/ms and more, with the slow results next to the fast ones.
