---
title: Record
description: record, orijinali üretilmiş uç durumlar, anlamsal tuzaklar, kendi testleri ve sizin girdilerinizle çalıştırır ve sonuçları cases.jsonl olarak kaydeder.
---

# Record

```sh
everything-rust record <crate>
everything-rust record <crate> --from-tests -- npx jest      # ya da: -- pytest
everything-rust record <crate> --inputs extra.jsonl
```

`record`, orijinal fonksiyonları çalıştırır ve her birinin döndürdüğünü ya da fırlattığını crate'in yanındaki `cases.jsonl` dosyasına kaydeder: portun eşleşmesi gereken davranış. Sonradan çıkan bir uyuşmazlık bir davranış farkıdır: kaydı değil, portu düzeltin.

## Vakalar nereden gelir

- **Üretilmiş uç durumlar**: ASCII olmayan karakterler, emoji, `-0`, `NaN`, float beklenen yerde int'ler ve daha fazlası (`--count` fonksiyon başına kaç tane olacağını belirler).
- Her parametrede **anlamsal tuzaklar**: string uzunluğu ve büyük/küçük harf, işaretli bölme, taşma sınırları, float yazdırma, map ve sıralama düzeni.
- Regex kullanan bir dosyadaki her fonksiyon için **regex tuzakları**: Türkçedeki noktasız `ı`, Kelvin işareti, uzun s, Arap-Hint rakamları, satır ayırıcıları, son sigma, zero-width joiner'lar.
- **Orijinalin kendi testleri**: `--from-tests -- <test command>`, test takımının birimin export edilen fonksiyonlarına yaptığı her çağrıyı kaydeder (herhangi bir runner ile TS/JS, pytest veya unittest ile Python). Kaynak sonrasında geri yüklenir. vercel/ms'te 167 Jest testinden 340 çağrı yakaladı.
- **Sizin girdileriniz**: satır başına bir `{"fn": name, "args": [...]}` içeren `--inputs cases.jsonl`. Gerçek girdiler ekleyin (production örnekleri, hata raporları); prod'u bozan kullanıcı girdisi nadiren bir üretecin aklına gelir.

Python'da çağıranlar `1`'i `1.0` kadar sık geçirir, bu yüzden kayıt her dört Python vakasından birinde tam sayı değerli sayıları int olarak geçirir.

## cases.jsonl içindeki işaretler

JSON `undefined`, NaN, Infinity, -0 veya tek başına surrogate'leri ifade edemez. Yapılandırılmış değerler, üretilen `index.cjs`'in çevirdiği işaretler taşır; yardımcılar crate'in `js.rs` dosyasındadır:

| İşaret | Anlamı |
| --- | --- |
| `{"$undefined": true}` | JavaScript `undefined` |
| `{"$num": "NaN"}` | JSON'un tutamayacağı bir sayı (`NaN`, `Infinity`, `-0`) |
| `{"$utf16": [...]}` | tek başına surrogate'ler için UTF-16 kod birimleri olarak verilmiş bir string |

Bir fonksiyonun sözleşmesi dışındaki C girdileri orijinali çökertebilir. C sürücüsü her vakayı fork edilmiş bir alt süreçte çalıştırır ve `crash: SIGSEGV` kaydeder; C'nin davranışı tanımsız olduğundan `diff` orada portun her davranışını kabul eder.

## İki çalıştırma ve farklı olanlar

`record` orijinali iki kez çalıştırır ve çalıştırmalar arasında farklı olanları ayıklar: değişen değerler (zamanlar, id'ler) joker olur; başka bir sırayla gelen aynı satırlar (hash map'ler, eşzamanlılık) herhangi bir sırada karşılaştırılır; farklı sayıda satır `...` olur ve her şeyle eşleşir. `record` her türü listeler. Her birine bakın, çünkü hash sırası veya eşzamanlılık farkı tam olarak bir portta gerçekten değişebilecek şeydir.

## Kayıt orijinalini hatırlar

`record`, birimin dosyalarının bir parmak izini kaydeder (`recorded-from.json`). Orijinal daha sonra değişirse, orijinalin eskiden yaptığına karşı geçmek yerine, siz yeniden kaydedene kadar `diff` uyarır ve `gate` başarısız olur. Bkz. [Diff](/docs/concepts/diff#stale-recordings).

## TypeScript'e özgü ayrıntılar

`record`, TS kaynaklarını olduğu gibi çalıştırır: bir Node resolve hook'u bundler tarzı import'ları izler (`./errors`'tan `./errors.ts`'e), repoda hiçbiri kurulu değilse `--resolve-from` npm paketlerini sağlar ve dönüştürme gerektiren TS (enum'lar, parametre özellikleri) `--experimental-transform-types` ile yeniden denenir. Doğrudan Node'da çalışamayan TS, `--module dist/x.mjs` ile derleme çıktısından ya da `--node-arg=--import=tsx` ile kaydedilebilir.

## Sayısal kod

Float'ların nadiren bit bit eşleştiği kod için `--float-tolerance` ile kaydedin. Bkz. [Kayan nokta toleransı](/docs/concepts/float-tolerance).
