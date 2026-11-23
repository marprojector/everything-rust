---
title: Lift
description: app lift macht rust/app zu einer reinen Rust-App, sobald das Abspielen mit Fallback-Rate 0 besteht und jede Unit gate besteht.
---

# Lift

Wenn das Abspielen mit Fallback-Rate 0 besteht und jede Unit gate besteht, mach daraus eine reine Rust-App:

```sh
everything-rust app lift .
```

- `app lift` verweigert sich, bis beides gilt, und sagt, was es blockiert: die Fälle, die noch auf den Fallback zurückgreifen, Glue-Code, der noch ein Stub ist, Units, die gate nicht bestehen.
- Zuerst auf einer Kopie entfernt es den Fallback-Pfad und die Notschalter, macht aus jedem Unit-Crate eine einfache Bibliothek von `rust/app` (kein `cdylib`, keine Bindings oder generierten Wrapper) und fügt `rust/app/tests/app_cases.rs` hinzu, das `rust/app-cases` mit trycmd bei `cargo test` ausführt, sodass die CI nichts außer cargo braucht.
- Dann arbeitet es die Lift-Checkliste ab: fmt, clippy `-D warnings` und Tests auf jeder Unit und auf `rust/app`, jedes in `rust/DECISIONS.md` genannte Target gebaut oder geprüft, wo es installiert ist, zur Laufzeit gelesene Dateien und nicht eingebettete Asset-Ordner, und das Abspielen auf der reinen Binary. Erst wenn alles besteht, werden die Änderungen zurückgeschrieben. Das Original wird nie angefasst.

## Nach dem Lift

Die Units haben keine Bindings mehr. Bei json-server spielt die `tests/recorded.rs` jedes Crates dessen aufgezeichnete Fälle auf der Portierung ab: Alle 3,399 laufen mit `cargo test`, neben den 514 HTTP-Fällen.

Sobald das Original weg ist, kannst du das Rust frei umstrukturieren; `app replay` beweist weiterhin das Verhalten.

## Dann umstellen

Der Workflow-Skill lässt den Agent erledigen, was er kann, und `rust/cutover.sh` schreiben, ein interaktives Skript für die Schritte, die nur du machen kannst (Deployments, DNS, Secrets), mit einem Weg zurück bei jedem Schritt.
