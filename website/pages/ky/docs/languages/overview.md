---
title: Тилдер
description: TypeScript/JavaScript, Python, C, Go, Ruby жана Java, ошондой эле preview абалындагы PHP жана C#. Ар бири үчүн бирдик, bridge жана жазып алуучу.
---

# Тилдер

Алты тил бүт циклден өтөт: Node.js (TypeScript/JavaScript), Python, C, Go, Ruby жана Java. PHP жана C# preview абалында: бүгүн `analyze` жана бүт колдонмолор, бирдик bridge'дери кийинки кезекте.

| Тил | Бирдик | Bridge | Ким жазып алат жана кайра ойнотот |
| --- | --- | --- | --- |
| TypeScript/JavaScript | папка же файл | генерацияланган бинардык wire менен napi-rs же wasm-bindgen; класстар жана async да бар | Node, `.ts` баштапкы файлдарын түз иштетет |
| Python | пакет же модуль | PyO3 (abi3), түз айландыруу, типтелген exception'дор | Python, түп нусканы импорттойт |
| C | `.c` файлы | түп нуска аталыштары жана header менен `extern "C"` функциялары | shared library катары компиляцияланган түп нуска, ctypes аркылуу чакырылат, ар бир кейске бир fork |
| Go | пакет | типтелген cgo чакыруулары жана генерацияланган бинардык wire, түп нуска API'си бар пакетте | генерацияланган Go драйвери, өзүнчө модуль катары курулат |
| Ruby | модуль функцияларынан турган файл же папка | түп нуска модулдардын астындагы magnus кеңейтүүсү | генерацияланган Ruby драйвери |
| Java | `public static` методдорунан турган пакет | типтелген JNI жана генерацияланган бинардык wire, түп нуска методдору бар wrapper класстарында | генерацияланган reflection драйвери |
| PHP (preview) | функциялар жана `public static` методдорунан турган папка | кийинки кезекте: C ABI аркылуу PHP FFI | бүгүн бүт колдонмолор: `app record`, `app replay` жана front door, CLI'лар жана серверлер үчүн (`php -S`) |
| C# (preview) | `public static` методдорунан турган папка | кийинки кезекте: C ABI аркылуу P/Invoke | бүгүн бүт колдонмолор: `app record`, `app replay` жана front door, CLI'лар жана ASP.NET серверлери үчүн (`dotnet <app>.dll`) |

C'дан башка ар бир тил batch класстарын да алат (`--batch`): элементтер чектен бир жолу өтүп, Rust
ичинде калат, ал эми ар бир чакыруу алардын баарын бардык ядролордо иштетет.

Тил репозиторийден аныкталат (`go.mod`, `pom.xml`, `.csproj` же `.sln`, `pyproject.toml`, `Gemfile`,
`composer.json`, `package.json`, болбосо эң көп кездешкен баштапкы файлдар боюнча); `--lang` аны кол
менен көрсөтөт. Типтер баштапкы коддон алынат: TS типтери, Python hint'тери, C, Go, Java жана C#
сигнатуралары, Ruby YARD тегдери, PHP типтери жана phpdoc; сан катары колдонулган типсиз параметрлер
сан деп эсептелет.

PHP preview абалында: `analyze` PHP колдонмосун пландайт (Composer пакеттери жана PSR-4 autoloading,
loose comparison жана integer'дин float'ка толуп кетиши сыяктуу тобокелдиктер, crate'терге шайкештелген
Composer пакеттери), ал эми PHP CLI же серверин жазып алууга, кайра ойнотууга жана Rust front door'дун
артына коюуга болот. Анын бирдиктерин PHP FFI аркылуу Rust'ка порттоо кийинки кадам
([issue #2](https://github.com/marprojector/everything-rust/issues/2)).

C# да ушундай эле preview абалында: `analyze` `.csproj` долбоорлорун жана алардын NuGet пакеттерин
окуйт (crate'терге шайкештелет), `using` директиваларын папкаларга чечет жана C# башкача кылган
нерселерди белгилейт (`decimal`, `dynamic`, LINQ'тун кийинкиге калтырылган суроо-талаптары, `Task`,
`lock`, reflection). C# CLI же ASP.NET серверин жазып алууга, кайра ойнотууга жана Rust front door'дун
артына коюуга болот; бирдиктерди P/Invoke аркылуу порттоо кийинки кезекте.
## Ар бир тилдин абалы

| Тил | Абалы | Акыркы чыныгы иштетүү |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | Текшерилген | json-server, Fuse.js, vercel/ms, x18n'дин `packages/core` |
| [Python](/docs/languages/python) | Текшерилген | CPython 3.13'төгү `colorsys.py`: 2,400/2,400 |
| [Go](/docs/languages/go) | Текшерилген | Go 1.24'төгү `path`: 2,642/2,642 |
| [Ruby](/docs/languages/ruby) | Текшерилген | Ruby 3.3'төгү `shellwords.rb`: 1,617/1,617 |
| [Java](/docs/languages/java) | Текшерилген | Facade артындагы JDK 21: 4,492/4,492 |
| [C](/docs/languages/c) | Текшерилген | Facade артындагы glibc: аныкталбаган жүрүм-турумдагы бир кейс алынып салынгандан кийин 1,859/1,859 |
| [PHP](/docs/languages/php) | Preview | `analyze` жана бүт колдонмолор гана |
| [C#](/docs/languages/csharp) | Preview | `analyze` жана бүт колдонмолор гана |

Буйруктары менен толук таблица [Абалы](/docs/project/status) барагында.
