# Checking the port against Prettier beyond the corpus

The recorded corpus shows the port matches Prettier 3.6.2 on 17,224 runs. These checks look for
inputs where it doesn't. Each check covers a bounded set of inputs, so passing them is evidence
of equivalence, not a proof of it.

- [`fuzz/`](fuzz): differential fuzzing. Generated inputs and options go through the port and
  through Prettier running in process, and the outputs are compared byte for byte.
- [`mutants/`](mutants): `cargo mutants` with the corpus replay as the test. A surviving mutant
  points at code the corpus doesn't check.
- [`doc-diff/`](doc-diff): compares the Rust `Doc` with Prettier's doc for each case. Equal
  docs print equally at every width, not only at the recorded one.
- [`doc-printer/`](doc-printer): every doc up to size 5 goes through both doc printers.
- [`cases.jsonl`](cases.jsonl): 355 minimized cases these checks found, in the `--inputs`
  format. Record and replay them like the corpus.

## Result

The first run found about 75 differences outside the corpus, over four rounds of fixing and
re-checking. Each one is now fixed or handed to Prettier, and each has a case in `cases.jsonl`.

| Check | Before the fixes | After |
| --- | --- | --- |
| Corpus through the front door | 17,224/17,224, 8.1% fallback | 17,224/17,224, 7.9% fallback |
| `cases.jsonl` through the front door | 119 of the first 200 | 355/355 |
| Fuzzing: mismatches / port accepts what Prettier rejects / panics | 2,408 / 1,257 / 136 in 1.84M runs | 0 / 0 / 0 in the last runs (below) |
| Docs equal to Prettier's, corpus | 15,776 of 15,797 | 15,797 of 15,797 |
| Doc printer, every native doc of size ≤ 5 | not checked | 19.4M docs, 0 mismatches |
| Mutation score, sample of 1 in 10 (without equivalent mutants) | 86.4% | 89.0%; 984 of 985 killed with `cases.jsonl` |

The last fuzzing runs: 921k JS/TS inputs and 339k corpus inputs with random options, with no
mismatch, no wrong acceptance, no panic, and no second-pass difference that Prettier doesn't
share. The 95% upper bound on the failure rate is about 6e-6 for JS and 9e-6 for TS, for inputs
these generators make. Per-generator bounds are in [`fuzz/results.md`](fuzz/results.md).

## What was wrong

- **TypeScript `--trailing-comma es5`** dropped the comma after a broken type-parameter list.
  This caused 98% of the TS mismatches.
- **Comment placement in `.js`**, where Babel's tree differs from oxc's: method keys and
  parameters, optional chains, `export * as`.
- **Input Prettier rejects**, which the port formatted: `break` outside a loop, decorators on
  literals, legacy octal, about 30 TypeScript parser rules. The rules are ported in
  `prettier-js-rs/src/soundness.rs`, and the port now errors where Prettier does.
- **CSS offsets** were counted in bytes where postcss counts UTF-16 units (`src/css/u16.rs`).
  There were also two panics and a hang on non-ASCII input.
- **Doc printer:** `align("")` indented, group ids repeated across Markdown code blocks, and
  several conditional-group edge cases.
- **Smaller ones:** `--end-of-line cr` in template literals, `--tab-width 0`, list numbers above
  2^53, `1e400`, duplicate `__proto__`, YAML core tags on collections.

Where an exact port was out of reach, the port refuses the exact shape and the front door hands
it to Prettier:

- JSX followed by `(` on the next line, which Babel and oxc parse differently.
- An embedded template under `prettier-ignore`.
- Templates with `--tab-width 0` and spaces, where Prettier hangs.
- TypeScript's top-level-`await` reparse.
- Two text-level approximations of TypeScript's speculative parsing.
- A few CSS shapes that hang or crash postcss.

These cost 0.1 percentage points of JS input and 0.03 of TS input that used to run in Rust.

## Not established

- Behaviour on input unlike both the corpus and the generators.
- The two TypeScript approximations: a gap there would show up as the port formatting input
  Prettier rejects. The last runs found none.
- The mutation score was measured after the first round of fixes, not the last.

## Running

The checks need a Prettier 3.6.2 build (see the folder README) and Node 24. The commands are in
each folder's README. To check the cases through the front door:

```sh
everything-rust app record . --name prettier --no-traps --inputs equiv/cases.jsonl -- node node_modules/prettier/bin/prettier.cjs
everything-rust app replay . --build app/target/release/prettier
```

Record with the same Prettier the front door falls back to. Prettier's crash messages contain its
install path.
