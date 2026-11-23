---
title: Diller
description: TypeScript/JavaScript, Python, C, Go, Ruby ve Java; PHP ve C# önizlemede. Her biri için birim, köprü ve kaydedici.
---

# Diller

Altı dil döngünün tamamından geçer: Node.js (TypeScript/JavaScript), Python, C, Go, Ruby ve Java. PHP ve C# önizlemededir: bugün `analyze` ve tüm uygulamalar, birim köprüleri sırada.

| Dil | Birim | Köprü | Kaydeden ve yeniden oynatan |
| --- | --- | --- | --- |
| TypeScript/JavaScript | klasör veya dosya | üretilmiş bir binary wire ile napi-rs veya wasm-bindgen; sınıflar ve async dahil | `.ts` kaynaklarını çalıştıran Node |
| Python | paket veya modül | PyO3 (abi3), doğrudan dönüşüm, tipli exception'lar | orijinali import eden Python |
| C | `.c` dosyası | orijinal adlarla `extern "C"` fonksiyonlar ve bir header | paylaşımlı kütüphane olarak derlenmiş orijinal, ctypes üzerinden çağrılır, vaka başına bir fork |
| Go | paket | orijinal API'ye sahip bir pakette tipli cgo çağrıları ve üretilmiş bir binary wire | kendi modülü olarak derlenen, üretilmiş bir Go sürücüsü |
| Ruby | modül fonksiyonlarından oluşan dosya veya klasör | orijinal modüllerin altında bir magnus eklentisi | üretilmiş bir Ruby sürücüsü |
| Java | `public static` metotlardan oluşan paket | orijinal metotlara sahip wrapper sınıflarda tipli JNI ve üretilmiş bir binary wire | üretilmiş bir reflection sürücüsü |
| PHP (önizleme) | fonksiyonlar ve `public static` metotlardan oluşan klasör | sırada: C ABI üzerinden PHP FFI | bugün tüm uygulamalar: CLI'lar ve sunucular (`php -S`) için `app record`, `app replay` ve front door |
| C# (önizleme) | `public static` metotlardan oluşan klasör | sırada: C ABI üzerinden P/Invoke | bugün tüm uygulamalar: CLI'lar ve ASP.NET sunucuları (`dotnet <app>.dll`) için `app record`, `app replay` ve front door |

C hariç her dil batch sınıfları da alır (`--batch`): öğeler köprüyü bir kez geçer ve Rust'ta kalır,
her çağrı tüm çekirdeklerde hepsinin üzerinden çalışır.

Dil, repodan tespit edilir (`go.mod`, `pom.xml`, bir `.csproj` veya `.sln`, `pyproject.toml`,
`Gemfile`, `composer.json`, `package.json`, aksi halde en yaygın kaynak dosyalar); `--lang` bunu
geçersiz kılar. Tipler kaynaktan gelir: TS tipleri, Python type hint'leri, C, Go, Java ve C#
imzaları, Ruby YARD etiketleri, PHP tipleri ve phpdoc; sayı olarak kullanılan tipsiz parametreler
sayı olarak çıkarılır.

PHP önizlemededir: `analyze` bir PHP uygulamasını planlar (Composer paketleri ve PSR-4 autoloading,
gevşek karşılaştırma ve tam sayı taşmasının float'a dönüşmesi gibi riskler, crate'lere eşlenmiş
Composer paketleri) ve bir PHP CLI'ı ya da sunucusu kaydedilebilir, yeniden oynatılabilir ve bir
Rust front door'unun arkasına konabilir. Birimlerini PHP'nin FFI'ı üzerinden Rust'a port etmek bir
sonraki adımdır ([issue #2](https://github.com/marprojector/everything-rust/issues/2)).

C# da aynı şekilde önizlemededir: `analyze` `.csproj` projelerini ve NuGet paketlerini okur
(crate'lere eşlenmiş olarak), `using` yönergelerini klasörlere çözümler ve C#'ın farklı yaptığı
şeyleri işaretler (`decimal`, `dynamic`, LINQ'in ertelenmiş sorguları, `Task`, `lock`, reflection).
Bir C# CLI'ı veya ASP.NET sunucusu kaydedilebilir, yeniden oynatılabilir ve bir Rust front door'unun
arkasına konabilir; birimleri P/Invoke üzerinden port etmek sırada.

## Dil başına durum

| Dil | Durum | Son gerçek çalıştırma |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | Doğrulandı | json-server, Fuse.js, vercel/ms, x18n'in `packages/core`'u |
| [Python](/docs/languages/python) | Doğrulandı | CPython 3.13'ten `colorsys.py`: 2,400/2,400 |
| [Go](/docs/languages/go) | Doğrulandı | Go 1.24'ten `path`: 2,642/2,642 |
| [Ruby](/docs/languages/ruby) | Doğrulandı | Ruby 3.3'ten `shellwords.rb`: 1,617/1,617 |
| [Java](/docs/languages/java) | Doğrulandı | Bir cephe arkasında JDK 21: 4,492/4,492 |
| [C](/docs/languages/c) | Doğrulandı | Bir cephe arkasında glibc: tanımsız davranışlı bir vaka çıkarıldıktan sonra 1,859/1,859 |
| [PHP](/docs/languages/php) | Önizleme | yalnızca `analyze` ve tüm uygulamalar |
| [C#](/docs/languages/csharp) | Önizleme | yalnızca `analyze` ve tüm uygulamalar |

Komutlarıyla birlikte tablonun tamamı [Durum](/docs/project/status) sayfasındadır.
