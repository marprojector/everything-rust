---
title: Diff und bekannte Abweichungen
description: diff spielt jeden aufgezeichneten Fall gegen das Rust ab. known-diffs.jsonl hält beabsichtigte Abweichungen fest, und veraltete Aufzeichnungen lassen das Gate fehlschlagen.
---

# Diff und bekannte Abweichungen

```sh
everything-rust build <crate>
everything-rust diff <crate>                     # muss N/N sein
everything-rust diff <crate> --json
```

`diff` spielt `cases.jsonl` gegen das gebaute Rust-Modul ab, aufgerufen so, wie die Aufrufer des Originals es aufrufen, und listet die Abweichungen. Eine Abweichung ist ein Verhaltensunterschied: Korrigiere die Portierung, nie die Aufzeichnung.

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

## Bekannte Abweichungen

Manchmal soll die Portierung abweichen: Der Nutzer möchte, dass sie einen Fehler des Originals behebt. Frag zuerst und füge dann pro Fall eine Zeile zu `<crate>/known-diffs.jsonl` hinzu:

**known-diffs.jsonl**
```json
{"fn": "...", "args": [...], "actual": ..., "reason": "..."}
```

Kopiere `fn`, `args` und `actual` aus `diff --json`. Dieser Fall besteht dann nur mit genau diesem Ergebnis, und `gate` schlägt fehl, sobald er nicht mehr abweicht; entferne die Zeile dann.

## Veraltete Aufzeichnungen

`record` speichert einen Fingerabdruck der Dateien der ursprünglichen Unit in `recorded-from.json`. Ändert sich das Original später, warnt `diff` und `gate` schlägt fehl, bis du erneut aufzeichnest, statt gegen das zu bestehen, was das Original früher tat. Sagt `gate`, dass die Aufzeichnung veraltet ist, zeichne erneut auf, bevor du weiter portierst.

## Wenn die Portierung stockt

`diff` verfolgt den Fortschritt über Builds hinweg. Sagt es, dass die Portierung stockt (sein `trend.stalled`: drei Änderungen an `port.rs` hintereinander brachten keinen zusätzlichen übereinstimmenden Fall), hör auf zu editieren. Die verbleibenden Abweichungen brauchen vielleicht eine Regel, die nur der Nutzer kennt, oder diese Funktionen bleiben vorerst im Original.

## Float-Toleranz

Wurde das Crate mit `--float-tolerance` aufgezeichnet, nutzt `diff` sie; `--float-tolerance` bei `diff` überschreibt sie für einen Lauf. Siehe [Float-Toleranz](/docs/concepts/float-tolerance).
