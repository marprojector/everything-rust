---
title: Bridge
description: bridge, tek bir birim için binding'leri ve bir wrapper'ı olan bir Rust crate'i iskeletler; böylece çağıranlar orijinal API'yi korur.
---

# Bridge

```sh
everything-rust bridge <repo> <unit-id>          # crate <repo>/rust/<name> içinde; asla üzerine yazmaz
everything-rust refresh <crate>                  # port.rs'te bir imzayı değiştirdikten sonra
```

`bridge`, binding'leri (`lib.rs` ve kaynak dilin yüklediği wrapper) ve port edilecek stub'ları (`port.rs`) içeren bir crate oluşturur. Portu yalnızca `src/port.rs` içinde yazarsınız; `src/lib.rs` ve wrapper'lar üretilir ve `refresh` bunları `port.rs`'teki imzalardan, yeni batch sınıflarını da ekleyerek yeniden üretir. `port.rs`'e asla dokunmaz.

Seçenekler:

- `--batch fn:param`, birçok `param` değerini Rust'ta tutan bir sınıf ekler. Bkz. [Batch'ler](/docs/concepts/batches).
- `--as-json T`, uygulamanın yalnızca veriyle doldurduğu bir tip parametresini (`paginate<T>(items: T[])` içindeki `T`) JSON değerleri olarak köprüler.
- `--keep "name: reason"`, kaynak dilde daha fazla export tutar; örneğin `Intl` verisine bağlı kod.
- `--kind`, planın seçtiği köprüyü geçersiz kılar (TypeScript için napi-rs veya wasm).

## Dil başına köprü

| Dil | Köprü |
| --- | --- |
| TypeScript/JavaScript | üretilmiş bir binary wire ile napi-rs veya wasm-bindgen; sınıflar ve async dahil |
| Python | PyO3 (abi3), doğrudan dönüşüm, tipli exception'lar |
| C | orijinal adlarla `extern "C"` fonksiyonlar ve bir header |
| Go | orijinal API'ye sahip bir pakette tipli cgo çağrıları ve üretilmiş bir binary wire |
| Ruby | orijinal modüllerin altında bir magnus eklentisi |
| Java | orijinal metotlara sahip wrapper sınıflarda tipli JNI ve üretilmiş bir binary wire |

PHP ve C# birimleri henüz köprülenemez; bkz. [PHP](/docs/languages/php) ve [C#](/docs/languages/csharp).

## bridge neye karar verir

- Bir köprüyü geçemeyen export'lar gerekçesiyle birlikte kaynak dilde kalır ve shadow modülü
  orijinali yeniden export eder. TS için:
  fonksiyon alan veya döndüren fonksiyonlar, generator'lar ve iterator'lar, RegExp, `instanceof` kontrolleri,
  generic'ler (`--as-json T` uygulamanın yalnızca veriyle doldurduğu birini köprüler) ve tipleri (import'lar, yeniden export'lar ve `extends` üzerinden izlenerek) fonksiyon
  içeren değerler. `--keep "name: reason"` daha fazlasını tutar; örneğin `Intl` verisine bağlı kod.
- TS tipleri Rust tiplerine eşlenir; tip takma adları import'lar üzerinden çözümlenir ve parametre
  tipleri varsayılan değerlerden çıkarılır; yapılandırılmış değerler üretilmiş bir binary wire ile
  geçer (JSON yalnızca wire'ın modellemediği yapılar için bir fallback'tir).
- JSON `undefined`, NaN, Infinity, -0 veya tek başına surrogate'leri ifade edemez: yapılandırılmış
  değerler, üretilen `index.cjs`'in çevirdiği işaretler taşır (`{"$undefined": true}`, `{"$num": "NaN"}`,
  `{"$utf16": [...]}`); yardımcılar crate'in `js.rs` dosyasındadır.

`record`, TS kaynaklarını olduğu gibi çalıştırır: bir Node resolve hook'u bundler tarzı import'ları
izler (`./errors`'tan `./errors.ts`'e), repoda hiçbiri kurulu değilse `--resolve-from` npm paketlerini
sağlar ve dönüştürme gerektiren TS (enum'lar, parametre özellikleri)
`--experimental-transform-types` ile yeniden denenir.
