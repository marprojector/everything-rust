---
title: "C# (preview)"
description: "C# preview күйінде. analyze .csproj жобаларын оқиды, ал C# CLI немесе ASP.NET серверін жазып алуға, қайта ойнатуға және Rust front door-ының артына қоюға болады."
---

# C# (preview)

| | |
| --- | --- |
| Бірлік | `public static` әдістерінен тұратын бума |
| Bridge | келесі кезекте: C ABI арқылы P/Invoke |
| Бүгін | бүкіл қолданбалар: `app record`, `app replay` және front door, CLI-лар мен ASP.NET серверлері үшін (`dotnet <app>.dll`) |
| Күйі | Preview: тек `analyze` және бүкіл қолданбалар; бірліктерге әлі bridge жасау мүмкін емес |

`analyze` `.csproj` жобалары мен олардың NuGet пакеттерін оқиды (crate-терге сәйкестендіріледі), `using` директиваларын бумаларға шешеді және C# басқаша істейтін нәрселерді белгілейді: `decimal`, `dynamic`, LINQ-тің кейінге қалдырылған сұраныстары, `Task`, `lock`, reflection.

Құрастырылған қолданбаны жазып алу:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

Бірліктерді P/Invoke арқылы порттау келесі кезекте. Оған дейін `bridge` C# бірліктерін түсінікті хабармен қабылдамайды.

CI .NET орнатпайды, сондықтан C# тесттері тек ол орнатылған машинада ғана іске қосылады. Kotlin мен Swift әзірге қолдау көрсетілмейді; өзіңізге керек тіл үшін issue ашыңыз (немесе барына дауыс беріңіз).
