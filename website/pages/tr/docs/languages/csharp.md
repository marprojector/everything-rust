---
title: C# (önizleme)
description: C# önizlemededir. analyze .csproj projelerini okur ve bir C# CLI'ı veya ASP.NET sunucusu kaydedilebilir, yeniden oynatılabilir ve bir Rust front door'unun arkasına konabilir.
---

# C# (önizleme)

| | |
| --- | --- |
| Birim | `public static` metotlardan oluşan bir klasör |
| Köprü | sırada: C ABI üzerinden P/Invoke |
| Bugün | tüm uygulamalar: CLI'lar ve ASP.NET sunucuları (`dotnet <app>.dll`) için `app record`, `app replay` ve front door |
| Durum | Önizleme: yalnızca `analyze` ve tüm uygulamalar; birimler henüz köprülenemez |

`analyze`, `.csproj` projelerini ve NuGet paketlerini (crate'lere eşlenmiş olarak) okur, `using` yönergelerini klasörlere çözümler ve C#'ın farklı yaptığı şeyleri işaretler: `decimal`, `dynamic`, LINQ'in ertelenmiş sorguları, `Task`, `lock`, reflection.

Derlenmiş bir uygulamayı kaydedin:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

Birimleri P/Invoke üzerinden port etmek sırada. O zamana kadar `bridge`, C# birimlerini açık bir mesajla reddeder.

CI .NET kurmaz, bu yüzden C# testleri yalnızca .NET kurulu bir makinede çalışır. Kotlin ve Swift henüz desteklenmiyor; ihtiyacınız olan dil için bir issue açın (ya da var olana oy verin).
