---
title: Fuse.js
description: Rust-da Fuse.js string siyahısı axtarışı. 3,414/3,414 qeydə alınmış hal uyğun gəlir, bayt-bayt eyni nəticələr, 50,000 string üzərində 32x daha sürətli.
---

# Fuse.js

[Fuse.js](https://github.com/krisk/Fuse) (20k ulduz, həftədə təxminən 20 milyon npm endirmə,
commit `edf2fb6`) saysız-hesabsız axtarış qutusunun arxasındakı qeyri-dəqiq axtarış kitabxanasıdır.
everything-rust 1.6 (buraxılmamış) ilə port edilib; crate [`demo/real-world/fuse-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/fuse-rs)-dədir.

`analyze` `src/`-in hər qovluğunu bir import dövrünə yerləşdirdi, buna görə Fuse fayl-fayl port edilə bilməz.
Port bunun əvəzinə axtarışın vaxt sərf etdiyi yolu götürür: 54 sətirlik adapter
(`adapter/relevance.ts`) Fuse-un `BitapSearch`, field norm və `computeScoreSingle` hissələrindən qurulmuş,
string siyahısının bir string-i üçün Fuse-un öz balını export edir. 50,000 string üzərində axtarış
məhz bu funksiyanı 50,000 dəfə işlədir. `bridge --batch relevance:text` string-ləri Rust-da saxlayır və
axtarışı hamısının üzərində, bütün nüvələrdə işlədir.

| Addım | Nəticə |
| --- | --- |
| adapter | heç bir Rust olmadan əvvəl adapterin sıralaması 81 sorğu və seçim dəstində `new Fuse(list).search(q)`-ə bərabərdir |
| `record --inputs` | 3,414 hal: Fuse-un öz fixture-larından başlıqlar, ASCII olmayan, emoji, 32 vahiddən uzun pattern-lər, dəstəklənən hər seçim və yanlış tipli seçimlər (rədd edilir, 149 hal) |
| port | əvəz etdiyi 839 sətir Fuse üçün 673 sətir Rust |
| `diff` | ilk build-də 3,414/3,414 uyğun gəlir |
| `gate` | keçir |
| drop-in | `withRust(Fuse)` string siyahısı axtarışlarını Rust-da cavablandırır, qalan hər şeyi Fuse-a ötürür; 360 axtarışda (12 seçim dəsti, 15 sorğu, `limit` ilə və onsuz) Fuse ilə bayt-bayt eyni nəticələr qaytarır |

Sorğu başına axtarış vaxtı, Apple M4 (10 nüvə), Node 24, ölçmədən əvvəl nəticələrin eyni olduğu yoxlanılıb:

| Siyahı | Fuse.js | Rust | Sürətlənmə |
| ---: | ---: | ---: | ---: |
| 1,000 string | 1.9 ms | 0.1 ms | 18x |
| 10,000 string | 19.1 ms | 0.7 ms | 27-30x |
| 50,000 string | 91.2 ms | 2.8 ms | **32x** |
| 50,000 string, bir nüvə | 92.0 ms | 17.5 ms | 5.3x |

Portun düzgün etməli olduğu şeylər:

- Bitap 32 bitlik JS tam ədədləri üzərində işləyir: `1 << 31` mənfidir, `(1 << 31) - 1` isə növbəti sürüşdürmədə
  `2147483647`-yə çevrilən double-dır. Port bu dəyərləri JS-dəki kimi saxlayır.
- Mətn UTF-16 kod vahidləri ilə indekslənir, buna görə emoji Fuse-dakı kimi iki mövqe tutur.
- `ignoreDiacritics` NFD üstəgəl birləşdirici işarə diapazonlarının siyahısı və açıq yazılmış bir neçə hərfdir
  (`ł`, `ß`, `ı`); port eyni siyahıdan istifadə edir.
- Fuse-un hansı string-ləri ötürdüyünü `String.prototype.trim` həll edir: Rust-un
  `char::is_whitespace`-i (U+0085-i daxil edir və U+FEFF-i kənarda qoyur) yox, JS boşluq simvolları.

Alətdə tapılıb və düzəldilib: elementləri string olan, amma nəticələri string olmayan batch crate-ləri
istifadə olunmayan köməkçi alırdı və `clippy -D warnings`-dən keçmirdi; `gate`-in ölçü yoxlaması isə
import etdiyi Fuse fayllarını yox, yalnız adapteri sayırdı (54 sətir, buna görə 12.5x "çox böyük").

Nəyi əhatə etmir: `keys` ilə obyekt siyahıları, genişləndirilmiş və token axtarışı, məntiqi sorğular,
`includeMatches`, xüsusi `sortFn` və ya `getFn`. Drop-in bunları Fuse-a dəyişmədən ötürür.
Sürətlənmə üçün siyahı axtarışlar arasında Rust-da qalmalıdır; onu qurmaq təxminən Fuse-un öz
indeksi qədər baha başa gəlir (50,000 string üçün 3.4 ms, 2.8 ms-ə qarşı).
