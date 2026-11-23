---
title: MCP vositalari
description: everything-rust MCP serveri taqdim etadigan har bir vosita, uning parametrlari va u nimalarga tegishi mumkinligi.
---

# MCP vositalari

`everything-rust mcp` har bir qadamni stdio orqali MCP vositasi sifatida taqdim etadi. `everything-rust setup` uni agentlaringizda ro'yxatdan o'tkazadi ([Agentingizni sozlash](/docs/introduction/agents)).

MCP serverining vositalar ro'yxatidan `tests/tools.rs` tomonidan generatsiya qilingan; qo'lda tahrirlamang (`UPDATE_TOOLS_DOC=1 cargo test --test tools` uni qayta yozadi). Har bir vosita xuddi shu parametrlarga ega CLI buyrug'i hamdir.

## Repozitoriyni tahlil qilish

Vosita `analyze` · CLI `everything-rust analyze`

Map a repository (TypeScript/JavaScript, Python, C, Go, Ruby or Java): units, port order (waves), risks with file:line, bridge per unit, dependency-to-crate fit, and `start_here`: the best first ports.

Faqat o'qiydi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `root` | string | ha | Repository root. |
| `lang` | string |  | Source language; detected from the repository when omitted. |
| `whole_app` | boolean |  | Plan a whole-app port: entry points and shape (cli, server, library), every dependency imported or declared with its crate and fit, pieces in port order (units that import each other together), and numbered decisions with a recommended answer for the grilling round. |

## Birlik crate'ini yaratish

Vosita `bridge` · CLI `everything-rust bridge`

Scaffold a Rust crate for one unit: napi-rs or wasm-bindgen bindings in src/lib.rs, stubs to port in src/port.rs, UTF-16 helpers when the unit uses string offsets. Never overwrites an existing folder.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `root` | string | ha |  |
| `unit` | string | ha | Unit id from analyze. |
| `kind` | string |  | Defaults to the plan's bridge. |
| `out` | string |  | Crate folder; defaults to <root>/rust/<name>. |
| `name` | string |  | Crate name; defaults to `<unit>-rs`. |
| `force` | boolean |  | Bridge a unit the plan marks blocked or keep. |
| `keep` | string[] |  | Exports to keep in the source language: `name` or `name: reason`. |
| `batch` | string[] |  | Batch classes, `function:param`: the function runs over many `param` values kept in Rust, on all cores, in one call. A `<Function>Index` struct in port.rs replaces the default per-item loop. |
| `as_json` | string[] |  | Type parameters the app only fills with data (`T` in `paginate<T>(items: T[])`): bridged as JSON values instead of keeping the export in TS. |

## Binding'larni qayta generatsiya qilish

Vosita `refresh` · CLI `everything-rust refresh`

Regenerate a crate's bindings (lib.rs, index.cjs, js.rs) from the signatures in src/port.rs, after changing them, adding any new batch classes. Never touches port.rs.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `crate` | string | ha | Folder of the generated crate (holds everything-rust.json). |
| `batch` | string[] |  | Batch classes to add, `function:param`: the function runs over many `param` values kept in Rust, on all cores, in one call. A `<Function>Index` struct in port.rs replaces the default per-item loop. |

## Asl nusxa xatti-harakatini yozib olish

Vosita `record` · CLI `everything-rust record`

Run the original functions on generated edge-case inputs (non-ASCII, emoji, -0, NaN, ...) plus optional extra inputs, and save the results as cases.jsonl: the behavior the port must match.

Asl nusxa kodi yoki buyruqlarini ishga tushiradi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `crate` | string | ha | Folder of the generated crate (holds everything-rust.json). |
| `count` | integer |  | Generated cases per function (default 200). |
| `inputs` | string |  | JSONL file of extra cases: {"fn": name, "args": [...]}. |
| `module` | string |  | JS only: import the original from this JS module instead of the TS source. |
| `resolve_from` | string |  | Resolve npm packages from this folder's node_modules. |
| `float_tolerance` | number |  | Numeric code: floats match when they differ by at most this times the larger magnitude (absolute below 1), e.g. 1e-9. Saved with the crate for diff and gate; 0 removes it. |
| `node_args` | string[] |  | Extra arguments for node, e.g. ["--import=tsx"]. |
| `test_command` | string[] |  | TS/JS or Python: also record every call the original's own test suite makes to the unit's functions, e.g. ["npx", "jest"] or ["pytest"]. Runs in the original's folder. |

## Crate'ni build qilish

Vosita `build` · CLI `everything-rust build`

Build the crate: the napi-rs .node, wasm-bindgen output, Python extension or shared library.

Asl nusxa kodi yoki buyruqlarini ishga tushiradi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `crate` | string | ha | Folder of the generated crate (holds everything-rust.json). |

## Portni yozuv bilan solishtirish

Vosita `diff` · CLI `everything-rust diff`

Replay cases.jsonl against the built Rust module and list mismatches.

Asl nusxa kodi yoki buyruqlarini ishga tushiradi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `crate` | string | ha | Folder of the generated crate (holds everything-rust.json). |
| `ignore_key_order` | boolean |  |  |
| `module` | string |  | Test this JS module instead of the crate's `index.cjs`. |
| `node_args` | string[] |  | Extra arguments for node, e.g. ["--import=tsx"]. |
| `float_tolerance` | number |  | Floats match within this tolerance for this run (the one saved by record otherwise). |

## Ish yukini profil qilish

Vosita `profile` · CLI `everything-rust profile`

Run a real workload (a command: node or python3 script, `go test -bench`, java, ruby) under the language's profiler and rank the repository's hot functions by inclusive time, with their callers, call counts and advice: what to port together, and which loops to batch (`bridge` with `batch`). Profile before porting: leaf functions called one at a time are slower in Rust.

Asl nusxa kodi yoki buyruqlarini ishga tushiradi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `root` | string | ha |  |
| `command` | string[] | ha | The workload, run in root: ["node", "bench.mjs"]. |
| `lang` | string |  | Detected from the repository when omitted. |

## Portni benchmark qilish

Vosita `bench` · CLI `everything-rust bench`

Time the recorded cases on the original and on the built Rust module, bridge included; speedup below 1 means the port is slower as its callers call it. Returns the rows and a verdict with advice: ship, keep the slower functions, or port a bigger unit.

Asl nusxa kodi yoki buyruqlarini ishga tushiradi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `crate` | string | ha | Folder of the generated crate (holds everything-rust.json). |
| `resolve_from` | string |  |  |
| `min_ms` | integer |  | Minimum measured time per function and side, in milliseconds (default 200). |
| `node_args` | string[] |  | Extra arguments for node, e.g. ["--import=tsx"]. |

## Shadow modulini generatsiya qilish

Vosita `shadow` · CLI `everything-rust shadow`

Generate a drop-in shadow (a TS or Python module, Go package, Ruby file, Java classes or C file) that runs the original and the Rust port side by side and reports mismatches; EVERYTHING_RUST_MODE=old|shadow|new switches at runtime.

Asl nusxa kodi yoki buyruqlarini ishga tushiradi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `crate` | string | ha | Folder of the generated crate (holds everything-rust.json). |
| `out` | string |  |  |

## Tayyorlik ta'rifini tekshirish

Vosita `gate` · CLI `everything-rust gate`

Definition of done: fmt, clippy -D warnings, cargo test, unsafe with SAFETY comments only, no unimplemented functions, build, API present, recorded behavior matches.

Asl nusxa kodi yoki buyruqlarini ishga tushiradi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `crate` | string | ha | Folder of the generated crate (holds everything-rust.json). |
| `node_args` | string[] |  | Extra arguments for node, e.g. ["--import=tsx"]. |

## Butun ilovani yozib olish

Vosita `app_record` · CLI `everything-rust app record`

Record the whole CLI app's behavior: run the original on its own documented examples, an inputs file and semantic traps (non-ASCII and emoji, negative division, overflow, float extremes, map order), twice each. What differs between runs is reported by kind in `noise`: values (now [..] wildcards), order (lines compared in any order) and lines (a ... that matches anything): show every one to the user. Cases are trycmd files in <repo>/rust/app-cases/ (argv, stdin, env, files in; stdout, stderr, exit code, files out).

Asl nusxa kodi yoki buyruqlarini ishga tushiradi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `repo` | string | ha | Repository root. |
| `run` | string[] |  | Command that runs the original, paths relative to repo: ["python3", "cli.py"]. Optional after the first record. |
| `inputs` | string |  | JSONL of extra cases: {"name", "args": [...], "stdin", "env": {...}, "files": {path: text}}. |
| `normalize` | string[] |  | Normalizers: timestamps, uuids, key-order, or name=regex. |
| `name` | string |  | Name the app is invoked by in its docs. |
| `timeout` | integer |  | Seconds before a run is killed (default 30). |
| `traps` | boolean |  | Add semantic-trap cases (default true). |
| `http` | boolean |  | The app is a server: `run` starts it (it takes its port from PORT or {port}); cases are requests from curl lines in its docs, `inputs` ({"name", "method", "path", "headers", "body"}) and traps. |
| `port` | integer |  | A port the server always listens on (implies http). |
| `args` | string[] |  | Servers: arguments every start gets after its command, the original's and every build's (["db.json"]). Implies http; saved. |
| `fresh` | string[] |  | Servers: repository files or folders the server reads or writes, snapshotted and copied into a fresh working directory before every start. Implies http; saved. |
| `pace` | integer |  | Servers: milliseconds to wait after each response, for work the server does after answering (a file watcher). Implies http; saved and used by replay; 0 removes it. |
| `float_tolerance` | number |  | Numeric output: numbers match when they differ by at most this times the larger magnitude (absolute below 1), e.g. 1e-9. Saved with the cases; 0 removes it. |

## Ilova holatlarini qayta ijro etish

Vosita `app_replay` · CLI `everything-rust app replay`

Replay <repo>/rust/app-cases/ against a build (the original, the front door, the final Rust binary): passed/total, fallback rate when the build reports it, and a report with a diff per failing case (rust/app-cases/replay-report.md).

Asl nusxa kodi yoki buyruqlarini ishga tushiradi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `repo` | string | ha | Repository root. |
| `build` | string |  | Command line of the build to test; the original when omitted. |
| `port` | integer |  | Servers: the port the build always listens on; otherwise it gets a free one through PORT and {port}. |
| `timeout` | integer |  | Seconds before a run is killed (default 30). |
| `case` | string |  | Replay only this case, by the name the report gives it (a library: only this unit): the fast loop for fixing one mismatch. |

## Rust front door'ini yaratish

Vosita `app_front_door` · CLI `everything-rust app front-door`

Scaffold <repo>/rust/app, the Rust binary that owns the CLI's entry point: a clap parser for the recorded commands and flags, every command running the original with the same argv, stdin and env (each fallback counted). For a server: an axum app that starts the original on a port of its own and proxies every route no unit answers. Builds it and replays the cases: 100% passed at 100% fallback. Needs app_record first; never overwrites rust/app.

Asl nusxa kodi yoki buyruqlarini ishga tushiradi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `repo` | string | ha | Repository root. |
| `check` | boolean |  | Build and replay (default true). |
| `library` | boolean |  | Treat the repository as a library even if it has a main: the front door is a Rust crate with the public API, re-exporting each wired unit's port. Found by default when there is no main and no server. |

## Birlikni ilovaga ulash

Vosita `app_wire` · CLI `everything-rust app wire`

After a unit passes gate, hand its commands to its port.rs: adds the unit crate to rust/app as a library, writes a glue stub (src/units/<unit>.rs, yours to write: parse args, call <crate>::port, print what the original prints), regenerates the routing with a kill switch EVERYTHING_RUST_UNIT_<NAME>=original, then builds and replays. Run it again after writing the glue: 100% passed and a lower fallback rate.

Asl nusxa kodi yoki buyruqlarini ishga tushiradi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `repo` | string | ha | Repository root. |
| `unit` | string | ha | Unit crate folder, its name under rust/, or the unit id it was bridged from. |
| `commands` | string[] |  | Commands the unit answers; kept from the last wire when omitted. |
| `routes` | string[] |  | Servers: routes the unit answers, axum syntax: "GET /users/{id}", "/path" for any method, "*" for everything else; kept from the last wire when omitted. |
| `skip_gate` | boolean |  | Do not run gate on the unit first. |
| `check` | boolean |  | Build and replay (default true). |

## Toza Rust ilovasiga lift qilish

Vosita `app_lift` · CLI `everything-rust app lift`

Make rust/app a pure Rust app. Refuses (and lists every blocker) unless replay passes with fallback rate 0 and every unit passes gate. Then, on a copy first: removes the fallback and kill switches, turns each unit into a plain library (no cdylib, bindings or generated wrappers), adds a trycmd test running rust/app-cases on cargo test, checks fmt/clippy/tests, DECISIONS.md targets and assets, and replays the pure binary. Never touches the original.

Fayllarni qayta yozishi mumkin, asl nusxa kodi yoki buyruqlarini ishga tushiradi.

| Parametr | Tip | Majburiy | Tavsif |
| --- | --- | --- | --- |
| `repo` | string | ha | Repository root. |