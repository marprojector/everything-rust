---
title: C# (preview)
description: C# preview ýagdaýynda. analyze .csproj taslamalaryny okaýar, C# CLI-ni ýa-da ASP.NET serwerini bolsa ýazga alyp, gaýtadan işledip we Rust front door-yň arkasyna goýup bolýar.
---

# C# (preview)

| | |
| --- | --- |
| Birlik | `public static` usullaryndan ybarat bukja |
| Bridge | indiki nobatda: C ABI arkaly P/Invoke |
| Häzir | tutuş programmalar: `app record`, `app replay` we front door, CLI-ler we ASP.NET serwerleri üçin (`dotnet <app>.dll`) |
| Ýagdaý | Preview: diňe `analyze` we tutuş programmalar; birlikleri entek bridge edip bolmaýar |

`analyze` `.csproj` taslamalaryny we olaryň NuGet paketlerini okaýar (crate-lere laýyklaşdyrylýar), `using` direktiwalaryny bukjalara çözýär we C#-yň başgaça edýän zatlaryny belleýär: `decimal`, `dynamic`, LINQ-iň yza süýşürilen soraglary, `Task`, `lock`, reflection.

Gurlan programmany ýazga alyň:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

Birlikleri P/Invoke arkaly geçirmek indiki nobatda. Oňa çenli `bridge` C# birliklerini düşnükli habar bilen ret edýär.

CI .NET gurnamaýar, şonuň üçin C# testleri diňe ol gurnalan maşynda işleýär. Kotlin we Swift entek goldanmaýar; size gerek dil üçin issue açyň (ýa-da bar bolanyna ses beriň).
