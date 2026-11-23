---
title: Profil çıkarma ve analiz
description: profile ile zamanın nereye gittiğini bulun, ardından analyze ile birimleri, port sırasını, riskleri ve crate'leri planlayın.
---

# Profil çıkarma ve analiz

## Profile: taşımaya değeri bulun

Amaç hız olduğunda, herhangi bir seçim yapmadan önce ölçün:

```sh
everything-rust profile <repo> -- node bench.mjs      # ya da: python3 bench.py, ruby bench.rb,
                                                    #     java -cp out Main, go test -run X -bench . ./pkg
```

`profile`, gerçek bir iş yükünü dilin profiler'ı altında çalıştırır (node, cProfile, `go test` pprof, JFR, bir Ruby tracer) ve sıcak fonksiyonları kapsayıcı süreye göre, onları çağıranlarla ve hangi döngülerin batch'leneceğiyle birlikte listeler. Go'da profil yalnızca `go test` üzerinden çıkarılır; C desteklenmez.

- Her öğe için bir kez çağrılan küçük bir fonksiyon tek başına port etmeye **değmez**: Rust'a yapılan tek bir çağrı onun yaptığı işten pahalıdır ve JIT'ler (V8, JVM, Go) onu nanosaniyeler içinde çalıştırır. Onu çağıranlarıyla birlikte port edin ya da [batch'leyin](/docs/concepts/batches). Öneri satırı hangisi olduğunu tam komutuyla söyler.
- Zaten derlenmiş kod (numba `@njit`, Cython, `@jax.jit`) makine kodu olarak çalışır: bir Rust portu pek hız kazandırmaz. `profile` bunu "Already compiled" altında listeler.

## Analyze: portu planlayın

```sh
everything-rust analyze <repo> -o plan.md        # insanlar için
everything-rust analyze <repo> --json            # ajanlar için: şema everything-rust/analyze@1
everything-rust analyze <repo> --whole-app       # artı giriş noktaları, yapı, bağımlılıklar, kararlar
```

`analyze`; birimleri, port sırasını, `file:line` ile riskleri, birim başına köprüyü, her bağımlılığın bir crate'e ne kadar iyi eşlendiğini ve `start_here`'ı (en iyi ilk portlar) listeler. `--whole-app` uygulamanın yapısını (CLI, sunucu veya kütüphane) ve giriş noktalarını, crate'iyle birlikte her bağımlılığı (import edilen ya da bildirilen), parçaları (birbirini import eden birimler birlikte taşınır) ve önerilen bir yanıtla numaralandırılmış kararları ekler. Bkz. [Tüm uygulamayı port etmek](/docs/whole-apps/overview).

- Paketler (`package.json`) ve kaynak dosyalar; `node_modules`, derleme çıktısı, fixture'lar,
  playground'lar ve araç yapılandırmaları atlanır. Vue/Svelte/Astro `<script>` blokları import'lar için ayrıştırılır.
- **Birimler**: kaynak dosyalardan oluşan bir klasör ya da bir paketin kaynak kökündeki bir dosya.
- **Dalgalar**: bir birimin import'ları daha önceki dalgalarda gelir; import döngüleri birlikte port edilir.
- **Köprü**: birim veya import ettiği herhangi bir şey Node'a ihtiyaç duyuyorsa napi-rs; taşınabilir
  kod için wasm-bindgen; UI ve editör kodu için JS'te tut.
- **Hazırlık** (ne kadar güvenli) ve **öncelik** (güvenli ve değerli: döngüler ve string işleri, testler, boyut).

| Risk | Önem | Örnek |
| --- | --- | --- |
| String ofsetleri | yüksek | `slice`, `indexOf`, `matchAll`, `d` bayrağı: UTF-16'ya karşı UTF-8 |
| Regex sözdizimi | yüksek | lookaround, geri başvurular, `y`: `regex` crate'inde yok |
| Dinamik regex | yüksek | `new RegExp(userPattern)`: çalışma zamanında JS sözdizimi |
| Dinamik kod | yüksek | `eval`, `new Function`, hesaplanmış `import()` |
| Dinamik tipleme | orta | `any`, `'key' in value` |
| Sıralı koleksiyonlar | orta | `Map`, `Set`, `Object.keys`: ekleme sırası |
| Async, generator'lar | orta | promise ve generator sınırları |
| Sınıf kalıtımı, reflection | orta | `extends`, `Proxy`, `Reflect`, `Symbol` |
| Callback biçimli API'ler | orta | fonksiyon tipli parametreler ve metotlar: geveze köprüler |
| Exception'lar, sayılar, JSON | düşük | `throw`, bit düzeyi işlemler, `JSON.stringify` sırası |

## Bağımlılıklar için crate'ler

`analyze`, kodun import ettiği her kütüphane için Rust crate'ini `good`, `partial` veya `none` uyum derecesiyle adlandırır. Tablosunda olmayan bir kütüphane `unknown` olarak raporlanır; tablo asla crate tahmin etmez. Bir tane eklemek için bkz. [Katkıda bulunma](/docs/project/contributing#add-a-crate-mapping).
