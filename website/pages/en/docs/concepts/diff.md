---
title: Diff and known diffs
description: diff replays every recorded case against the Rust. known-diffs.jsonl records intended differences, and stale recordings fail the gate.
---

# Diff and known diffs

```sh
everything-rust build <crate>
everything-rust diff <crate>                     # must be N/N
everything-rust diff <crate> --json
```

`diff` replays `cases.jsonl` against the built Rust module, called the way the original's callers call it, and lists the mismatches. A mismatch is a behavior difference: fix the port, never the recording.

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

## Known diffs

Sometimes the port is meant to differ: the user wants it to fix a bug of the original. Ask first, then add one line per case to `<crate>/known-diffs.jsonl`:

**known-diffs.jsonl**
```json
{"fn": "...", "args": [...], "actual": ..., "reason": "..."}
```

Copy `fn`, `args` and `actual` from `diff --json`. That case then passes only with exactly that result, and `gate` fails once it no longer differs, so remove the line then.

## Stale recordings

`record` saves a fingerprint of the original unit's files in `recorded-from.json`. When the original changes later, `diff` warns and `gate` fails until you record again, instead of passing against what the original used to do. If `gate` says the recording is stale, record again before porting further.

## When the port stalls

`diff` tracks progress across builds. When it says the port is stalled (its `trend.stalled`: three changes to `port.rs` in a row gained no matching case), stop editing. The remaining mismatches may need a rule only the user knows, or those functions may stay in the original for now.

## Float tolerance

If the crate was recorded with `--float-tolerance`, `diff` uses it; `--float-tolerance` on `diff` overrides it for one run. See [Float tolerance](/docs/concepts/float-tolerance).
