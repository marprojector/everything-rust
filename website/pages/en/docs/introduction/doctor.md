---
title: Doctor
description: everything-rust doctor checks the language runtimes, the agent setup and the MCP server, with the command that fixes each problem.
---

# Doctor

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor` checks the language runtimes, the agent setup and the MCP server, and prints the command that fixes each problem. It exits 1 when something required is missing.

Run it when a step fails because a runtime, the agent setup or the MCP server is missing or out of date. The workflow skill tells agents to do the same.

## What each language needs

`record`, `diff` and `bench` run the original in its own runtime: Node 22.6+ for TS (it runs `.ts` sources directly), Python 3.9+ for Python and C (ctypes), Go 1.21+, Ruby 3+, JDK 17+ (`javac` and `java`). `profile` on Node needs Node 24+, which allows `--cpu-prof` in `NODE_OPTIONS`. C also needs a C compiler. wasm bridges also need `rustup target add wasm32-unknown-unknown` and the matching `wasm-bindgen-cli`; `build` names the exact version.

## For contributors

`cargo run -- doctor` lists which runtimes you have, their versions, and the command that installs or updates each one. See [Contributing](/docs/project/contributing).
