---
title: Телләр
description: TypeScript/JavaScript, Python, C, Go, Ruby һәм Java, өстәвенә preview рәвешендә PHP һәм C#. Һәрберсе өчен берәмлек, bridge һәм язып алучы.
---

# Телләр

Алты тел бөтен цикл аша уза: Node.js (TypeScript/JavaScript), Python, C, Go, Ruby һәм Java. PHP һәм C# preview рәвешендә: бүген `analyze` һәм бөтен кушымталар, киләсе адымда берәмлек bridge'лары.

| Тел | Берәмлек | Bridge | Кем язып ала һәм кабат уйната |
| --- | --- | --- | --- |
| TypeScript/JavaScript | папка яки файл | ясалган бинар wire форматлы napi-rs яки wasm-bindgen; класслар һәм async та керә | Node, `.ts` чыганакларын җибәреп |
| Python | пакет яки модуль | PyO3 (abi3), туры үзгәртү, типлы исключениеләр | Python, оригиналны импортлап |
| C | `.c` файл | оригинал исемле `extern "C"` функцияләр һәм header | уртак китапханәгә компиляцияләнгән оригинал, ctypes аша чакырыла, һәр очракка бер fork |
| Go | пакет | типлы cgo чакырулары һәм ясалган бинар wire формат, оригинал API'лы пакетта | ясалган Go драйверы, аерым модуль итеп җыела |
| Ruby | модуль функцияләре булган файл яки папка | оригинал модульләр астында magnus киңәйтмәсе | ясалган Ruby драйверы |
| Java | `public static` методлар пакеты | типлы JNI һәм ясалган бинар wire формат, оригинал методлы wrapper класслар эчендә | ясалган reflection драйверы |
| PHP (preview) | функцияләр һәм `public static` методлар папкасы | киләсе адым: C ABI өстендә PHP FFI | бүген үк бөтен кушымталар: `app record`, `app replay` һәм front door, CLI һәм серверлар өчен (`php -S`) |
| C# (preview) | `public static` методлар папкасы | киләсе адым: C ABI өстендә P/Invoke | бүген үк бөтен кушымталар: `app record`, `app replay` һәм front door, CLI һәм ASP.NET серверлар өчен (`dotnet <app>.dll`) |

C'тан кала һәр тел batch класслар да ала (`--batch`): элементлар бер тапкыр күчә һәм
Rust'та кала, ә һәр чакыру аларның барысын да барлык ядроларда эшкәртә.

Тел репозиторийдан билгеләнә (`go.mod`, `pom.xml`, `.csproj` яки `.sln`,
`pyproject.toml`, `Gemfile`, `composer.json`, `package.json`, юкса иң күп очрый торган чыганак файллар);
`--lang` аны алыштыра. Типлар чыганак кодтан алына: TS типлары, Python type hints, C, Go, Java һәм C#
сигнатуралары, Ruby YARD теглары, PHP типлары һәм phpdoc; сан итеп кулланылган типсыз параметрлар
сан дип чыгарыла.

PHP preview рәвешендә: `analyze` PHP кушымтасын планлаштыра (Composer пакетлары һәм PSR-4 autoloading, катгый булмаган
чагыштыру һәм бөтен саннарның float'ка ташып чыгуы кебек куркынычлар, Composer пакетларына туры килгән crate'лар),
ә PHP CLI яки серверны язып алып, кабат уйнатып, Rust front door артына куеп була. Аның
берәмлекләрен PHP FFI аша Rust'ка күчерү киләсе адым ([issue #2](https://github.com/marprojector/everything-rust/issues/2)).

C# та шулай ук preview рәвешендә: `analyze` `.csproj` проектларын һәм аларның NuGet пакетларын укый
(crate'ларга туры китереп), `using` директиваларын папкаларга бәйли һәм C# нәрсәне башкача эшләгәнен билгели
(`decimal`, `dynamic`, LINQ'ның кичектерелгән сораулары, `Task`, `lock`, reflection). C# CLI яки ASP.NET
серверын язып алып, кабат уйнатып, Rust front door артына куеп була; берәмлекләрне P/Invoke аша күчерү
киләсе адым.
## Телләр буенча халәт

| Тел | Халәт | Соңгы чын җибәрү |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | Тикшерелгән | json-server, Fuse.js, vercel/ms, x18n'ның `packages/core` |
| [Python](/docs/languages/python) | Тикшерелгән | CPython 3.13'тән `colorsys.py`: 2,400/2,400 |
| [Go](/docs/languages/go) | Тикшерелгән | Go 1.24'тән `path`: 2,642/2,642 |
| [Ruby](/docs/languages/ruby) | Тикшерелгән | Ruby 3.3'тән `shellwords.rb`: 1,617/1,617 |
| [Java](/docs/languages/java) | Тикшерелгән | фасад артында JDK 21: 4,492/4,492 |
| [C](/docs/languages/c) | Тикшерелгән | фасад артында glibc: билгеләнмәгән тәртипле бер очракны алып ташлаганнан соң 1,859/1,859 |
| [PHP](/docs/languages/php) | Preview | `analyze` һәм бөтен кушымталар гына |
| [C#](/docs/languages/csharp) | Preview | `analyze` һәм бөтен кушымталар гына |

Командалар белән тулы таблица [Халәт](/docs/project/status) битендә.
