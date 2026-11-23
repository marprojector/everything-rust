---
title: Diff ve bilinen farklar
description: diff kaydedilen her vakayı Rust'a karşı yeniden oynatır. known-diffs.jsonl kasıtlı farkları kaydeder ve eskimiş kayıtlar gate'i başarısız kılar.
---

# Diff ve bilinen farklar

```sh
everything-rust build <crate>
everything-rust diff <crate>                     # N/N olmalı
everything-rust diff <crate> --json
```

`diff`, `cases.jsonl`'i derlenmiş Rust modülüne karşı, orijinali çağıranların çağırdığı şekilde yeniden oynatır ve uyuşmazlıkları listeler. Bir uyuşmazlık bir davranış farkıdır: kaydı değil, portu düzeltin.

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

## Bilinen farklar

Bazen portun farklı olması amaçlanır: kullanıcı, orijinaldeki bir hatayı düzeltmesini ister. Önce sorun, ardından her vaka için `<crate>/known-diffs.jsonl` dosyasına bir satır ekleyin:

**known-diffs.jsonl**
```json
{"fn": "...", "args": [...], "actual": ..., "reason": "..."}
```

`fn`, `args` ve `actual` değerlerini `diff --json`'dan kopyalayın. O vaka bundan sonra yalnızca tam olarak o sonuçla geçer ve artık farklı olmadığında `gate` başarısız olur; o zaman satırı kaldırın.

## Eskimiş kayıtlar

`record`, orijinal birimin dosyalarının bir parmak izini `recorded-from.json` içine kaydeder. Orijinal daha sonra değişirse, orijinalin eskiden yaptığına karşı geçmek yerine, siz yeniden kaydedene kadar `diff` uyarır ve `gate` başarısız olur. `gate` kaydın eskidiğini söylüyorsa, port etmeye devam etmeden önce yeniden kaydedin.

## Port tıkandığında

`diff`, derlemeler boyunca ilerlemeyi izler. Portun tıkandığını söylediğinde (`trend.stalled`: `port.rs`'e art arda yapılan üç değişiklik hiçbir yeni eşleşen vaka kazandırmadı), düzenlemeyi bırakın. Kalan uyuşmazlıklar yalnızca kullanıcının bildiği bir kural gerektirebilir ya da o fonksiyonlar şimdilik orijinalde kalabilir.

## Kayan nokta toleransı

Crate `--float-tolerance` ile kaydedildiyse `diff` onu kullanır; `diff` üzerindeki `--float-tolerance` bunu tek bir çalıştırma için geçersiz kılar. Bkz. [Kayan nokta toleransı](/docs/concepts/float-tolerance).
