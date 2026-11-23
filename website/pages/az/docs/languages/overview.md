---
title: Dillər
description: "TypeScript/JavaScript, Python, C, Go, Ruby və Java, PHP və C# önizləmədə. Hər biri üçün vahid, körpü və qeydə alan."
---

# Dillər

Altı dil bütöv dövrdən keçir: Node.js (TypeScript/JavaScript), Python, C, Go, Ruby və Java. PHP və C# önizləmədədir: bu gün `analyze` və bütöv tətbiqlər, vahid körpüləri isə növbədə.

| Dil | Vahid | Körpü | Qeydə alan və təkrar oynadan |
| --- | --- | --- | --- |
| TypeScript/JavaScript | qovluq və ya fayl | yaradılmış binary wire ilə napi-rs və ya wasm-bindgen; siniflər və async daxil | `.ts` mənbələrini işlədən Node |
| Python | paket və ya modul | PyO3 (abi3), birbaşa çevirmə, tipli exception-lar | orijinalı import edən Python |
| C | `.c` faylı | orijinal adlarla `extern "C"` funksiyaları və header | paylaşılan kitabxana kimi kompilyasiya olunmuş orijinal, ctypes ilə çağırılır, hər hal üçün bir fork |
| Go | paket | orijinal API-li paketdə tipli cgo çağırışları və yaradılmış binary wire | öz modulu kimi qurulan yaradılmış Go drayveri |
| Ruby | modul funksiyalarından ibarət fayl və ya qovluq | orijinal modulların altında magnus genişlənməsi | yaradılmış Ruby drayveri |
| Java | `public static` metodlardan ibarət paket | orijinal metodlara malik wrapper siniflərdə tipli JNI və yaradılmış binary wire | yaradılmış reflection drayveri |
| PHP (önizləmə) | funksiyalardan və `public static` metodlardan ibarət qovluq | növbədə: C ABI üzərindən PHP FFI | bu gün bütöv tətbiqlər: CLI-lar və serverlər (`php -S`) üçün `app record`, `app replay` və front door |
| C# (önizləmə) | `public static` metodlardan ibarət qovluq | növbədə: C ABI üzərindən P/Invoke | bu gün bütöv tətbiqlər: CLI-lar və ASP.NET serverləri (`dotnet <app>.dll`) üçün `app record`, `app replay` və front door |

C-dən başqa hər dil batch sinifləri də alır (`--batch`): elementlər körpünü bir dəfə keçir və
Rust-da qalır, hər çağırış isə bütün nüvələrdə hamısının üzərində işləyir.

Dil repozitoriyadan müəyyən edilir (`go.mod`, `pom.xml`, `.csproj` və ya `.sln`,
`pyproject.toml`, `Gemfile`, `composer.json`, `package.json`, əks halda ən çox rast gəlinən mənbə faylları);
`--lang` bunu əvəz edir. Tiplər mənbədən gəlir: TS tipləri, Python type hint-ləri, C, Go, Java və C#
imzaları, Ruby YARD teqləri, PHP tipləri və phpdoc; rəqəm kimi istifadə olunan tipsiz parametrlər
rəqəm kimi müəyyən edilir.

PHP önizləmədədir: `analyze` PHP tətbiqini planlaşdırır (Composer paketləri və PSR-4 autoloading,
sərbəst müqayisə və tam ədədin float-a daşması kimi risklər, crate-lərə uyğunlaşdırılmış Composer paketləri),
PHP CLI və ya serveri isə qeydə alına, təkrar oynadıla və Rust front door-unun arxasına qoyula bilər. Onun
vahidlərini PHP-nin FFI-ı vasitəsilə Rust-a port etmək növbəti addımdır ([issue #2](https://github.com/marprojector/everything-rust/issues/2)).

C# da eyni şəkildə önizləmədədir: `analyze` `.csproj` layihələrini və onların NuGet paketlərini
(crate-lərə uyğunlaşdırılmış) oxuyur, `using` direktivlərini qovluqlara həll edir və C#-ın fərqli etdiyi
şeyləri işarələyir (`decimal`, `dynamic`, LINQ-in təxirə salınmış sorğuları, `Task`, `lock`, reflection). C# CLI və ya ASP.NET
serveri qeydə alına, təkrar oynadıla və Rust front door-unun arxasına qoyula bilər; vahidləri P/Invoke vasitəsilə
port etmək növbədədir.
## Dillər üzrə vəziyyət

| Dil | Vəziyyət | Son real işləmə |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | Yoxlanılıb | json-server, Fuse.js, vercel/ms, x18n-in `packages/core` paketi |
| [Python](/docs/languages/python) | Yoxlanılıb | CPython 3.13-dən `colorsys.py`: 2,400/2,400 |
| [Go](/docs/languages/go) | Yoxlanılıb | Go 1.24-dən `path`: 2,642/2,642 |
| [Ruby](/docs/languages/ruby) | Yoxlanılıb | Ruby 3.3-dən `shellwords.rb`: 1,617/1,617 |
| [Java](/docs/languages/java) | Yoxlanılıb | Fasad arxasında JDK 21: 4,492/4,492 |
| [C](/docs/languages/c) | Yoxlanılıb | Fasad arxasında glibc: bir qeyri-müəyyən davranış halı çıxarıldıqdan sonra 1,859/1,859 |
| [PHP](/docs/languages/php) | Önizləmə | Yalnız `analyze` və bütöv tətbiqlər |
| [C#](/docs/languages/csharp) | Önizləmə | Yalnız `analyze` və bütöv tətbiqlər |

Əmrlərlə birlikdə tam cədvəl [Vəziyyət](/docs/project/status) səhifəsindədir.
