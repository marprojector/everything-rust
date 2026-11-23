---
title: Float rugsady
description: Sanly kod geçirilenden soň seýrek bitme-bit birmeňzeş float-lar berýär. --float-tolerance float-lara has uly bahanyň belli bir paýynyň çäginde gabat gelmäge rugsat berýär.
---

# Float rugsady

```sh
everything-rust record <crate> --float-tolerance 1e-9
everything-rust app record . --float-tolerance 1e-9 -- python3 cli.py
```

Sanly kod (fizika, statistika, grafika) geçirilenden soň seýrek bitme-bit birmeňzeş float-lar berýär: amallaryň tertibi, fused multiply-add we matematika kitaphanasy kompilýatorlaryň arasynda tapawutlanýar. Muny kabul edýän bolsaňyz, `--float-tolerance` we öz sanyňyz bilen ýazga alyň. Şonda float-lar has uly bahanyň şol paýynyň çäginde gabat gelýär (1-den kiçi bahalarda absolýut), `diff` we `gate` hem ony ulanýar. `0` ony aýyrýar.

Henizem takyk gabat gelmeli zatlar:

- bitin sanlar, tekst we gurluş;
- başgaça ýazylan şol bir baha (`3` we `3.0`);
- formatlamak. Sanlary tekste öwürmek diller arasynda tapawutlanýar (`1e21`, `-0`, sifrleriň sany); duzaklar şol bahalary öz içine alýar, `--float-tolerance` bolsa diňe tegeleklemäni bagyşlaýar, hiç haçan formatlamany däl.

Gabat gelmezligi ýok etmek üçin hiç haçan rugsady giňeltmäň: ondan uly tapawut portdaky ýalňyşlykdyr.
