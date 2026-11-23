---
title: "C# (попередній перегляд)"
description: "C# у стадії попереднього перегляду. analyze читає проєкти .csproj, а CLI чи сервер ASP.NET на C# можна записати, відтворити і поставити за front door на Rust."
---

# C# (попередній перегляд)

| | |
| --- | --- |
| Модуль | тека з методами `public static` |
| Міст | далі: P/Invoke через C ABI |
| Сьогодні | цілі застосунки: `app record`, `app replay` і front door, для CLI та серверів ASP.NET (`dotnet <app>.dll`) |
| Статус | Попередній перегляд: лише `analyze` і цілі застосунки; модулі поки не можна з'єднати мостом |

`analyze` читає проєкти `.csproj` та їхні пакети NuGet (зіставлені з крейтами), перетворює директиви `using` на теки і позначає, що C# робить інакше: `decimal`, `dynamic`, відкладені запити LINQ, `Task`, `lock`, рефлексію.

Запишіть зібраний застосунок:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

Наступним кроком буде портування модулів через P/Invoke. Доти `bridge` відхиляє модулі C# із зрозумілим повідомленням.

CI не встановлює .NET, тож тести C# запускаються лише на машині, де він є. Kotlin і Swift поки не підтримуються; відкрийте issue (або проголосуйте за наявний) для потрібної вам мови.
