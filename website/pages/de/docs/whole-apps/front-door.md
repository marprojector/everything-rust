---
title: Front Door und Wire
description: app front-door stellt eine Rust-Binary vor das Original; app wire übergibt Befehle an die Portierung einer Unit, sobald sie gate besteht.
---

# Front Door und Wire

Verlege zuerst den Einstiegspunkt nach Rust und übergib ihm dann Befehle, eine Unit nach der anderen:

```sh
everything-rust app front-door .                            # rust/app: jeder Befehl führt das Original aus
everything-rust app wire . textkit --command upper          # nachdem textkit-rs gate besteht
# den Glue-Code in rust/app/src/units/textkit.rs schreiben, dann:
everything-rust app wire . textkit                          # wieder 100%, niedrigere Fallback-Rate
```

## Die Front Door

`app front-door` legt `rust/app` an, eine Rust-Binary, die nach der App benannt ist: ein clap-Parser für die Befehle und Flags in den aufgezeichneten Fällen, wobei jeder Befehl das Original mit denselben argv, stdin und derselben Umgebung ausführt und dessen Ausgabe und Exit-Code durchreicht. Jeder Fallback wird gezählt, sodass das Abspielen der frischen Front Door 100% bestanden bei 100% Fallback ergibt (sie baut und spielt ab, um das zu zeigen). `rust/app` wird nie überschrieben: Der Code gehört ab dann dir.

## Eine Unit verdrahten

`app wire <repo> <unit> --command <name>`:

1. führt `gate` auf dem Unit-Crate aus (`--skip-gate`, wenn es gerade bestanden hat);
2. fügt es `rust/app` als Bibliothek hinzu;
3. schreibt einmalig `src/units/<unit>.rs`, das du oder dein Agent ausfüllt: die Argumente parsen, `<crate>::port` aufrufen, ausgeben, was das Original ausgibt;
4. leitet die Befehle dorthin, baut dann und spielt ab.

Führe es nach dem Schreiben des Glue-Codes erneut aus: 100% bestanden und eine niedrigere Fallback-Rate. `EVERYTHING_RUST_UNIT_<NAME>=original` schickt eine verdrahtete Unit bis zum Lift zurück zum Original.

Verdrahte den eigenen Namen der App als Befehl (`--command mytool`) für argv, das keinen Befehl nennt: keine Argumente, unbekannte Befehle, Aufruffehler.

## Nichts ruft zurück ins Original

Die Front Door führt das Original als separaten Prozess aus (oder leitet HTTP dorthin weiter), gebridgte Units werden vom Original aufgerufen und nie umgekehrt, und Callbacks bleiben in der Quellsprache.

Server nutzen Routen statt Befehlen: siehe [Server](/docs/whole-apps/servers). Bibliotheken nutzen ihre öffentliche API: siehe [Bibliotheken](/docs/whole-apps/libraries).
