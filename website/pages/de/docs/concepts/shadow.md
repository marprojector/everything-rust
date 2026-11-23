---
title: Shadow
description: shadow erzeugt ein Ersatzmodul, das das Original und das Rust nebeneinander ausführt und Abweichungen meldet.
---

# Shadow

```sh
everything-rust shadow <crate>
```

`shadow` erzeugt einen direkten Ersatz mit der API des Originals: ein TS- oder Python-Modul, ein Go-Package, eine Ruby-Datei, Java-Klassen oder eine C-Datei. Lade ihn dort, wo die App das Original geladen hat. Er führt beide aus, gibt das Ergebnis des Originals zurück und meldet Abweichungen des Rust, während du in der Produktion Vertrauen aufbaust.

| `EVERYTHING_RUST_MODE` | Verhalten |
| --- | --- |
| `shadow` (Standard) | führt beide aus, gibt das Ergebnis des Originals zurück, meldet Abweichungen |
| `new` | schaltet auf Rust um |
| `old` | schaltet Rust ab |

Die öffentliche API ändert sich nicht. Aufrufer importieren dieselben Funktionen oder an ihrer Stelle das Shadow-Modul.
