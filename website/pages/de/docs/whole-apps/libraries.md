---
title: Bibliotheken
description: Eine Bibliothek hat keinen Prozess zum Abspielen. Ihre öffentliche API ist die Front Door, und die aufgezeichneten Fälle jeder Unit sind der Beweis.
---

# Bibliotheken

Eine Bibliothek hat keinen Prozess zum Abspielen: Ihre öffentliche API ist die Front Door, und die aufgezeichneten
Fälle jeder Unit sind der Beweis.

```sh
everything-rust app front-door .        # rust/app: ein Rust-Crate mit der öffentlichen API
# jede Unit durch den Ablauf: bridge, record, port.rs, build, diff, gate; dann:
everything-rust app wire . textkit-rs   # aus rust/app re-exportiert; das Original importiert seinen Shadow
everything-rust app replay .            # die Fälle jeder verdrahteten Unit; Fallback = Exporte noch im Original
everything-rust app lift .
```

- `app front-door` findet die öffentliche API (die Exporte jeder Unit), wenn das Repository kein `main`
  und keinen Server hat (sonst `--library`), und legt `rust/app` als Bibliotheks-Crate an. Jede verdrahtete
  Unit wird zu einem Modul davon: `pub use textkit_rs::port as textkit;`.
- `app wire` nimmt ein von `bridge` erzeugtes Crate, führt `gate` aus und schreibt den Shadow-Ersatz der Unit:
  Das ursprüngliche Package importiert ihn anstelle der Unit, sodass seine Aufrufer die ursprüngliche API behalten und
  beim Rust landen. `EVERYTHING_RUST_UNIT_<NAME>=original` schickt die Unit zurück (C: ohne die
  Flags des Shadows bauen).
- `app replay` führt die aufgezeichneten Fälle jeder verdrahteten Unit über ihre Bindings aus. Die Fallback-Rate ist
  der Anteil öffentlicher Exporte, die Rust noch nicht beantwortet: nicht verdrahtete Units und Exporte, die `bridge`
  in der Originalsprache belässt.
- `app lift` braucht Fallback 0 und jede Unit durch `gate`; es entfernt Bindings, Wrapper und
  Shadows aus den Units und hinterlässt `rust/app` als eine reine Rust-Bibliothek. Es fasst kein
  `port.rs` an, sodass die Fälle, die über die Bindings übereinstimmten, es weiterhin beschreiben; die Checkliste
  warnt vor Dateien des Originals, die noch die Shadows importieren.

Alle fünf sind auch MCP-Tools: `app_record`, `app_replay`, `app_front_door`, `app_wire` und
`app_lift`.
