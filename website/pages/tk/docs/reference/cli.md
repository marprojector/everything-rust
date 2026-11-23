---
title: CLI gollanmasy
description: "everything-rust-iň ähli buýruklary we opsiýalary, everything-rust --help we her subbuýrugyň --help çykyşyndan."
---

# CLI gollanmasy

`everything-rust --help` we her subbuýrugyň `--help` çykyşyndan döredildi (everything-rust 1.5.1).

## `everything-rust`

```text
Rewrite TypeScript/JavaScript, Python, C, Go, Ruby or Java apps in Rust, one verified piece at a time

Usage: everything-rust <COMMAND>

Commands:
  analyze  1. Map a repository and write a ranked, ordered migration plan
  bridge   2. Scaffold a Rust crate with JS bindings for one unit
  refresh  2b. Regenerate the bindings (`lib.rs`, `index.cjs`, `js.rs`) from the signatures in `port.rs`
  record   3a. Record what the TS functions return on generated and given inputs
  build    3b. Build the crate's JS module (napi-rs `.node` or wasm-bindgen output)
  diff     3c. Replay recorded cases against the Rust module
  bench    3d. Time the recorded cases on the TS original and on Rust, bridge included
  profile  Run a real workload under the language's profiler and say what to port: hot functions, their callers, and loops to batch. `everything-rust profile <repo> -- node bench.mjs` (also `python3 x.py`, `go test -bench . ./pkg`, `java ...`, `ruby x.rb`)
  shadow   4. Generate a drop-in module that runs TS and Rust side by side
  gate     5. Check the definition of done for a port
  app      Record the whole app's behavior and replay any build against it (CLIs)
  mcp      Serve every step as MCP tools over stdio
  setup    Register everything-rust with your coding agents (Claude Code, Codex, Cursor, Gemini CLI): the MCP server and, where the agent reads skills, the workflow skill. Keeps your other settings; run it again any time
  doctor   Check the language runtimes, the agent setup and the MCP server, with the command that fixes each problem. Exits 1 when something required is missing
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

Loop per unit: analyze -> bridge -> record -> port src/port.rs -> build -> diff -> shadow -> gate
```

## `everything-rust analyze`

```text
1. Map a repository and write a ranked, ordered migration plan

Usage: everything-rust analyze [OPTIONS] [ROOT]

Arguments:
  [ROOT]  Repository root [default: .]

Options:
      --json         Print the machine-readable report (schema `everything-rust/analyze@1`) instead of Markdown
  -o, --out <OUT>    Write to this file instead of stdout
      --lang <LANG>  Source language; detected from the repository when omitted [possible values: ts, python, c, go, ruby, java, php, csharp]
      --whole-app    Plan a whole-app port: entry points and shape, every dependency (imported or declared) with its crate, pieces in port order, and decisions for the user
  -h, --help         Print help
```

## `everything-rust bridge`

```text
2. Scaffold a Rust crate with JS bindings for one unit

Usage: everything-rust bridge [OPTIONS] <ROOT> <UNIT>

Arguments:
  <ROOT>  Repository root
  <UNIT>  Unit id, as `analyze` lists it

Options:
      --kind <KIND>           Bridge to generate; defaults to the plan's choice [possible values: napi, wasm]
  -o, --out <OUT>             Crate folder; defaults to `<root>/rust/<name>`
      --name <NAME>           Crate name; defaults to `<unit>-rs`
      --force                 Bridge a unit the plan marks blocked or keep
      --keep <KEEP>           Keep an export in TS: `name` or `"name: reason"`. Repeatable
      --batch <BATCH>         Add a batch class: `function:param` runs `function` over many `param` values kept in Rust, on all cores (what `profile` recommends for leaves called in loops)
      --as-json <TYPE_PARAM>  Bridge a type parameter as JSON values (`--as-json T`) when the app only fills it with data, instead of keeping the generic export in TS. Repeatable
  -h, --help                  Print help
```

## `everything-rust refresh`

```text
2b. Regenerate the bindings (`lib.rs`, `index.cjs`, `js.rs`) from the signatures in `port.rs`

Usage: everything-rust refresh [OPTIONS] [CRATE_DIR]

Arguments:
  [CRATE_DIR]  [default: .]

Options:
      --batch <BATCH>  Add a batch class: `function:param` runs `function` over many `param` values kept in Rust, on all cores. Repeat for more
  -h, --help           Print help
```

## `everything-rust record`

```text
3a. Record what the TS functions return on generated and given inputs

Usage: everything-rust record [OPTIONS] [CRATE_DIR] [-- <TEST_COMMAND>...]

Arguments:
  [CRATE_DIR]        Crate folder made by `bridge` [default: .]
  [TEST_COMMAND]...  The test command for `--from-tests`, after `--`; it runs in the original's folder

Options:
      --count <COUNT>
          Generated cases per function [default: 200]
      --inputs <INPUTS>
          Extra cases: JSONL of `{"fn": name, "args": [...]}`
      --module <MODULE>
          Import the original from this JS module (e.g. built output) instead of the TS source
      --resolve-from <RESOLVE_FROM>
          Resolve npm packages from this folder's `node_modules` (when the repo has none installed)
      --node-arg <NODE_ARGS>
          Extra arguments for node, e.g. `--node-arg=--import=tsx`
      --float-tolerance <FLOAT_TOLERANCE>
          Floats match when they differ by at most this times the larger magnitude (absolute below 1), e.g. 1e-9 for numeric code. Saved with the crate for diff and gate; 0 removes it
      --from-tests
          Also record every call the original's own test suite makes to the unit's functions: `--from-tests -- npx jest` (TS/JS, any runner) or `-- pytest` (Python)
  -h, --help
          Print help
```

## `everything-rust build`

```text
3b. Build the crate's JS module (napi-rs `.node` or wasm-bindgen output)

Usage: everything-rust build [CRATE_DIR]

Arguments:
  [CRATE_DIR]  [default: .]

Options:
  -h, --help  Print help
```

## `everything-rust diff`

```text
3c. Replay recorded cases against the Rust module

Usage: everything-rust diff [OPTIONS] [CRATE_DIR]

Arguments:
  [CRATE_DIR]  [default: .]

Options:
      --module <MODULE>
          Test this JS module instead of the crate's `index.cjs`
      --ignore-key-order
          Compare objects regardless of key order
      --float-tolerance <FLOAT_TOLERANCE>
          Floats match within this tolerance for this run (the one saved by `record` otherwise)
      --json
          
      --node-arg <NODE_ARGS>
          
  -h, --help
          Print help
```

## `everything-rust bench`

```text
3d. Time the recorded cases on the TS original and on Rust, bridge included

Usage: everything-rust bench [OPTIONS] [CRATE_DIR]

Arguments:
  [CRATE_DIR]  [default: .]

Options:
      --min-ms <MIN_MS>              Minimum measured time per function and side, in milliseconds [default: 200]
      --resolve-from <RESOLVE_FROM>  Resolve npm packages the original imports from this folder's `node_modules`
      --json                         
      --node-arg <NODE_ARGS>         
  -h, --help                         Print help
```

## `everything-rust profile`

```text
Run a real workload under the language's profiler and say what to port: hot functions, their callers, and loops to batch. `everything-rust profile <repo> -- node bench.mjs` (also `python3 x.py`, `go test -bench . ./pkg`, `java ...`, `ruby x.rb`)

Usage: everything-rust profile [OPTIONS] <ROOT> -- <COMMAND>...

Arguments:
  <ROOT>        Repository root
  <COMMAND>...  The workload, after `--`

Options:
      --json         
      --lang <LANG>  Source language; detected from the repository when omitted [possible values: ts, python, c, go, ruby, java, php, csharp]
  -h, --help         Print help
```

## `everything-rust shadow`

```text
4. Generate a drop-in module that runs TS and Rust side by side

Usage: everything-rust shadow [OPTIONS] [CRATE_DIR]

Arguments:
  [CRATE_DIR]  [default: .]

Options:
  -o, --out <OUT>  Where to write the module; defaults to `<crate>/shadow/<name>.shadow.ts`
  -h, --help       Print help
```

## `everything-rust gate`

```text
5. Check the definition of done for a port

Usage: everything-rust gate [OPTIONS] [CRATE_DIR]

Arguments:
  [CRATE_DIR]  [default: .]

Options:
      --json                  
      --node-arg <NODE_ARGS>  
  -h, --help                  Print help
```

## `everything-rust app`

```text
Record the whole app's behavior and replay any build against it (CLIs)

Usage: everything-rust app <COMMAND>

Commands:
  record      Run the original CLI on cases (its own examples, `--inputs`, semantic traps), twice each, and store them as trycmd files in `<repo>/rust/app-cases/`. `everything-rust app record . -- python3 cli.py`
  replay      Run a build against `<repo>/rust/app-cases/`, print passed/total, write a report with a diff per failing case; exits non-zero on any mismatch
  front-door  Scaffold `<repo>/rust/app`, the Rust binary that owns the CLI's entry point: a clap parser for the recorded commands and flags, every command running the original until a unit is wired. Builds it and replays the cases (100% passed, 100% fallback)
  wire        Hand commands to a unit's `port.rs` (after it passes gate): adds the crate to `rust/app`, a glue file to write once, the routing and a kill switch (`EVERYTHING_RUST_UNIT_<NAME>=original`), then builds and replays
  lift        Make `rust/app` a pure Rust app, once replay passes with fallback rate 0 and every unit passes gate: removes the fallback and the kill switches, turns the units into plain libraries, runs the lift checklist and replays the pure binary. Checked on a copy first; the original stays in place
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## `everything-rust app record`

```text
Run the original CLI on cases (its own examples, `--inputs`, semantic traps), twice each, and store them as trycmd files in `<repo>/rust/app-cases/`. `everything-rust app record . -- python3 cli.py`

Usage: everything-rust app record [OPTIONS] [REPO] [-- <RUN>...]

Arguments:
  [REPO]    Repository root [default: .]
  [RUN]...  The command that runs the original, after `--`; paths are relative to the repository. Taken from the saved cases when omitted

Options:
      --inputs <INPUTS>
          Extra cases: JSONL of `{"name", "args": [...], "stdin", "env": {...}, "files": {path: text}}`
      --normalize <NORMALIZE>
          Normalizer: `timestamps`, `uuids`, `key-order`, or `name=regex` for your own. Repeatable; saved with the cases
      --name <NAME>
          The name the app is invoked by in its docs (found from the command and manifests when omitted)
      --no-traps
          Leave out the semantic-trap inputs
      --timeout <TIMEOUT>
          Seconds before a run is killed (default 30)
      --http
          The app is a server: the command starts it, taking its port from PORT or `{port}`; cases are requests (curl lines in its docs, `--inputs`, traps), kept in `http/`
      --port <PORT>
          A port the server always listens on (implies --http)
      --arg <ARG>
          An argument every server start gets after its command, the original's and every build's, so the Rust binary is started as the original is (`--arg db.json`). Repeatable; implies --http; saved with the cases
      --fresh <FRESH>
          A file or folder of the repository the server reads or writes (a database file, a public folder): snapshotted into `http/files/` and copied into a fresh working directory before every start. Repeatable; implies --http; saved with the cases
      --pace <MS>
          Milliseconds to wait after each response before the next request, for work a server does after answering (a file watcher reloading what it just wrote). Implies --http; saved with the cases and used by replay; 0 removes it
      --float-tolerance <FLOAT_TOLERANCE>
          Numbers in the output match when they differ by at most this times the larger magnitude (absolute below 1), e.g. 1e-9 for numeric code. Saved with the cases; 0 removes it
      --json
          
  -h, --help
          Print help
```

## `everything-rust app replay`

```text
Run a build against `<repo>/rust/app-cases/`, print passed/total, write a report with a diff per failing case; exits non-zero on any mismatch

Usage: everything-rust app replay [OPTIONS] [REPO]

Arguments:
  [REPO]  Repository root [default: .]

Options:
      --build <BUILD>      Command line of the build to test (the original, the front door, the final binary); the original when omitted
      --timeout <TIMEOUT>  Seconds before a run is killed
      --port <PORT>        Servers: the port the build always listens on; otherwise it gets a free one through PORT and `{port}`
      --case <CASE>        Replay only this case, by the name the report gives it (a library: only this unit). The fast, repeatable loop for fixing one mismatch
      --json               
  -h, --help               Print help
```

## `everything-rust app front-door`

```text
Scaffold `<repo>/rust/app`, the Rust binary that owns the CLI's entry point: a clap parser for the recorded commands and flags, every command running the original until a unit is wired. Builds it and replays the cases (100% passed, 100% fallback)

Usage: everything-rust app front-door [OPTIONS] [REPO]

Arguments:
  [REPO]  Repository root [default: .]

Options:
      --no-check  Only write the files; skip the build and replay
      --library   Treat the repository as a library even if it has a `main`: the front door is a Rust crate with the public API (found by default when there is no main and no server)
      --json      
  -h, --help      Print help
```

## `everything-rust app wire`

```text
Hand commands to a unit's `port.rs` (after it passes gate): adds the crate to `rust/app`, a glue file to write once, the routing and a kill switch (`EVERYTHING_RUST_UNIT_<NAME>=original`), then builds and replays

Usage: everything-rust app wire [OPTIONS] <REPO> <UNIT>

Arguments:
  <REPO>  Repository root
  <UNIT>  Unit crate: its folder, its name under `rust/`, or the unit id it was bridged from

Options:
      --command <COMMANDS>  A command the unit answers. Repeatable; kept from the last wire when omitted
      --route <ROUTES>      Servers: a route the unit answers, axum syntax (`GET /users/{id}`, `/path` for any method, `*` for everything else). Repeatable; kept from the last wire when omitted
      --skip-gate           Do not run gate on the unit first (it just passed)
      --no-check            Only write the files; skip the build and replay
      --json                
  -h, --help                Print help
```

## `everything-rust app lift`

```text
Make `rust/app` a pure Rust app, once replay passes with fallback rate 0 and every unit passes gate: removes the fallback and the kill switches, turns the units into plain libraries, runs the lift checklist and replays the pure binary. Checked on a copy first; the original stays in place

Usage: everything-rust app lift [OPTIONS] [REPO]

Arguments:
  [REPO]  Repository root [default: .]

Options:
      --json  
  -h, --help  Print help
```

## `everything-rust mcp`

```text
Serve every step as MCP tools over stdio

Usage: everything-rust mcp

Options:
  -h, --help  Print help
```

## `everything-rust setup`

```text
Register everything-rust with your coding agents (Claude Code, Codex, Cursor, Gemini CLI): the MCP server and, where the agent reads skills, the workflow skill. Keeps your other settings; run it again any time

Usage: everything-rust setup [OPTIONS]

Options:
      --agent <AGENTS>
          Agents to set up (repeatable); every agent found by default

          Possible values:
          - claude: Claude Code: `~/.claude.json`, skill in `~/.claude/skills`
          - codex:  OpenAI Codex CLI: `~/.codex/config.toml`, skill in `~/.codex/skills`
          - cursor: Cursor: `~/.cursor/mcp.json`
          - gemini: Gemini CLI: `~/.gemini/settings.json`

      --remove
          Take the everything-rust entries out again

      --dry-run
          Show what would change, write nothing

      --json
          

  -h, --help
          Print help (see a summary with '-h')
```

## `everything-rust doctor`

```text
Check the language runtimes, the agent setup and the MCP server, with the command that fixes each problem. Exits 1 when something required is missing

Usage: everything-rust doctor [OPTIONS]

Options:
      --json  
  -h, --help  Print help
```