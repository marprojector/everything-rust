---
title: Lift
description: app lift, yeniden oynatma fallback oranı 0 ile geçtiğinde ve her birim gate'i geçtiğinde rust/app'i saf bir Rust uygulamasına dönüştürür.
---

# Lift

Yeniden oynatma fallback oranı 0 ile geçtiğinde ve her birim gate'i geçtiğinde, onu saf bir Rust uygulamasına dönüştürün:

```sh
everything-rust app lift .
```

- `app lift` ikisi de sağlanana kadar reddeder ve neyin engellediğini söyler: hâlâ fallback'e düşen vakalar, hâlâ stub olan yapıştırıcı kod, gate'i geçemeyen birimler.
- Önce bir kopya üzerinde fallback yolunu ve kill switch'leri kaldırır, her birim crate'ini `rust/app`'in düz bir kütüphanesine dönüştürür (`cdylib`, binding'ler ya da üretilmiş wrapper'lar olmadan) ve `cargo test` sırasında `rust/app-cases`'i trycmd ile çalıştıran `rust/app/tests/app_cases.rs`'i ekler; böylece CI'ın cargo dışında hiçbir şeye ihtiyacı kalmaz.
- Ardından lift kontrol listesini çalıştırır: her birimde ve `rust/app` üzerinde fmt, clippy `-D warnings` ve testler; `rust/DECISIONS.md` içinde adı geçen her hedefin kurulu olduğu yerde derlenmesi ya da kontrol edilmesi; çalışma zamanında okunan dosyaların ve gömülmemiş varlık klasörlerinin kontrolü; ve saf binary üzerinde yeniden oynatma. Değişiklikler ancak bunların hepsi geçtiğinde geri yazılır. Orijinale asla dokunulmaz.

## Lift'ten sonra

Birimlerin artık binding'i yoktur. json-server'da her crate'in `tests/recorded.rs` dosyası kayıtlı vakalarını port üzerinde yeniden oynatır: 514 HTTP vakasının yanında 3,399 vakanın tamamı `cargo test` ile çalışır.

Orijinal gittikten sonra Rust'ı özgürce yeniden yapılandırın; `app replay` davranışı hâlâ kanıtlar.

## Sonra geçişi yapın

İş akışı skill'i, ajanın yapabildiğini yapmasını ve yalnızca sizin yapabileceğiniz adımlar (deploy'lar, DNS, secret'lar) için her adımda geri dönüş yolu olan etkileşimli bir betik olan `rust/cutover.sh`'i yazmasını sağlar.
