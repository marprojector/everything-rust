---
title: Gate
description: gate, bir port için "bitti"nin tanımıdır. Çalıştırdığı her kontrol ve başarısız olanın nasıl düzeltileceği.
---

# Gate

```sh
everything-rust gate <crate>
```

Bitti demek, `gate`'in geçmesi demektir. Şunları kontrol eder:

- `cargo fmt`, `-D warnings` ile clippy ve cargo testleri;
- `unsafe` yalnızca `// SAFETY:` yorumlarıyla ve hiçbir şeyin `unimplemented!` olmaması;
- orijinalin bilerek başarısız olduğu her yer için (`throw`, `raise`, bir `error` sonucu) kaydedilmiş başarısız bir vaka. gate birini adlandırdığında `record --inputs` ile ekleyin;
- orijinali hata döndüren bir fonksiyonda `unwrap`, `expect` veya `panic!` olmaması. `Err` döndürün; orijinalin de imkânsız saydığı bir durum için `// INFALLIBLE: <why>` yorumu ekleyin;
- portun en fazla orijinalin satır sayısının 3 katı olması (import'lar ve imzalar için artı 50 satır);
- modülün derlenmesi ve her export'un mevcut olması;
- [bilinen farklar](/docs/concepts/diff#known-diffs) dahil kaydedilen her vakanın eşleşmesi ve kaydın [eskimiş](/docs/concepts/diff#stale-recordings) olmaması;
- TS/JS için bir [kapsam](/docs/concepts/coverage) satırı: vakaların orijinalin ne kadarını çalıştırdığı.

Başarısız her kontrol bir `fix` taşır: önce onu çalıştırın.

## Sonra raporlayın ve bekleyin

Gate geçtiğinde iş akışı skill'i, ajanın neyin değiştiğini (dosyalar, fonksiyonlar, köprü, batch'ler), gate çıktısını, diff sayılarını ve bench rakamlarını, kalan davranış farklarını veya riskleri ve planda neyin kaldığını raporlamasını ve bir sonraki birimden önce onay beklemesini sağlar.
