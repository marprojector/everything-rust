# Install, update and develop

Every way to install everything-rust, keep it up to date, remove it, and build it from source. Back to the [README](../README.md).

## Install

```sh
cargo install --path .
```

`record`, `diff` and `bench` run the original in its own runtime: Node 22.6+ for TS (it runs `.ts`
sources directly), Python 3.9+ for Python and C (ctypes), Go 1.21+, Ruby 3+, JDK 17+
(`javac` and `java`). `profile` on Node needs Node 24+, which allows `--cpu-prof` in `NODE_OPTIONS`. C also needs a C compiler. wasm bridges also need `rustup target add wasm32-unknown-unknown` and
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked`; `build` names
the exact version.

### Windows

Native Windows CI uses the `x86_64-pc-windows-msvc` Rust target. Install the stable MSVC
toolchain and Visual Studio Build Tools with the **Desktop development with C++** workload; the
C compiler and linker are required for C bridges and native Rust extensions. Install the runtime
for the source language you are migrating. Python may be available as either `python3` or
`python`; everything-rust detects both. The GNU Windows target is not exercised by CI.

Run the native bridge integration suite from a Developer PowerShell prompt:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

The suite builds and loads real DLL-backed Node and Python extensions and the supported native
bridges for each runtime present on `PATH`. Missing optional runtimes are reported as skipped;
Cargo, the active Rust standard library, and the MSVC linker are required. If Cargo reports that
it cannot find `std`, reinstall the active target with
`rustup target remove x86_64-pc-windows-msvc` followed by
`rustup target add x86_64-pc-windows-msvc`. If linking fails, run from a Visual Studio Developer
PowerShell and verify that `where.exe link` finds the MSVC linker. Tests use temporary folders,
including a path with spaces and Unicode characters, and remove generated artifacts after a
successful run.

For agents, `everything-rust setup` registers everything (see Quick start). The same MCP server and
skill also ship as a Codex plugin from `.agents/plugins/marketplace.json`
(`codex plugin marketplace add marprojector/everything-rust`, then
`codex plugin add everything-rust@everything-rust`). To add the MCP server to one project by hand
instead (`.mcp.json`):

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

OpenCode takes a local server under `mcp.servers` and the whole argv in one `command` array, in
`~/.config/opencode/opencode.json` (an older file that lists servers directly under `mcp` keeps
that shape):

```json
{
  "mcp": {
    "servers": {
      "everything-rust": { "type": "local", "command": ["everything-rust", "mcp"] }
    }
  }
}
```

Its skill goes in `~/.config/opencode/skills/everything-rust/` (`SKILL.md` and `whole-app.md` from
[`skills/everything-rust/`](../skills/everything-rust)).

Every tool, its parameters and what it may touch: [docs/tools.md](tools.md). The server also
offers two prompts, `port_unit` and `port_whole_app`, built from the skill.

## Development

```sh
cargo test                        # unit tests, fixture repo, record/diff/shadow loop (needs node), app record/replay per language
cargo clippy --all-targets -- -D warnings
cargo fmt
```

Design choices and their reasons: [DECISIONS.md](../DECISIONS.md).

## Update or remove

```sh
cargo install everything-rust                                    # update (Cargo)
npm install -g everything-rust@latest                            # update (npm)
everything-rust setup                                            # refresh agent settings
everything-rust setup --remove && cargo uninstall everything-rust  # remove (Cargo)
everything-rust setup --remove && npm uninstall -g everything-rust # remove (npm)
```

`setup --remove` takes out only everything-rust's entries; the copies it saved
(`<file>.everything-rust.bak`) stay until you delete them.
