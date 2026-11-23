---
title: Record and replay
description: app record runs the whole app on its documented examples, your inputs and traps; app replay runs any build against the cases.
---

# Record and replay

`record` and `diff` prove one unit; `app record` and `app replay` prove the app, for CLIs and servers in any of the six languages. The app is a black box.

```sh
everything-rust app record . -- python3 cli.py          # the command that runs the original
everything-rust app replay .                            # the original must pass 100%
everything-rust app replay . --build "target/release/mytool"
```

## Where the cases come from

`app record` runs the original on:

- the app's own documented examples: `$ mytool ...` lines in its Markdown and shell files, with the files they name;
- an `--inputs` JSONL file, one `{"name", "args", "stdin", "env", "files"}` per line;
- semantic traps derived from those cases: non-ASCII and emoji strings, negative division and modulo, overflow, large and tiny floats, map order, missing files.

Each case runs twice; what differs between the two runs becomes a `[..]` wildcard. `--normalize timestamps|uuids|key-order` or `--normalize name=regex` adds normalizers, saved with the cases. `app record` reports what differed by kind: values (now wildcards), order (lines compared in any order) and lines (a `...` that matches anything). Look at every one.

## The cases are plain files

Cases are [trycmd](https://docs.rs/trycmd) files in `rust/app-cases/`:

| File | Holds |
| --- | --- |
| `<case>.toml` | argv, env, exit code |
| `<case>.stdin` | standard input |
| `<case>.stdout`, `<case>.stderr` | what the original printed |
| `<case>.in/` | files in |
| `<case>.out/` | files written |

Read and edit them by hand; a Rust test can run them with `trycmd` once the port has a binary.

## Replay

- `app replay` runs any build against the cases, prints `passed/total`, writes `rust/app-cases/replay-report.md` with a diff per failing case, and exits non-zero on any mismatch.
- A build that appends to the file named by `EVERYTHING_RUST_FALLBACK_LOG` (one line per fallback to the original), as the [front door](/docs/whole-apps/front-door) does, also gets its fallback rate reported.
- `app replay --case <name>` reruns one case by the name the report gives it: the fast, repeatable loop for fixing a mismatch before running everything again.

## Replay in CI

The report has no timings or temporary paths: two replays of the same build write the same file. Commit it, then run in CI:

```sh
everything-rust app replay .
git diff --exit-code rust/app-cases/replay-report.md
```

A half-ported app then fails CI on any change in what passes, better or worse, until the report is updated with it.

## Numeric output

`app record --float-tolerance 1e-9` lets numbers match within that share of the larger value. See [Float tolerance](/docs/concepts/float-tolerance).
