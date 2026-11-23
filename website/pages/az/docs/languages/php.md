---
title: PHP (önizləmə)
description: PHP önizləmədədir. analyze PHP tətbiqini planlaşdırır, PHP CLI və ya serveri isə qeydə alına, təkrar oynadıla və Rust front door-unun arxasına qoyula bilər.
---

# PHP (önizləmə)

| | |
| --- | --- |
| Vahid | funksiyalardan və `public static` metodlardan ibarət qovluq |
| Körpü | növbədə: C ABI üzərindən PHP FFI |
| Bu gün | bütöv tətbiqlər: CLI-lar və serverlər (`php -S`) üçün `app record`, `app replay` və front door |
| Vəziyyət | Önizləmə: yalnız `analyze` və bütöv tətbiqlər; vahidlər hələ körpü ilə bağlana bilmir |

`analyze` PHP tətbiqini planlaşdırır: Composer paketləri və PSR-4 autoloading, sərbəst müqayisə və tam ədədin float-a daşması kimi risklər, crate-lərə uyğunlaşdırılmış Composer paketləri. Tiplər PHP tiplərindən və phpdoc-dan gəlir.

PHP CLI və ya serveri qeydə alına, təkrar oynadıla və Rust front door-unun arxasına qoyula bilər:

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

Onun vahidlərini PHP-nin FFI-ı vasitəsilə Rust-a port etmək növbəti addımdır ([issue #2](https://github.com/marprojector/everything-rust/issues/2)). O vaxta qədər `bridge` PHP vahidlərini aydın mesajla rədd edir.

CI PHP quraşdırmır, buna görə PHP testləri yalnız PHP olan maşında işləyir.
