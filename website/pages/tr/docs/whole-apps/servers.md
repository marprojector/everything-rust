---
title: Sunucular
description: Aynı komutlar HTTP sunucularını da kanıtlar. --http, --fresh, --arg ve --pace ile hiçbir birimin yanıtlamadığı her route'u proxy'leyen bir axum front door.
---

# Sunucular

Aynı komutlar HTTP sunucularını da kanıtlar. `--http`, komutun bir sunucu başlattığını söyler; sunucu
portunu `PORT`'tan ya da komuttaki `{port}`'tan alır (her zaman aynı portu dinleyen bir sunucu için
`--port`):

```sh
everything-rust app record . --http -- node server.js
everything-rust app replay .
everything-rust app front-door .                                    # axum: her route proxy'lenir
everything-rust app wire . api --route 'GET /users/{id}' --route '*'
everything-rust app lift .
```

- İstekler uygulamanın dokümanlarındaki `curl` satırlarından (`curl -X POST -d ... http://localhost:8080/x`),
  `--inputs`'tan (`{"name", "method", "path", "headers", "body"}`) ve tuzaklardan gelir: yol segmentleri ve sorgu
  değerleri CLI argümanlarıyla aynı tuzaklarla, gövdeler metin ve JSON tuzaklarıyla değiştirilir, bir de
  bilinmeyen bir route eklenir. Her çalıştırma sunucuyu sıfırdan başlatır ve istekleri ad sırasıyla gönderir,
  böylece istekler arasındaki durum korunur.
- Durumu dosyalarda tutan bir sunucu her başlatmada yeni bir kopya alır: `--fresh db.json` (bir dosya
  ya da klasör, tekrarlanabilir) `http/files/` içine anlık görüntü olarak alınır ve orijinal, front door
  ya da bir derleme her başladığında yeni bir çalışma dizinine kopyalanır; `--arg db.json` her başlatmanın
  komutundan sonra aldığı bir argümandır, böylece Rust binary'si orijinalle aynı şekilde başlatılır.
  `--pace 50`, sunucunun yanıtladıktan sonra yaptığı işler için (az önce yazdığını yeniden yükleyen bir
  dosya izleyici) her yanıttan sonra 50 ms bekler.
- Vakalar `rust/app-cases/http/<case>.toml` (istek, durum kodu ve önemli header'lar) ve `<case>.body`
  (iki çalıştırmanın farklı olduğu yerlerde `[..]` içeren yanıt gövdesi) dosyalarıdır. Date, server ve
  diğer yanıta özgü header'lar dışarıda bırakılır; önemsemediğiniz header satırlarını silin.
- Front door, orijinali kendine ait bir portta başlatan ve hiçbir birimin yanıtlamadığı her route'u
  proxy'leyen bir axum sunucusudur. `wire --route` axum route'ları alır: `METHOD /path/{param}`, her metot için
  `/path`, geri kalan her şey için `*`; yapıştırıcı kod route'u, parametrelerini ve isteği alır.
  `lift` proxy'yi kaldırır; HTTP vakaları bundan sonra CI'da `app replay` ile çalışır.

## Gerçek bir örnek

[json-server](/docs/real-world/json-server) `--fresh`, `--arg` ve `--pace` gerektirdi: her yazmada veritabanını yeniden yazar ve dosya izleyicisi hemen ardından gelen bir yazmayı geri alabilir.
