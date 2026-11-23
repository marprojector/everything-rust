---
title: Port a whole app
description: From the first recording to a pure Rust binary. The goal is a Rust app that behaves like the original, with the original kept as reference.
---

# Port a whole app

The goal is a pure Rust app that behaves like the original, with the original kept as reference. Your agent writes the Rust; everything-rust orders the work, scaffolds it, and proves every step. CLIs, HTTP servers and libraries.

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in    pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down proved by
 decisions to make      outside         original answers                                           replay
```

## 1. Plan

```sh
everything-rust analyze . --whole-app
```

`--whole-app` adds the app's shape (CLI, server or library) and entry points, every external dependency (imported or declared) with its Rust crate and fit, the port order as pieces (units that import each other move together), and numbered decisions with a recommended answer: keep units, blocked units, dependencies with no or a partial crate.

## 2. Record the whole app, then move the entry point

1. [`app record` and `app replay`](/docs/whole-apps/record-replay) prove the program from the outside, as a black box.
2. [`app front-door`](/docs/whole-apps/front-door) puts a Rust binary in front of the original; every command still runs the original.
3. Each unit goes through [the loop](/docs/concepts/the-loop), then `app wire` hands its commands to Rust.
4. [`app lift`](/docs/whole-apps/lift) removes the original once replay passes with fallback rate 0 and every unit passes gate.

[Servers](/docs/whole-apps/servers) and [libraries](/docs/whole-apps/libraries) use the same commands.

## How the agent works through it

The [whole-app skill](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) walks the agent through it, starting with the decisions only you can make:

- **Grilling**: the agent asks in rounds, each question with its recommended answer, until every decision the port depends on is settled, and writes each one as an ADR in `rust/DECISIONS.md`, with `rust/GLOSSARY.md` mapping the original's terms to Rust names.
- **Research**: background agents read each dependency's primary sources and write `rust/research/<topic>.md`, with every semantic trap they found.
- **Across sessions**: `rust/MAP.md` lists every unit with its state; a session claims one unit before it starts, and each checkpoint leaves a `rust/HANDOFF.md` for the next session. Units in one wave can be ported in parallel by subagents.
- **Cutover**: the agent does what it can and writes `rust/cutover.sh`, an interactive script for the steps only you can do (deploys, DNS, secrets), with a way back at each step.

Grilling, research and these practices are adapted from [Matt Pocock's skills](https://github.com/mattpocock/skills).

## Won't it lock the Rust into the old design?

Only while both run side by side. The recordings check behavior at the app's edges (a CLI's output, a server's responses, a module's exported functions), not its internals. Mirror the old structure while units still cross the bridge; once `app lift` has removed the original, restructure the Rust freely, and `app replay` still proves the behavior.

## A real one

[json-server](/docs/real-world/json-server) went through every step: 514 recorded HTTP cases, a front door at 99.8% fallback, reads wired first (22.6% fallback), then every route (0%), and a lift to a pure Rust binary that passes 514/514.
