---
title: Tutuş programmany geçirmek
description: Ilkinji ýazgydan arassa Rust binar faýlyna çenli. Maksat asyl nusga ýaly özüni alyp barýan Rust programmasy, asyl nusga bolsa salgylanma hökmünde saklanýar.
---

# Tutuş programmany geçirmek

Maksat asyl nusga ýaly özüni alyp barýan arassa Rust programmasy, asyl nusga bolsa salgylanma hökmünde saklanýar. Rust koduny agentiňiz ýazýar; everything-rust işi tertipleşdirýär, onuň karkasyny döredýär we her ädimi subut edýär. CLI-ler, HTTP serwerler we kitaphanalar.

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in    pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down proved by
 decisions to make      outside         original answers                                           replay
```

## 1. Meýilleşdirmek

```sh
everything-rust analyze . --whole-app
```

`--whole-app` programmanyň görnüşini (CLI, serwer ýa-da kitaphana) we giriş nokatlaryny, her daşky baglylygy (import edilen ýa-da yglan edilen) onuň Rust crate-i we laýyklygy bilen, geçiriş tertibini bölekler hökmünde (biri-birini import edýän birlikler bilelikde göçýär) we maslahat berilýän jogaply belgilenen kararlary goşýar: galdyrylýan birlikler, böwetlenen birlikler, crate-i ýok ýa-da bölekleýin bolan baglylyklar.

## 2. Tutuş programmany ýazga alyň, soňra giriş nokadyny göçüriň

1. [`app record` we `app replay`](/docs/whole-apps/record-replay) programmany daşyndan, gara guty hökmünde subut edýär.
2. [`app front-door`](/docs/whole-apps/front-door) asyl nusganyň öňünde Rust binar faýlyny goýýar; her buýruk henizem asyl nusgany işledýär.
3. Her birlik [aýlawdan](/docs/concepts/the-loop) geçýär, soňra `app wire` onuň buýruklaryny Rust-a tabşyrýar.
4. Replay 0 fallback paýy bilen geçende we her birlik gate-den geçende, [`app lift`](/docs/whole-apps/lift) asyl nusgany aýyrýar.

[Serwerler](/docs/whole-apps/servers) we [kitaphanalar](/docs/whole-apps/libraries) şol bir buýruklary ulanýar.

## Agent muny nähili ýerine ýetirýär

[Tutuş programma skill-i](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) agenti ädimme-ädim alyp barýar, diňe siziň kabul edip bilýän kararlaryňyzdan başlap:

- **Sorag-jogap (grilling)**: agent tapgyrlar bilen soraýar, her sorag maslahat berilýän jogaby bilen, portuň bagly her karary kesgitlenýänçä, we her birini `rust/DECISIONS.md` içinde ADR hökmünde ýazýar, `rust/GLOSSARY.md` bolsa asyl nusganyň adalgalaryny Rust atlaryna laýyklaşdyrýar.
- **Gözleg**: fon agentleri her baglylygyň esasy çeşmelerini okaýar we tapan her semantik duzagy bilen `rust/research/<topic>.md` ýazýar.
- **Sessiýalaryň arasynda**: `rust/MAP.md` her birligi onuň ýagdaýy bilen sanaýar; sessiýa başlamazdan öň bir birligi eýeleýär, her barlag nokady bolsa indiki sessiýa üçin `rust/HANDOFF.md` galdyrýar. Bir tolkundaky birlikleri subagentler parallel geçirip bilýär.
- **Geçiş (cutover)**: agent edip bilýän zadyny edýär we `rust/cutover.sh` ýazýar, diňe siziň edip bilýän ädimleriňiz (deploy-lar, DNS, gizlin maglumatlar) üçin interaktiw skript, her ädimde yzyna gaýtmak ýoly bilen.

Sorag-jogap, gözleg we bu tejribeler [Matt Pocock-yň skill-lerinden](https://github.com/mattpocock/skills) uýgunlaşdyryldy.

## Ol Rust-y köne dizaýna baglap goýmazmy?

Diňe ikisi ýanaşyk işleýän wagty. Ýazgylar özüni alyp barşy programmanyň içki gurluşynda däl, gyralarynda barlaýar (CLI-iň çykyşy, serweriň jogaplary, modulyň eksport edýän funksiýalary). Birlikler henizem bridge-den geçýän wagty köne gurluşy gaýtalaň; `app lift` asyl nusgany aýyrandan soň Rust-y erkin täzeden düzüň, `app replay` bolsa özüni alyp barşy henizem subut edýär.

## Hakyky mysal

[json-server](/docs/real-world/json-server) her ädimden geçdi: ýazga alnan 514 HTTP ýagdaýy, 99.8% fallback bilen front door, ilki baglanan okamalar (22.6% fallback), soňra ähli marşrutlar (0%) we 514/514 geçýän arassa Rust binar faýlyna lift.
