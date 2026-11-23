---
title: Go
description: Go paketini tipli cgo çagyryşlary we döredilen binar wire arkaly, asyl API-li paketde Rust-a geçiriň.
---

# Go

| | |
| --- | --- |
| Birlik | paket |
| Bridge | tipli cgo çagyryşlary we döredilen binar wire, asyl API-li paketde |
| Kim ýazga alýar we gaýtadan işledýär | döredilen Go draýweri, aýratyn modul hökmünde gurulýar |
| Näme gerek | Go 1.21+ |
| Ýagdaý | Go 1.24-iň `path` paketinde barlandy: 2,642/2,642 (542-si Go-nyň öz test tablisalaryndan), gate geçýär |

`profile` `go test` arkaly işleýär: `everything-rust profile <repo> -- go test -run X -bench . ./pkg`. Batch-lar `NewFnBatch(items)`; iş gutarandan soň olary ýapyň.

## Portuň dogry etmeli zatlary

- `path.Clean`: Go-nyň `lazybuf`-y yza süýşenden soň ýazuw indeksindäki baýty okaýar, soňky saklanan baýty däl (9 ýagdaý, Go-nyň öz test tablisasyndan).
- `strings.ToLower` her rune-y aýratynlykda kiçeldýär: `ΣΊΣΥΦΟΣ` `σίσυφοσ` bolýar, Rust-yň `to_lowercase` funksiýasy bolsa soňky sigma düzgünini ulanýar. Go şeýle hem `İ`-ni `i`-ä öwürýär, Rust bolsa `i̇` berýär.
- Go setirleri baýtlar boýunça kesýär; rune-yň ortasyndan geçýän kesik JSON-a her baýt üçin bir U+FFFD bolup ýetýär.

## Tizlik

Go ýaprak funksiýalaryny birnäçe nanosekuntda işledýär, ýeke cgo çagyryşynyň özi takmynan 80 ns durýar. `path`-da `bench` portuň bir çagyryşda 0.04x bolandygyny ölçedi: has uly birlikleri geçiriň. Batch hökmünde iş ähli ýadrolarda Rust-a göçýär: 50,000 setir boýunça `Slugify` Go-da 49 ms aldy, batch bolsa **4x has çalt** işledi (her çagyryş porty: 0.93x). Batch crate-leri ýady mimalloc bilen bölüp berýär: ulgam allocator-y bilen Go batch-y bir akymdan hem haýal boldy.

Jikme-jiklikler: [Alty dil](/docs/real-world/six-languages).
