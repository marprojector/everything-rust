---
title: MCP gurallary
description: everything-rust MCP serweriniň berýän ähli gurallary, olaryň parametrleri we nämä degip bilýändikleri.
---

# MCP gurallary

`everything-rust mcp` her ädimi stdio arkaly MCP guraly hökmünde hödürleýär. `everything-rust setup` ony agentleriňiz bilen hasaba alýar ([Agentiňizi sazlaň](/docs/introduction/agents)).

`tests/tools.rs` tarapyndan MCP serweriniň gurallar sanawyndan döredildi; el bilen üýtgetmäň (`UPDATE_TOOLS_DOC=1 cargo test --test tools` ony täzeden ýazýar). Her gural şol bir opsiýalary bolan CLI buýrugy hem.

## Repozitoriýi analiz etmek

Gural `analyze` · CLI `everything-rust analyze`

Repozitoriýiň kartasyny düzýär (TypeScript/JavaScript, Python, C, Go, Ruby ýa-da Java): birlikler, geçiriş tertibi (tolkunlar), file:line bilen töwekgelçilikler, her birlik üçin bridge, garaşlylyklaryň crate-lere laýyklygy we `start_here`: ilki geçirmek üçin iň gowy birlikler.

Diňe okaýar.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `root` | string | hawa | Repozitoriýiň köki. |
| `lang` | string |  | Çeşme dili; görkezilmese repozitoriýden kesgitlenýär. |
| `whole_app` | boolean |  | Tutuş programmanyň portuny meýilleşdirýär: giriş nokatlary we görnüşi (cli, server, library), import edilen ýa-da yglan edilen her garaşlylyk öz crate-i we laýyklygy bilen, geçiriş tertibindäki bölekler (biri-birini import edýän birlikler bilelikde) we sorag tapgyry üçin maslahat berilýän jogaby bolan belgilenen kararlar. |

## Birlik crate-ini döretmek

Gural `bridge` · CLI `everything-rust bridge`

Bir birlik üçin Rust crate-ini döredýär: src/lib.rs içinde napi-rs ýa-da wasm-bindgen baglanyşyklary, src/port.rs içinde geçirmeli boş funksiýalar, birlik setir orunlaryny ulanýan bolsa UTF-16 kömekçileri. Bar bolan bukjanyň üstüne hiç haçan ýazmaýar.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `root` | string | hawa |  |
| `unit` | string | hawa | analyze-den birlik id-si. |
| `kind` | string |  | Bellenmedik ýagdaýynda meýilnamanyň bridge-i. |
| `out` | string |  | Crate bukjasy; bellenmedik ýagdaýynda <root>/rust/<name>. |
| `name` | string |  | Crate ady; bellenmedik ýagdaýynda `<unit>-rs`. |
| `force` | boolean |  | Meýilnamanyň blocked ýa-da keep diýip belleýän birligi üçin hem bridge döret. |
| `keep` | string[] |  | Çeşme dilinde galdyrmaly eksportlar: `name` ýa-da `name: reason`. |
| `batch` | string[] |  | Batch klaslary, `function:param`: funksiýa Rust-da saklanýan köp `param` bahasynyň üstünden ähli ýadrolarda bir çagyryşda işleýär. port.rs içindäki `<Function>Index` struct-y her element üçin adaty aýlawy çalyşýar. |
| `as_json` | string[] |  | Programmanyň diňe maglumat bilen dolduryan tip parametrleri (`paginate<T>(items: T[])` içindäki `T`): eksporty TS-de galdyrmagyň ýerine JSON bahalary hökmünde bridge edilýär. |

## Baglanyşyklary täzeden döretmek

Gural `refresh` · CLI `everything-rust refresh`

src/port.rs içindäki signaturalar üýtgedilenden soň, crate-iň baglanyşyklaryny (lib.rs, index.cjs, js.rs) şol signaturalardan täzeden döredýär we täze batch klaslaryny goşýar. port.rs-e hiç haçan degmeýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `crate` | string | hawa | Döredilen crate-iň bukjasy (everything-rust.json saklaýar). |
| `batch` | string[] |  | Goşmaly batch klaslary, `function:param`: funksiýa Rust-da saklanýan köp `param` bahasynyň üstünden ähli ýadrolarda bir çagyryşda işleýär. port.rs içindäki `<Function>Index` struct-y her element üçin adaty aýlawy çalyşýar. |

## Asyl özüni alyp barşy ýazga almak

Gural `record` · CLI `everything-rust record`

Asyl funksiýalary döredilen çäk ýagdaýly girişlerde (ASCII däl, emoji, -0, NaN, ...) we goşmaça girişlerde işledýär we netijeleri cases.jsonl hökmünde saklaýar: portuň gabat gelmeli özüni alyp barşy.

Asyl nusganyň kodyny ýa-da buýruklaryny işledýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `crate` | string | hawa | Döredilen crate-iň bukjasy (everything-rust.json saklaýar). |
| `count` | integer |  | Her funksiýa üçin döredilýän ýagdaýlar (bellenmedik ýagdaýynda 200). |
| `inputs` | string |  | Goşmaça ýagdaýlaryň JSONL faýly: {"fn": name, "args": [...]}. |
| `module` | string |  | Diňe JS: asyl nusgany TS çeşmesiniň ýerine şu JS modulyndan import et. |
| `resolve_from` | string |  | npm paketlerini şu bukjanyň node_modules-yndan tap. |
| `float_tolerance` | number |  | Sanly kod: float-lar uly modulyň şu paýyndan (1-den kiçi bahalarda absolýut) köp tapawutlanmasa gabat gelýär, meselem 1e-9. diff we gate üçin crate bilen saklanýar; 0 ony aýyrýar. |
| `node_args` | string[] |  | node üçin goşmaça argumentler, meselem ["--import=tsx"]. |
| `test_command` | string[] |  | TS/JS ýa-da Python: asyl nusganyň öz test toplumynyň birligiň funksiýalaryna edýän her çagyryşyny hem ýazga al, meselem ["npx", "jest"] ýa-da ["pytest"]. Asyl nusganyň bukjasynda işleýär. |

## Crate-i gurmak

Gural `build` · CLI `everything-rust build`

Crate-i gurýar: napi-rs .node, wasm-bindgen çykyşy, Python giňeltmesi ýa-da shared library.

Asyl nusganyň kodyny ýa-da buýruklaryny işledýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `crate` | string | hawa | Döredilen crate-iň bukjasy (everything-rust.json saklaýar). |

## Porty ýazgy bilen deňeşdirmek

Gural `diff` · CLI `everything-rust diff`

cases.jsonl-y gurlan Rust moduluna garşy gaýtadan işledýär we gabat gelmezlikleri sanaýar.

Asyl nusganyň kodyny ýa-da buýruklaryny işledýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `crate` | string | hawa | Döredilen crate-iň bukjasy (everything-rust.json saklaýar). |
| `ignore_key_order` | boolean |  |  |
| `module` | string |  | Crate-iň `index.cjs` faýlynyň ýerine şu JS modulyny synag et. |
| `node_args` | string[] |  | node üçin goşmaça argumentler, meselem ["--import=tsx"]. |
| `float_tolerance` | number |  | Bu işletmede float-lar şu rugsadyň çäginde gabat gelýär (bolmasa record saklan rugsat). |

## Iş ýüküni profillemek

Gural `profile` · CLI `everything-rust profile`

Hakyky iş ýüküni (buýruk: node ýa-da python3 skripti, `go test -bench`, java, ruby) diliň profilleýjisi astynda işledýär we repozitoriýiň hot funksiýalaryny öz içine alýan wagty boýunça tertipleşdirýär, olaryň çagyryjylary, çagyryş sanlary we maslahatlar bilen: nämäni bilelikde geçirmeli we haýsy aýlawlary batch etmeli (`batch` bilen `bridge`). Geçirmezden öň profilläň: birme-bir çagyrylýan ýaprak funksiýalar Rust-da haýalrak.

Asyl nusganyň kodyny ýa-da buýruklaryny işledýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `root` | string | hawa |  |
| `command` | string[] | hawa | root içinde işledilýän iş ýüki: ["node", "bench.mjs"]. |
| `lang` | string |  | Görkezilmese repozitoriýden kesgitlenýär. |

## Porty benchmark etmek

Gural `bench` · CLI `everything-rust bench`

Ýazga alnan ýagdaýlaryň wagtyny asyl nusgada we gurlan Rust modulynda, bridge hem hasaba alnyp, ölçeýär; 1-den kiçi tizlenme portuň çagyryjylarynyň ony çagyryşy ýaly haýalrakdygyny aňladýar. Setirleri we maslahatly netijäni gaýtarýar: çykar, haýal funksiýalary galdyr ýa-da has uly birligi geçir.

Asyl nusganyň kodyny ýa-da buýruklaryny işledýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `crate` | string | hawa | Döredilen crate-iň bukjasy (everything-rust.json saklaýar). |
| `resolve_from` | string |  |  |
| `min_ms` | integer |  | Her funksiýa we tarap üçin iň az ölçenýän wagt, millisekuntda (bellenmedik ýagdaýynda 200). |
| `node_args` | string[] |  | node üçin goşmaça argumentler, meselem ["--import=tsx"]. |

## Shadow modulyny döretmek

Gural `shadow` · CLI `everything-rust shadow`

Asyl nusgany we Rust portuny ýanaşyk işledýän we gabat gelmezlikleri habar berýän, göni çalşyp bolýan shadow döredýär (TS ýa-da Python moduly, Go paketi, Ruby faýly, Java klaslary ýa-da C faýly); EVERYTHING_RUST_MODE=old|shadow|new işleýiş wagtynda geçirýär.

Asyl nusganyň kodyny ýa-da buýruklaryny işledýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `crate` | string | hawa | Döredilen crate-iň bukjasy (everything-rust.json saklaýar). |
| `out` | string |  |  |

## Taýýarlyk kesgitlemesini barlamak

Gural `gate` · CLI `everything-rust gate`

Taýýarlyk kesgitlemesi: fmt, clippy -D warnings, cargo test, unsafe diňe SAFETY teswirleri bilen, durmuşa geçirilmedik funksiýa ýok, gurluş, API bar, ýazga alnan özüni alyp barşy gabat gelýär.

Asyl nusganyň kodyny ýa-da buýruklaryny işledýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `crate` | string | hawa | Döredilen crate-iň bukjasy (everything-rust.json saklaýar). |
| `node_args` | string[] |  | node üçin goşmaça argumentler, meselem ["--import=tsx"]. |

## Tutuş programmany ýazga almak

Gural `app_record` · CLI `everything-rust app record`

Tutuş CLI programmasynyň özüni alyp barşyny ýazga alýar: asyl nusgany onuň öz resminamalaryndaky mysallarda, girişler faýlynda we semantik duzaklarda (ASCII däl we emoji, otrisatel bölmek, doly geçme, float-yň çäk bahalary, map tertibi) her birini iki gezek işledýär. Işletmeleriň arasynda tapawutlanýan zatlar `noise` içinde görnüşi boýunça habar berilýär: bahalar (indi [..] wildcard-lar), tertip (setirler islendik tertipde deňeşdirilýär) we setirler (islendik zada gabat gelýän ...): olaryň hersini ulanyja görkeziň. Ýagdaýlar <repo>/rust/app-cases/ içindäki trycmd faýllary (girişde argv, stdin, env, faýllar; çykyşda stdout, stderr, çykyş kody, faýllar).

Asyl nusganyň kodyny ýa-da buýruklaryny işledýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `repo` | string | hawa | Repozitoriýiň köki. |
| `run` | string[] |  | Asyl nusgany işledýän buýruk, ýollar repo-a görä: ["python3", "cli.py"]. Ilkinji ýazgydan soň hökmany däl. |
| `inputs` | string |  | Goşmaça ýagdaýlaryň JSONL faýly: {"name", "args": [...], "stdin", "env": {...}, "files": {path: text}}. |
| `normalize` | string[] |  | Normalizatorlar: timestamps, uuids, key-order ýa-da name=regex. |
| `name` | string |  | Programmanyň öz resminamalarynda çagyrylýan ady. |
| `timeout` | integer |  | Işletme togtadylmazdan öňki sekuntlar (bellenmedik ýagdaýynda 30). |
| `traps` | boolean |  | Semantik duzak ýagdaýlaryny goş (bellenmedik ýagdaýynda true). |
| `http` | boolean |  | Programma serwer: `run` ony başladýar (ol portuny PORT-dan ýa-da {port}-dan alýar); ýagdaýlar onuň resminamalaryndaky curl setirlerinden, `inputs`-dan ({"name", "method", "path", "headers", "body"}) we duzaklardan alnan soraglar. |
| `port` | integer |  | Serweriň hemişe diňleýän porty (http-ni aňladýar). |
| `args` | string[] |  | Serwerler: her başlangyçda buýrugyndan soň berilýän argumentler, asyl nusga üçin hem, her gurluş üçin hem (["db.json"]). http-ni aňladýar; saklanýar. |
| `fresh` | string[] |  | Serwerler: serweriň okaýan ýa-da ýazýan repozitoriý faýllary ýa-da bukjalary, olaryň suraty alynýar we her başlangyçdan öň täze iş bukjasyna göçürilýär. http-ni aňladýar; saklanýar. |
| `pace` | integer |  | Serwerler: serweriň jogap berenden soň edýän işi (faýl gözegçisi) üçin her jogapdan soň garaşmaly millisekuntlar. http-ni aňladýar; saklanýar we replay tarapyndan ulanylýar; 0 ony aýyrýar. |
| `float_tolerance` | number |  | Sanly çykyş: sanlar uly modulyň şu paýyndan (1-den kiçi bahalarda absolýut) köp tapawutlanmasa gabat gelýär, meselem 1e-9. Ýagdaýlar bilen saklanýar; 0 ony aýyrýar. |

## Programmanyň ýagdaýlaryny gaýtadan işletmek

Gural `app_replay` · CLI `everything-rust app replay`

<repo>/rust/app-cases/ ýagdaýlaryny bir gurluşa (asyl nusga, front door, soňky Rust binar faýly) garşy gaýtadan işledýär: geçen/jemi, gurluş habar berýän bolsa fallback paýy we her şowsuz ýagdaý üçin diff bolan hasabat (rust/app-cases/replay-report.md).

Asyl nusganyň kodyny ýa-da buýruklaryny işledýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `repo` | string | hawa | Repozitoriýiň köki. |
| `build` | string |  | Synag edilýän gurluşyň buýruk setiri; görkezilmese asyl nusga. |
| `port` | integer |  | Serwerler: gurluşyň hemişe diňleýän porty; bolmasa ol PORT we {port} arkaly boş port alýar. |
| `timeout` | integer |  | Işletme togtadylmazdan öňki sekuntlar (bellenmedik ýagdaýynda 30). |
| `case` | string |  | Diňe şu ýagdaýy, hasabatda berilýän ady boýunça gaýtadan işlet (kitaphana: diňe şu birlik): bir gabat gelmezligi düzetmek üçin çalt aýlaw. |

## Rust front door döretmek

Gural `app_front_door` · CLI `everything-rust app front-door`

CLI-iň giriş nokadyna eýelik edýän Rust binar faýly bolan <repo>/rust/app-y döredýär: ýazga alnan buýruklar we flag-lar üçin clap parser, her buýruk asyl nusgany şol bir argv, stdin we env bilen işledýär (her fallback sanalýar). Serwer üçin: asyl nusgany öz portunda başladýan we hiç bir birligiň jogap bermeýän her marşrutyny proxy edýän axum programmasy. Ony gurýar we ýagdaýlary gaýtadan işledýär: 100% fallback bilen 100% geçýär. Ilki app_record gerek; rust/app-yň üstüne hiç haçan ýazmaýar.

Asyl nusganyň kodyny ýa-da buýruklaryny işledýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `repo` | string | hawa | Repozitoriýiň köki. |
| `check` | boolean |  | Gur we gaýtadan işlet (bellenmedik ýagdaýynda true). |
| `library` | boolean |  | Repozitoriýde main bar bolsa-da, ony kitaphana hökmünde kabul et: front door her baglanan birligiň portuny gaýtadan eksport edýän, umumy API-li Rust crate-i bolýar. main we serwer ýok bolsa öz-özünden kesgitlenýär. |

## Birligi programma baglamak

Gural `app_wire` · CLI `everything-rust app wire`

Birlik gate-den geçenden soň, onuň buýruklaryny port.rs-e tabşyrýar: birlik crate-ini rust/app-a kitaphana hökmünde goşýar, glue taslamasyny ýazýar (src/units/<unit>.rs, ony siz ýazýarsyňyz: argumentleri seljeriň, <crate>::port çagyryň, asyl nusganyň çap edýän zadyny çap ediň), marşrutlaşdyrmany EVERYTHING_RUST_UNIT_<NAME>=original öçüriji bilen täzeden döredýär, soňra gurýar we gaýtadan işledýär. Glue-ny ýazanyňyzdan soň ony ýene işlediň: 100% geçýär we fallback paýy azalýar.

Asyl nusganyň kodyny ýa-da buýruklaryny işledýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `repo` | string | hawa | Repozitoriýiň köki. |
| `unit` | string | hawa | Birlik crate-iniň bukjasy, onuň rust/ içindäki ady ýa-da onuň bridge edilen birlik id-si. |
| `commands` | string[] |  | Birligiň jogap berýän buýruklary; görkezilmese soňky wire-den saklanýar. |
| `routes` | string[] |  | Serwerler: birligiň jogap berýän marşrutlary, axum sintaksisi: "GET /users/{id}", islendik usul üçin "/path", galan hemme zat üçin "*"; görkezilmese soňky wire-den saklanýar. |
| `skip_gate` | boolean |  | Ilki birlikde gate işletme. |
| `check` | boolean |  | Gur we gaýtadan işlet (bellenmedik ýagdaýynda true). |

## Arassa Rust programmasyna lift etmek

Gural `app_lift` · CLI `everything-rust app lift`

rust/app-y arassa Rust programmasyna öwürýär. Replay 0 fallback paýy bilen geçmese we her birlik gate-den geçmese, ret edýär (we her päsgelçiligi sanaýar). Soňra, ilki nusgada: fallback-y we öçürijileri aýyrýar, her birligi ýönekeý kitaphana öwürýär (cdylib, baglanyşyklar ýa-da döredilen wrapper-ler ýok), cargo test-de rust/app-cases işledýän trycmd testini goşýar, fmt/clippy/testleri, DECISIONS.md maksatlaryny we aktiwleri barlaýar we arassa binar faýly gaýtadan işledýär. Asyl nusga hiç haçan degmeýär.

Faýllaryň üstüne ýazyp biler, asyl nusganyň kodyny ýa-da buýruklaryny işledýär.

| Parametr | Tip | Hökmany | Düşündiriş |
| --- | --- | --- | --- |
| `repo` | string | hawa | Repozitoriýiň köki. |
