---
title: "C# (önizləmə)"
description: "C# önizləmədədir. analyze .csproj layihələrini oxuyur, C# CLI və ya ASP.NET serveri isə qeydə alına, təkrar oynadıla və Rust front door-unun arxasına qoyula bilər."
---

# C# (önizləmə)

| | |
| --- | --- |
| Vahid | `public static` metodlardan ibarət qovluq |
| Körpü | növbədə: C ABI üzərindən P/Invoke |
| Bu gün | bütöv tətbiqlər: CLI-lar və ASP.NET serverləri (`dotnet <app>.dll`) üçün `app record`, `app replay` və front door |
| Vəziyyət | Önizləmə: yalnız `analyze` və bütöv tətbiqlər; vahidlər hələ körpü ilə bağlana bilmir |

`analyze` `.csproj` layihələrini və onların NuGet paketlərini (crate-lərə uyğunlaşdırılmış) oxuyur, `using` direktivlərini qovluqlara həll edir və C#-ın fərqli etdiyi şeyləri işarələyir: `decimal`, `dynamic`, LINQ-in təxirə salınmış sorğuları, `Task`, `lock`, reflection.

Qurulmuş tətbiqi qeydə alın:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

Vahidləri P/Invoke vasitəsilə port etmək növbədədir. O vaxta qədər `bridge` C# vahidlərini aydın mesajla rədd edir.

CI .NET quraşdırmır, buna görə C# testləri yalnız .NET olan maşında işləyir. Kotlin və Swift hələ dəstəklənmir; lazım olan dil üçün issue açın (və ya mövcud olana səs verin).
