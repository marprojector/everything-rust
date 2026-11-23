---
title: Diller
description: TypeScript/JavaScript, Python, C, Go, Ruby we Java, PHP we C# preview ýagdaýynda. Her biri üçin birlik, bridge we ýazga alyjy.
---

# Diller

Alty dil tutuş aýlawdan geçýär: Node.js (TypeScript/JavaScript), Python, C, Go, Ruby we Java. PHP we C# preview ýagdaýynda: häzir `analyze` we tutuş programmalar, indiki nobatda birlik bridge-leri.

| Dil | Birlik | Bridge | Kim ýazga alýar we gaýtadan işledýär |
| --- | --- | --- | --- |
| TypeScript/JavaScript | bukja ýa-da faýl | döredilen binar wire bilen napi-rs ýa-da wasm-bindgen; klaslar we async hem bar | Node, `.ts` çeşme faýllaryny göni işledýär |
| Python | paket ýa-da modul | PyO3 (abi3), göni öwrülişik, tipli exception-lar | Python, asyl nusgany import edýär |
| C | `.c` faýly | asyl atlary we header bilen `extern "C"` funksiýalary | shared library hökmünde kompilirlenen asyl nusga, ctypes arkaly çagyrylýar, her ýagdaý üçin bir fork |
| Go | paket | tipli cgo çagyryşlary we döredilen binar wire, asyl API-li paketde | döredilen Go draýweri, aýratyn modul hökmünde gurulýar |
| Ruby | modul funksiýalaryndan ybarat faýl ýa-da bukja | asyl modullaryň aşagyndaky magnus giňeltmesi | döredilen Ruby draýweri |
| Java | `public static` usullaryndan ybarat paket | tipli JNI we döredilen binar wire, asyl usullary bolan wrapper klaslarynda | döredilen reflection draýweri |
| PHP (preview) | funksiýalardan we `public static` usullaryndan ybarat bukja | indiki nobatda: C ABI arkaly PHP FFI | häzir tutuş programmalar: `app record`, `app replay` we front door, CLI-ler we serwerler üçin (`php -S`) |
| C# (preview) | `public static` usullaryndan ybarat bukja | indiki nobatda: C ABI arkaly P/Invoke | häzir tutuş programmalar: `app record`, `app replay` we front door, CLI-ler we ASP.NET serwerleri üçin (`dotnet <app>.dll`) |

C-den başga her dil batch klaslaryny hem alýar (`--batch`): elementler serhetden bir gezek geçýär we
Rust-da galýar, her çagyryş bolsa olaryň hemmesini ähli ýadrolarda işleýär.

Dil repozitoriýden kesgitlenýär (`go.mod`, `pom.xml`, `.csproj` ýa-da `.sln`, `pyproject.toml`,
`Gemfile`, `composer.json`, `package.json`, bolmasa iň köp duş gelýän çeşme faýllary boýunça); `--lang`
ony el bilen görkezýär. Tipler çeşme kodundan alynýar: TS tipleri, Python hint-leri, C, Go, Java we C#
signaturalary, Ruby YARD tegleri, PHP tipleri we phpdoc; san hökmünde ulanylýan tipsiz parametrler san
diýlip kabul edilýär.

PHP preview ýagdaýynda: `analyze` PHP programmasyny meýilleşdirýär (Composer paketleri we PSR-4
autoloading, loose comparison we integer-iň float-a doly geçmegi ýaly töwekgelçilikler, crate-lere
laýyklaşdyrylan Composer paketleri), PHP CLI-ni ýa-da serwerini bolsa ýazga alyp, gaýtadan işledip we
Rust front door-yň arkasyna goýup bolýar. Onuň birliklerini PHP FFI arkaly Rust-a geçirmek indiki ädim
([issue #2](https://github.com/marprojector/everything-rust/issues/2)).

C# hem edil şeýle preview ýagdaýynda: `analyze` `.csproj` taslamalaryny we olaryň NuGet paketlerini
okaýar (crate-lere laýyklaşdyrylýar), `using` direktiwalaryny bukjalara çözýär we C#-yň başgaça
edýän zatlaryny belleýär (`decimal`, `dynamic`, LINQ-iň yza süýşürilen soraglary, `Task`, `lock`,
reflection). C# CLI-ni ýa-da ASP.NET serwerini ýazga alyp, gaýtadan işledip we Rust front door-yň
arkasyna goýup bolýar; birlikleri P/Invoke arkaly geçirmek indiki nobatda.
## Her dil boýunça ýagdaý

| Dil | Ýagdaý | Soňky hakyky işletme |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | Barlanan | json-server, Fuse.js, vercel/ms, x18n-iň `packages/core` |
| [Python](/docs/languages/python) | Barlanan | CPython 3.13-iň `colorsys.py`: 2,400/2,400 |
| [Go](/docs/languages/go) | Barlanan | Go 1.24-iň `path`: 2,642/2,642 |
| [Ruby](/docs/languages/ruby) | Barlanan | Ruby 3.3-iň `shellwords.rb`: 1,617/1,617 |
| [Java](/docs/languages/java) | Barlanan | fasadyň arkasyndaky JDK 21: 4,492/4,492 |
| [C](/docs/languages/c) | Barlanan | fasadyň arkasyndaky glibc: kesgitlenmedik özüni alyp barşly bir ýagdaý aýrylandan soň 1,859/1,859 |
| [PHP](/docs/languages/php) | Preview | diňe `analyze` we tutuş programmalar |
| [C#](/docs/languages/csharp) | Preview | diňe `analyze` we tutuş programmalar |

Buýruklar bilen doly tablisa [Ýagdaý](/docs/project/status) sahypasynda.
