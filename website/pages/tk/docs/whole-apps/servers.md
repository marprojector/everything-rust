---
title: Serwerler
description: Şol bir buýruklar HTTP serwerlerini subut edýär. --http, --fresh, --arg we --pace, şeýle hem hiç bir birligiň jogap bermeýän her marşrutyny proksi edýän axum front door.
---

# Serwerler

Şol bir buýruklar HTTP serwerlerini subut edýär. `--http` buýrugyň serwer başladýandygyny aýdýar; ol öz
portuny `PORT`-dan ýa-da buýrukdaky `{port}`-dan alýar (hemişe şol bir portda diňleýän serwer üçin `--port`):

```sh
everything-rust app record . --http -- node server.js
everything-rust app replay .
everything-rust app front-door .                                    # axum: her marşrut proksi edilýär
everything-rust app wire . api --route 'GET /users/{id}' --route '*'
everything-rust app lift .
```

- Soraglar programmanyň resminamalaryndaky `curl` setirlerinden (`curl -X POST -d ... http://localhost:8080/x`),
  `--inputs`-dan (`{"name", "method", "path", "headers", "body"}`) we duzaklardan gelýär: ýol segmentleri we sorag
  bahalary CLI argumentleri ýaly duzaklar bilen çalşyrylýar, bedenler tekst we JSON duzaklary bilen, şeýle hem
  näbelli marşrut. Her işletme serweri täzeden başladýar we soraglary at tertibinde iberýär, şonuň üçin
  soraglaryň arasyndaky ýagdaý saklanýar.
- Ýagdaýy faýllarda saklaýan serwer her başlangyçda täze nusga alýar: `--fresh db.json` (faýl
  ýa-da bukja, gaýtalap bolýar) `http/files/` içine surata düşürilýär we asyl nusga, front door ýa-da
  gurluş her gezek başlanda täze iş bukjasyna göçürilýär; `--arg db.json` her başlangyjyň öz buýrugyndan soň
  alýan argumenti, şonuň üçin Rust binar faýly asyl nusga ýaly başladylýar.
  `--pace 50` serweriň jogap berenden soň edýän işi üçin (ýaňy ýazan zadyny täzeden ýükleýän faýl
  gözegçisi) her jogapdan soň 50 ms garaşýar.
- Ýagdaýlar `rust/app-cases/http/<case>.toml` (sorag, status we ähmiýetli header-ler)
  we `<case>.body` (jogabyň bedeni, iki işletmäniň tapawutlanan ýerinde `[..]` bilen). Date, server we
  her jogaba mahsus beýleki header-ler goşulmaýar; özüňize gerek däl islendik header setirini pozuň.
- Front door asyl nusgany özüne degişli portda başladýan we hiç bir birligiň jogap bermeýän her marşrutyny
  proksi edýän axum serweri. `wire --route` axum marşrutlaryny alýar: `METHOD /path/{param}`, islendik usul üçin `/path`,
  galan hemme zat üçin `*`; glue marşruty, onuň parametrlerini we soragy alýar.
  `lift` proksini aýyrýar; şondan soň HTTP ýagdaýlary CI-de `app replay` bilen işleýär.
## Hakyky mysal

[json-server](/docs/real-world/json-server) üçin `--fresh`, `--arg` we `--pace` gerek boldy: ol her ýazuwda maglumat binasyny täzeden ýazýar, onuň faýl gözegçisi bolsa çalt yzyna gelýän ýazuwy yzyna gaýtaryp biler.
