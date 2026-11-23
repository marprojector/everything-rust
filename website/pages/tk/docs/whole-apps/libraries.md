---
title: Kitaphanalar
description: Kitaphanada gaýtadan işletmek üçin proses ýok. Onuň köpçülikleýin API-si front door, her birligiň ýazga alnan ýagdaýlary bolsa subutnama.
---

# Kitaphanalar

Kitaphanada gaýtadan işletmek üçin proses ýok: onuň köpçülikleýin API-si front door, her birligiň ýazga alnan
ýagdaýlary bolsa subutnama.

```sh
everything-rust app front-door .        # rust/app: köpçülikleýin API-li Rust crate-i
# her birlik aýlawdan geçýär: bridge, record, port.rs, build, diff, gate; soňra:
everything-rust app wire . textkit-rs   # rust/app-dan täzeden eksport edilýär; asyl nusga onuň shadow-yny import edýär
everything-rust app replay .            # baglanan her birligiň ýagdaýlary; fallback = entek asyl nusgadaky eksportlar
everything-rust app lift .
```

- Repozitoriýde `main` we serwer ýok bolanda `app front-door` köpçülikleýin API-ni (her birligiň eksportlaryny) tapýar
  (beýle bolmasa `--library`) we `rust/app`-y kitaphana crate-i hökmünde döredýär. Baglanan her
  birlik onuň moduly bolýar: `pub use textkit_rs::port as textkit;`.
- `app wire` `bridge` tarapyndan döredilen crate-i alýar, `gate` işledýär we birligiň shadow çalşygyny ýazýar:
  asyl paket ony birligiň ýerine import edýär, şonuň üçin onuň çagyryjylary asyl API-ni saklaýar we
  Rust-a ýetýär. `EVERYTHING_RUST_UNIT_<NAME>=original` birligi yzyna iberýär (C: shadow-yň
  baýdaklary bolmazdan guruň).
- `app replay` baglanan her birligiň ýazga alnan ýagdaýlaryny onuň binding-leri arkaly işledýär. Fallback paýy
  Rust-yň entek jogap bermeýän köpçülikleýin eksportlarynyň paýy: baglanmadyk birlikler we `bridge`-iň
  asyl dilde galdyrýan eksportlary.
- `app lift` 0 fallback we `gate`-den geçen her birligi talap edýär; ol birliklerden binding-leri, wrapper-leri we
  shadow-lary aýyrýar we `rust/app`-y ýeke arassa Rust kitaphanasy hökmünde galdyrýar. Ol hiç bir
  `port.rs` faýlyna degmeýär, şonuň üçin binding-ler arkaly gabat gelen ýagdaýlar ony henizem beýan edýär;
  barlag sanawy henizem shadow-lary import edýän asyl nusganyň faýllary barada duýduryş berýär.

Bäşisi hem MCP gurallary: `app_record`, `app_replay`, `app_front_door`, `app_wire` we
`app_lift`.
