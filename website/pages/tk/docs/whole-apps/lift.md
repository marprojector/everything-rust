---
title: Lift
description: Replay 0 fallback paýy bilen geçende we her birlik gate-den geçende, app lift rust/app-y arassa Rust programmasyna öwürýär.
---

# Lift

Replay 0 fallback paýy bilen geçende we her birlik gate-den geçende, ony arassa Rust programmasyna öwüriň:

```sh
everything-rust app lift .
```

- `app lift` iki şert hem ýerine ýetýänçä ret edýär we nämäniň böwet bolýandygyny aýdýar: henizem fallback-e düşýän ýagdaýlar, henizem stub bolan glue, gate-den geçmeýän birlikler.
- Ilki nusgada ol fallback ýoluny we öçüriji açarlary aýyrýar, her birlik crate-ini `rust/app`-yň ýönekeý kitaphanasyna öwürýär (`cdylib`, binding-ler ýa-da döredilen wrapper-ler bolmazdan) we `rust/app/tests/app_cases.rs` goşýar, ol `cargo test` wagtynda `rust/app-cases`-i trycmd bilen işledýär, şonuň üçin CI-e cargo-dan başga hiç zat gerek däl.
- Soňra ol lift barlag sanawyny işledýär: her birlikde we `rust/app`-da fmt, clippy `-D warnings` we testler, `rust/DECISIONS.md` içinde görkezilen her target gurnalan ýerinde gurulýar ýa-da barlanýar, işleýiş wagtynda okalýan faýllar we içine goşulmadyk asset bukjalary, we arassa binar faýlda replay. Diňe hemmesi geçende üýtgeşmeler yzyna ýazylýar. Asyl nusga hiç haçan üýtgedilmeýär.

## Lift-den soň

Birliklerde indi binding ýok. json-server-de her crate-iň `tests/recorded.rs` faýly onuň ýazga alnan ýagdaýlaryny portda gaýtadan işledýär: 3,399-synyň hemmesi `cargo test` bilen işleýär, 514 HTTP ýagdaýynyň ýanynda.

Asyl nusga aýrylandan soň Rust-y erkin täzeden düzüň; `app replay` özüni alyp barşy henizem subut edýär.

## Soňra geçiň

Workflow skill-i agente edip bilýän zadyny etdirýär we `rust/cutover.sh` ýazdyrýar, diňe siziň edip bilýän ädimleriňiz (deploy-lar, DNS, gizlin maglumatlar) üçin interaktiw skript, her ädimde yzyna gaýtmak ýoly bilen.
