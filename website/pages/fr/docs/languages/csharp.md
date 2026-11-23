---
title: "C# (préversion)"
description: "C# est en préversion. analyze lit les projets .csproj, et une CLI C# ou un serveur ASP.NET peut être enregistré, rejoué et placé derrière une porte d'entrée Rust."
---

# C# (préversion)

| | |
| --- | --- |
| Unité | un dossier de méthodes `public static` |
| Bridge | à venir : P/Invoke via une ABI C |
| Aujourd'hui | applications entières : `app record`, `app replay` et la porte d'entrée, pour les CLI et les serveurs ASP.NET (`dotnet <app>.dll`) |
| État | Préversion : `analyze` et applications entières uniquement ; les unités ne peuvent pas encore passer par un bridge |

`analyze` lit les projets `.csproj` et leurs packages NuGet (mis en correspondance avec des crates), résout les directives `using` en dossiers, et signale ce que C# fait différemment : `decimal`, `dynamic`, les requêtes différées de LINQ, `Task`, `lock`, la réflexion.

Enregistrer une application compilée :

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

Le portage des unités via P/Invoke est la prochaine étape. D'ici là, `bridge` refuse les unités C# avec un message clair.

La CI n'installe pas .NET, donc les tests C# ne tournent que sur une machine qui l'a. Kotlin et Swift ne sont pas encore pris en charge ; ouvrez une issue (ou votez pour une existante) pour le langage dont vous avez besoin.
