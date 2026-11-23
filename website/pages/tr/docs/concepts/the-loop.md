---
title: Döngü
description: profile, analyze, bridge, record, port, diff, bench, shadow ve gate. Her birimin geçtiği adımlar.
---

# Döngü

Tek bir sıcak fonksiyondan tüm bir uygulamaya kadar her port, birim birim aynı adımlardan geçer. Bir port ancak `gate` geçtiğinde biter.

everything-rust bir portu nasıl kontrol eder, komutları neye karar verir ve sınırları nerededir. [README](/)'ye dön.

## Nasıl çalışır

```
profile ──> analyze ──> bridge ──> record ──> port.rs ──> diff ──> bench ──> shadow ──> gate
 where the    what to     the Rust   what the    you or      every     faster?    both side   done means
 time goes    port first  crate and  original    your agent  recorded  the        by side in  this passes
                          wrapper    does        write it    case      verdict    production
```

Doğruluğun kaynağı orijinal kod olarak kalır: `record` onu uç durumlarla (ve sizin girdilerinizle)
çalıştırır ve döndürdüğünü ya da fırlattığını kaydeder; `diff` her vakayı Rust'a karşı yeniden oynatır;
hepsi eşleşip `gate` geçene kadar hiçbir şey yayınlanmaz. Çağıranlar baştan sona orijinal API'yi korur.

Tüm bir uygulama da aynı döngüden birim birim geçer; bunu programın tamamını kanıtlayan bir çerçeve
içinde yapar ([Tüm uygulamayı port etmek](/docs/whole-apps/overview)):

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop above, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in         pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down      proved by
 decisions to make      outside         original answers                                                replay
```

## Döngü

| Adım | Komut | Ne yapar |
| --- | --- | --- |
| 0. Profil | `everything-rust profile <repo> -- node bench.mjs` | Gerçek bir iş yükünü dilin profiler'ı altında çalıştırır (node, cProfile, `go test` pprof, JFR, bir Ruby tracer): kapsayıcı süreye göre sıcak fonksiyonlar, onları çağıranlar ve hangi döngülerin batch'leneceği; Rust'ın pek kazandırmadığı, zaten derlenmiş kodu (numba, Cython, JAX) işaretler |
| 1. Plan | `everything-rust analyze <repo>` | Birimler, port sırası, `file:line` ile riskler, birim başına köprü, bağımlılığın crate'e uyumu, en iyi ilk portlar; `--whole-app` giriş noktalarını ve yapıyı, crate'iyle birlikte her bağımlılığı (import edilen ya da bildirilen), parçaları (birbirini import eden birimler birlikte taşınır) ve önerilen bir yanıtla numaralandırılmış kararları ekler |
| 2. Köprü | `everything-rust bridge <repo> <unit>` | Binding'leri (`lib.rs` ve kaynak dilin yüklediği wrapper) ve port edilecek stub'ları (`port.rs`) içeren crate; `--batch fn:param` birçok `param` değerini Rust'ta tutan bir sınıf ekler; `--as-json T` uygulamanın yalnızca veriyle doldurduğu bir tip parametresini JSON değerleri olarak köprüler |
| 3. Kayıt | `everything-rust record <crate>` | Orijinali üretilmiş uç durumlarla (ASCII olmayan karakterler, emoji, `-0`, `NaN`, float beklenen yerde int'ler, ...) ve her parametredeki her anlamsal tuzakla (string uzunluğu ve büyük/küçük harf, işaretli bölme, taşma sınırları, float yazdırma, map ve sıralama düzeni) çalıştırır ve `cases.jsonl`'i kaydeder; `--from-tests -- npx jest` (ya da `-- pytest`) orijinalin kendi test takımının yaptığı her çağrıyı da kaydeder; `--float-tolerance 1e-9` sayısal kodda float'ların bu oran içinde eşleşmesine izin verir |
| 3. Derleme | `everything-rust build <crate>` | Rust tarafını derler: `.node`, wasm paketi, Python eklentisi ya da paylaşımlı kütüphane |
| 3. Diff | `everything-rust diff <crate>` | Kaydedilen vakaları, orijinali çağıranların çağırdığı şekilde Rust'a karşı yeniden oynatır; uyuşmazlıkları listeler |
| 3. Bench | `everything-rust bench <crate>` | Kaydedilen vakaları orijinal ve Rust üzerinde köprü dahil ölçer |
| 4. Shadow | `everything-rust shadow <crate>` | İkisini de çalıştıran, orijinalin sonucunu döndüren ve uyuşmazlıkları raporlayan drop-in; `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. Gate | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, testler, `unsafe` denetimi, port edilmemiş bir şey kalmaması, orijinalin her hata yolu için kayıtlı başarısız bir vaka, orijinalin hata döndürdüğü yerde `unwrap`/`expect`/`panic!` olmaması (`// INFALLIBLE: <why>` ile işaretlenmedikçe), derleme, API, davranış |
| Ajanlar | `everything-rust setup` | MCP sunucusunu (`everything-rust mcp`, her adım bir araç olarak) ve iş akışı skill'ini ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) kodlama ajanlarınıza kaydeder |

## Hot path'ler: veriyi Rust'ta tutun

Her öğe için bir kez çağrılan bir port, köprünün bedelini her öğe için bir kez öder ve bir JIT'e
yenilir. Kazandıran, döngüyü taşımaktır: `profile` onu bulur ve `--batch`, öğeleri bir kez alıp
her çağrıda fonksiyonu hepsinin üzerinde, tüm çekirdeklerde, typed array sonuçlarla çalıştıran bir sınıf üretir.

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore: çalışmanın dörtte biri, `matched` tarafından anahtar başına çağrılıyor: batch'leyin
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

`port.rs` içindeki bir `FuzzyScoreIndex` struct'ı, varsayılan döngüyü öğeleri bir kez hazırlayan
ve kendi başına sıralayabilen (`rank`) bir döngüyle değiştirir: anahtarları her biri için bir karakter
maskesiyle küçük harfe çevrilmiş olarak, her anahtarda her yaygın karakterin ilk konumunu gösteren
bir tabloyu (tek harfli bir sorgu tek bir sütundur) ve son sorgunun eşleşmelerini (`s`'den sonra `se`
yazmak yalnızca `s`'nin eşleştiklerini yeniden puanlar) tutar. x18n'in 50,000 anahtar üzerindeki
kenar çubuğu aramasında, harf harf yazılarak, aynı sonuçlarla:

| | Sorgu başına | TS'e göre |
| --- | ---: | ---: |
| TS: `map(fuzzyScore)`, filtre, sıralama | 10 ms | 1x |
| İndeksle aynı daraltmayı yapan TS | 4.1-5.4 ms | 2x |
| Aynı port, anahtar başına çağrılarak | - | 0.2-0.4x |
| `FuzzyScoreBatch`, varsayılan indeks (4 çekirdek) | 5.1 ms | 1.7x |
| `FuzzyScoreIndex` ile `FuzzyScoreBatch` (napi, 4 çekirdek) | 0.45-0.56 ms | **18-22x** |
| Aynısı wasm ile (tek thread) | 0.95 ms | **11.5x** |

Aynı daraltmayı yapan TS'e karşı bile Rust hâlâ 7-12x daha hızlıdır (wasm 5x). Döndürülen
indeksleri düz bir döngüyle anahtarlara çevirin: `Array.from(order, i => keys[i])` 20x daha pahalıydı, 0.4 ms.

Her dil batch alır ve çağrı başına portların kaybettiği yerde bile orijinali geçer. Aynı iş
4 çekirdekte, aynı sonuçlar ve hatalarla (benchmark'lar her demo klasöründe):

| Dil | İş yükü | Orijinal | Çağrı başına portun orijinale oranı | Batch'in orijinale oranı |
| --- | --- | ---: | ---: | ---: |
| TS | x18n araması, 50,000 anahtar | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`, 20,000 string | 98 ms | - | **17x** |
| Ruby | `slugify`, 20,000 string | 110 ms | - | **12.7x** |
| Ruby | `shellescape` (stdlib), 50,000 karışık değer | 88 ms | - | **11.8x** |
| Go | `Slugify`, 50,000 string | 49 ms | 0.93x | **4x** |
| Java | `slugify`, 50,000 string | 43 ms | 0.68x | **3x** |

Derlenen orijinaller daha az kazanır: öğe başına iş zaten hızlıdır, bu yüzden batch çekirdek sayısı
kadar kazanır. Batch crate'leri bellek ayırmayı mimalloc ile yapar: sistem ayırıcısıyla Go batch'i
tek thread'den daha yavaştı. Demo: [`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md).

## `bridge` neye karar verir

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

## `analyze` neye bakar

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

## Neyi yakalayamaz

Geçmek, Rust'ın kaydedilen her vakayla eşleştiği anlamına gelir; fazlası değil. Yeniden yazımları
vuran şeyler genellikle kaydın dışında yaşar, bu yüzden sınırların nerede olduğunu bilin:

- **Kimsenin kaydetmediği girdiler.** Üretilen vakalar, tuzaklar, orijinalin test takımı
  (`record --from-tests`) ve sizin `--inputs` girdileriniz tavandır. Gerçek girdiler ekleyin
  (production örnekleri, hata raporları); prod'u bozan kullanıcı girdisi nadiren bir üretecin aklına gelir.
- **Değişmiş bir orijinal.** `record`, birimin dosyalarının bir parmak izini kaydeder
  (`recorded-from.json`); orijinal daha sonra değişirse, orijinalin eskiden yaptığına karşı geçmek
  yerine, siz yeniden kaydedene kadar `diff` uyarır ve `gate` başarısız olur.
- **Regex'ler.** `regex` crate'inde lookaround ya da geri başvuru yoktur ve derlenen bir desen bile
  ASCII olmayan metinde `\w`, `\b`, `\d` ve büyük/küçük harf duyarsız eşleşmeyi JS, Python veya
  Java'dan farklı ele alabilir. `fancy-regex`'e geçmek ya da deseni yeniden yazmak kaydedilen her
  vakayı geçip yine de Türkçedeki noktasız `ı` veya bir zero-width joiner üzerinde farklı davranabilir. `record`,
  regex kullanan bir dosyadaki her fonksiyona regex tuzakları ekler (bunlar, Kelvin işareti, uzun s,
  Arap-Hint rakamları, satır ayırıcıları, son sigma), ama gerçek desenlerinizi gerçek metne karşı da fuzz edin.
- **Çalıştırmalar arasında değişenler.** `record` orijinali iki kez çalıştırır ve farklı olanları
  ayıklar: değişen değerler (zamanlar, id'ler) joker olur; başka bir sırayla gelen aynı satırlar
  (hash map'ler, eşzamanlılık) herhangi bir sırada karşılaştırılır, yani bir port yine de tam olarak
  o satırları üretmelidir; farklı sayıda satır `...` olur ve her şeyle eşleşir. `record` her türü
  listeler: her birine bakın, çünkü hash sırası veya eşzamanlılık farkı tam olarak bir portta
  gerçekten değişebilecek şeydir.
- **Float'lar.** Sayıdan metne biçimlendirme diller arasında farklıdır (`1e21`, `-0`, basamak
  sayıları). Tuzaklar bu değerleri içerir ve tam olarak eşleşmeleri gerekir; `--float-tolerance`
  yalnızca yuvarlamayı affeder, biçimlendirmeyi asla.
- **Çıktıda olmayan her şey.** Zamanlama, bellek, yük altında eşzamanlılık ve stdout, stderr, çıkış
  kodları, yazılan dosyalar ve HTTP yanıtları dışındaki yan etkiler (ağ çağrıları, veritabanları,
  sinyaller) karşılaştırılmaz.
- **Diğer platformlar.** Bir kayıt, davranışı onu çalıştıran makinede kanıtlar. Sonuçları tip
  boyutlarına, endianness'a veya derleyiciye bağlı olan C kodu hedefte farklı davranabilir; `analyze`
  bu nedenle donanıma dönük C kodunu C'de tutar.

## Her adım

| Adım | Komut | Ne yapar |
| --- | --- | --- |
| 0. Profil | `everything-rust profile <repo> -- node bench.mjs` | Gerçek bir iş yükünü dilin profiler'ı altında çalıştırır (node, cProfile, `go test` pprof, JFR, bir Ruby tracer): kapsayıcı süreye göre sıcak fonksiyonlar, onları çağıranlar ve hangi döngülerin batch'leneceği; Rust'ın pek kazandırmadığı, zaten derlenmiş kodu (numba, Cython, JAX) işaretler |
| 1. Plan | `everything-rust analyze <repo>` | Birimler, port sırası, `file:line` ile riskler, birim başına köprü, bağımlılığın crate'e uyumu, en iyi ilk portlar; `--whole-app` giriş noktalarını ve yapıyı, crate'iyle birlikte her bağımlılığı (import edilen ya da bildirilen), parçaları (birbirini import eden birimler birlikte taşınır) ve önerilen bir yanıtla numaralandırılmış kararları ekler |
| 2. Köprü | `everything-rust bridge <repo> <unit>` | Binding'leri (`lib.rs` ve kaynak dilin yüklediği wrapper) ve port edilecek stub'ları (`port.rs`) içeren crate; `--batch fn:param` birçok `param` değerini Rust'ta tutan bir sınıf ekler; `--as-json T` uygulamanın yalnızca veriyle doldurduğu bir tip parametresini JSON değerleri olarak köprüler |
| 3. Kayıt | `everything-rust record <crate>` | Orijinali üretilmiş uç durumlarla (ASCII olmayan karakterler, emoji, `-0`, `NaN`, float beklenen yerde int'ler, ...) ve her parametredeki her anlamsal tuzakla (string uzunluğu ve büyük/küçük harf, işaretli bölme, taşma sınırları, float yazdırma, map ve sıralama düzeni) çalıştırır ve `cases.jsonl`'i kaydeder; `--from-tests -- npx jest` (ya da `-- pytest`) orijinalin kendi test takımının yaptığı her çağrıyı da kaydeder; `--float-tolerance 1e-9` sayısal kodda float'ların bu oran içinde eşleşmesine izin verir |
| 3. Derleme | `everything-rust build <crate>` | Rust tarafını derler: `.node`, wasm paketi, Python eklentisi ya da paylaşımlı kütüphane |
| 3. Diff | `everything-rust diff <crate>` | Kaydedilen vakaları, orijinali çağıranların çağırdığı şekilde Rust'a karşı yeniden oynatır; uyuşmazlıkları listeler |
| 3. Bench | `everything-rust bench <crate>` | Kaydedilen vakaları orijinal ve Rust üzerinde köprü dahil ölçer |
| 4. Shadow | `everything-rust shadow <crate>` | İkisini de çalıştıran, orijinalin sonucunu döndüren ve uyuşmazlıkları raporlayan drop-in; `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. Gate | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, testler, `unsafe` denetimi, port edilmemiş bir şey kalmaması, orijinalin her hata yolu için kayıtlı başarısız bir vaka, orijinalin hata döndürdüğü yerde `unwrap`/`expect`/`panic!` olmaması (`// INFALLIBLE: <why>` ile işaretlenmedikçe), derleme, API, davranış |
| Ajanlar | `everything-rust setup` | MCP sunucusunu (`everything-rust mcp`, her adım bir araç olarak) ve iş akışı skill'ini ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) kodlama ajanlarınıza kaydeder |

## Sonraki

Her adımın kendi sayfası vardır: [Profil çıkarma ve analiz](/docs/concepts/analyze), [Bridge](/docs/concepts/bridge), [Record](/docs/concepts/record), [Port](/docs/concepts/port), [Diff](/docs/concepts/diff), [Bench](/docs/concepts/bench), [Shadow](/docs/concepts/shadow) ve [Gate](/docs/concepts/gate).
