---
title: Go
description: Bir Go paketini, orijinal API'ye sahip bir pakette tipli cgo çağrıları ve üretilmiş bir binary wire üzerinden Rust'a port edin.
---

# Go

| | |
| --- | --- |
| Birim | bir paket |
| Köprü | orijinal API'ye sahip bir pakette tipli cgo çağrıları ve üretilmiş bir binary wire |
| Kaydeden ve yeniden oynatan | kendi modülü olarak derlenen, üretilmiş bir Go sürücüsü |
| Gerekenler | Go 1.21+ |
| Durum | Go 1.24'ün `path` paketi üzerinde doğrulandı: 2,642/2,642 (542'si Go'nun kendi test tablolarından), gate geçiyor |

`profile`, `go test` üzerinden çalışır: `everything-rust profile <repo> -- go test -run X -bench . ./pkg`. Batch'ler `NewFnBatch(items)` şeklindedir; işiniz bittiğinde onları kapatın.

## Bir portun doğru yapması gerekenler

- `path.Clean`: Go'nun `lazybuf`'ı, yazma indeksi geri gittikten sonra son tutulan baytı değil, o indeksteki baytı okur (Go'nun kendi test tablosundan 9 vaka).
- `strings.ToLower` her rune'u tek başına küçültür: `ΣΊΣΥΦΟΣ`, `σίσυφοσ` olur; Rust'ın `to_lowercase`'i ise son sigma kuralını uygular. Go ayrıca `İ`'yi `i`'ye eşler, Rust ise `i̇` verir.
- Go string'leri baytlara göre dilimler; bir rune'un ortasından kesmek JSON'a bayt başına bir U+FFFD olarak ulaşır.

## Hız

Go yaprak fonksiyonları birkaç nanosaniyede çalıştırır ve tek bir cgo çağrısı tek başına yaklaşık 80 ns tutar. `path` üzerinde `bench`, portu çağrı başına 0.04x ölçtü: daha büyük birimleri port edin. Batch olarak iş tüm çekirdeklerde Rust'a taşınır: 50,000 string üzerinde `Slugify` Go'da 49 ms sürdü, batch ise **4x daha hızlı** çalıştı (çağrı başına port: 0.93x). Batch crate'leri bellek ayırmayı mimalloc ile yapar: sistem ayırıcısıyla Go batch'i tek thread'den daha yavaştı.

Ayrıntılar: [Altı dil](/docs/real-world/six-languages).
