---
title: Doctor
description: everything-rust doctor тел runtime'ларын, агент көйләнмәләрен һәм MCP серверны тикшерә һәм һәр проблеманы төзәтүче команданы күрсәтә.
---

# Doctor

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor` тел runtime'ларын, агент көйләнмәләрен һәм MCP серверны тикшерә һәм һәр проблеманы төзәтүче команданы чыгара. Кирәкле нәрсәдер җитмәсә, ул 1 коды белән тәмамлана.

Runtime, агент көйләнмәләре яки MCP сервер булмаганга яки искергәнгә күрә берәр адым уңышсыз тәмамланса, аны җибәрегез. Workflow skill агентларга да шулай эшләргә куша.

## Һәр телгә нәрсә кирәк

`record`, `diff` һәм `bench` оригиналны аның үз runtime'ында җибәрә: TS өчен Node 22.6+ (ул `.ts` чыганакларын турыдан-туры җибәрә), Python һәм C өчен Python 3.9+ (ctypes), Go 1.21+, Ruby 3+, JDK 17+ (`javac` һәм `java`). Node'та `profile` өчен Node 24+ кирәк, ул `NODE_OPTIONS` эчендә `--cpu-prof` рөхсәт итә. C өчен тагын C компиляторы кирәк. wasm bridge'лар өчен шулай ук `rustup target add wasm32-unknown-unknown` һәм туры килгән `wasm-bindgen-cli` кирәк; төгәл версияне `build` әйтә.

## Катнашучылар өчен

`cargo run -- doctor` сездә кайсы runtime'лар барлыгын, аларның версияләрен һәм һәрберсен урнаштыру яки яңарту командасын күрсәтә. [Катнашу](/docs/project/contributing) битен карагыз.
