---
title: C# (preview)
description: C# is in preview. analyze reads .csproj projects, and a C# CLI or ASP.NET server can be recorded, replayed and put behind a Rust front door.
---

# C# (preview)

| | |
| --- | --- |
| Unit | a folder of `public static` methods |
| Bridge | coming next: P/Invoke over a C ABI |
| Today | whole apps: `app record`, `app replay` and the front door, for CLIs and ASP.NET servers (`dotnet <app>.dll`) |
| Status | Preview: `analyze` and whole apps only; units cannot be bridged yet |

`analyze` reads `.csproj` projects and their NuGet packages (mapped to crates), resolves `using` directives to folders, and flags what C# does differently: `decimal`, `dynamic`, LINQ's deferred queries, `Task`, `lock`, reflection.

Record a built app:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

Porting units through P/Invoke comes next. Until then, `bridge` refuses C# units with a clear message.

CI does not install .NET, so the C# tests run only on a machine that has it. Kotlin and Swift are not supported yet; open an issue (or upvote one) for the language you need.
