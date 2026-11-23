---
title: "C#（预览）"
description: "C# 处于预览阶段。analyze 会读取 .csproj 项目，C# CLI 或 ASP.NET 服务端可以被录制、回放，并放到 Rust 前门之后。"
---

# C#（预览）

| | |
| --- | --- |
| 单元 | 包含 `public static` 方法的文件夹 |
| 桥接层 | 即将推出：基于 C ABI 的 P/Invoke |
| 目前可用 | 整个应用：`app record`、`app replay` 和前门，适用于 CLI 与 ASP.NET 服务端（`dotnet <app>.dll`） |
| 状态 | 预览：仅支持 `analyze` 和整个应用；单元暂时还不能桥接 |

`analyze` 会读取 `.csproj` 项目及其 NuGet 包（映射为 crate），把 `using` 指令解析到对应文件夹，并标出 C# 行为不同的地方：`decimal`、`dynamic`、LINQ 的延迟查询、`Task`、`lock`、反射。

录制一个已构建的应用：

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

通过 P/Invoke 迁移单元是下一步。在此之前，`bridge` 会拒绝 C# 单元，并给出明确的提示。

CI 不安装 .NET，所以 C# 测试只会在装有 .NET 的机器上运行。Kotlin 和 Swift 暂不支持；如果你需要某种语言，请提一个 issue（或为已有的 issue 点赞）。
