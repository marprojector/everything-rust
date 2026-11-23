---
title: Float-Toleranz
description: Numerischer Code liefert nach der Portierung selten bitgleiche Floats. --float-tolerance lässt Floats innerhalb eines Anteils des größeren Werts übereinstimmen.
---

# Float-Toleranz

```sh
everything-rust record <crate> --float-tolerance 1e-9
everything-rust app record . --float-tolerance 1e-9 -- python3 cli.py
```

Numerischer Code (Physik, Statistik, Grafik) liefert nach der Portierung selten bitgleiche Floats: Die Reihenfolge der Operationen, Fused Multiply-Add und die Mathebibliothek unterscheiden sich zwischen Compilern. Wenn du das akzeptierst, zeichne mit `--float-tolerance` und deinem Wert auf. Floats stimmen dann innerhalb dieses Anteils des größeren Werts überein (absolut unterhalb von 1), und `diff` und `gate` nutzen ihn. `0` entfernt ihn.

Was weiterhin exakt übereinstimmen muss:

- Ganzzahlen, Text und Struktur;
- derselbe Wert in anderer Schreibweise (`3` und `3.0`);
- Formatierung. Die Umwandlung von Zahlen in Text unterscheidet sich zwischen Sprachen (`1e21`, `-0`, Anzahl der Ziffern); die Fallen enthalten diese Werte, und `--float-tolerance` verzeiht nur Rundung, nie Formatierung.

Erweitere nie eine Toleranz, um eine Abweichung verschwinden zu lassen: Ein Unterschied darüber hinaus ist ein Fehler in der Portierung.
