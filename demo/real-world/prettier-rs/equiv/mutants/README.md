# Mutation testing

`cargo mutants` measures how much of `prettier-js-rs` the recorded cases actually check. The test
is `prettier-js-rs/tests/replay.rs`. It replays every recorded Prettier CLI case through the port
and fails if any of these happen:

- the output differs from Prettier's recorded stdout;
- the port panics;
- the port newly refuses a case it formats unmutated.

A mutant that survives either exposes a gap in the cases or can't be told apart from the
original code. `triage.tsv` records which, for every survivor of the current sample.

## Running

You need the recorded cases (an `app-cases` folder written by `everything-rust app record`). Record
them from the corpus inputs (see `../../README.md`) plus `../cases.jsonl`:

```sh
everything-rust app record . --name prettier --no-traps --inputs equiv/cases.jsonl -- node node_modules/prettier/bin/prettier.cjs
```

Record with the same Prettier the front door falls back to (`demo/real-world/node_modules`).
Otherwise Prettier's crash messages differ by install path.

Then, from `prettier-js-rs/`:

```sh
# 1. The cases the unmutated port formats (refusing one of these counts as a kill)
CLAIMED=claimed.txt cargo run --profile mutants --example check <app-cases>

# 2. Coverage over those cases
REPLAY_CASES=<app-cases> REPLAY_CLAIMED=claimed.txt REPLAY_ONLY_CLAIMED=1 \
  cargo llvm-cov --release --test replay

# 3. The 1-in-10 mutant sample
env -u CARGO_TARGET_DIR REPLAY_CASES=<app-cases> REPLAY_CLAIMED=claimed.txt \
  cargo mutants --profile mutants --cargo-test-arg=--test=replay \
  --shard 0/10 --sharding round-robin -j 4
```

`claimed.txt` must be regenerated whenever the cases or the port change.

Harness notes:

- **Leave `CARGO_TARGET_DIR` unset.** cargo-mutants gives each job its own target dir. A shared one
  makes jobs test each other's binaries, because cargo gives every copy of the crate the same
  hash. In a valid run, every log under `mutants.out/log` contains `Compiling prettier-js-rs`.
- **Heap cap.** `replay.rs` caps the live heap at 8 GB, which `REPLAY_HEAP_CAP_GB` overrides. A
  mutant that loops while allocating aborts with `replay: heap cap exceeded` instead of exhausting
  RAM. That counts as caught. Count these aborts separately when comparing runs.
- **Timeouts.** cargo-mutants sets the timeout from the baseline. On a loaded machine that is too
  short, and slow-but-correct mutants get counted as timeouts (caught). Rerun the
  `mutants.out/timeout.txt` entries with `--timeout 180` and `-j 1` or `-j 2`.
- **Filtering.** In cargo-mutants 27.1.0, `-F`/`-E` don't filter "delete field" mutants, so a
  filtered run also tests those.

## Triaging a survivor

For each line in `mutants.out/missed.txt`:

1. Read the mutant's diff (`mutants.out/diff/`) and the JavaScript it ports. Each module's header
   names its Prettier, postcss, graphql-js, yaml or remark source.
2. Decide between two verdicts.
   - **EQUIVALENT:** no input can change the output, the refusal or a panic. The reason must be an
     argument, such as dead code, a condition already implied, or a value nothing reads. Failing
     to find an input is not an argument.
   - **GAP:** otherwise. Find an input where the mutant and the unmutated port differ:
     1. Apply the diff to a copy of the crate.
     2. Build the `fmt` example in both copies. `cargo run --profile mutants --example fmt -- x.md`
        formats a file with default options and picks the language from the extension. For CLI
        flags, use `examples/case.rs` on a recorded case.
     3. Run Prettier 3.6.2 on the input (`prettier --stdin-filepath x.md [flags]`).
3. Check the killing input against Prettier.
   - **If the unmutated port matches Prettier,** append the input to `../cases.jsonl` as
     `{"name", "args": ["--stdin-filepath", "x.ext", ...flags], "stdin"}`.
   - **If it refuses where Prettier also errors,** add the same line. Recording captures
     Prettier's exit status, and replay fails when the mutant prints output for it.
   - **If the port differs from Prettier,** you have found a port bug. Fix the port, not the case.
4. Record the verdict in `triage.tsv`. Then re-record and rerun the sample to confirm the case
   kills the mutant.

## Current numbers

| | First run | Rerun |
| --- | ---: | ---: |
| Port | before the fixes | after the first round of fixes |
| Sample (viable) | 1,168 (1,034) | 1,223 (1,077) |
| Killed | 831 (80.4%) | 877 (81.4%, 95% CI 79.0–83.6) |
| Equivalent survivors | 72 | 92 |
| Score without equivalents | 86.4% | 89.0% |
| Line coverage | 94.7% | 95.2% |

The cases in `../cases.jsonl` kill 107 of the rerun's 108 GAP survivors. With them recorded,
984 of the 985 non-equivalent mutants are killed. The exception is R161: it only drops a refusal,
and its output is then exactly Prettier's, so no case can catch it.

The rerun sample is for the port after the first round of fixes. Since then, `comments.rs`, `css/mod.rs`, `css/postcss.rs`,
`jsx.rs`, `postprocess.rs`, `printer.rs` and `soundness.rs` have changed, so their rows (and line numbers in `triage.tsv`) describe that
state.

## triage.tsv

One row per survivor of the rerun sample. Tab-separated, with a header:

| Column | Meaning |
| --- | --- |
| `id` | `R###` is the line number in that run's `missed.txt`. `T##` is a mutant counted as a timeout that survived when rerun with a longer timeout. |
| `mutant` | the cargo-mutants name: `file:line:col: description` |
| `verdict` | `EQUIVALENT` or `GAP` |
| `reason` | why it is equivalent, or what input distinguishes it. A port bug it led to is noted in brackets. |
| `killer` | the name of the killing case in `../cases.jsonl`, or `-` |
