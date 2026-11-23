---
title: Doctor
description: everything-rust doctor dil runtime-laryny, agent sazlamasyny we MCP serweri barlaýar we her meseläni düzedýän buýrugy görkezýär.
---

# Doctor

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor` dil runtime-laryny, agent sazlamasyny we MCP serweri barlaýar we her meseläni düzedýän buýrugy çap edýär. Zerur bir zat ýetmezçilik edende ol 1 kody bilen çykýar.

Runtime, agent sazlamasy ýa-da MCP serwer ýok ýa-da köne bolany sebäpli bir ädim şowsuz bolanda ony işlediň. Workflow skill-i agentlere hem şeýle etmegi aýdýar.

## Her dile näme gerek

`record`, `diff` we `bench` asyl nusgany onuň öz runtime-ynda işledýär: TS üçin Node 22.6+ (ol `.ts` çeşme faýllaryny göni işledýär), Python we C (ctypes) üçin Python 3.9+, Go 1.21+, Ruby 3+, JDK 17+ (`javac` we `java`). Node-da `profile` üçin Node 24+ gerek, ol `NODE_OPTIONS` içinde `--cpu-prof` ulanmaga rugsat berýär. C üçin şeýle hem C kompilýatory gerek. wasm bridge-leri üçin goşmaça `rustup target add wasm32-unknown-unknown` we oňa laýyk `wasm-bindgen-cli` gerek; takyk wersiýany `build` aýdýar.

## Goşant goşýanlar üçin

`cargo run -- doctor` haýsy runtime-laryňyzyň bardygyny, olaryň wersiýalaryny we olaryň her birini gurnaýan ýa-da täzeleýän buýrugy görkezýär. [Goşant goşmak](/docs/project/contributing) sahypasyna serediň.
