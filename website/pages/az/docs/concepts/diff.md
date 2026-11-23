---
title: Diff və məlum fərqlər
description: diff hər qeydə alınmış halı Rust-a qarşı təkrar oynadır. known-diffs.jsonl nəzərdə tutulmuş fərqləri qeyd edir, köhnəlmiş qeydlər isə gate-dən keçmir.
---

# Diff və məlum fərqlər

```sh
everything-rust build <crate>
everything-rust diff <crate>                     # N/N olmalıdır
everything-rust diff <crate> --json
```

`diff` `cases.jsonl`-i qurulmuş Rust moduluna qarşı, orijinalı çağıranların çağırdığı kimi təkrar oynadır və uyğunsuzluqları siyahıya alır. Uyğunsuzluq davranış fərqidir: qeydi yox, portu düzəldin.

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

## Məlum fərqlər

Bəzən port qəsdən fərqli olmalıdır: istifadəçi onun orijinaldakı bir xətanı düzəltməsini istəyir. Əvvəlcə soruşun, sonra hər hal üçün `<crate>/known-diffs.jsonl`-ə bir sətir əlavə edin:

**known-diffs.jsonl**
```json
{"fn": "...", "args": [...], "actual": ..., "reason": "..."}
```

`fn`, `args` və `actual`-ı `diff --json`-dan köçürün. Bundan sonra həmin hal yalnız məhz bu nəticə ilə keçir, artıq fərqlənmədikdə isə `gate` uğursuz olur, o zaman sətri silin.

## Köhnəlmiş qeydlər

`record` orijinal vahidin fayllarının barmaq izini `recorded-from.json`-da saxlayır. Orijinal sonradan dəyişəndə `diff` xəbərdarlıq edir, `gate` isə siz yenidən qeydə alana qədər uğursuz olur, orijinalın əvvəllər etdiyinə qarşı keçmək əvəzinə. `gate` qeydin köhnəldiyini deyirsə, port etməyə davam etməzdən əvvəl yenidən qeydə alın.

## Port ilişib qalanda

`diff` build-lər arasında irəliləyişi izləyir. Portun ilişib qaldığını deyəndə (onun `trend.stalled` sahəsi: `port.rs`-də ardıcıl üç dəyişiklik heç bir uyğun hal qazandırmayıb), redaktəni dayandırın. Qalan uyğunsuzluqlar yalnız istifadəçinin bildiyi bir qayda tələb edə bilər, ya da həmin funksiyalar hələlik orijinalda qala bilər.

## Float tolerantlığı

Crate `--float-tolerance` ilə qeydə alınıbsa, `diff` ondan istifadə edir; `diff`-dəki `--float-tolerance` bir işləmə üçün onu əvəz edir. Bax [Float tolerantlığı](/docs/concepts/float-tolerance).
