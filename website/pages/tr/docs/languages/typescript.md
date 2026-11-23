---
title: TypeScript/JavaScript
description: Node.js kodunu (TypeScript veya JavaScript), orijinal API korunarak napi-rs ya da wasm-bindgen üzerinden Rust'a port edin.
---

# TypeScript/JavaScript

| | |
| --- | --- |
| Birim | bir klasör ya da dosya |
| Köprü | üretilmiş bir binary wire ile napi-rs veya wasm-bindgen; sınıflar ve async dahil |
| Kaydeden ve yeniden oynatan | `.ts` kaynaklarını çalıştıran Node |
| Gerekenler | Node 22.6+ (`profile` için Node 24+ gerekir) |
| Durum | Doğrulandı |

## Köprüyü seçmek

`analyze`, birim veya import ettiği herhangi bir şey Node'a ihtiyaç duyuyorsa napi-rs'i, taşınabilir kod için wasm-bindgen'i (tarayıcılarda da çalışır) seçer ve UI ile editör kodunu JS'te tutar. `bridge --kind napi|wasm` bunu geçersiz kılar.

Bir köprüyü geçemeyen export'lar gerekçesiyle birlikte TS'te kalır ve shadow modülü orijinali yeniden export eder: fonksiyon alan veya döndüren fonksiyonlar, generator'lar ve iterator'lar, RegExp, `instanceof` kontrolleri, generic'ler (`--as-json T` uygulamanın yalnızca veriyle doldurduğu birini köprüler) ve tipleri fonksiyon içeren değerler. Bkz. [Bridge](/docs/concepts/bridge).

## TS'i olduğu gibi kaydetmek

`record`, TS kaynaklarını doğrudan çalıştırır: bir Node resolve hook'u bundler tarzı import'ları izler, repoda hiçbiri kurulu değilse `--resolve-from` npm paketlerini sağlar ve dönüştürme gerektiren TS (enum'lar, parametre özellikleri) `--experimental-transform-types` ile yeniden denenir. `record --from-tests -- npx jest`, herhangi bir runner ile orijinalin kendi test takımının yaptığı her çağrıyı kaydeder. Orijinalin kapsamı TS/JS için ölçülür ([Kapsam](/docs/concepts/coverage)).

## Bir portun doğru yapması gerekenler

Gerçek kod üzerindeki kayıtlı vakalarla bulunanlar:

- `str.length`, `indexOf` ve `slice` UTF-16 kod birimlerini sayar, bu yüzden bir emoji iki konum tutar. Üretilen `utf16` yardımcılarını kullanın.
- `u` bayrağı olmadan `/.../i` yalnızca ASCII harflerini katlar, Rust'ın `regex`'i ise Unicode'u katlar; bu yüzden vercel/ms'te `"1 ſ"` (uzun s) ve Kelvin işareti saniye olarak ayrıştırılmış olurdu.
- `Math.round` yarımları yukarı yuvarlar (`-2.5`, `-2` verir); Rust'ın `round`'u sıfırdan uzağa yuvarlar. Üretilen `js.rs` içinde `js::math_round` vardır.
- `${n}`, `1e+21`, `5e-324` ve `-0` için `0` yazdırır. `js::number_to_string` bunu yeniden üretir.
- `String.prototype.trim`, Rust'ın `char::is_whitespace`'ini değil, JS boşluk karakterlerini kullanır.

## Hız

Çağrı başına bir port her seferinde köprünün bedelini öder ve V8 küçük fonksiyonları nanosaniyeler içinde çalıştırır. vercel/ms, kayıtlı vakaları üzerinde çağrı başına 0.56x hızında çalıştı; x18n çekirdeğinin portları JS'ten çağrıldığı haliyle TS'ten daha yavaştır. Batch olarak döngü Rust'a taşınır:

| İş yükü | Orijinal | everything-rust ile |
| --- | ---: | ---: |
| x18n'in kenar çubuğu araması, 50,000 anahtar (Node) | sorgu başına 10 ms | 0.5 ms: 20x daha hızlı |
| Aynı arama tarayıcıda (wasm, tek thread) | 10 ms | 0.95 ms: 11.5x |
| Fuse.js, 50,000 string | 91.2 ms | 2.8 ms: 32x |
| vercel/ms `parse` x100,000 | 11.8 ms | 0.65 ms: 18x (devir teslim dahil 3.4x) |
| vercel/ms `format` x100,000 | 2.1 ms | devir teslim ile 2.5 ms: 0.8x |

Vaka çalışmaları: [json-server](/docs/real-world/json-server), [Fuse.js](/docs/real-world/fuse-js), [vercel/ms](/docs/real-world/vercel-ms), [x18n](/docs/real-world/x18n).
