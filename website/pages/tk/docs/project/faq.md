---
title: Köp soralýan soraglar
description: Ol meniň kodumy awtomatik täzeden ýazýarmy? Rust hemişe has çaltmy? Float-larym gabat gelermi? Umumy soraglara jogaplar.
---

# Köp soralýan soraglar

everything-rust barada adamlaryň berýän soraglary. [README](/) sahypasyna gaýdyp barmak.

## Köp soralýan soraglar

**Ol meniň kodumy awtomatik täzeden ýazýarmy?** Ýok. Ol nämäni göçürmelidigini tapýar, bridge
döredýär, asyl nusganyň näme edýändigini ýazga alýar we Rust-y şoňa garşy barlaýar. Rust koduny
`port.rs` içinde siz ýa-da agentiňiz ýazýar; onuň haçan dogrydygyny `diff` we `gate` kesgitleýär.

**Programmamyň API-si üýtgärmi?** Ýok. Çagyryjylar şol bir funksiýalary import edýär (ýa-da ikisini
hem işledýän we siz ynam gazanýançaňyz asyl nusganyň netijesini gaýtarýan shadow modulyny).

**Rust hemişe has çaltmy?** Ýok, everything-rust hem muny aýdýar. Her element üçin bir gezek çagyrylýan
kiçijik funksiýa Rust-da köplenç haýalrak, sebäbi her çagyryş bridge-den geçýär; V8, JVM we Go beýle
funksiýalary nanosekuntlarda işledýär. `profile` onuň daşyndaky aýlawy tapýar, `--batch` tutuş aýlawy
göçürýär, `bench` bolsa siz çykarmazdan öň netije berýär. numba, Cython ýa-da JAX tarapyndan eýýäm
kompilirlenen kod hem maşyn kody hökmünde işleýär: `profile` we `analyze` ony belleýär, sebäbi ony
geçirmek tizlige az goşant goşýar.

**Meniň kodum sanly; float-lar takyk gabat gelermi?** Köplenç ýok: amallaryň tertibi, fused
multiply-add we matematika kitaphanasy kompilýatorlaryň arasynda tapawutlanýar. `--float-tolerance 1e-9`
bilen ýazga alyň (`record` ýa-da `app record`), şonda float-lar uly bahanyň şol paýynyň çäginde gabat
gelýär (1-den kiçi bahalarda absolýut). Bitin sanlar, tekst we gurluş bolsa henizem takyk gabat
gelmeli.

**Diňe APK, JAR ýa-da .exe görnüşinde bar bolan programmamy geçirip bilermi?** Ýok. Ol çeşme kodyny
geçirýär: funksiýalary seljerýär, olaryň hersiniň näme edýändigini ýazga alýar we Rust-y işläp duran
asyl nusga garşy barlaýar. Kompilirlenen programmada bularyň hiç biri ýok, `analyze` hem muny aýdýar.
Binar faýlyň görünýän özüni alyp barşyndan täze Rust programmasyny ýazmak başga taslama.

**Synagdan geçen C kodumy geçirmelimi?** Köplenç ýok, `analyze` hem nirededigini aýdýar. Enjam
registrlerine degýän, kompilýatorynyň tip ölçeglerine ýa-da ýat ýerleşişine daýanýan ýa-da inline
assembly ulanýan C kody (`volatile`, bitfield-ler, packed struct-lar, `__asm__`, adrese öwrülýän
bitin sanlar) "C-de galdyr" diýip bellenýär: siziň maşynyňyzdaky ýazgy maksat enjamy barada hiç zady
subut etmeýär, durnukly draýwer ýa-da firmware kody bolsa täzeden ýazmakdan az peýda alýar. Onuň
daşyndaky göçürip bolýan logika (seljermek, protokollar, maglumatlary işlemek) gowy göçýär.

**Electron programmamy geçirip bilermi?** Logikasyny, hawa; penjireler web sahypalary bolup galýar.
`analyze --whole-app` Electron-y tanaýar we renderer bolşy ýaly galýan wagtynda main prosesi
bölek-bölekden meýilleşdirýär. UI-ni Tauri-a (Rust backend-däki şol bir web UI) göçürmek logika Rust
bolanyndan soň aýratyn ädim.

**C#, Kotlin ýa-da Swift barada näme?** C# preview ýagdaýynda ([Diller](/docs/languages/overview)
sahypasyna serediň); Kotlin we Swift entek goldanmaýar. Her dile parser, ýazyjy, bridge we replay
draýweri gerek, hemmesi başdan-aýak synag edilen. Size gerek dil üçin issue açyň (ýa-da bar bolanyna
ses beriň).

**Rust kody asyl nusgadan has uly bolmazmy?** Bolmaly däl. `gate` portuň setirlerini asyl nusganyňky
bilen deňeşdirýär we 3x-den (üstesine importlar we signaturalar üçin 50 setir) geçse şowsuz bolýar:
bu adatça crate-iň eýýäm edýän zadyny el bilen ýazmagy ýa-da ýagdaýlary birme-bir işlemegi aňladýar.
`analyze` ulanmaly crate-leri sanaýar.

**Bölek-bölekden geçirmek Rust-y köne programmanyň dizaýnyna baglamaýarmy?** Diňe ikisi ýanaşyk
işleýänçä. Ýazgylar özüni alyp barşy programmanyň içki gurluşynda däl-de, onuň gyralarynda barlaýar
(CLI-iň çykyşy, serweriň jogaplary, modulyň eksport edýän funksiýalary). Birlikler entek bridge-den
geçýänçä köne gurluşy gaýtalaň; `app lift` asyl nusgany aýyrandan soň Rust-y erkin üýtgedip guruň,
`app replay` bolsa özüni alyp barşy henizem subut edýär. Hiç zat asyl nusga yzyna çagyrmaýar hem:
front door ony aýratyn proses hökmünde işledýär (ýa-da HTTP-ni oňa ugradýar), bridge edilen birlikleri
asyl nusga çagyrýar, tersine hiç haçan däl, callback-ler bolsa çeşme dilinde galýar.

**Kodum maşynymdan çykýarmy?** Ýok: everything-rust ýerli işleýär we kodyňyzy hiç ýere ibermeýär.
Cargo crate-leri adatdakysy ýaly ýükleýär, agentiňiziň model üpjün edijisi bolsa agentiňiziň oňa
iberýän zadyny öz syýasaty boýunça görýär.

**Näme gurnalmaly?** Rust we siziň geçirýän diliňiziň runtime-y
([Gurnamak](/docs/introduction/install) sahypasyna serediň).

**Bir zat döwüldi.** Buýruk, onuň çykyşy we platformaňyz görkezilen issue açyň.
