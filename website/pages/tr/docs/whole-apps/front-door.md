---
title: Front door ve wire
description: app front-door orijinalin önüne bir Rust binary'si koyar; app wire, bir birimin portu gate'i geçtiğinde komutları ona devreder.
---

# Front door ve wire

Önce giriş noktasını Rust'a taşıyın, ardından komutları ona birim birim devredin:

```sh
everything-rust app front-door .                            # rust/app: her komut orijinali çalıştırır
everything-rust app wire . textkit --command upper          # textkit-rs gate'i geçtikten sonra
# yapıştırıcı kodu rust/app/src/units/textkit.rs içine yazın, ardından:
everything-rust app wire . textkit                          # yine %100, daha düşük fallback oranı
```

## Front door

`app front-door`, uygulamanın adını taşıyan bir Rust binary'si olan `rust/app`'i iskeletler: kayıtlı vakalardaki komutlar ve bayraklar için bir clap ayrıştırıcısı; burada her komut orijinali aynı argv, stdin ve ortamla çalıştırır ve çıktısını ve çıkış kodunu olduğu gibi aktarır. Her fallback sayılır, bu yüzden yeni front door'u yeniden oynatmak %100 fallback ile %100 geçiş verir (bunu göstermek için derler ve yeniden oynatır). `rust/app`'in üzerine asla yazmaz: bundan sonra kod sizindir.

## Bir birimi bağlamak

`app wire <repo> <unit> --command <name>`:

1. birim crate'i üzerinde `gate` çalıştırır (az önce geçtiyse `--skip-gate`);
2. onu `rust/app`'e bir kütüphane olarak ekler;
3. sizin ya da ajanınızın dolduracağı `src/units/<unit>.rs` dosyasını bir kez yazar: argümanları ayrıştırın, `<crate>::port`'u çağırın, orijinalin yazdırdığını yazdırın;
4. komutları ona yönlendirir, ardından derler ve yeniden oynatır.

Yapıştırıcı kodu yazdıktan sonra yeniden çalıştırın: %100 geçiş ve daha düşük bir fallback oranı. `EVERYTHING_RUST_UNIT_<NAME>=original`, bağlanmış bir birimi lift'e kadar orijinale geri gönderir.

Hiçbir komut adlandırmayan argv için (argüman yok, bilinmeyen komutlar, kullanım hataları) uygulamanın kendi adını bir komut olarak bağlayın (`--command mytool`).

## Hiçbir şey orijinale geri çağrı yapmaz

Front door orijinali ayrı bir süreç olarak çalıştırır (ya da HTTP'yi ona iletir), köprülenmiş birimler orijinal tarafından çağrılır, asla tersi değil, ve callback'ler kaynak dilde kalır.

Sunucular komutlar yerine route'lar kullanır: bkz. [Sunucular](/docs/whole-apps/servers). Kütüphaneler public API'lerini kullanır: bkz. [Kütüphaneler](/docs/whole-apps/libraries).
