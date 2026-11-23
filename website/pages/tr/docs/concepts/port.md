---
title: Port
description: Sizin ya da ajanınızın yazdığı tek dosya olan port.rs'i yazmak ve onu orijinalle eşleşir tutan kurallar.
---

# Port

Portu siz ya da ajanınız `src/port.rs` içinde yazarsınız. Crate'teki diğer her şey üretilir. `port.rs`'te bir imzayı değiştirdikten sonra `everything-rust refresh <crate>` çalıştırın.

## Bir diff turunu kurtaran kurallar

- Kod yazmadan önce planın birim için listelediği her riski kendi `file:line` konumunda okuyun.
- **String ofsetleri**: JS ve Java UTF-16 kod birimleriyle, Rust UTF-8 baytlarıyla indeksler. Üretilen `src/utf16.rs`'i kullanın.
- **Hatalar**: `Err("TypeName: message")` döndürün. Bu Python, Ruby ve Java'da o exception'a, Go'da ise `error` metnine dönüşür. Bir Rust panic'i asla kaydedilmiş bir hatayla eşleşmez.
- **Python ve Ruby'de sayılar**: orijinalin olduğu gibi geçirdiği int'ler int olarak geri dönmelidir. Python portlarında int-veya-float için `serde_json::Value` değil, `crate::pynum::Num` kullanın.
- **C**: `crash:` olarak kaydedilen girdiler tanımsız davranıştır; her sonuç geçer.
- **Sıralı map'ler ve nesne anahtarları**: sıra çıktıya yansıyorsa `indexmap`.
- **Regex lookaround veya geri başvurular**: `regex` değil, `fancy-regex`. Her iki durumda da `\w`, `\b`, `\d` ve büyük/küçük harf duyarsız eşleşme, ASCII olmayan metni kaynak dilin motorundan farklı ele alır. Orijinalin `\w`'si yalnızca ASCII olduğunda (`u` olmadan JS, Java) `unicode(false)` ayarlayın ya da ASCII sınıflarını açıkça yazın (`[A-Za-z0-9_]`).
- **`unsafe` yok.** Gerçekten gerekiyorsa: en az düzeyde, yalıtılmış ve bir `// SAFETY:` yorumuyla.
- **Yardımcıları private tutun.** Yalnızca birimin export'ları `pub`'dır; `pub` bir yardımcı köprülenen API'nin parçası olur.
- Üzerinde akıl yürüttüğünüz vakalar için `port.rs` içine Rust birim testleri ekleyin.

Üretilen `js.rs`, Rust'ın farklı davrandığı yerlerde JavaScript'i yeniden üreten yardımcılar taşır; örneğin `js::number_to_string` (JS `String(n)`) ve `js::math_round` (`Math.round`).

## Bir örnek: bir konum kaydıran emoji

[x18n](https://github.com/x18n-dev/x18n)'den `packages/core/src/fuzzy.ts`, [`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo)
içinde iki şekilde port edildi:

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: hazır, wasm-bindgen (core tarayıcılarda çalışır)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# src/port.rs'i yazın
everything-rust gate demo/fuzzy-rs
```

Rust `char` ile indeksleyen ilk port basit testleri geçti ama `diff`'te kaldı:

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

JS `indexOf` UTF-16 kod birimlerini sayar, bu yüzden bir emoji iki konum tutar. `demo/*/src/port.rs`
içindeki port üretilen `utf16` yardımcılarını kullanır ve hem napi-rs (`demo/fuzzy-napi`) hem de
wasm-bindgen (`demo/fuzzy-rs`) olarak 2000/2000 vakayı ve gate'i geçer.

everything-rust'in x18n'in tamamı için yazdığı plan: [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md).

## Ne kadar büyük olmalı?

`gate`, portun satırlarını orijinalinkilerle karşılaştırır ve 3x'in üzerinde başarısız olur (import'lar ve imzalar için artı 50 satır). Bunun üzeri genellikle bir crate'in zaten yaptığını elle yazmak ya da vakaları tek tek ele almak demektir. `analyze` kullanılacak crate'leri listeler.
