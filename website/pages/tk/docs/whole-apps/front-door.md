---
title: Front door we wire
description: app front-door asyl nusganyň öňünde Rust binar faýlyny goýýar; app wire bolsa birligiň porty gate-den geçenden soň buýruklary oňa tabşyrýar.
---

# Front door we wire

Ilki giriş nokadyny Rust-a göçüriň, soňra buýruklary oňa bir gezekde bir birlik tabşyryň:

```sh
everything-rust app front-door .                            # rust/app: her buýruk asyl nusgany işledýär
everything-rust app wire . textkit --command upper          # textkit-rs gate-den geçenden soň
# rust/app/src/units/textkit.rs içinde glue ýazyň, soňra:
everything-rust app wire . textkit                          # ýene 100%, has pes fallback paýy
```

## Front door

`app front-door` `rust/app` döredýär, programmanyň adyny göterýän Rust binar faýly: ýazga alnan ýagdaýlardaky buýruklar we baýdaklar üçin clap parser-i, bu ýerde her buýruk asyl nusgany şol bir argv, stdin we gurşaw bilen işledýär we onuň çykyşyny hem-de çykyş koduny üýtgetmän geçirýär. Her fallback sanalýar, şonuň üçin täze front door gaýtadan işledilende 100% fallback bilen 100% geçýär (ony görkezmek üçin ol gurýar we gaýtadan işledýär). Ol hiç haçan `rust/app` faýlyny üstünden ýazmaýar: şondan soň kod siziňki.

## Birligi baglamak

`app wire <repo> <unit> --command <name>`:

1. birligiň crate-inde `gate` işledýär (eger ýaňy geçen bolsa `--skip-gate`);
2. ony `rust/app`-a kitaphana hökmünde goşýar;
3. `src/units/<unit>.rs` faýlyny bir gezek ýazýar, siziň ýa-da agentiňiziň doldurmagy üçin: argumentleri parse etmek, `<crate>::port` çagyrmak, asyl nusganyň çap edýän zadyny çap etmek;
4. buýruklary oňa ugrukdyrýar, soňra gurýar we gaýtadan işledýär.

Glue ýazandan soň ony ýene işlediň: 100% geçýär we fallback paýy has pes. `EVERYTHING_RUST_UNIT_<NAME>=original` baglanan birligi lift-e çenli asyl nusga yzyna iberýär.

Hiç bir buýrugy görkezmeýän argv üçin programmanyň öz adyny buýruk hökmünde baglaň (`--command mytool`): argumentsiz, näbelli buýruklar, ulanyş ýalňyşlyklary.

## Hiç zat asyl nusga yzyna çagyrmaýar

Front door asyl nusgany aýratyn proses hökmünde işledýär (ýa-da HTTP-ni oňa ugradýar), bridge edilen birlikleri asyl nusga çagyrýar, hiç haçan tersine däl, callback-ler bolsa çeşme dilinde galýar.

Serwerler buýruklaryň ýerine marşrutlary ulanýar: [Serwerler](/docs/whole-apps/servers) sahypasyna serediň. Kitaphanalar öz köpçülikleýin API-sini ulanýar: [Kitaphanalar](/docs/whole-apps/libraries) sahypasyna serediň.
