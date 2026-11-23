---
title: PHP (önizleme)
description: PHP önizlemededir. analyze bir PHP uygulamasını planlar ve bir PHP CLI'ı ya da sunucusu kaydedilebilir, yeniden oynatılabilir ve bir Rust front door'unun arkasına konabilir.
---

# PHP (önizleme)

| | |
| --- | --- |
| Birim | fonksiyonlar ve `public static` metotlardan oluşan bir klasör |
| Köprü | sırada: C ABI üzerinden PHP FFI |
| Bugün | tüm uygulamalar: CLI'lar ve sunucular (`php -S`) için `app record`, `app replay` ve front door |
| Durum | Önizleme: yalnızca `analyze` ve tüm uygulamalar; birimler henüz köprülenemez |

`analyze` bir PHP uygulamasını planlar: Composer paketleri ve PSR-4 autoloading, gevşek karşılaştırma ve tam sayı taşmasının float'a dönüşmesi gibi riskler ve crate'lere eşlenmiş Composer paketleri. Tipler PHP tiplerinden ve phpdoc'tan gelir.

Bir PHP CLI'ı ya da sunucusu kaydedilebilir, yeniden oynatılabilir ve bir Rust front door'unun arkasına konabilir:

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

Birimlerini PHP'nin FFI'ı üzerinden Rust'a port etmek bir sonraki adımdır ([issue #2](https://github.com/marprojector/everything-rust/issues/2)). O zamana kadar `bridge`, PHP birimlerini açık bir mesajla reddeder.

CI PHP kurmaz, bu yüzden PHP testleri yalnızca PHP kurulu bir makinede çalışır.
