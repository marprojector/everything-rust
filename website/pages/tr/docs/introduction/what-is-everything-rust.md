---
title: everything-rust nedir
description: everything-rust uygulamanızın bugün ne yaptığını kaydeder ve Rust ile yeniden yazılan her parçayı, uygulama tamamen Rust olana kadar bu kayda karşı kontrol eder.
---

# everything-rust nedir

<p class="lead">Uygulamanızın tamamını Rust ile, parça parça yeniden yazın. everything-rust, Node.js (TypeScript/JavaScript), Python, C, Go, Ruby veya Java uygulamanızın bugün ne yaptığını kaydeder, ardından Rust ile yeniden yazılan her parçayı bir sonrakine geçmeden önce bu kayda karşı kontrol eder; ta ki uygulama tamamen Rust olana kadar.</p>

CLI'lar, HTTP sunucuları ve kütüphaneler üzerinde çalışır. Rust'ı kodlama ajanınız (Claude Code, Codex, Cursor, Gemini CLI) ya da siz yazarsınız. PHP ve C# [önizlemededir](/docs/languages/php).

## Ne yapar, ne yapmaz

everything-rust kodu çevirmez. Bir ajanın güvenilir biçimde tahmin edemeyeceği şeyleri sağlar:

- **zamanın nereye gittiği**: `profile` gerçek bir iş yükünü çalıştırır ve sıcak fonksiyonları sıralar;
- **önce neyin ve hangi sırayla port edileceği**: `analyze` birimleri, dalgaları, riskleri ve crate'leri planlar;
- kodun geri kalanının Rust'ı API'si değişmeden çağırabilmesi için **bir köprü** (`bridge`);
- **portun eşleşmesi gereken davranışın tam hali**: `record` orijinali uç durumlarla çalıştırır ve döndürdüğünü ya da fırlattığını kaydeder;
- **onu yayına almanın güvenli bir yolu**: `shadow` ikisini yan yana çalıştırır;
- **bir portun ne zaman bittiğini söyleyen bir gate**: `gate`.

Doğruluğun kaynağı orijinal kod olarak kalır. `diff`, kaydedilen her vakayı Rust'a karşı yeniden oynatır; hepsi eşleşip `gate` geçene kadar hiçbir şey yayınlanmaz.

## Dört adım

| Adım | Ne olur |
| --- | --- |
| **Kaydet** | `record`, orijinali üretilmiş uç durumlar, anlamsal tuzaklar ve sizin girdilerinizle çalıştırır; döndürdüğünü ya da fırlattığını kaydeder. |
| **Port et** | `bridge`, Rust crate'ini ve bir wrapper üretir; böylece çağıranlar orijinal API'yi kullanmaya devam eder. Rust'ı siz ya da ajanınız yazar. |
| **Kontrol et** | `diff`, kaydedilen her vakayı Rust'a karşı yeniden oynatır; hepsi eşleşip `gate` geçene kadar hiçbir şey yayınlanmaz. |
| **Lift** | Yeniden oynatma fallback olmadan geçtiğinde ve her birim gate'i geçtiğinde, `app lift` onu saf bir Rust uygulamasına dönüştürür. Orijinale asla dokunulmaz. |

## Rust her zaman daha mı hızlı?

Hayır, ve everything-rust bunu söyler. Her öğe için bir kez çağrılan küçük bir fonksiyon Rust'ta çoğu zaman daha yavaştır, çünkü her çağrı bir köprüyü geçer; V8, JVM ve Go bu tür fonksiyonları nanosaniyeler içinde çalıştırır. `profile` onu çevreleyen döngüyü bulur, `--batch` döngünün tamamını taşır ve `bench` siz yayına almadan önce bir karar verir. Yavaş olanlar dahil ölçülmüş sayılar için bkz. [Batch'ler ve hot path'ler](/docs/concepts/batches).

## Yalnızca kaydın kapsadığını yakalayabilir

Geçmek, Rust'ın kaydedilen her vakayla eşleştiği anlamına gelir; fazlası değil. Yeşil bir gate'e güvenmeden önce [Neyi yakalayamaz](/docs/concepts/limits) sayfasını okuyun.

::: warning Bir kopya üzerinde çalıştırın
`record` ve `app record` orijinal kodunuzu çalıştırır; her fonksiyonu (ya da uygulamanın tamamını) çok sayıda üretilmiş girdiyle çağırır ve `app record` her vakayı iki kez çalıştırır. `diff`, `gate` ve `app replay` aynı vakaları port ya da orijinal üzerinde yeniden çalıştırır. Dosya yazan, bir veritabanıyla konuşan veya ağa çağrı yapan kod bunu her vaka için bir kez yapar. Bunları projenin bir kopyası üzerinde, bir test veritabanıyla ve ortamda hiçbir production kimlik bilgisi olmadan çalıştırın.
:::

## Sonraki adımlar

- [Hızlı başlangıç](/docs/introduction/quick-start): kurun ve ajanınızdan ilk portu isteyin.
- [Döngü](/docs/concepts/the-loop): tek bir birim için her adım.
- [Tüm uygulamayı port etmek](/docs/whole-apps/overview): ilk kayıttan saf bir Rust binary'sine.
- [Gerçek dünya portları](/docs/real-world/overview): json-server, Fuse.js, vercel/ms ve daha fazlası, yavaş sonuçlar hızlıların yanında.
