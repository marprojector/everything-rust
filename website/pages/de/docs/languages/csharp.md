---
title: "C# (Vorschau)"
description: "C# ist in der Vorschau. analyze liest .csproj-Projekte, und eine C#-CLI oder ein ASP.NET-Server lässt sich aufzeichnen, abspielen und hinter eine Rust-Front-Door stellen."
---

# C# (Vorschau)

| | |
| --- | --- |
| Unit | ein Ordner mit `public static`-Methoden |
| Bridge | folgt als Nächstes: P/Invoke über eine C-ABI |
| Heute | ganze Apps: `app record`, `app replay` und die Front Door, für CLIs und ASP.NET-Server (`dotnet <app>.dll`) |
| Status | Vorschau: nur `analyze` und ganze Apps; Units lassen sich noch nicht bridgen |

`analyze` liest `.csproj`-Projekte und ihre NuGet-Pakete (auf Crates abgebildet), löst `using`-Direktiven zu Ordnern auf und markiert, was C# anders macht: `decimal`, `dynamic`, die verzögerten Abfragen von LINQ, `Task`, `lock`, Reflection.

Eine gebaute App aufzeichnen:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

Das Portieren von Units über P/Invoke folgt als Nächstes. Bis dahin lehnt `bridge` C#-Units mit einer klaren Meldung ab.

Die CI installiert kein .NET, daher laufen die C#-Tests nur auf einem Rechner, auf dem es vorhanden ist. Kotlin und Swift werden noch nicht unterstützt; eröffne ein Issue (oder stimme für eines) für die Sprache, die du brauchst.
