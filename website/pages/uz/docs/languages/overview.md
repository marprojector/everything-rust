---
title: Tillar
description: "TypeScript/JavaScript, Python, C, Go, Ruby va Java, shuningdek preview'dagi PHP va C#. Har biri uchun birlik, ko'prik va yozib oluvchi."
---

# Tillar

Oltita til butun sikldan o'tadi: Node.js (TypeScript/JavaScript), Python, C, Go, Ruby va Java. PHP va C# preview holatida: bugun `analyze` va butun ilovalar, birlik ko'priklari navbatda.

| Til | Birlik | Ko'prik | Yozib oluvchi va qayta ijro etuvchi |
| --- | --- | --- | --- |
| TypeScript/JavaScript | papka yoki fayl | generatsiya qilingan binary wire bilan napi-rs yoki wasm-bindgen; klasslar va async ham | `.ts` manbalarini ishga tushiradigan Node |
| Python | paket yoki modul | PyO3 (abi3), to'g'ridan-to'g'ri konvertatsiya, tiplangan exception'lar | asl nusxani import qiladigan Python |
| C | `.c` fayl | asl nomlar bilan `extern "C"` funksiyalar va header | umumiy kutubxona sifatida kompilyatsiya qilingan asl nusxa, ctypes orqali chaqiriladi, har bir holat uchun bitta fork |
| Go | paket | asl API'li paketda tiplangan cgo chaqiruvlari va generatsiya qilingan binary wire | o'z moduli sifatida build qilinadigan generatsiya qilingan Go drayveri |
| Ruby | modul funksiyalaridan iborat fayl yoki papka | asl modullar ostida magnus kengaytmasi | generatsiya qilingan Ruby drayveri |
| Java | `public static` metodlardan iborat paket | asl metodlarga ega wrapper klasslarda tiplangan JNI va generatsiya qilingan binary wire | generatsiya qilingan reflection drayveri |
| PHP (preview) | funksiyalar va `public static` metodlardan iborat papka | navbatda: C ABI orqali PHP FFI | bugun butun ilovalar: CLI'lar va serverlar (`php -S`) uchun `app record`, `app replay` va front door |
| C# (preview) | `public static` metodlardan iborat papka | navbatda: C ABI orqali P/Invoke | bugun butun ilovalar: CLI'lar va ASP.NET serverlari (`dotnet <app>.dll`) uchun `app record`, `app replay` va front door |

C'dan tashqari har bir til batch klasslarini ham oladi (`--batch`): elementlar ko'prikdan bir marta
o'tadi va Rust'da qoladi, har bir chaqiruv esa barcha yadrolarda ularning hammasi ustida ishlaydi.

Til repozitoriydan aniqlanadi (`go.mod`, `pom.xml`, `.csproj` yoki `.sln`, `pyproject.toml`,
`Gemfile`, `composer.json`, `package.json`, aks holda eng ko'p uchraydigan manba fayllari); `--lang`
buni almashtiradi. Tiplar manbadan olinadi: TS tiplari, Python type hint'lari, C, Go, Java va C#
signaturalari, Ruby YARD teglari, PHP tiplari va phpdoc; son sifatida ishlatilgan tipsiz parametrlar
son deb aniqlanadi.

PHP preview'da: `analyze` PHP ilovasini rejalashtiradi (Composer paketlari va PSR-4 autoloading,
erkin taqqoslash va butun sonning float'ga to'lib o'tishi kabi xavflar, crate'larga moslangan Composer
paketlari), PHP CLI yoki serverini esa yozib olish, qayta ijro etish va Rust front door'i ortiga qo'yish
mumkin. Uning birliklarini PHP FFI orqali Rust'ga port qilish keyingi qadam
([issue #2](https://github.com/marprojector/everything-rust/issues/2)).

C# ham xuddi shunday preview'da: `analyze` `.csproj` loyihalarini va ularning NuGet paketlarini
o'qiydi (crate'larga moslangan), `using` direktivalarini papkalarga bog'laydi va C# boshqacha
qiladigan narsalarni belgilaydi (`decimal`, `dynamic`, LINQ'ning kechiktirilgan so'rovlari, `Task`,
`lock`, reflection). C# CLI yoki ASP.NET serverini yozib olish, qayta ijro etish va Rust front door'i
ortiga qo'yish mumkin; birliklarni P/Invoke orqali port qilish navbatda.
## Har bir til holati

| Til | Holat | Oxirgi haqiqiy ishga tushirish |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | Tekshirilgan | json-server, Fuse.js, vercel/ms, x18n'ning `packages/core` |
| [Python](/docs/languages/python) | Tekshirilgan | CPython 3.13'dagi `colorsys.py`: 2,400/2,400 |
| [Go](/docs/languages/go) | Tekshirilgan | Go 1.24'dagi `path`: 2,642/2,642 |
| [Ruby](/docs/languages/ruby) | Tekshirilgan | Ruby 3.3'dagi `shellwords.rb`: 1,617/1,617 |
| [Java](/docs/languages/java) | Tekshirilgan | Fasad ortidagi JDK 21: 4,492/4,492 |
| [C](/docs/languages/c) | Tekshirilgan | Fasad ortidagi glibc: bitta aniqlanmagan xatti-harakat holati olib tashlangandan keyin 1,859/1,859 |
| [PHP](/docs/languages/php) | Preview | faqat `analyze` va butun ilovalar |
| [C#](/docs/languages/csharp) | Preview | faqat `analyze` va butun ilovalar |

Buyruqlar bilan to'liq jadval [Holat](/docs/project/status) sahifasida.
