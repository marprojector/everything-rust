---
title: Diff we belli tapawutlar
description: diff ýazga alnan her ýagdaýy Rust-a garşy gaýtadan işledýär. known-diffs.jsonl bilkastlaýyn tapawutlary ýazga alýar, köne ýazgylar bolsa gate-den geçmeýär.
---

# Diff we belli tapawutlar

```sh
everything-rust build <crate>
everything-rust diff <crate>                     # N/N bolmaly
everything-rust diff <crate> --json
```

`diff` `cases.jsonl` faýlyny gurlan Rust moduluna garşy, asyl nusganyň çagyryjylary nähili çagyrýan bolsa, şeýle gaýtadan işledýär we gabat gelmezlikleri sanaýar. Gabat gelmezlik özüni alyp barşdaky tapawutdyr: porty düzediň, hiç haçan ýazgyny däl.

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

## Belli tapawutlar

Käwagt port bilkastlaýyn tapawutlanmaly: ulanyjy onuň asyl nusganyň ýalňyşlygyny düzetmegini isleýär. Ilki soraň, soňra `<crate>/known-diffs.jsonl` faýlyna her ýagdaý üçin bir setir goşuň:

**known-diffs.jsonl**
```json
{"fn": "...", "args": [...], "actual": ..., "reason": "..."}
```

`fn`, `args` we `actual` bahalaryny `diff --json`-dan göçüriň. Şondan soň şol ýagdaý diňe edil şol netije bilen geçýär, tapawut ýok bolanda bolsa `gate` şowsuz bolýar, şonda setiri aýyryň.

## Köne ýazgylar

`record` asyl birligiň faýllarynyň barmak yzyny `recorded-from.json` içinde saklaýar. Asyl nusga soňra üýtgände, asyl nusganyň öň eden zadyna garşy geçmegiň ýerine, siz täzeden ýazga alýançaňyz `diff` duýduryş berýär we `gate` şowsuz bolýar. Eger `gate` ýazgynyň könedigini aýtsa, geçirmegi dowam etmezden öň täzeden ýazga alyň.

## Port duranda

`diff` gurluşlaryň arasynda ösüşi yzarlaýar. Ol portuň durandygyny aýdanda (onuň `trend.stalled`-y: `port.rs`-e yzygiderli üç üýtgeşme hiç bir gabat gelýän ýagdaý goşmady), üýtgetmegi bes ediň. Galan gabat gelmezlikler diňe ulanyjynyň bilýän düzgünini talap edip biler ýa-da şol funksiýalar häzirlikçe asyl nusgada galyp biler.

## Float rugsady

Eger crate `--float-tolerance` bilen ýazga alnan bolsa, `diff` ony ulanýar; `diff`-däki `--float-tolerance` ony bir işletme üçin üýtgedýär. [Float rugsady](/docs/concepts/float-tolerance) sahypasyna serediň.
