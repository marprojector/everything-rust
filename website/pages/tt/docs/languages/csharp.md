---
title: C# (preview)
description: C# preview рәвешендә. analyze .csproj проектларын укый, ә C# CLI яки ASP.NET серверын язып алып, кабат уйнатып, Rust front door артына куеп була.
---

# C# (preview)

| | |
| --- | --- |
| Берәмлек | `public static` методлар папкасы |
| Bridge | киләсе адым: C ABI өстендә P/Invoke |
| Бүген | бөтен кушымталар: `app record`, `app replay` һәм front door, CLI һәм ASP.NET серверлар өчен (`dotnet <app>.dll`) |
| Халәт | Preview: `analyze` һәм бөтен кушымталар гына; берәмлекләр өчен bridge әлегә юк |

`analyze` `.csproj` проектларын һәм аларның NuGet пакетларын укый (crate'ларга туры китереп), `using` директиваларын папкаларга бәйли һәм C# нәрсәне башкача эшләгәнен билгели: `decimal`, `dynamic`, LINQ'ның кичектерелгән сораулары, `Task`, `lock`, reflection.

Җыелган кушымтаны язып алыгыз:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

Берәмлекләрне P/Invoke аша күчерү киләсе адым. Ул вакытка кадәр `bridge` C# берәмлекләрен аңлаешлы хәбәр белән кире кага.

CI .NET урнаштырмый, шуңа күрә C# тестлары бары ул булган машинада гына җибәрелә. Kotlin һәм Swift әлегә ярдәм ителми; сезгә кирәкле тел өчен issue ачыгыз (яки барына тавыш бирегез).
