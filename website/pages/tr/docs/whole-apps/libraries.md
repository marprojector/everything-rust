---
title: Kütüphaneler
description: Bir kütüphanenin yeniden oynatılacak bir süreci yoktur. Public API'si front door'udur ve her birimin kayıtlı vakaları kanıttır.
---

# Kütüphaneler

Bir kütüphanenin yeniden oynatılacak bir süreci yoktur: public API'si front door'udur ve her birimin
kayıtlı vakaları kanıttır.

```sh
everything-rust app front-door .        # rust/app: public API'ye sahip bir Rust crate'i
# her birim döngüden geçer: bridge, record, port.rs, build, diff, gate; ardından:
everything-rust app wire . textkit-rs   # rust/app'ten yeniden export edilir; orijinal onun shadow'unu import eder
everything-rust app replay .            # bağlanmış her birimin vakaları; fallback = hâlâ orijinalde olan export'lar
everything-rust app lift .
```

- `app front-door`, depoda `main` ve sunucu olmadığında public API'yi (her birimin export'ları)
  bulur (aksi halde `--library`) ve `rust/app`'i bir kütüphane crate'i olarak iskeletler. Bağlanan her
  birim onun bir modülü olur: `pub use textkit_rs::port as textkit;`.
- `app wire`, `bridge` ile yapılmış bir crate alır, `gate` çalıştırır ve birimin shadow drop-in'ini yazar:
  orijinal paket birim yerine onu import eder, böylece çağıranlar orijinal API'yi korur ve Rust'a
  ulaşır. `EVERYTHING_RUST_UNIT_<NAME>=original` birimi geri gönderir (C: shadow'un bayrakları olmadan derleyin).
- `app replay`, bağlanmış her birimin kayıtlı vakalarını binding'leri üzerinden çalıştırır. Fallback oranı,
  Rust'ın henüz yanıtlamadığı public export'ların oranıdır: bağlanmamış birimler ve `bridge`'in
  orijinal dilde tuttuğu export'lar.
- `app lift`, fallback'in 0 olmasını ve her birimin `gate`'ten geçmesini ister; birimlerden binding'leri,
  wrapper'ları ve shadow'ları çıkarır ve `rust/app`'i tek bir saf Rust kütüphanesi olarak bırakır. Hiçbir
  `port.rs`'e dokunmaz, bu yüzden binding'ler üzerinden eşleşen vakalar onu hâlâ tanımlar; kontrol
  listesi, orijinalde hâlâ shadow'ları import eden dosyalar için uyarır.

Beşi de aynı zamanda MCP aracıdır: `app_record`, `app_replay`, `app_front_door`, `app_wire` ve
`app_lift`.
