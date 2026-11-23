---
title: Kayan nokta toleransı
description: Sayısal kod port edildiğinde nadiren bit bit aynı float'lar verir. --float-tolerance, float'ların büyük değerin belli bir oranı içinde eşleşmesine izin verir.
---

# Kayan nokta toleransı

```sh
everything-rust record <crate> --float-tolerance 1e-9
everything-rust app record . --float-tolerance 1e-9 -- python3 cli.py
```

Sayısal kod (fizik, istatistik, grafik) port edildiğinde nadiren bit bit aynı float'lar verir: işlem sırası, fused multiply-add ve matematik kütüphanesi derleyiciden derleyiciye değişir. Bunu kabul ediyorsanız `--float-tolerance` ve kendi değerinizle kaydedin. Float'lar o zaman büyük değerin bu oranı içinde eşleşir (1'in altında mutlak değer olarak) ve `diff` ile `gate` bunu kullanır. `0` toleransı kaldırır.

Hâlâ tam olarak eşleşmesi gerekenler:

- tam sayılar, metin ve yapı;
- farklı yazılmış aynı değer (`3` ve `3.0`);
- biçimlendirme. Sayıdan metne biçimlendirme diller arasında farklıdır (`1e21`, `-0`, basamak sayıları); tuzaklar bu değerleri içerir ve `--float-tolerance` yalnızca yuvarlamayı affeder, biçimlendirmeyi asla.

Bir uyuşmazlığı ortadan kaldırmak için toleransı asla genişletmeyin: onun ötesindeki bir fark portta bir hatadır.
