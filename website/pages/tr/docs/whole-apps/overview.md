---
title: Tüm uygulamayı port etmek
description: İlk kayıttan saf bir Rust binary'sine. Hedef, orijinal referans olarak tutulurken orijinal gibi davranan bir Rust uygulamasıdır.
---

# Tüm uygulamayı port etmek

Hedef, orijinal referans olarak tutulurken orijinal gibi davranan saf bir Rust uygulamasıdır. Rust'ı ajanınız yazar; everything-rust işi sıralar, iskeletini kurar ve her adımı kanıtlar. CLI'lar, HTTP sunucuları ve kütüphaneler.

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in    pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down proved by
 decisions to make      outside         original answers                                           replay
```

## 1. Plan

```sh
everything-rust analyze . --whole-app
```

`--whole-app`, uygulamanın yapısını (CLI, sunucu veya kütüphane) ve giriş noktalarını, Rust crate'i ve uyumuyla birlikte her dış bağımlılığı (import edilen ya da bildirilen), parçalar halinde port sırasını (birbirini import eden birimler birlikte taşınır) ve önerilen bir yanıtla numaralandırılmış kararları ekler: tutulacak birimler, engellenen birimler, crate'i olmayan ya da kısmi olan bağımlılıklar.

## 2. Uygulamanın tamamını kaydedin, sonra giriş noktasını taşıyın

1. [`app record` ve `app replay`](/docs/whole-apps/record-replay) programı dışarıdan, bir kara kutu olarak kanıtlar.
2. [`app front-door`](/docs/whole-apps/front-door) orijinalin önüne bir Rust binary'si koyar; her komut hâlâ orijinali çalıştırır.
3. Her birim [döngüden](/docs/concepts/the-loop) geçer, ardından `app wire` onun komutlarını Rust'a devreder.
4. [`app lift`](/docs/whole-apps/lift), yeniden oynatma fallback oranı 0 ile geçtiğinde ve her birim gate'i geçtiğinde orijinali kaldırır.

[Sunucular](/docs/whole-apps/servers) ve [kütüphaneler](/docs/whole-apps/libraries) aynı komutları kullanır.

## Ajan bunu nasıl yürütür

[whole-app skill'i](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md), yalnızca sizin verebileceğiniz kararlardan başlayarak ajanı adım adım yönlendirir:

- **Sorgulama (grilling)**: ajan, portun bağlı olduğu her karar netleşene kadar, her soruyu önerilen yanıtıyla birlikte turlar halinde sorar ve her birini `rust/DECISIONS.md` içinde bir ADR olarak yazar; `rust/GLOSSARY.md` ise orijinalin terimlerini Rust adlarına eşler.
- **Araştırma**: arka plan ajanları her bağımlılığın birincil kaynaklarını okur ve buldukları her anlamsal tuzakla birlikte `rust/research/<topic>.md` dosyasını yazar.
- **Oturumlar arası**: `rust/MAP.md` her birimi durumuyla listeler; bir oturum başlamadan önce bir birimi üstlenir ve her kontrol noktası bir sonraki oturum için bir `rust/HANDOFF.md` bırakır. Aynı dalgadaki birimler alt ajanlar tarafından paralel olarak port edilebilir.
- **Geçiş (cutover)**: ajan yapabildiğini yapar ve yalnızca sizin yapabileceğiniz adımlar (deploy'lar, DNS, secret'lar) için her adımda geri dönüş yolu olan etkileşimli bir betik olan `rust/cutover.sh`'i yazar.

Sorgulama, araştırma ve bu uygulamalar [Matt Pocock'un skill'lerinden](https://github.com/mattpocock/skills) uyarlanmıştır.

## Bu, Rust'ı eski tasarıma kilitlemez mi?

Yalnızca ikisi yan yana çalıştığı sürece. Kayıtlar davranışı uygulamanın iç yapısında değil, kenarlarında kontrol eder (bir CLI'ın çıktısı, bir sunucunun yanıtları, bir modülün export edilen fonksiyonları). Birimler hâlâ köprüyü geçerken eski yapıyı yansıtın; `app lift` orijinali kaldırdıktan sonra Rust'ı özgürce yeniden yapılandırın, `app replay` davranışı hâlâ kanıtlar.

## Gerçek bir örnek

[json-server](/docs/real-world/json-server) her adımdan geçti: 514 kayıtlı HTTP vakası, %99.8 fallback ile bir front door, önce bağlanan okumalar (%22.6 fallback), ardından her route (%0) ve 514/514 geçen saf bir Rust binary'sine lift.
