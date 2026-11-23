<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="everything-rust крабы">
</p>

<h1 align="center">everything-rust</h1>

<p align="center"><sub><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · Татарча · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> · <a href="README_VI.md">Tiếng Việt</a> · <a href="README_ES.md">Español</a> · <a href="README_IT.md">Italiano</a></sub></p>
<p align="center"><sub>Бу тәрҗемәне җәмәгатьчелек алып бара, шуңа күрә ул <a href="../README.md">инглизчә README</a> файлыннан калышырга мөмкин.</sub></p>

<p align="center"><b>Кушымтагызны Rust телендә яңадан языгыз һәм аның элеккечә эшләвен исбатлагыз.</b><br>
Ул кодны үзе тәрҗемә итми: Rust'ны кодлау агентыгыз (яки сез) яза, ә everything-rust аны оригинал белән тикшерә.</p>

<p align="center">
  <img src="https://img.shields.io/github/languages/top/marprojector/everything-rust?logo=rust&label=Rust&color=CE422B" alt="Rust share of the code">
  <img src="https://img.shields.io/badge/macOS%20·%20Linux%20·%20Windows-native-2563EB" alt="Native binaries for macOS, Linux and Windows">
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-555" alt="MIT license"></a>
  <br>
  <a href="https://github.com/rust-unofficial/awesome-rust#development-tools"><img src="https://awesome.re/mentioned-badge.svg" alt="Mentioned in Awesome Rust"></a>
  <a href="https://github.com/AIshihzong/awesome-reverse-engineer-anything"><img src="https://img.shields.io/badge/Featured%20in-Awesome%20REA-f0b429" alt="Featured in Awesome REA"></a>
  <a href="https://github.com/marprojector/everything-rust"><img src="https://img.shields.io/github/stars/marprojector/everything-rust?style=social" alt="GitHub stars"></a>
</p>



<p align="center"><b><a href="#тиз-башлау">Тиз башлау</a></b> · <a href="https://github.com/marprojector/everything-rust/releases">Releases</a> · <a href="#тиз-башлау">Claude Code plugin</a> · <a href="https://registry.modelcontextprotocol.io/v0.1/servers?search=everything-rust">MCP Registry</a> · <a href="real-world.md">Чын портлар</a> · <a href="faq.md#faq">Еш бирелә торган сораулар</a><br>
<sub><a href="#телләр">Телләр</a> · <a href="how-it-works.md#how-it-works">Ничек эшли</a> · <a href="whole-apps.md#port-a-whole-app">Бөтен кушымталар</a> · <a href="../LICENSE">MIT</a></sub><br>
<p align="center">Made by <a href="https://github.com/marprojector">M. Ammar R. Arief</a> · Based on <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> by Erkam Yaman</p>

<p align="center">2.0 версиясендә яңа: <a href="real-world.md#prettier">Prettier, Rust'ка күчерелде</a>. 19x тизрәк · язып алынган 17,224 җибәрүнең 15,861се тулысынча Rust'та · барысы да байтка байт туры килә</p>

<p align="center">
  <img src="assets/prettier.gif" alt="Prettier һәм аның Rust порты шул ук 59 чын файлны форматлый, һәр файлга бер процесс: Prettier 3.63 s, Rust порты 0.20 s сарыф итә, нәтиҗә байтка-байт бертөрле." width="820">
  <br>
  <sub>ms, Fuse.js һәм json-server проектларыннан алынган 59 чын файлда Prettier һәм аның Rust порты (Apple M4). Нәтиҗә байтка-байт бертөрле. <a href="real-world.md#prettier">Ничек үлчәнде</a>.</sub>
</p>

<p align="center"><sub>PHP һәм C#: бүген язып алу, кабат уйнату һәм front door; модуль портлары киләсе чиратта.</sub></p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Язып алу</b><br>
      <code>record</code> оригиналны ясалган чик очракларда, семантик тозакларда һәм сезнең үз кертмәләрегездә җибәрә һәм ул нәрсә кайтарганын яки нинди хата ыргытканын саклый.
    </td>
    <td width="25%" valign="top">
      <b>Күчерү</b><br>
      <code>bridge</code> Rust crate'ын һәм wrapper ясый, шуңа күрә чакыручылар оригинал API'ны саклый. Rust кодын сез яки агентыгыз яза.
    </td>
    <td width="25%" valign="top">
      <b>Тикшерү</b><br>
      <code>diff</code> язып алынган һәр очракны Rust'та кабат уйната, һәм алар барысы да туры килмичә һәм <code>gate</code> узмыйча, бернәрсә дә чыгарылмый.
    </td>
    <td width="25%" valign="top">
      <b>Lift</b><br>
      Кабат уйнату fallback'сыз узса һәм һәр берәмлек gate'ны узса, <code>app lift</code> аны саф Rust кушымтасына әйләндерә. Оригиналга беркайчан да кагылмый.
    </td>
  </tr>
</table>

**Бөтен кушымтагызны Rust телендә өлешләп яңадан языгыз.** everything-rust сезнең Node.js (TypeScript/JavaScript),
Python, C, Go, Ruby яки Java кушымтагыз бүген нәрсә эшләгәнен язып ала, аннары Rust телендәге
яңа версиянең һәр өлешен, алга барганчы, шул язма белән чагыштырып тикшерә, кушымта саф Rust булганчы. CLI, HTTP
серверлар һәм китапханәләр; Rust кодын сезнең программалау агенты (яки сез үзегез) яза. Ул язма
нәрсәне колачлаганын гына тотып кала ала: [Ул нәрсәне тотып кала алмый](how-it-works.md#what-it-cant-catch) бүлеген карагыз. Бөтен кораллар һәм серверлар, яки әкрен өлеш кенә,
бер тапкырда бер кайнар юл (hot path):

| Чын проект, нәтиҗә оригиналдагы кебек | Оригинал | everything-rust белән |
| --- | ---: | ---: |
| [Prettier](real-world.md#prettier) 3.6.2, командалар юлыннан 2,360 JS/TS файл | 128.6 s | **6.9 s: 19x тизрәк** |
| Prettier, командалар юлыннан 6,305 Markdown файл | 352 s | **22.5 s: 15.6x тизрәк** |
| [json-server](real-world.md#json-server), бөтен бер Node сервер: буш торганда хәтер | 74 MB | **3.5 MB**, бер 5.3 MB бинар файл |
| json-server, җибәрүдән беренче җавапка кадәр | 79 белән 85 ms арасы | **8 белән 10 ms арасы** |
| [Fuse.js](real-world.md#fusejs) белән 50,000 юл буенча эзләү, бер сорауга | 91.2 ms | **2.8 ms: 32x тизрәк** |

Prettier: язып алынган 17,224 җибәрүдән 15,861 җибәрү 10 телдә (JavaScript, TypeScript, JSX, JSON,
GraphQL, YAML, Markdown, CSS, SCSS, Less) тулысынча Rust'та форматлана, һәм бөтен 17,224 җибәрү дә
Prettier белән байтка-байт туры килә, чөнки әле күчерелмәгән һәрнәрсә Prettier'га кире кайта (fallback).
json-server: язып алынган 514/514 HTTP очрагы саф Rust бинар файлында туры килә. Apple M4'та үлчәнде,
һәрберсе ике тапкыр җибәрелде, һәр нәтиҗә вакыт үлчәнгәнче чагыштырылды ([тулырак](real-world.md)).
Һәр кечкенә функция өчен Rust'ны аерым чакыру гадәттә JIT'тан *әкренрәк*; everything-rust күчерергә
лаеклы циклны таба һәм аны күчерә.

## Әкрен өлеш кенә

Бөтен кушымтаны күчерергә кирәкми. `go`'ны артык әкрен эшләгән командага юнәлтегез, ул вакытны
ашаган бер генә функцияне күчерә:

```sh
everything-rust go --agent claude -- python3 app.py
```

Ул җибәрүне профильли, күчерергә лаеклы иң тышкы функцияне сайлый (эчендәге кечкенә чакыруны түгел,
циклны; профиль шулай дисә, batch итеп), аны ясалган кертмәләрдә һәм сезнең җибәрү ясаган
чакыруларда язып ала һәм `src/port.rs` файлын агентыгызга бирә (`claude`, `codex` яки stdin'нан
промпт укучы теләсә нинди команда; `--agent` булмаса, сезнең өчен `TASK.md` яза һәм туктый).
Функцияне язып алынган һәр очрак туры килгәндә һәм `gate` узганда гына кабыза. Аннары:

```sh
everything-rust report rust/dedupe-rs     # pull request өчен REPORT.md һәм card.svg
everything-rust disable rust/dedupe-rs    # оригиналга кире кайту, байтка-байт
```

Югарыдагы [Fuse.js](real-world.md#fusejs) рәте дә шундый төзелгән: Fuse'ның бер юл өчен баллы бөтен
исемлек буенча batch итеп Rust'ка күчерелде, ә объект ачкычлары, киңәйтелгән эзләү һәм калганы
JavaScript'та калды. Нәтиҗәдә 50,000 юл буенча бер сорауга 91.2 ms'тан 2.8 ms'ка кадәр. `go`'ны
20 юллык Python dedupe булган [`tests/fixtures/go-fuzzy`](../tests/fixtures/go-fuzzy) өстендә
сынап карагыз. Портны кабызу һәм сүндерүнең барлык ысуллары:
[Switch a port on and off](how-it-works.md#switch-a-port-on-and-off).

<p align="center">
  <img src="assets/json-server.gif" alt="json-server Node сервер буларак һәм бер Rust бинар файлы буларак: буш торганда хәтер 74 MB һәм 3.5 MB, җибәрүдән беренче җавапка кадәр 79 белән 85 ms һәм 8 белән 10 ms арасы, язып алынган 514 HTTP очракның 514е туры килә." width="820">
  <br>
  <sub>json-server, бөтен бер Node сервер, саф Rust бинар файлына әйләндерелде (Apple M4). <a href="real-world.md#json-server">Ничек үлчәнде</a>.</sub>
</p>

> everything-rust бәйсез проект, ул Rust Project яки Rust Foundation белән бәйле түгел һәм алар тарафыннан хупланмаган.
> "Rust" Rust Foundation'ның сәүдә билгесе.

> [!WARNING]
> `record` һәм `app record` сезнең оригинал кодны җибәрә: һәр функцияне (яки бөтен кушымтаны) күп санлы
> ясалган кертмәләр белән чакыра, ә `app record` һәр очракны ике тапкыр җибәрә. `diff`, `gate` һәм `app replay`
> шул ук очракларны портта яки оригиналда тагын җибәрә. Файл язучы, мәгълүмат базасы белән эшләүче яки
> челтәргә мөрәҗәгать итүче код моны һәр очрак саен бер тапкыр эшли. Аларны проектның күчермәсендә,
> тест базасы белән һәм мохиттә production сертификатлары булмаганда җибәрегез.

## Тиз башлау

Моны Claude Code, Codex, Cursor, Gemini CLI яки OpenCode эченә куегыз һәм агентыгыз аны урнаштырсын:

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

Яки үзегез:

```sh
cargo install everything-rust   # crates.io'дан
everything-rust setup           # аны Claude Code, Codex, Cursor, Gemini CLI һәм OpenCode'та теркәп куя
```

Әле Rust toolchain юкмы? macOS, Linux һәм Windows өчен әзер бинар файллар һәр
[GitHub релизына](https://github.com/marprojector/everything-rust/releases) беркетелгән, ә JavaScript
проектлары өчен `npm install -g everything-rust` шул ук бинар файлны урнаштыра.

Claude Code'та аны плагин итеп тә урнаштырып була (MCP сервер һәм skill; өстәге CLI
сезнең `PATH` эчендә булырга тиеш):

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

`setup` тапкан һәр агентка MCP серверны өсти һәм workflow skill'ны агент skill'ларны укыган урынга
урнаштыра. Ул сезнең башка көйләнмәләрегезне саклый, аны кабат җибәрү куркынычсыз, ә `--remove` аны кире кайтара.
Аннары агентыгыздан сорагыз, мәсәлән:

- *"Бу CLI'ны everything-rust ярдәмендә башыннан ахырына кадәр Rust телендә яңадан яз һәм аның шулай ук эшләвен исбатла."*
- *"Бу эзләү әкрен кебек. Аны everything-rust белән профильлә һәм кайнар циклны Rust'ка күчер."*
- *"everything-rust белән `src/parser/` папкасын Rust'ка күчерүне планлаштыр; беренче берәмлекне күчер һәм исбатла."*
- *"`rust/text-rs` эчендәге Rust портын оригинал белән бенчмаркта чагыштыр һәм аны чыгарыргамы икәнен әйт."*

### Кайсы агентлар белән эшли

| Агент | `everything-rust setup` нәрсә өсти |
| --- | --- |
| Claude Code | `~/.claude.json` эчендә MCP сервер, `~/.claude/skills/` эчендә skill |
| Codex | `~/.codex/config.toml` эчендә MCP сервер, `~/.codex/skills/` эчендә skill |
| Cursor | `~/.cursor/mcp.json` эчендә MCP сервер |
| Gemini CLI | `~/.gemini/settings.json` эчендә MCP сервер |
| OpenCode | `~/.config/opencode/opencode.json` эчендә MCP сервер (`mcp.servers`), `~/.config/opencode/skills/` эчендә skill |
| Башка теләсә нинди MCP клиент | `everything-rust mcp` командасын stdio сервер итеп кулдан өстәгез ([Урнаштыру](install.md#install) бүлеген карагыз) |

Булган файлны үзгәртер алдыннан `setup` аның күчермәсен янәшәдә саклый (`<file>.everything-rust.bak`).

OpenCode'ны кулдан тоташтыру өчен серверны `~/.config/opencode/opencode.json` эченә куегыз
(`command` бөтен argv була) һәм [`skills/everything-rust/`](../skills/everything-rust) папкасын
`~/.config/opencode/skills/everything-rust/` эченә күчерегез:

```json
{
  "mcp": {
    "servers": {
      "everything-rust": { "type": "local", "command": ["everything-rust", "mcp"] }
    }
  }
}
```

Яки адымнарны үзегез башкарыгыз:

```sh
everything-rust profile . -- node bench.mjs              # нәрсә кайнар һәм аны кем чакыра
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs языгыз (яки агентка тапшырыгыз), аннары:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # язып алынган һәр очрак туры килергә тиеш
everything-rust bench rust/fuzzy-rs                      # тизрәкме? нәтиҗә әйтә
everything-rust gate rust/fuzzy-rs                       # бу узса, эш бетте
```

everything-rust кодны тәрҗемә итми: Rust кодын сезнең программалау агенты (Claude Code, Codex, Cursor, OpenCode, ...) яки сез
үзегез язасыз. everything-rust агент ышанычлы итеп чамалый алмаган нәрсәләрне бирә: вакыт кая китә, нәрсәне
беренче һәм нинди тәртиптә күчерергә, калган код Rust'ны API үзгәрмичә чакырсын өчен bridge,
порт туры килергә тиешле төгәл тәртип, аны чыгаруның куркынычсыз ысулы һәм порт кайчан
әзер икәнен әйтүче gate.

Бертөрле нәтиҗә генә җитми. Кушымта тышка нәрсә җибәргәнен дә язып алыгыз, шунда хатны ике тапкыр
җибәрүче яңа версия узмый:

<p align="center">
  <img src="assets/effects.gif" alt="--effects smtp белән app record һәр теркәлү өчен бер сәламләү хатын язып ала; шул ук нәтиҗәне чыгаручы, ләкин хатны ике тапкыр җибәрүче яңа версиядә app replay 0/2 очрак узганын һәм effects буенча уңышсыз булганын хәбәр итә, ә хисап артык хатны күрсәтә." width="820">
  <br>
  <sub><code>app record --effects</code> һәм <code>app replay</code> командаларының чын җибәрелеше. <a href="whole-apps.md#side-effects-email-webhooks-and-databases">Ян йогынтылар</a> бүлеген карагыз.</sub>
</p>

## Телләр

| Тел | Берәмлек | Bridge | Кем язып ала һәм кабат уйната |
| --- | --- | --- | --- |
| TypeScript/JavaScript | папка яки файл | ясалган бинар wire форматлы napi-rs яки wasm-bindgen; класслар һәм async та керә | Node, `.ts` чыганакларын җибәреп |
| Python | пакет яки модуль | PyO3 (abi3), туры үзгәртү, типлы исключениеләр | Python, оригиналны импортлап |
| C | `.c` файл | оригинал исемле `extern "C"` функцияләр һәм header | уртак китапханәгә компиляцияләнгән оригинал, ctypes аша чакырыла, һәр очракка бер fork |
| Go | пакет | типлы cgo чакырулары һәм ясалган бинар wire формат, оригинал API'лы пакетта | ясалган Go драйверы, аерым модуль итеп җыела |
| Ruby | модуль функцияләре булган файл яки папка | оригинал модульләр астында magnus киңәйтмәсе | ясалган Ruby драйверы |
| Java | `public static` методлар пакеты | типлы JNI һәм ясалган бинар wire формат, оригинал методлы wrapper класслар эчендә | ясалган reflection драйверы |
| PHP | функцияләр һәм `public static` методлар папкасы | киләсе адым: C ABI өстендә PHP FFI | бүген үк бөтен кушымталар: `app record`, `app replay` һәм front door, CLI һәм серверлар өчен (`php -S`) |
| C# | `public static` методлар папкасы | киләсе адым: C ABI өстендә P/Invoke | бүген үк бөтен кушымталар: `app record`, `app replay` һәм front door, CLI һәм ASP.NET серверлар өчен (`dotnet <app>.dll`) |

C'тан кала һәр тел batch класслар да ала (`--batch`): элементлар бер тапкыр күчә һәм
Rust'та кала, ә һәр чакыру аларның барысын да барлык ядроларда эшкәртә.

Тел репозиторийдан билгеләнә (`go.mod`, `pom.xml`, `.csproj` яки `.sln`,
`pyproject.toml`, `Gemfile`, `composer.json`, `package.json`, юкса иң күп очрый торган чыганак файллар);
`--lang` аны алыштыра. Типлар чыганак кодтан алына: TS типлары, Python type hints, C, Go, Java һәм C#
сигнатуралары, Ruby YARD теглары, PHP типлары һәм phpdoc; сан итеп кулланылган типсыз параметрлар
сан дип чыгарыла.

PHP өчен `analyze` PHP кушымтасын планлаштыра (Composer пакетлары һәм PSR-4 autoloading, катгый булмаган
чагыштыру һәм бөтен саннарның float'ка ташып чыгуы кебек куркынычлар, Composer пакетларына туры килгән crate'лар),
ә PHP CLI яки серверны язып алып, кабат уйнатып, Rust front door артына куеп була. Аның
берәмлекләрен PHP FFI аша Rust'ка күчерү киләсе адым ([issue #2](https://github.com/marprojector/everything-rust/issues/2)).

C# та шулай ук эшли: `analyze` `.csproj` проектларын һәм аларның NuGet пакетларын укый
(crate'ларга туры китереп), `using` директиваларын папкаларга бәйли һәм C# нәрсәне башкача эшләгәнен билгели
(`decimal`, `dynamic`, LINQ'ның кичектерелгән сораулары, `Task`, `lock`, reflection). C# CLI яки ASP.NET
серверын язып алып, кабат уйнатып, Rust front door артына куеп була; берәмлекләрне P/Invoke аша күчерү
киләсе адым.

## Күбрәк белү

- [Ничек эшли](how-it-works.md): цикл, `bridge` һәм `analyze` нәрсәне хәл итә, кайнар юллар, ул нәрсәне тотып кала алмый, чын порт нәрсә таләп итә һәм оригиналдан калышмау (`map`)
- [Бөтен кушымтаны күчерү](whole-apps.md): серверлар, CLI һәм китапханәләр, front door'дан `app lift`'ка кадәр
- [Мисаллар һәм тикшеренүләр](examples.md): x18n һәм алты телдәге чын код
- [Чын портлар](real-world.md): json-server, Fuse.js, vercel/ms, бенчмарклар белән
- [Урнаштыру, яңарту һәм эшләнмә](install.md): урнаштыруның һәр ысулы, Windows һәм чыганак коддан җыю
- [Еш бирелә торган сораулар](faq.md) · [Халәт](STATUS.md) · [MCP кораллары](tools.md)

## Спонсорлар

<p align="center">
  <a href="https://github.com/marprojector/everything-rust">everything-rust</a> бушлай һәм ачык чыганаклы. Әгәр ул эшегездә ярдәм итсә, зинһар, аңа спонсорлык итүне уйлап карагыз.<br>
  Сезнең ярдәм эшләнмәне дәвам итәргә мөмкинлек бирә.
</p>

<p align="center">
</p>

## Катнашу

Issue'лар һәм pull request'лар хуплана; [CONTRIBUTING.md](../CONTRIBUTING.md) көйләүне, тестларны
кая куярга һәм яңа телне ничек өстәргә икәнен аңлата. Проект карарлары һәм аларның сәбәпләре
[DECISIONS.md](../DECISIONS.md) эчендә. Чын портлар турында мисаллар аеруча хуплана. Зинһар, куркынычсызлык
проблемалары турында шәхси рәвештә хәбәр итегез ([SECURITY.md](../SECURITY.md)) һәм
[тәртип кодексы](../CODE_OF_CONDUCT.md) белән танышыгыз.

## Лицензия

[MIT](../LICENSE) © [M. Ammar R. Arief](https://github.com/marprojector). Репозиторийыгыздагы портлар һәм ясалган код сезнеке.
