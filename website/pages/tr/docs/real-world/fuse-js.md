---
title: Fuse.js
description: Rust'ta Fuse.js string listesi araması. 3,414/3,414 kayıtlı vaka eşleşiyor, bayt bayt aynı sonuçlar, 50,000 string'te 32x daha hızlı.
---

# Fuse.js

[Fuse.js](https://github.com/krisk/Fuse) (20k yıldız, haftada yaklaşık 20 milyon npm indirmesi,
commit `edf2fb6`), sayısız arama kutusunun arkasındaki bulanık arama kütüphanesidir. everything-rust 1.6
(yayınlanmamış) ile port edildi; crate [`demo/real-world/fuse-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/fuse-rs) içindedir.

`analyze`, `src/`'nin her klasörünü tek bir import döngüsüne koydu, bu yüzden Fuse dosya dosya port
edilemez. Port bunun yerine bir aramanın zamanını harcadığı yolu alır: 54 satırlık bir adaptör
(`adapter/relevance.ts`), Fuse'un `BitapSearch`'ü, alan normu ve `computeScoreSingle`'ından
oluşturulmuş, bir string listesindeki tek bir string için Fuse'un kendi puanını export eder. Bu
fonksiyon, 50,000 string üzerindeki bir aramanın 50,000 kez çalıştırdığı şeydir. `bridge --batch relevance:text`
string'leri Rust'ta tutar ve bir aramayı hepsinin üzerinde, tüm çekirdeklerde çalıştırır.

| Adım | Sonuç |
| --- | --- |
| adaptör | adaptörün sıralaması, hiç Rust yokken 81 sorgu ve seçenek kümesinde `new Fuse(list).search(q)` ile aynı |
| `record --inputs` | 3,414 vaka: Fuse'un kendi fixture'larından başlıklar, ASCII olmayan karakterler, emoji, 32 birimden uzun desenler, desteklenen her seçenek ve yanlış tipli seçenekler (reddedilen, 149 vaka) |
| port | yerini aldığı 839 satırlık Fuse için 673 satır Rust |
| `diff` | ilk derlemede 3,414/3,414 eşleşme |
| `gate` | geçiyor |
| drop-in | `withRust(Fuse)` string listesi aramalarını Rust'ta yanıtlar ve geri kalan her şeyi Fuse'a iletir; 360 aramada (12 seçenek kümesi, 15 sorgu, `limit` ile ve olmadan) Fuse ile bayt bayt aynı sonuçları döndürür |

Sorgu başına arama süresi, Apple M4 (10 çekirdek), Node 24, sonuçlar ölçümden önce aynı olduğu kontrol edilerek:

| Liste | Fuse.js | Rust | Hızlanma |
| ---: | ---: | ---: | ---: |
| 1,000 string | 1.9 ms | 0.1 ms | 18x |
| 10,000 string | 19.1 ms | 0.7 ms | 27 ila 30x |
| 50,000 string | 91.2 ms | 2.8 ms | **32x** |
| 50,000 string, tek çekirdek | 92.0 ms | 17.5 ms | 5.3x |

Portun doğru yapması gerekenler:

- Bitap 32 bit JS tam sayıları üzerinde çalışır: `1 << 31` negatiftir ve `(1 << 31) - 1` bir sonraki
  kaydırmada `2147483647` olan bir double'dır. Port bu değerleri JS'teki gibi tutar.
- Metin UTF-16 kod birimleriyle indekslenir, bu yüzden Fuse'taki gibi bir emoji iki konum tutar.
- `ignoreDiacritics`, NFD artı bir birleşik işaret aralıkları listesi ve açıkça yazılmış birkaç harftir
  (`ł`, `ß`, `ı`); port aynı listeyi kullanır.
- Fuse'un hangi string'leri atlayacağına `String.prototype.trim` karar verir: Rust'ın
  `char::is_whitespace`'i değil (U+0085'i içerir ve U+FEFF'i dışarıda bırakır), JS boşluk karakterleri.

Araçta bulunan ve düzeltilenler: öğeleri string olan ama sonuçları string olmayan batch crate'leri
kullanılmayan bir yardımcı alıyor ve `clippy -D warnings`'te kalıyordu; ve `gate`'in boyut kontrolü,
import ettiği Fuse dosyaları yerine yalnızca adaptörü sayıyordu (54 satır, bu yüzden 12.5x "çok büyük").

Kapsamadıkları: `keys` içeren nesne listeleri, genişletilmiş ve token araması, mantıksal sorgular,
`includeMatches`, özel bir `sortFn` ya da `getFn`. Drop-in bunları değiştirmeden Fuse'a verir.
Hızlanma, listenin aramalar arasında Rust'ta kalmasını gerektirir; onu oluşturmak Fuse'un kendi
indeksi kadar tutar (50,000 string için 2.8 ms'ye karşı 3.4 ms).
