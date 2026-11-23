---
title: Gate
description: gate port üçin taýýarlygyň kesgitlemesi. Onuň işledýän her barlagy we şowsuz barlagy nähili düzetmeli.
---

# Gate

```sh
everything-rust gate <crate>
```

Taýýar diýmek `gate` geçýär diýmek. Ol şulary barlaýar:

- `cargo fmt`, `-D warnings` bilen clippy we cargo testleri;
- `unsafe` diňe `// SAFETY:` teswirleri bilen, hiç zat `unimplemented!` däl;
- asyl nusganyň bilkastlaýyn şowsuz bolýan her ýeri (`throw`, `raise`, `error` netijesi) üçin ýazga alnan şowsuz ýagdaý. Gate ony görkezende `record --inputs` bilen goşuň;
- asyl nusgasy ýalňyşlyk gaýtarýan funksiýada `unwrap`, `expect` ýa-da `panic!` ýok. `Err` gaýtaryň; asyl nusganyň hem mümkin däl hasaplaýan ýagdaýy üçin `// INFALLIBLE: <why>` teswiri;
- port asyl nusganyň setirlerinden iň köp 3x (import we signaturalar üçin goşmaça 50 setir);
- modul gurulýar we her eksport bar;
- ýazga alnan her ýagdaý gabat gelýär, [belli tapawutlar](/docs/concepts/diff#known-diffs) hem goşmak bilen, ýazgy bolsa [köne](/docs/concepts/diff#stale-recordings) däl;
- TS/JS üçin [örtük](/docs/concepts/coverage) setiri: ýagdaýlaryň asyl nusganyň näçesini işledendigi.

Her şowsuz barlag `fix` göterýär: ilki şony işlediň.

## Soňra hasabat beriň we garaşyň

Gate geçende workflow skill-i agente nämäniň üýtgändigini (faýllar, funksiýalar, bridge, batch-lar), gate çykyşyny, diff sanlaryny we bench sanlaryny, galan özüni alyp barş tapawutlaryny ýa-da töwekgelçilikleri we meýilnamada nämäniň galandygyny habar berdirýär we indiki birlikden öň rugsada garaşdyrýar.
