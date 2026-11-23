---
title: Kaydetme ve yeniden oynatma
description: app record uygulamanın tamamını belgelenmiş örnekleri, sizin girdileriniz ve tuzaklarla çalıştırır; app replay herhangi bir derlemeyi vakalara karşı çalıştırır.
---

# Kaydetme ve yeniden oynatma

`record` ve `diff` tek bir birimi kanıtlar; `app record` ve `app replay` ise altı dilin herhangi birindeki CLI'lar ve sunucular için uygulamayı kanıtlar. Uygulama bir kara kutudur.

```sh
everything-rust app record . -- python3 cli.py          # orijinali çalıştıran komut
everything-rust app replay .                            # orijinal %100 geçmeli
everything-rust app replay . --build "target/release/mytool"
```

## Vakalar nereden gelir

`app record` orijinali şunlarla çalıştırır:

- uygulamanın kendi belgelenmiş örnekleri: Markdown ve shell dosyalarındaki `$ mytool ...` satırları, adını verdikleri dosyalarla birlikte;
- satır başına bir `{"name", "args", "stdin", "env", "files"}` içeren bir `--inputs` JSONL dosyası;
- bu vakalardan türetilen anlamsal tuzaklar: ASCII olmayan ve emoji içeren string'ler, negatif bölme ve mod alma, taşma, çok büyük ve çok küçük float'lar, map sırası, eksik dosyalar.

Her vaka iki kez çalışır; iki çalıştırma arasında farklı olan şey bir `[..]` jokeri olur. `--normalize timestamps|uuids|key-order` ya da `--normalize name=regex`, vakalarla birlikte kaydedilen normalleştiriciler ekler. `app record` farklı olanları türüne göre raporlar: değerler (artık joker), sıra (satırlar herhangi bir sırada karşılaştırılır) ve satırlar (her şeyle eşleşen bir `...`). Her birine bakın.

## Vakalar düz dosyalardır

Vakalar `rust/app-cases/` içindeki [trycmd](https://docs.rs/trycmd) dosyalarıdır:

| Dosya | İçerik |
| --- | --- |
| `<case>.toml` | argv, ortam, çıkış kodu |
| `<case>.stdin` | standart girdi |
| `<case>.stdout`, `<case>.stderr` | orijinalin yazdırdıkları |
| `<case>.in/` | giren dosyalar |
| `<case>.out/` | yazılan dosyalar |

Onları elle okuyup düzenleyin; port bir binary'ye sahip olduğunda bir Rust testi onları `trycmd` ile çalıştırabilir.

## Yeniden oynatma

- `app replay` herhangi bir derlemeyi vakalara karşı çalıştırır, `passed/total` yazdırır, başarısız her vaka için bir diff içeren `rust/app-cases/replay-report.md` dosyasını yazar ve herhangi bir uyuşmazlıkta sıfır olmayan bir kodla çıkar.
- [Front door](/docs/whole-apps/front-door)'un yaptığı gibi `EVERYTHING_RUST_FALLBACK_LOG` ile adı verilen dosyaya ekleme yapan (orijinale her fallback için bir satır) bir derlemenin fallback oranı da raporlanır.
- `app replay --case <name>`, raporun verdiği adla tek bir vakayı yeniden çalıştırır: her şeyi yeniden çalıştırmadan önce bir uyuşmazlığı düzeltmek için hızlı, tekrarlanabilir döngü.

## CI'da yeniden oynatma

Raporda süre ölçümleri ya da geçici yollar yoktur: aynı derlemenin iki yeniden oynatması aynı dosyayı yazar. Onu commit'leyin, ardından CI'da çalıştırın:

```sh
everything-rust app replay .
git diff --exit-code rust/app-cases/replay-report.md
```

Yarı port edilmiş bir uygulama böylece, rapor buna göre güncellenene kadar, geçenlerdeki her değişiklikte (daha iyi ya da daha kötü) CI'da başarısız olur.

## Sayısal çıktı

`app record --float-tolerance 1e-9`, sayıların büyük değerin bu oranı içinde eşleşmesine izin verir. Bkz. [Kayan nokta toleransı](/docs/concepts/float-tolerance).
