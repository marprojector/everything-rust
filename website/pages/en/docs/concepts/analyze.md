---
title: Profile and analyze
description: Find where the time goes with profile, then plan units, port order, risks and crates with analyze.
---

# Profile and analyze

## Profile: find what is worth moving

When the goal is speed, measure before choosing anything:

```sh
everything-rust profile <repo> -- node bench.mjs      # or: python3 bench.py, ruby bench.rb,
                                                    #     java -cp out Main, go test -run X -bench . ./pkg
```

`profile` runs a real workload under the language's profiler (node, cProfile, `go test` pprof, JFR, a Ruby tracer) and lists hot functions by inclusive time, their callers, and which loops to batch. Go profiles only through `go test`; C is not supported.

- A small function called once per item is **not** worth porting alone: one call into Rust costs more than its work, and JITs (V8, the JVM, Go) run it in nanoseconds. Port it together with its callers, or [batch it](/docs/concepts/batches). The advice line says which, with the exact command.
- Code that is already compiled (numba `@njit`, Cython, `@jax.jit`) runs as machine code: a Rust port gains little speed. `profile` lists it under "Already compiled".

## Analyze: plan the port

```sh
everything-rust analyze <repo> -o plan.md        # for people
everything-rust analyze <repo> --json            # for agents: schema everything-rust/analyze@1
everything-rust analyze <repo> --whole-app       # plus entry points, shape, dependencies, decisions
```

`analyze` lists units, the port order, risks with `file:line`, the bridge per unit, how well each dependency maps to a crate, and `start_here`: the best first ports. `--whole-app` adds the app's shape (CLI, server or library) and entry points, every dependency (imported or declared) with its crate, pieces (units that import each other move together) and numbered decisions with a recommended answer. See [Port a whole app](/docs/whole-apps/overview).

<!-- include: docs/how-it-works.md#what-analyze-looks-at -->

## Crates for dependencies

`analyze` names the Rust crate for each library the code imports, with a fit of `good`, `partial` or `none`. A library missing from its table is reported as `unknown`; the table never guesses a crate. To add one, see [Contributing](/docs/project/contributing#add-a-crate-mapping).
