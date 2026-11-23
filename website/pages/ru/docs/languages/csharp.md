---
title: "C# (превью)"
description: "C# находится в превью. analyze читает проекты .csproj, а C# CLI или сервер ASP.NET можно записать, воспроизвести и поставить за входную точку на Rust."
---

# C# (превью)

| | |
| --- | --- |
| Модуль | каталог методов `public static` |
| Мост | далее: P/Invoke поверх C ABI |
| Уже сейчас | целые приложения: `app record`, `app replay` и входная точка, для CLI и серверов ASP.NET (`dotnet <app>.dll`) |
| Статус | Превью: только `analyze` и целые приложения; модули пока нельзя подключить через мост |

`analyze` читает проекты `.csproj` и их пакеты NuGet (сопоставляемые с крейтами), разрешает директивы `using` в каталоги и отмечает, что C# делает иначе: `decimal`, `dynamic`, отложенные запросы LINQ, `Task`, `lock`, рефлексия.

Запись собранного приложения:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

Портирование модулей через P/Invoke появится следующим. До тех пор `bridge` отклоняет модули C# с понятным сообщением.

CI не устанавливает .NET, поэтому тесты C# запускаются только на машине, где он есть. Kotlin и Swift пока не поддерживаются; откройте issue (или проголосуйте за существующий) для нужного вам языка.
