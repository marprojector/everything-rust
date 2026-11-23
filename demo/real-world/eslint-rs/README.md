# eslint-rs

[ESLint](https://github.com/eslint/eslint) 10.15.0 (tag `v10.15.0`, commit `a2101d9`) behind a
Rust front door: `eslint --no-config-lookup --rule ... --stdin` runs with the 90 ported rules
are linted in Rust, with output byte-identical to ESLint (messages, locations, fixes,
suggestions, `json` and `stylish` output, exit codes); every other run goes to ESLint unchanged.
The case study is in [`docs/real-world.md`](../../../docs/real-world.md#eslint).

Install it as a command: `npm install -g eslint-rs` runs this front door (macOS and Linux;
Windows runs ESLint itself) and depends on ESLint 10.15.0 for everything it hands back
([what runs where](../../../README.md#install-the-ports)). `everything-rust package app` lays out
the packages ([how](../../../docs/packaging.md#a-whole-cli-front-door)).

ESLint is MIT licensed, Copyright OpenJS Foundation and other contributors; its license is in
[`LICENSE-eslint`](LICENSE-eslint). The Rust in this folder ports ESLint's code and the parts
of the packages it runs on: eslint-scope 9.1.2 and espree 11.2.0's token translation
(BSD-2-Clause, [`LICENSE-eslint-scope`](LICENSE-eslint-scope), [`LICENSE-espree`](LICENSE-espree)),
@eslint-community/eslint-utils 4.10.1 and @eslint-community/regexpp 4.12.3 (MIT,
[`LICENSE-eslint-utils`](LICENSE-eslint-utils), [`LICENSE-regexpp`](LICENSE-regexpp)),
@eslint/plugin-kit 0.7.3's directive parser and eslint-visitor-keys 5.0.1 (Apache-2.0,
[`LICENSE-plugin-kit`](LICENSE-plugin-kit), [`LICENSE-eslint-visitor-keys`](LICENSE-eslint-visitor-keys)).
Each module names the file it ports. No ESLint
source, test or recorded case is copied here: the scripts below regenerate them from a clone.

## Scope

In Rust (see [`DECISIONS.md`](DECISIONS.md)):

- parsing with oxc 0.158, turned into espree's tree, tokens and comments (UTF-16 ranges, espree
  token types, template element ranges, shebang comments); a file oxc or its syntax checks
  reject goes to ESLint
- `SourceCode`, the token store, the traverser with esquery selector ordering, `eslint-disable`,
  `-enable`, `-line`, `-next-line` directives with justifications and unused-directive reports
  and fixes, `/* eslint */` inline config, `/* global */` and `/* exported */`, the report
  translator, the fix loop (10 passes), `--fix-type`, `--quiet`, `--max-warnings`
- eslint-scope, ESLint's code path analysis, eslint-utils' `getStaticValue` and
  `ReferenceTracker`, regexpp's parser and validator
- the 64 rules of `eslint:recommended` and 26 more: `curly`, `default-case`, `dot-notation`,
  `eqeqeq`, `no-alert`, `no-console`, `no-else-return`, `no-empty-function`, `no-eval`,
  `no-implied-eval`, `no-lonely-if`, `no-nested-ternary`, `no-new-wrappers`, `no-param-reassign`,
  `no-shadow`, `no-throw-literal`, `no-unneeded-ternary`, `no-use-before-define`,
  `no-useless-constructor`, `no-useless-return`, `no-var`, `object-shorthand`, `prefer-const`,
  `prefer-template`, `radix`, `yoda` ([`scripts/rules.txt`](scripts/rules.txt))
- the `json` and `stylish` formatters (with `--color`), the CLI's exit codes

In Node, and why: `eslint.config.js` and every config file (JavaScript that has to be evaluated),
plugins, custom parsers (TypeScript, Babel) and processors (JavaScript modules loaded at run
time), the other formatters, linting files named on the command line (file enumeration, ignore
files, caching, suppressions, `--fix` writing files), parse errors (espree's messages), config
errors, the other 202 core rules, and rule options the port does not validate (option regexes
outside a small matcher, `ecmaVersion`, `impliedStrict`).

## Results

| Check | Result |
| --- | --- |
| Corpus | 10,585 CLI cases: 8,856 from ESLint's own RuleTester cases for the 90 rules, 902 real files (ESLint's `lib/` and `bin/`, ms, Fuse.js, json-server) linted with all 90 rules through `json --fix-dry-run` and `stylish`, 555 linter-core cases, 272 traps |
| Parser | oxc's tree, tokens and comments equal espree's on 30,082 of 30,099 inputs from all of ESLint's rule tests; the other 17 are refused (go to ESLint), none differ |
| Code paths | the step and event sequence of ESLint's `CodePathAnalyzer` and segment graphs, compared on 9,706 inputs: all identical |
| Regex | regexpp's `validatePattern` and parse tree compared on 31,232 pattern/mode pairs: all identical |
| `gate eslint-stdin-rs` | passes, 10,192/10,192 recorded `lintStdin` calls |
| `app replay --build` | 10,585/10,585, fallback 3.6% (381 cases): 10,204 cases answered fully in Rust |

## Folders

| Folder | What it is |
| --- | --- |
| `app` | the front door: `cargo build --release`, then `eslint --no-config-lookup --rule '{"eqeqeq":"error"}' --stdin --stdin-filename x.js < x.js`; `src/glue/` parses the flags, reads stdin, lints with `linter-rs`, and hands the run with the bytes it read to ESLint on anything outside the port |
| `linter-rs` | the port (about 33,700 lines of Rust): `source.rs` (parsing, tokens, locations), `linter.rs`, `directives.rs`, `report.rs`, `cli.rs`, `scope*` (eslint-scope), `code_path/`, `regexpp/`, `static_value.rs`, `rules/` (one file per rule); `examples/check.rs` runs every recorded case through it in about 3 s, `examples/dump.rs` and `cpa_events.rs` print the tree and the code path events for comparison with ESLint, `examples/bench.rs` times it |
| `eslint-stdin-rs` | `wrappers/eslint-stdin.js` (`lintStdin(args, text)`: ESLint's `cli.execute` in process) bridged and backed by `linter-rs` |
| `wrappers` | that wrapper, copied into the clone by `scripts/setup.sh` |
| `scripts` | `setup.sh` (clones, installs, the app folder, the inputs), `espree-dump.cjs`, `code-path-dump.cjs` and `regexpp-corpus.cjs` (the parser, code path and regexpp comparisons), `harvest-rule-tests.cjs` (RuleTester cases as CLI runs), `gen-real-inputs.cjs`, `gen-core-inputs.cjs`, `unit-inputs.cjs` (the `lintStdin` calls for the claimed cases), `rules.txt` |
| `bench` | `cli.mjs` (one process per file, ESLint's CLI against the front door) and `linter-node.mjs` (ESLint's `Linter` in process) |

## Reproduce

```sh
scripts/setup.sh <work>
cd <work>/eslint-app
everything-rust app record . --name eslint --inputs ../inputs.jsonl --normalize timestamps -- node node_modules/eslint/bin/eslint.js
(cd rust/app && EVERYTHING_RUST_REPO=$PWD/../.. cargo build --release)
everything-rust app replay . --build rust/app/target/release/eslint
cargo run --release --manifest-path <this folder>/linter-rs/Cargo.toml --example check -- rust/app-cases
```

The gate unit: point `eslint-stdin-rs/everything-rust.json`'s `root` at the clone, then

```sh
CLAIMED=claimed.txt cargo run --release --manifest-path linter-rs/Cargo.toml --example check -- <work>/eslint-app/rust/app-cases
node scripts/unit-inputs.cjs <work>/inputs.jsonl claimed.txt unit-inputs.jsonl
everything-rust record eslint-stdin-rs --count 0 --inputs unit-inputs.jsonl
everything-rust gate eslint-stdin-rs
```

The benchmark (from any folder; the CLI bench makes its own empty one):

```sh
node bench/cli.mjs <work>/eslint-app/node_modules/eslint/bin/eslint.js app/target/release/eslint <work>/inputs.jsonl names.txt 3
node bench/linter-node.mjs <work>/eslint <work>/inputs.jsonl names.txt 20
cargo run --release --manifest-path linter-rs/Cargo.toml --example bench -- <work>/inputs.jsonl names.txt 20
```

`names.txt` lists the real-file `json` cases the port answers (`grep '^real-' claimed.txt | grep -v '^real-stylish'`).

## Benchmark

Apple M4 (10 cores), macOS 27.3, Node 24.15.0, the front door and `linter-rs` built with
`--release`. The 443 real files the port answers (ESLint's `lib/` and `bin/`, ms, Fuse.js,
json-server; from 2 to 3,053 lines), all 90 rules on, `-f json
--fix-dry-run`. Two runs back to back, 1-minute load average 3.0 at the start and 3.9 at the end
(a first attempt at load 19 to 22, while Spotlight indexed, is not used).

| | ESLint | Rust | Ratio (run 1, run 2) |
| --- | ---: | ---: | ---: |
| CLI, one process per file (median of 3 per file), total | 53.5 s, 56.4 s | 2.07 s, 2.15 s | 25.9x, 26.3x |
| CLI, per file | | | median 34.5x, 34.5x; lowest 6.3x, 6.3x; none slower |
| in process (`Linter#verifyAndFix` warm in V8 against `verify_and_fix`, median of 20 per file), total | 2,368 ms, 2,553 ms | 754 ms, 759 ms | 3.1x, 3.4x |
| in process, per file | | | median 2.9x, 3.0x; lowest 1.5x, 1.2x; none slower |
| the file ESLint took longest on (Fuse.js's `dist` bundle, 2,030 lines, in process) | 214 ms, 239 ms | 66 ms, 67 ms | 3.2x, 3.6x |

Every CLI run printed the same bytes from both. A run that falls back costs the front door's
own start on top of ESLint's: 87.4 ms against 84.9 ms (median of 20, `-f json-with-metadata`).

Most of the CLI gap is Node starting and loading ESLint (about 85 ms before any linting). In
process the port is about 3x: a first build measured 2.3x and 2.1x (at load 19 to 22), and two changes moved it to 3x, both
checked against every recorded case again: reading oxc's ESTree JSON straight into the arena
instead of through `serde_json::Value` (which hashed every key into an `IndexMap`), and doing the
listener ordering once per node type and run instead of once per node. What is left is mostly
allocation: the JSON round trip from oxc's AST and the code path analysis, which builds ESLint's
segment objects (with their string ids) as ESLint does.
