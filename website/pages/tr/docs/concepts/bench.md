---
title: Bench
description: bench, kaydedilen vakaları orijinal ve Rust üzerinde köprü dahil ölçer ve bir karar verir. Rust her zaman daha hızlı değildir.
---

# Bench

```sh
everything-rust bench <crate>
```

`bench`, kaydedilen vakaları orijinal ve derlenmiş Rust modülü üzerinde köprü dahil ölçer; satırları ve öneri içeren bir karar döndürür: yayına alın, yavaş fonksiyonları kaynak dilde tutun ya da daha büyük bir birimi port edin. 1'in altındaki bir hızlanma, portun çağıranların çağırdığı şekliyle daha yavaş olduğu anlamına gelir. `--min-ms`, fonksiyon ve taraf başına ölçülen asgari süreyi belirler (varsayılan 200).

Karara uyun. Çağıranları yavaşlatan bir portu bunu söylemeden yayına almayın.

## Bir port neden daha yavaş olabilir

Her öğe için bir kez çağrılan bir port, köprünün bedelini her öğe için bir kez öder. Belirleyici olanlar:

- **Yorumlanan orijinaller (Python, Ruby)**: Rust'a yapılan çağrı yaklaşık 100-300 ns tutar ve Rust kodu yorumlayıcıdan çok daha hızlıdır, bu yüzden küçük fonksiyonlar bile kazanır.
- **Derlenen orijinaller (Go, JIT sonrası Java)**: orijinal yaprak fonksiyon 4-90 ns sürer; tek bir cgo çağrısı tek başına yaklaşık 80 ns tutar, string'li bir JNI çağrısı daha fazla. Bu diller ancak tek bir çağrı çok iş yaptığında kazanır: bir fonksiyonu çağıranlarıyla birlikte port edin.
- **C**: sınır geçişi yoktur; Rust kodu doğrudan C koduyla karşılaştırılır.

Gerçek kod üzerinde çağrı başına ölçülenler ([altı dil](/docs/real-world/six-languages)):

| Dil | Orijinale karşı Rust (`bench`, geometrik ortalama) | Karar |
| --- | --- | --- |
| Ruby | **4.4x daha hızlı** (fonksiyon başına 3.6-7x) | yayına al |
| Python | **1.35x daha hızlı** (çoğu çağrıda 1.5-1.9x) | yayına al |
| C | 0.87x: ctype 1.1-1.4x daha hızlı, glibc'nin ince ayarlı `strtod`, `strspn`, `strcmp` fonksiyonları 0.5-0.7x | optimize et ya da yavaş olanları tut |
| Java | 0.18x | daha büyük birimleri port et |
| Go | 0.04x | daha büyük birimleri port et |

x18n çekirdeğinin API değişikliği olmadan taşınabilen 29 fonksiyonunun tamamı port edildi: her gate geçiyor ve 7,925 kayıtlı vaka tam olarak eşleşiyor; `bench` ise portların JS'ten çağrıldığı haliyle TS'ten daha yavaş olduğunu gösterdi, çünkü köprünün maliyeti yapılan işten fazla ([rapor](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-core/README.md)). vercel/ms, kaydedilen vakaları üzerinde çağrı başına 0.56x hızında çalıştı ([vaka çalışması](/docs/real-world/vercel-ms)).

## Kazandıran: döngüyü taşımak

`profile`, sıcak bir fonksiyonu çevreleyen döngüyü bulur ve `--batch` döngünün tamamını tüm çekirdeklerde Rust'a taşır. Bkz. [Batch'ler ve hot path'ler](/docs/concepts/batches).

::: tip Sakin bir makinede ölçün
`bench`'i sakin bir makinede çalıştırın, iki kez çalıştırın ve yayımlanan her sayının yanına donanımı ve çekirdek sayısını yazın.
:::
