---
title: Real portlar
description: everything-rust real kodla qarşılaşanda nə baş verdi, yavaş nəticələr sürətlilərin yanında, və bunun nəticəsində alətdə nə dəyişdi.
---

# Real portlar

everything-rust real kodla qarşılaşanda nə baş verdi və bunun nəticəsində alətdə nə dəyişdi. Aşağıdakı hər rəqəm əvvəlcə nəticələrin eyni olduğunu yoxlayan benchmark-dan gəlir, yavaş hallar isə sürətlilərin yanında göstərilib.

| Port | Nədir | Nəticə |
| --- | --- | --- |
| [json-server](/docs/real-world/json-server) | `app record`, `front-door`, `wire` və `lift` ilə başdan-başa port edilmiş bütöv Node serveri | qeydə alınmış 514/514 HTTP halından keçən təmiz Rust binary-si; altı vahid `gate`-dən keçir |
| [Fuse.js](/docs/real-world/fuse-js) | qeyri-dəqiq axtarış, axtarışın vaxt sərf etdiyi yol | 3,414/3,414 hal uyğun gəlir; 50,000 string üzərində 32x daha sürətli (bir nüvədə 5.3x) |
| [vercel/ms](/docs/real-world/vercel-ms) | kiçik müddət parser-i, başdan-başa | 975/975 hal uyğun gəlir; çağırış başına 0.56x, `parse` batch-i kimi 18x, `format` batch-i kimi 0.8x |
| [x18n](/docs/real-world/x18n) | nümunə araşdırması repozitoriyası | `packages/core`-da hər gate keçir, körpü vasitəsilə TS-dən daha yavaş; onun axtarışı batch kimi 18-22x |
| [Altı dil](/docs/real-world/six-languages) | Go-nun `path`, Ruby-nin `shellwords`, Python-un `colorsys` modulları, kiçik fasadlar arxasında JDK və glibc davranışı | hər port öz gate-indən keçir, cəmi 13,010 qeydə alınmış hal |
| [Fixture-lar və statik işləmələr](/docs/real-world/fixtures) | aləti sınamaq üçün yazılmış fixture-lar və analiz edilmiş, amma işə salınmamış kitabxanalar | |

Real nəsə port etmisiniz? Yazı xoş qarşılanır: bax [Töhfə vermək](/docs/project/contributing#share-a-real-port).
