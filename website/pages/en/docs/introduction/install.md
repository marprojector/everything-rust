---
title: Install
description: Install everything-rust from crates.io, npm or a release binary, and the runtimes each language needs.
---

# Install

## From crates.io, npm or a release

::: tabs
== cargo
```sh
cargo install everything-rust
everything-rust setup
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

npm installs the same binary, for JavaScript projects.

Prebuilt binaries for macOS, Linux and Windows are attached to each [GitHub release](https://github.com/marprojector/everything-rust/releases). In Claude Code you can also install it as a plugin (the MCP server and the skill; the CLI must be on your `PATH`):

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## From source

<!-- include: docs/install.md#install -->

## Update or remove

<!-- include: docs/install.md#update-or-remove -->
