---
title: Gate
description: gate port üçün hazırlığın tərifidir. İşə saldığı hər yoxlama və uğursuz yoxlamanı necə düzəltmək.
---

# Gate

```sh
everything-rust gate <crate>
```

Hazırdır, yəni `gate` keçir. O, bunları yoxlayır:

- `cargo fmt`, `-D warnings` ilə clippy və cargo testləri;
- `unsafe` yalnız `// SAFETY:` şərhləri ilə, heç bir `unimplemented!` olmaması;
- orijinalın qəsdən uğursuz olduğu hər yer üçün (`throw`, `raise`, `error` nəticəsi) qeydə alınmış uğursuz hal. gate onu adlandıranda `record --inputs` ilə əlavə edin;
- orijinalı xəta qaytaran funksiyada `unwrap`, `expect` və ya `panic!` olmaması. `Err` qaytarın; orijinalın da mümkünsüz saydığı vəziyyət üçün `// INFALLIBLE: <why>` şərhi;
- port orijinalın sətirlərinin ən çoxu 3 qatı (üstəgəl import-lar və imzalar üçün 50 sətir);
- modul qurulur və hər export mövcuddur;
- hər qeydə alınmış hal uyğun gəlir, [məlum fərqlər](/docs/concepts/diff#known-diffs) daxil olmaqla, və qeyd [köhnəlməyib](/docs/concepts/diff#stale-recordings);
- TS/JS üçün [əhatə](/docs/concepts/coverage) sətri: halların orijinalın nə qədərini işlətdiyi.

Hər uğursuz yoxlama `fix` daşıyır: əvvəlcə onu işə salın.

## Sonra hesabat verin və gözləyin

Gate keçəndə iş axını skill-i agentə nəyin dəyişdiyini (fayllar, funksiyalar, körpü, batch-lər), gate çıxışını, diff saylarını və bench rəqəmlərini, qalan davranış fərqlərini və ya riskləri, planda nəyin qaldığını bildirməyi və növbəti vahiddən əvvəl razılıq gözləməyi tapşırır.
