---
title: "C# (preview)"
description: "C# preview holatida. analyze .csproj loyihalarini o'qiydi, C# CLI yoki ASP.NET serverini esa yozib olish, qayta ijro etish va Rust front door'i ortiga qo'yish mumkin."
---

# C# (preview)

| | |
| --- | --- |
| Birlik | `public static` metodlardan iborat papka |
| Ko'prik | navbatda: C ABI orqali P/Invoke |
| Bugun | butun ilovalar: CLI'lar va ASP.NET serverlari (`dotnet <app>.dll`) uchun `app record`, `app replay` va front door |
| Holat | Preview: faqat `analyze` va butun ilovalar; birliklarni hozircha ko'prik orqali ulab bo'lmaydi |

`analyze` `.csproj` loyihalarini va ularning NuGet paketlarini o'qiydi (crate'larga moslangan), `using` direktivalarini papkalarga bog'laydi va C# boshqacha qiladigan narsalarni belgilaydi: `decimal`, `dynamic`, LINQ'ning kechiktirilgan so'rovlari, `Task`, `lock`, reflection.

Build qilingan ilovani yozib oling:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

Birliklarni P/Invoke orqali port qilish navbatda. Ungacha `bridge` C# birliklarini aniq xabar bilan rad etadi.

CI .NET'ni o'rnatmaydi, shuning uchun C# testlari faqat u o'rnatilgan mashinada ishlaydi. Kotlin va Swift hozircha qo'llab-quvvatlanmaydi; sizga kerak bo'lgan til uchun issue oching (yoki mavjudiga ovoz bering).
