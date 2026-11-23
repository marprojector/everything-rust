---
title: Mitwirken
description: Einrichtung, wohin Tests gehören, wie man ein Crate-Mapping oder eine Sprache hinzufügt und wie man eine echte Portierung teilt.
---

# Mitwirken

Danke für deine Hilfe. Fehlerberichte, Korrekturen, Unterstützung für neue Sprachen und Berichte über echte Portierungen sind alle
willkommen, und für einen Pull Request mit einer Fehlerkorrektur musst du nicht vorher fragen.

## Bevor du anfängst

- **Ein Fehler:** Eröffne ein [Issue](https://github.com/marprojector/everything-rust/issues/new/choose) mit
  dem Befehl, den du ausgeführt hast, seiner Ausgabe, deiner Plattform und der everything-rust-Version
  (`everything-rust --version`). Ein winziges Projekt zum Reproduzieren hilft am meisten.
- **Eine größere Änderung** (eine neue Sprache, ein neuer Befehl, eine Änderung daran, was eine Unit oder eine Aufzeichnung ist):
  Eröffne zuerst ein Issue, damit wir uns auf die Form einigen können, bevor du sie schreibst. Die Gründe für frühere
  Entscheidungen stehen in [DECISIONS.md](https://github.com/marprojector/everything-rust/blob/main/DECISIONS.md).
- **Ein Sicherheitsproblem:** Eröffne kein öffentliches Issue; siehe [SECURITY.md](https://github.com/marprojector/everything-rust/blob/main/SECURITY.md).

## Einrichtung

Du brauchst Rust (stable, 1.97 oder neuer). Die Tests führen auch die Sprache aus, die jeder Teil behandelt, und
überspringen einen Teil, dessen Runtime fehlt, also installiere die, an denen du arbeitest:

| Für | Installieren |
| --- | --- |
| TypeScript/JavaScript | Node 24 (22.6+ funktioniert, außer für `profile`) |
| Python | Python 3.9+ |
| C | einen C-Compiler (`cc`) |
| Go | Go 1.22+ |
| Ruby | Ruby 3.x |
| Java | ein JDK (17+) |
| PHP | PHP 8 |
| C# | das .NET SDK |
| wasm-Bridges | `rustup target add wasm32-unknown-unknown` und die passende `wasm-bindgen-cli` |

```sh
git clone https://github.com/marprojector/everything-rust
cd everything-rust
git config core.hooksPath .githooks
cargo test
```

`cargo run -- doctor` listet auf, welche dieser Runtimes du hast, ihre Versionen und den Befehl,
der jede installiert oder aktualisiert. Damit eine fehlende Runtime ihre Tests fehlschlagen lässt, statt sie zu überspringen
(das tut die CI), nenne sie in `EVERYTHING_RUST_REQUIRE`, z. B.
`EVERYTHING_RUST_REQUIRE=node,python3,go,ruby,java cargo test`.

Der Hook führt `scripts/check-secrets.sh` aus, das Keys, Tokens und persönliche Home-Ordner-Pfade
in deinen Commits ablehnt. Die CI führt es auf jeder Datei aus.

## Die Änderung vornehmen

- **Jede Korrektur kommt mit einem Test, der ohne sie fehlschlägt.** Führe den Test einmal gegen den alten Code aus, um
  sicherzugehen, dass er das tut.
- **Wohin Tests gehören:**
  - `tests/analyze.rs`: Spracherkennung, Units, Risiken, Abhängigkeiten, `--whole-app`.
  - `tests/loop.rs`: bridge, record, diff und shadow pro Sprache, mit Platzhaltern statt Builds.
  - `tests/native.rs`: echt gebaute Bridges, die dann geladen und abgespielt werden.
  - `tests/app.rs` und `tests/http.rs`: record, replay, Front Door und Lift für ganze Apps, für CLIs und
    Server.
  - `tests/library.rs`: Portierungen ganzer Bibliotheken.
  - Unit-Tests liegen neben dem Code.
- **Fixtures** sind kleine Apps unter `tests/fixtures/`. Die CLI- und HTTP-Fixtures implementieren dieselben
  Befehle und Routen in jeder Sprache (`tcli upper|len|div|words|copy|now`, `/hello`, `/upper`,
  `/div/{a}/{b}`, `/now`), also halte ein neues im Verhalten identisch zu den anderen.
- **Generierter Code** (die Bridges, Treiber und Front Doors, die everything-rust schreibt) muss ebenfalls
  `clippy -D warnings` bestehen, da `gate` es auf den Crates der Nutzer ausführt.
- **Stil:** Passe dich dem umgebenden Code an. Behalte Kommentare für das, was der Code nicht selbst sagen kann, und
  schreib die Begründung für eine Änderung stattdessen in den Pull Request.

## Ein Crate-Mapping hinzufügen

`analyze` nennt das Rust-Crate für jede Bibliothek, die der Code importiert, aus `src/crates.toml`. Um
eines hinzuzufügen, hänge einen Eintrag unter seiner Sprache an: `names` (exakte Paketnamen) oder `prefixes` (der Anfang
eines Namens, für Java und C#), `crates`, `fit` (`good`, `partial` oder `none`) und eine `note`, wenn die Passung
einer Erklärung bedarf. Der erste passende Eintrag gewinnt, also stelle spezifische vor breite Präfixe. Eine
in der Tabelle fehlende Bibliothek wird als `unknown` gemeldet; rate nie ein Crate.

## Eine Sprache hinzufügen

PHP und C# wurden als Vorschau in zwei Commits hinzugefügt, die du zuerst lesen solltest:
[e8ac243](https://github.com/marprojector/everything-rust/commit/e8ac243) (PHP) und
[caea59a](https://github.com/marprojector/everything-rust/commit/caea59a) (C#). Eine neue Sprache beginnt
genauso:

1. Ein Plugin in `src/lang/<name>.rs`: Erkennung, Scan (Imports und Risiken), Auflösung und Exporte,
   plus die Crates ihrer Bibliotheken als `[[<name>]]`-Einträge in `src/crates.toml`.
2. Erkennung und Dispatch in `src/lang/mod.rs`.
3. Unterstützung für ganze Apps in `src/whole_app.rs`: Einstiegspunkte, Server, deklarierte Abhängigkeiten.
4. Die CLI- und HTTP-Fixtures, ein Bibliotheks-Fixture und Tests in `tests/analyze.rs`, `tests/app.rs`
   und `tests/http.rs`.
5. `bridge` lehnt die Units der Sprache mit einer klaren Meldung ab, bis eine Bridge existiert.

## Die README übersetzen

Übersetzungen sind willkommen und werden von der Community gepflegt. Eine neue:

1. Liegt in `docs/README_<CODE>.md` (zum Beispiel `docs/README_PT.md`); die `README.md` im Wurzelverzeichnis bleibt
   Englisch und wird nie umbenannt oder ersetzt.
2. Nutzt Pfade, die von `docs/` aus funktionieren: `assets/logo.svg`, `../LICENSE`, `../README.md`.
3. Beginnt mit derselben Sprachzeile wie die anderen Übersetzungen, mit ihrer eigenen Sprache als einfachem
   Text, gefolgt von dem Hinweis, dass sie von der Community gepflegt wird und hinter der englischen
   README zurückliegen kann (kopiere ihn aus `docs/README_DE.md` und übersetze ihn).
4. Kommt ans **Ende** der Sprachzeile, in `README.md` und in jeder Übersetzung. Die Reihenfolge der
   bereits aufgeführten Sprachen ändert sich nicht.

## Doku-Website

Die Dokumentationswebsite liegt in `website/` (Void, `@void/md`, React). Viele ihrer Seiten binden
beim Build Abschnitte aus `README.md`, `CONTRIBUTING.md` und `docs/*.md` ein, also bearbeite diese Dateien, und die
Website folgt. Starte sie mit `cd website && npm ci --ignore-scripts && npm run dev` und baue sie mit
`npm run build`. Die CLI-Referenz stammt aus einem Snapshot von `--help` in
`website/content/cli-help.json`; nach einer Änderung an der CLI aktualisierst du ihn mit `cargo build --release` und
`npm run cli-help` in `website/`.

## Den Pull Request eröffnen

Bevor du pushst:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

- Sag, was falsch war, wie man es reproduziert und wie die Änderung es behebt.
- Verlinke das Issue (`Fixes #123`).
- Vermerke alles, was du nicht testen konntest (ein anderes Betriebssystem, eine Runtime, die du nicht hast).
- Beschränke dich auf eine Änderung: Zwei Korrekturen lassen sich als zwei Pull Requests leichter prüfen.

Die CI läuft auf Ubuntu und macOS mit allen Runtimes installiert. Bei Erstbeitragenden wartet GitHub
darauf, dass ein Maintainer den CI-Lauf freigibt, rechne also mit einer kurzen Verzögerung, bevor die Prüfungen starten.

Pull Requests werden mit ihren Commits gemergt, sodass deine Historie deine bleibt. Braucht eine Änderung
kleine Anpassungen, fügt ein Maintainer eventuell einen Commit obendrauf hinzu, statt um eine weitere Runde zu bitten.

## Eine echte Portierung teilen

Portierungen echter Projekte sind der beste Beleg dafür, was funktioniert und was bricht. Wenn du
everything-rust an etwas Echtem eingesetzt hast, ist ein Issue oder ein Pull Request, der es zu
[docs/real-world.md](/docs/real-world/overview) hinzufügt, sehr willkommen. Nenne das Projekt, die Befehle, die du
ausgeführt hast, die Anzahl der Fälle und alles, was dich überrascht hat.

## Verhaltenskodex

Von allen Beteiligten wird erwartet, dass sie den [Verhaltenskodex](https://github.com/marprojector/everything-rust/blob/main/CODE_OF_CONDUCT.md) befolgen.
