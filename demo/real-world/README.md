# Real-world ports

The crates everything-rust produced in the runs described in
[`docs/real-world-languages.md`](../../docs/real-world-languages.md): each holds the Rust port
(`src/port.rs`), the generated bridge, `cases.jsonl` (the recorded behavior of the original) and,
where used, `inputs.jsonl` (extra inputs passed to `everything-rust record --inputs`).

| Crate | Original | Where it came from |
| --- | --- | --- |
| `go-path-rs` | Go's `path` package | `$(go env GOROOT)/src/path` (Go 1.24), as module `example.com/gopath`, with `internal/bytealg` calls replaced by their `strings` equivalents |
| `ruby-shellwords-rs` | Ruby's `shellwords.rb` | `RbConfig::CONFIG["rubylibdir"]/shellwords.rb` (Ruby 3.3), as `lib/shellwords.rb` of a project with a `Gemfile` |
| `python-colorsys-rs` | Python's `colorsys.py` | CPython 3.13's `Lib/colorsys.py`, as `src/colorsys_pkg/colorsys.py` of a project with a `pyproject.toml` |
| `java-jdk-rs` | JDK 21 behind `original/Jdk.java` | `original/Jdk.java` in `src/main/java/com/example/jdk/` of a project with a `pom.xml` |
| `c-libc-rs` | glibc behind `original/facade.c` | `original/facade.c` in `src/` of a project with a `Makefile` |
| `php-xml-util-rs` | PEAR `XML_Util` 1.4.5 behind `original/xml_util.php` | `original/xml_util.php` as `src/xml_util.php` beside PEAR's `XML/Util.php` (from Homebrew's PHP) as `src/XML/Util.php`, with `original/composer.json` at the root (see its README) |
| `csharp-protobuf-rs` | Google.Protobuf's `CodedOutputStream.ComputeSize.cs` (BSD-3-Clause) | protobuf's `csharp/` folder at tag `v29.0` as the root (the unit is `src/Google.Protobuf/CodedOutputStream.ComputeSize.cs`; `record` compiles the whole `Google.Protobuf` project); `known-diffs.jsonl` lists the two lone-surrogate cases a Rust `String` cannot receive |
| `vercel-ms-rs` | [vercel/ms](https://github.com/vercel/ms) (MIT) | a clone at commit `4ff48ce`, with the crate at `rust/index-rs` (where `bench_batch.mjs` finds `src/index.ts`) |
| `fuse-rs` | [Fuse.js](https://github.com/krisk/Fuse) (Apache-2.0) | a clone at commit `edf2fb6` with `adapter/relevance.ts` copied to `src/rust/relevance.ts`, bundled by `esbuild src/rust/relevance.ts --bundle --format=esm --platform=node --outfile=build/relevance.mjs` and recorded with `--module build/relevance.mjs`; `bench.mjs` also needs Fuse bundled to `build/fuse.mjs` |
| `json-server-rs` | [json-server](https://github.com/typicode/json-server) (MIT) | a clone at commit `89a34a4`, `pnpm install --frozen-lockfile --ignore-scripts`, with this folder at `rust/` and `adapter/*.ts` copied to `src/rust/`; not a single crate but the whole lifted app (see its README) |
| `prettier-rs` | [Prettier](https://github.com/prettier/prettier) (MIT) | a clone at tag `3.6.2` (commit `7a8b05f`), `corepack yarn build --no-minify`, with this folder at `rust/`; recorded from a scratch folder holding `dist/prettier` as `node_modules/prettier` (see its README) |
| `eslint-rs` | [ESLint](https://github.com/eslint/eslint) (MIT) | `scripts/setup.sh` clones tag `v10.15.0` (commit `a2101d9`), installs its production dependencies, builds the app folder (ESLint as `node_modules/eslint`) and generates the inputs; the front door and the port crates are here, no cases (see its README) |
| `ffmpeg-rs` | [FFmpeg](https://ffmpeg.org)'s `libavutil` (LGPL-2.1+) | `scripts/prepare.sh` clones tag `n7.1` (commit `b08d796`), builds `libavutil.a` and assembles the tree the crates record from; `scripts/verify.sh` records, diffs and gates every crate (no cases are kept here; see its README) |

The originals are not copied here: the standard-library sources keep their own licenses, and the
ports name theirs in `src/port.rs`. To rerun one, put the original in place as described, point
`everything-rust.json`'s `root` at that project, and run:

```sh
everything-rust record <crate> --inputs <crate>/inputs.jsonl   # or keep the recorded cases.jsonl
everything-rust gate <crate>
everything-rust bench <crate>
```
