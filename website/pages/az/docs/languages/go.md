---
title: Go
description: Go paketini orijinal API-li paketdə tipli cgo çağırışları və yaradılmış binary wire vasitəsilə Rust-a port edin.
---

# Go

| | |
| --- | --- |
| Vahid | paket |
| Körpü | orijinal API-li paketdə tipli cgo çağırışları və yaradılmış binary wire |
| Qeydə alan və təkrar oynadan | öz modulu kimi qurulan yaradılmış Go drayveri |
| Tələb olunur | Go 1.21+ |
| Vəziyyət | Go 1.24-ün `path` paketi üzərində yoxlanılıb: 2,642/2,642 (542-si Go-nun öz test cədvəllərindən), gate keçir |

`profile` `go test` vasitəsilə işləyir: `everything-rust profile <repo> -- go test -run X -bench . ./pkg`. Batch-lər `NewFnBatch(items)`-dir; işiniz bitəndə onları bağlayın.

## Portun düzgün etməli olduğu şeylər

- `path.Clean`: Go-nun `lazybuf`-ı geri çəkildikdən sonra yazma indeksindəki baytı oxuyur, son saxlanılan baytı yox (9 hal, Go-nun öz test cədvəlindən).
- `strings.ToLower` hər rune-u ayrıca kiçildir: `ΣΊΣΥΦΟΣ` `σίσυφοσ` olur, Rust-un `to_lowercase` funksiyası isə son sigma qaydasını tətbiq edir. Go həmçinin `İ`-ni `i`-yə çevirir, Rust isə `i̇` verir.
- Go string-ləri baytlarla kəsir; rune-un ortasından keçən kəsik JSON-a hər bayt üçün bir U+FFFD kimi çatır.

## Sürət

Go yarpaq funksiyaları bir neçə nanosaniyədə işlədir, tək bir cgo çağırışı isə təxminən 80 ns tutur. `path` üzərində `bench` portu çağırış başına 0.04x ölçdü: daha böyük vahidləri port edin. Batch kimi iş bütün nüvələrdə Rust-a köçür: 50,000 string üzərində `Slugify` Go-da 49 ms çəkdi, batch isə **4x daha sürətli** işlədi (çağırış başına port: 0.93x). Batch crate-ləri mimalloc ilə yaddaş ayırır: sistem allocator-u ilə Go batch-i bir thread-dən daha yavaş idi.

Təfərrüatlar: [Altı dil](/docs/real-world/six-languages).
