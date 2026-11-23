---
title: Record
description: record runs the original on generated edge cases, semantic traps, its own tests and your inputs, and saves the results as cases.jsonl.
---

# Record

```sh
everything-rust record <crate>
everything-rust record <crate> --from-tests -- npx jest      # or: -- pytest
everything-rust record <crate> --inputs extra.jsonl
```

`record` runs the original functions and saves what each one returns or throws in `cases.jsonl`, next to the crate: the behavior the port must match. A mismatch later is a behavior difference: fix the port, never the recording.

## Where the cases come from

- **Generated edge cases**: non-ASCII, emoji, `-0`, `NaN`, ints where floats are expected, and more (`--count` sets how many per function).
- **Semantic traps** in every parameter: string length and case, signed division, overflow edges, float printing, map and sort order.
- **Regex traps** for every function in a file that uses regexes: the Turkish dotless `ı`, the Kelvin sign, long s, Arabic-Indic digits, line separators, final sigma, zero-width joiners.
- **The original's own tests**: `--from-tests -- <test command>` records every call the test suite makes to the unit's exported functions (TS/JS with any runner, Python with pytest or unittest). The source is restored afterwards. On vercel/ms it captured 340 calls from 167 Jest tests.
- **Your inputs**: `--inputs cases.jsonl` with one `{"fn": name, "args": [...]}` per line. Add real inputs (production samples, bug reports); the user input that breaks prod is rarely one a generator thinks of.

Python callers pass `1` as often as `1.0`, so recording passes integral numbers as ints in every fourth Python case.

## Markers in cases.jsonl

JSON cannot express `undefined`, NaN, Infinity, -0 or lone surrogates. Structured values carry markers that the generated `index.cjs` translates, with helpers in the crate's `js.rs`:

| Marker | Means |
| --- | --- |
| `{"$undefined": true}` | JavaScript `undefined` |
| `{"$num": "NaN"}` | a number JSON cannot hold (`NaN`, `Infinity`, `-0`) |
| `{"$utf16": [...]}` | a string given as UTF-16 code units, for lone surrogates |

C inputs outside a function's contract can crash the original. The C driver runs each case in a forked child and records `crash: SIGSEGV`; `diff` accepts any port behavior there, since C's is undefined.

## Two runs, and what differs

`record` runs the original twice and sorts out what differed between the runs: changed values (times, ids) become wildcards; the same lines in another order (hash maps, concurrency) are compared in any order; a different number of lines becomes `...`, which matches anything. `record` lists each kind. Look at every one, since a hash-order or concurrency difference is exactly what can change for real in a port.

## The recording remembers its original

`record` saves a fingerprint of the unit's files (`recorded-from.json`). When the original changes later, `diff` warns and `gate` fails until you record again, instead of passing against what the original used to do. See [Diff](/docs/concepts/diff#stale-recordings).

## TypeScript specifics

`record` runs TS sources as they are: a Node resolve hook follows bundler-style imports (`./errors` to `./errors.ts`), `--resolve-from` supplies npm packages when the repo has none installed, and TS that needs transforming (enums, parameter properties) is retried with `--experimental-transform-types`. TS that cannot run directly in Node can be recorded from built output with `--module dist/x.mjs`, or with `--node-arg=--import=tsx`.

## Numeric code

For code where floats rarely match bit for bit, record with `--float-tolerance`. See [Float tolerance](/docs/concepts/float-tolerance).
