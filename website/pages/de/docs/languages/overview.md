---
title: Sprachen
description: "TypeScript/JavaScript, Python, C, Go, Ruby und Java, dazu PHP und C# in der Vorschau. Die Unit, die Bridge und der Recorder für jede Sprache."
---

# Sprachen

Sechs Sprachen durchlaufen den ganzen Ablauf: Node.js (TypeScript/JavaScript), Python, C, Go, Ruby und Java. PHP und C# sind in der Vorschau: heute `analyze` und ganze Apps, Unit-Bridges als Nächstes.

| Sprache | Unit | Bridge | Aufgezeichnet und abgespielt durch |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Ordner oder Datei | napi-rs oder wasm-bindgen mit einem generierten binären Wire; Klassen und async inklusive | Node, das die `.ts`-Quellen ausführt |
| Python | Package oder Modul | PyO3 (abi3), direkte Konvertierung, typisierte Exceptions | Python, das das Original importiert |
| C | `.c`-Datei | `extern "C"`-Funktionen mit den ursprünglichen Namen und einem Header | das Original, kompiliert zu einer Shared Library, aufgerufen über ctypes, ein Fork pro Fall |
| Go | Package | typisierte cgo-Aufrufe und ein generierter binärer Wire, in einem Package mit der ursprünglichen API | einen generierten Go-Treiber, gebaut als eigenes Modul |
| Ruby | Datei oder Ordner mit Modulfunktionen | eine magnus-Erweiterung unter den ursprünglichen Modulen | einen generierten Ruby-Treiber |
| Java | Package mit `public static`-Methoden | typisiertes JNI und ein generierter binärer Wire, in Wrapper-Klassen mit den ursprünglichen Methoden | einen generierten Reflection-Treiber |
| PHP (Vorschau) | Ordner mit Funktionen und `public static`-Methoden | folgt als Nächstes: PHP FFI über eine C-ABI | heute ganze Apps: `app record`, `app replay` und die Front Door, für CLIs und Server (`php -S`) |
| C# (Vorschau) | Ordner mit `public static`-Methoden | folgt als Nächstes: P/Invoke über eine C-ABI | heute ganze Apps: `app record`, `app replay` und die Front Door, für CLIs und ASP.NET-Server (`dotnet <app>.dll`) |

Jede Sprache außer C bekommt auch Batch-Klassen (`--batch`): Die Elemente wechseln einmal hinüber und bleiben in
Rust, und jeder Aufruf läuft über alle, auf allen Kernen.

Die Sprache wird aus dem Repository erkannt (`go.mod`, `pom.xml`, eine `.csproj` oder `.sln`,
`pyproject.toml`, `Gemfile`, `composer.json`, `package.json`, sonst die häufigsten Quelldateien);
`--lang` überschreibt das. Typen kommen aus dem Quellcode: TS-Typen, Python-Hints, C-, Go-, Java- und C#-Signaturen,
Ruby-YARD-Tags, PHP-Typen und phpdoc; untypisierte Parameter, die als Zahlen verwendet werden, werden
als Zahlen erkannt.

PHP ist in der Vorschau: `analyze` plant eine PHP-App (Composer-Pakete und PSR-4-Autoloading, Risiken
wie loser Vergleich und Ganzzahlüberlauf in Floats, Composer-Pakete auf Crates abgebildet),
und eine PHP-CLI oder ein PHP-Server lässt sich aufzeichnen, abspielen und hinter eine Rust-Front-Door stellen. Das Portieren
der Units nach Rust über PHPs FFI ist der nächste Schritt ([Issue #2](https://github.com/marprojector/everything-rust/issues/2)).

C# ist auf dieselbe Weise in der Vorschau: `analyze` liest `.csproj`-Projekte und ihre NuGet-Pakete
(auf Crates abgebildet), löst `using`-Direktiven zu Ordnern auf und markiert, was C# anders macht
(`decimal`, `dynamic`, die verzögerten Abfragen von LINQ, `Task`, `lock`, Reflection). Eine C#-CLI oder ein ASP.NET-Server
lässt sich aufzeichnen, abspielen und hinter eine Rust-Front-Door stellen; das Portieren von Units über P/Invoke
folgt als Nächstes.
## Status pro Sprache

| Sprache | Status | Letzter echter Lauf |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | Verifiziert | json-server, Fuse.js, vercel/ms, `packages/core` von x18n |
| [Python](/docs/languages/python) | Verifiziert | `colorsys.py` aus CPython 3.13: 2,400/2,400 |
| [Go](/docs/languages/go) | Verifiziert | `path` aus Go 1.24: 2,642/2,642 |
| [Ruby](/docs/languages/ruby) | Verifiziert | `shellwords.rb` aus Ruby 3.3: 1,617/1,617 |
| [Java](/docs/languages/java) | Verifiziert | JDK 21 hinter einer Fassade: 4,492/4,492 |
| [C](/docs/languages/c) | Verifiziert | glibc hinter einer Fassade: 1,859/1,859 nach dem Entfernen eines Falls mit undefiniertem Verhalten |
| [PHP](/docs/languages/php) | Vorschau | nur `analyze` und ganze Apps |
| [C#](/docs/languages/csharp) | Vorschau | nur `analyze` und ganze Apps |

Die vollständige Tabelle mit den Befehlen steht auf der Seite [Status](/docs/project/status).
