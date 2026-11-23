---
title: vercel/ms
description: Uçtan uca port edilmiş vercel/ms. Kaydedilen her vakada doğru, çağrı başına daha yavaş, yalnızca bir program tek seferde çok sayıda süre ayrıştırdığında daha hızlı.
---

# vercel/ms

[vercel/ms](https://github.com/vercel/ms) (5.5k yıldız, commit `4ff48ce`) `"2 days"`'i milisaniyeye
ve geri çevirir; çoğu Node projesi onu bir bağımlılık üzerinden çeker. everything-rust 1.5.1 ile uçtan uca
port edildi; crate
[`demo/real-world/vercel-ms-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/vercel-ms-rs) içindedir.

| Adım | Sonuç |
| --- | --- |
| `analyze` | 1 birim (`src/index.ts`), hazır, hazırlık 96 |
| `bridge --kind napi` | `ms`, `parse`, `parseStrict` ve `format` köprülendi, hiçbiri TS'te tutulmadı |
| `record --from-tests -- jest` | 975 vaka: üretilmiş olanlar, tuzaklar ve vercel/ms'in 167 Jest testinden yakalanan 340 çağrı (ilk port onlardan elle kopyalanmış 149 girdi kullandı) |
| port | 184 satır TS için 200 satır Rust, batch başına `options`'ı bir kez okuyan bir `FormatIndex` dahil |
| `diff` | ilk derlemede 784/784 eşleşti; yakalanan çağrılarla 975/975 |
| `gate` | geçiyor (fmt, clippy, testler, hata yolları, boyut, davranış) |
| `bench`, çağrı başına | kayıtlı vakalar üzerinde 0.56x (başta 0.39x): normal bir string üzerinde `parse` 79 ns sürüyor, JS ile başa baş, ama uç durumlar ve fırlatılan hatalar köprü üzerinden hâlâ daha pahalı |
| batch, `parse` x100,000 | JS 11.8 ms, Rust 0.65 ms: **18x**; string'lerin tek seferlik devir teslimi sayılınca 3.4x (string'ler birleştirilmiş geçmeden önce 2.1x) |
| batch, `format` x100,000 | JS 2.1 ms, devir teslim ile Rust 2.5 ms: **0.8x** (başta 0.2x); `format(x, { long: true })` 1.2x |

Portun doğru yapması gerekenler, hepsi kayıtlı vakalar veya tuzaklar tarafından yakalandı:

- `u` bayrağı olmadan `/.../i` yalnızca ASCII harflerini katlar, Rust'ın `regex`'i ise Unicode'u katlar;
  bu yüzden `"1 ſ"` (uzun s) ve Kelvin işareti saniye olarak ayrıştırılmış olurdu. Port birimleri elle,
  yalnızca ASCII olarak eşleştirir.
- `Math.round` yarımları yukarı yuvarlar (`-2.5`, `-2` verir); Rust'ın `round`'u sıfırdan uzağa yuvarlar.
- `${n}`, `1e+21`, `5e-324` ve `-0` için `0` yazdırır; port JS'in sayıdan string'e dönüşümünü yeniden
  üretir. İlk sürümünde ayrıca ms'in tam sayı çıktılarının hiç ulaşmadığı bir eşitlik hatası vardı
  (`25.728225708007812`, `...813` olarak yazdırılıyordu).
- `str.length` UTF-16 birimlerini sayar, bu yüzden 100 karakterlik sınır bir emojiyi iki sayar.

Araçta bulunan ve düzeltilenler: yapılandırılmış bir parametreye (`format`'ın `options`'ı) sahip bir
napi batch metodu derlenmiyordu, çünkü üretilen binding, napi'nin `serde-json` özelliği olmadan
dönüştüremediği bir `serde_json::Value` alıyordu. Batch sınıfı olan crate'ler artık onu etkinleştiriyor.

Bu porttan araca geri alınanlar:

- Üretilen `js.rs` içinde, 20,000 rastgele double üzerinde Node'a karşı kontrol edilmiş
  `js::number_to_string` ve `js::math_round` var; port kendi kopyaları yerine bunları kullanıyor.
  `number_to_string` basamaklarını serde_json'ın yazıcısından alır: tam sayılar için `format!`'tan 5x,
  kesirler için 3x daha hızlı.
- napi batch'leri string öğeleri UTF-16 bitiş ofsetleriyle tek bir birleştirilmiş string olarak, sayı
  öğelerini tek bir `Float64Array` olarak gönderir ve string sonuçları birleştirilmiş olarak döndürür
  (`index.cjs` onları `slice` ile böler): öğe başına bir JS değerine göre string'ler için 2x, sayılar
  için 20x daha hızlı. `new Batch(items)` ve `run()` biçimlerini korur.
- `analyze`, `Math.round`'u, `toFixed`'ı ve `s` ya da `k` içeren `u`'suz `/i` regex'lerini her biri için
  yönlendirmeyle işaretler.
- Çağrılar daha ucuz: napi'de her sync fonksiyon napi argümanı olmadan binary wire üzerinden geçer
  (bir argüman yaklaşık 55 ns, çağrının tamamı 22 ns tutar), fonksiyon başına iki yerine tek bir JS
  wrapper vardır, `serde_json::Value` parametreleri herhangi bir biçimde etiketli değerler olarak geçer
  ve hatalar wire buffer'ında geri döner. Çağrı başına `parse` 121 ns'den 79 ns'ye indi.
- `record --from-tests`, test girdilerini elle kopyalamanın yerini alır: testler çalışırken test takımının
  yaptığı her çağrıyı kaydeder, yakalama kodu kapsam dışında tutulur (ms %100 kapsam zorunlu kılar).

Karar: `ms` iyi bir port ama kötü bir hız hedefi. Kaydedilen her vakada doğru, yalnızca bir program
tek seferde çok sayıda süre ayrıştırdığında daha hızlı.
