---
title: Тілдер
description: "TypeScript/JavaScript, Python, C, Go, Ruby және Java, PHP мен C# preview күйінде. Әрқайсысының бірлігі, bridge-і және жазып алушысы."
---

# Тілдер

Алты тіл бүкіл циклдан өтеді: Node.js (TypeScript/JavaScript), Python, C, Go, Ruby және Java. PHP мен C# preview күйінде: бүгін `analyze` және бүкіл қолданбалар, бірлік bridge-тері келесі кезекте.

| Тіл | Бірлік | Bridge | Жазып алатын және қайта ойнататын |
| --- | --- | --- | --- |
| TypeScript/JavaScript | бума немесе файл | генерацияланған бинарлық wire бар napi-rs немесе wasm-bindgen; кластар мен async те бар | Node, `.ts` бастапқы файлдарын тікелей іске қосады |
| Python | пакет немесе модуль | PyO3 (abi3), тікелей түрлендіру, типтелген exception-дар | Python, түпнұсқаны импорттайды |
| C | `.c` файлы | түпнұсқа атаулары мен header бар `extern "C"` функциялары | shared library ретінде компиляцияланған түпнұсқа, ctypes арқылы шақырылады, әр кейске бір fork |
| Go | пакет | типтелген cgo шақырулары және генерацияланған бинарлық wire, түпнұсқа API бар пакетте | генерацияланған Go драйвері, жеке модуль ретінде құрастырылады |
| Ruby | модуль функцияларынан тұратын файл немесе бума | түпнұсқа модульдер астындағы magnus кеңейтімі | генерацияланған Ruby драйвері |
| Java | `public static` әдістерінен тұратын пакет | типтелген JNI және генерацияланған бинарлық wire, түпнұсқа әдістері бар wrapper кластарында | генерацияланған reflection драйвері |
| PHP (preview) | функциялар мен `public static` әдістерінен тұратын бума | келесі кезекте: C ABI арқылы PHP FFI | бүгін бүкіл қолданбалар: `app record`, `app replay` және front door, CLI-лар мен серверлер үшін (`php -S`) |
| C# (preview) | `public static` әдістерінен тұратын бума | келесі кезекте: C ABI арқылы P/Invoke | бүгін бүкіл қолданбалар: `app record`, `app replay` және front door, CLI-лар мен ASP.NET серверлері үшін (`dotnet <app>.dll`) |

C-дан басқа әр тіл batch кластарын да алады (`--batch`): элементтер шекарадан бір рет өтіп, Rust ішінде
қалады, ал әр шақыру олардың барлығын барлық ядроларда өңдейді.

Тіл репозиторийден анықталады (`go.mod`, `pom.xml`, `.csproj` немесе `.sln`, `pyproject.toml`,
`Gemfile`, `composer.json`, `package.json`, әйтпесе ең көп кездесетін бастапқы файлдар бойынша);
`--lang` оны қолмен көрсетеді. Типтер бастапқы кодтан алынады: TS типтері, Python hint-тері, C, Go, Java
және C# сигнатуралары, Ruby YARD тегтері, PHP типтері және phpdoc; сан ретінде қолданылатын типсіз
параметрлер сан деп есептеледі.

PHP preview күйінде: `analyze` PHP қолданбасын жоспарлайды (Composer пакеттері және PSR-4 autoloading,
loose comparison және integer-дің float-қа толып кетуі сияқты тәуекелдер, crate-терге сәйкестендірілген
Composer пакеттері), ал PHP CLI немесе серверін жазып алуға, қайта ойнатуға және Rust front door-ының
артына қоюға болады. Оның бірліктерін PHP FFI арқылы Rust-қа порттау келесі қадам
([issue #2](https://github.com/marprojector/everything-rust/issues/2)).

C# та дәл солай preview күйінде: `analyze` `.csproj` жобалары мен олардың NuGet пакеттерін оқиды
(crate-терге сәйкестендіріледі), `using` директиваларын бумаларға шешеді және C# басқаша істейтін
нәрселерді белгілейді (`decimal`, `dynamic`, LINQ-тің кейінге қалдырылған сұраныстары, `Task`, `lock`,
reflection). C# CLI немесе ASP.NET серверін жазып алуға, қайта ойнатуға және Rust front door-ының
артына қоюға болады; бірліктерді P/Invoke арқылы порттау келесі кезекте.
## Тілдер бойынша күйі

| Тіл | Күйі | Соңғы нақты іске қосу |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | Тексерілген | json-server, Fuse.js, vercel/ms, x18n-нің `packages/core` бумасы |
| [Python](/docs/languages/python) | Тексерілген | CPython 3.13-тегі `colorsys.py`: 2,400/2,400 |
| [Go](/docs/languages/go) | Тексерілген | Go 1.24-тегі `path`: 2,642/2,642 |
| [Ruby](/docs/languages/ruby) | Тексерілген | Ruby 3.3-тегі `shellwords.rb`: 1,617/1,617 |
| [Java](/docs/languages/java) | Тексерілген | Facade артындағы JDK 21: 4,492/4,492 |
| [C](/docs/languages/c) | Тексерілген | Facade артындағы glibc: undefined behavior бар бір кейсті алып тастағаннан кейін 1,859/1,859 |
| [PHP](/docs/languages/php) | Preview | Тек `analyze` және бүкіл қолданбалар |
| [C#](/docs/languages/csharp) | Preview | Тек `analyze` және бүкіл қолданбалар |

Командалары бар толық кесте [Күйі](/docs/project/status) бетінде.
