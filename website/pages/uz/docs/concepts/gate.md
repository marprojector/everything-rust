---
title: Gate
description: gate port uchun tayyorlik ta'rifi. U bajaradigan har bir tekshiruv va muvaffaqiyatsiz tekshiruvni qanday tuzatish.
---

# Gate

```sh
everything-rust gate <crate>
```

Tayyor degani `gate` o'tadi degani. U quyidagilarni tekshiradi:

- `cargo fmt`, `-D warnings` bilan clippy va cargo testlari;
- `unsafe` faqat `// SAFETY:` izohlari bilan va hech qayerda `unimplemented!` yo'q;
- asl nusxa ataylab xato beradigan har bir joy (`throw`, `raise`, `error` natijasi) uchun yozib olingan xatoli holat. gate uni nomlaganda `record --inputs` bilan qo'shing;
- asl nusxasi xato qaytaradigan funksiyada `unwrap`, `expect` yoki `panic!` yo'q. `Err` qaytaring; asl nusxa ham imkonsiz deb hisoblaydigan holat uchun `// INFALLIBLE: <why>` izohini yozing;
- port asl nusxa qatorlarining ko'pi bilan 3x qismi (importlar va signaturalar uchun yana 50 qator);
- modul build bo'ladi va har bir eksport mavjud;
- har bir yozib olingan holat, [ma'lum farqlar](/docs/concepts/diff#known-diffs) bilan birga, mos keladi va yozuv [eskirmagan](/docs/concepts/diff#stale-recordings);
- TS/JS uchun [qamrov](/docs/concepts/coverage) qatori: holatlar asl nusxaning qancha qismini ishga tushirgani.

Har bir muvaffaqiyatsiz tekshiruvda `fix` bor: avval uni ishga tushiring.

## So'ng hisobot bering va kuting

Gate o'tganda workflow skill'i agentga nima o'zgarganini (fayllar, funksiyalar, ko'prik, batch'lar), gate chiqishini, diff sonlari va bench raqamlarini, qolgan xatti-harakat farqlari yoki xavflarni hamda rejada nima qolganini xabar qildiradi va keyingi birlikdan oldin ruxsat kutadi.
