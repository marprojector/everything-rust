---
title: Only the slow part
description: go ports the one function a slow command spends its time in, checks it, and switches it on only when every recorded case matches.
---

# Only the slow part

You don't have to port the whole app. Point `go` at a command that is too slow, and it ports the one function that takes the time:

```sh
everything-rust go --agent claude -- python3 app.py
```

| Step | What `go` does |
| --- | --- |
| Profile | Runs the command under the language's profiler ([Profile and analyze](/docs/concepts/analyze)) |
| Pick | The outermost exported function of a portable unit that takes arguments: the loop, not the small call inside it. Where `profile` advises a batch, it bridges with `--batch` ([Batches and hot paths](/docs/concepts/batches)). `--function` picks another |
| Bridge | That function alone, into `rust/<function>-rs`; the unit's other functions stay in the original |
| Record | Generated cases plus the real calls the command makes |
| Port | Writes `TASK.md` and hands `src/port.rs` to the agent: `claude`, `codex`, or any command that reads a prompt on stdin. Failing cases go back to it, up to `--rounds` turns. Without `--agent` it stops here; port it yourself and run the same command again |
| Check | Build, [diff](/docs/concepts/diff), [gate](/docs/concepts/gate) and [bench](/docs/concepts/bench) |
| Switch on | `enable`, only when every case matches, gate passes and the port is not slower as its callers call it ([Switch on and off](/docs/concepts/switch)). A small function called once per item usually is slower: then `go` leaves it off and names the batch class to call from the loop, or the caller to port with it |

Then measure it for the pull request:

```sh
everything-rust report rust/dedupe-rs     # report/REPORT.md and report/card.svg
everything-rust disable rust/dedupe-rs    # back to the original, byte for byte
```

`report` builds, runs `diff` and `bench`, and writes the recorded cases matched, the speedup with the call included, the fallback rate, whether the unit is on, and the machine with its core count.

Try it on [`tests/fixtures/go-fuzzy`](https://github.com/marprojector/everything-rust/tree/main/tests/fixtures/go-fuzzy) (a Python dedupe, where `go` picks the outer `dedupe` over the inner `levenshtein`) or [`tests/fixtures/go-search-ts`](https://github.com/marprojector/everything-rust/tree/main/tests/fixtures/go-search-ts) (a TypeScript leaf called in a loop, which `go` bridges with a batch).
