---
title: FAQ
description: Schreibt es meinen Code automatisch um? Ist Rust immer schneller? Stimmen meine Floats überein? Antworten auf häufige Fragen.
---

# FAQ

Fragen, die Leute zu everything-rust stellen. Zurück zur [README](/).

## FAQ

**Schreibt es meinen Code automatisch um?** Nein. Es findet, was verlagert werden soll, erzeugt die Bridge,
zeichnet auf, was das Original tut, und prüft das Rust dagegen. Du oder dein Agent schreibt das Rust
in `port.rs`; `diff` und `gate` entscheiden, wann es richtig ist.

**Ändert sich die API meiner App?** Nein. Aufrufer importieren dieselben Funktionen (oder das Shadow-Modul, das
beide ausführt und das Ergebnis des Originals zurückgibt, während du Vertrauen aufbaust).

**Ist Rust immer schneller?** Nein, und everything-rust sagt das auch. Eine kleine Funktion, die einmal pro Element aufgerufen wird,
ist in Rust oft langsamer, weil jeder Aufruf eine Bridge überquert; V8, die JVM und Go führen solche Funktionen
in Nanosekunden aus. `profile` findet die Schleife drumherum, `--batch` verlegt die ganze Schleife, und `bench`
gibt ein Urteil ab, bevor du auslieferst. Code, der bereits von numba, Cython oder JAX kompiliert wird, läuft ebenfalls als Maschinencode:
`profile` und `analyze` markieren ihn, da eine Portierung kaum Geschwindigkeit bringt.

**Mein Code ist numerisch; stimmen die Floats exakt überein?** Oft nicht: Die Reihenfolge der Operationen, Fused
Multiply-Add und die Mathebibliothek unterscheiden sich zwischen Compilern. Zeichne mit `--float-tolerance 1e-9` auf
(`record` oder `app record`), dann stimmen Floats innerhalb dieses Anteils des größeren Werts überein (absolut unterhalb
von 1). Ganzzahlen, Text und Struktur müssen weiterhin exakt übereinstimmen.

**Kann es eine App portieren, die ich nur als APK, JAR oder .exe habe?** Nein. Es portiert Quellcode: Es
parst die Funktionen, zeichnet auf, was jede tut, und prüft das Rust gegen das laufende
Original. Eine kompilierte App bietet nichts davon, und `analyze` sagt das. Eine neue Rust-App
aus dem sichtbaren Verhalten einer Binary zu schreiben, ist ein anderes Projekt.

**Sollte ich bewährtes C portieren?** Oft nicht, und `analyze` sagt dir, wo. C, das Hardware-Register
anfasst, sich auf die Typgrößen oder das Speicherlayout seines Compilers verlässt oder Inline-Assembler nutzt
(`volatile`, Bitfelder, gepackte Structs, `__asm__`, Ganzzahlen, die in Adressen gecastet werden), wird als „in C belassen“
markiert: Eine Aufzeichnung auf deinem Rechner beweist nichts über das Zielsystem, und stabiler Treiber- oder Firmware-Code
gewinnt durch eine Neufassung wenig. Was sich gut verlagern lässt, ist die portable Logik drumherum (Parsing, Protokolle, Datenverarbeitung).

**Kann es meine Electron-App portieren?** Die Logik ja; die Fenster bleiben Webseiten. `analyze
--whole-app` erkennt Electron und plant den Main-Prozess Stück für Stück, während der Renderer bleibt,
wie er ist. Die UI nach Tauri zu verlagern (dieselbe Web-UI auf einem Rust-Backend) ist ein separater Schritt, sobald die Logik
Rust ist.

**Was ist mit C#, Kotlin oder Swift?** C# ist in der Vorschau (siehe [Sprachen](/docs/languages/overview)); Kotlin und
Swift werden noch nicht unterstützt. Jede Sprache braucht einen Parser, einen Recorder, eine Bridge und einen Replay-Treiber,
alle durchgehend getestet. Eröffne ein Issue (oder stimme für eines) für die Sprache, die du brauchst.

**Wird das Rust nicht viel größer als das Original?** Sollte es nicht. `gate` vergleicht die Zeilen der Portierung
mit denen des Originals und schlägt oberhalb von 3x fehl (plus 50 Zeilen für Imports und Signaturen): Das bedeutet meist, dass von Hand geschrieben wird, was ein Crate bereits
tut, oder dass Fälle einzeln behandelt werden. `analyze` listet die zu verwendenden Crates.

**Bindet das stückweise Portieren das Rust nicht an das Design der alten App?** Nur solange beide
nebeneinander laufen. Die Aufzeichnungen prüfen das Verhalten an den Rändern der App (die Ausgabe einer CLI, die Antworten eines Servers,
die exportierten Funktionen eines Moduls), nicht ihr Inneres. Spiegle die alte Struktur, solange
Units noch die Bridge überqueren; sobald `app lift` das Original entfernt hat, strukturiere das Rust
frei um, und `app replay` beweist weiterhin das Verhalten. Auch ruft nichts zurück ins Original:
Die Front Door führt es als separaten Prozess aus (oder leitet HTTP dorthin weiter), gebridgte Units werden
vom Original aufgerufen und nie umgekehrt, und Callbacks bleiben in der Quellsprache.

**Verlässt mein Code meinen Rechner?** Nein: everything-rust läuft lokal und schickt deinen Code
nirgendwohin. Cargo lädt Crates wie üblich herunter, und der Modellanbieter deines Agents sieht, was dein Agent
ihm schickt, nach dessen eigenen Richtlinien.

**Was muss installiert sein?** Rust, plus die Runtime der Sprache, die du portierst (siehe
[Installation](/docs/introduction/install)).

**Etwas ist kaputt.** Eröffne ein Issue mit dem Befehl, seiner Ausgabe und deiner Plattform.
