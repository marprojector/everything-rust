---
title: C# (preview)
description: C# preview абалында. analyze .csproj долбоорлорун окуйт, ал эми C# CLI же ASP.NET серверин жазып алууга, кайра ойнотууга жана Rust front door'дун артына коюуга болот.
---

# C# (preview)

| | |
| --- | --- |
| Бирдик | `public static` методдорунан турган папка |
| Bridge | кийинки кезекте: C ABI аркылуу P/Invoke |
| Бүгүн | бүт колдонмолор: `app record`, `app replay` жана front door, CLI'лар жана ASP.NET серверлери үчүн (`dotnet <app>.dll`) |
| Абалы | Preview: `analyze` жана бүт колдонмолор гана; бирдиктерди азырынча bridge аркылуу өткөрүүгө болбойт |

`analyze` `.csproj` долбоорлорун жана алардын NuGet пакеттерин окуйт (crate'терге шайкештелет), `using` директиваларын папкаларга чечет жана C# башкача кылган нерселерди белгилейт: `decimal`, `dynamic`, LINQ'тун кийинкиге калтырылган суроо-талаптары, `Task`, `lock`, reflection.

Курулган колдонмону жазып алыңыз:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

Бирдиктерди P/Invoke аркылуу порттоо кийинки кезекте. Ага чейин `bridge` C# бирдиктерин түшүнүктүү билдирүү менен четке кагат.

CI .NET орнотпойт, ошондуктан C# тесттери ал орнотулган машинада гана иштейт. Kotlin жана Swift азырынча колдоого алынбайт; керектүү тилиңиз үчүн issue ачыңыз (же бар issue'ну колдоңуз).
