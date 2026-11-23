---
title: Gerçek dünya portları
description: everything-rust gerçek kodla karşılaştığında ne oldu, yavaş sonuçlar hızlıların yanında, ve bu yüzden araçta ne değişti.
---

# Gerçek dünya portları

everything-rust gerçek kodla karşılaştığında ne oldu ve bu yüzden araçta ne değişti. Aşağıdaki her sayı, önce sonuçların aynı olduğunu kontrol eden bir benchmark'tan gelir ve yavaş vakalar hızlıların yanında listelenir.

| Port | Ne olduğu | Sonuç |
| --- | --- | --- |
| [json-server](/docs/real-world/json-server) | `app record`, `front-door`, `wire` ve `lift` ile uçtan uca port edilmiş tam bir Node sunucusu | 514/514 kayıtlı HTTP vakasını geçen saf bir Rust binary'si; altı birim `gate`'i geçiyor |
| [Fuse.js](/docs/real-world/fuse-js) | bulanık arama, bir aramanın zamanını harcadığı yol | 3,414/3,414 vaka eşleşiyor; 50,000 string'te 32x daha hızlı (tek çekirdekte 5.3x) |
| [vercel/ms](/docs/real-world/vercel-ms) | küçücük bir süre ayrıştırıcısı, uçtan uca | 975/975 vaka eşleşiyor; çağrı başına 0.56x, `parse` batch'i olarak 18x, `format` batch'i olarak 0.8x |
| [x18n](/docs/real-world/x18n) | vaka çalışması deposu | `packages/core` üzerinde her gate geçiyor, köprü üzerinden TS'ten daha yavaş; araması batch olarak 18-22x |
| [Altı dil](/docs/real-world/six-languages) | Go'nun `path`'i, Ruby'nin `shellwords`'ü, Python'un `colorsys`'i, küçük cepheler arkasında JDK ve glibc davranışı | her port kendi gate'ini geçiyor, toplamda 13,010 kayıtlı vaka |
| [Fixture'lar ve statik çalıştırmalar](/docs/real-world/fixtures) | aracı zorlamak için yazılmış fixture'lar ve analiz edilip çalıştırılmamış kütüphaneler | |

Gerçek bir şeyi port ettiniz mi? Yazılı bir anlatım memnuniyetle karşılanır: bkz. [Katkıda bulunma](/docs/project/contributing#share-a-real-port).
