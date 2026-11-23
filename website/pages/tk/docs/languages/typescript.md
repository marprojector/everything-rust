---
title: TypeScript/JavaScript
description: Node.js kodyny, TypeScript ýa-da JavaScript, napi-rs ýa-da wasm-bindgen arkaly, asyl API-ni saklap Rust-a geçiriň.
---

# TypeScript/JavaScript

| | |
| --- | --- |
| Birlik | bukja ýa-da faýl |
| Bridge | döredilen binar wire bilen napi-rs ýa-da wasm-bindgen; klaslar we async hem bar |
| Kim ýazga alýar we gaýtadan işledýär | Node, `.ts` çeşme faýllaryny göni işledýär |
| Näme gerek | Node 22.6+ (`profile` üçin Node 24+ gerek) |
| Ýagdaý | Barlanan |

## Bridge saýlamak

Birligiň özi ýa-da onuň import edýän islendik zady Node talap edende `analyze` napi-rs saýlaýar, göçürip bolýan kod üçin wasm-bindgen (ol brauzerlerde hem işleýär), UI we redaktor koduny bolsa JS-de galdyrýar. `bridge --kind napi|wasm` muny üýtgedýär.

Bridge-den geçip bilmeýän eksportlar sebäbi bilen TS-de galýar, shadow moduly bolsa asyl nusgany täzeden eksport edýär: funksiýa alýan ýa-da gaýtarýan funksiýalar, generator-lar we iterator-lar, RegExp, `instanceof` barlaglary, generic-ler (`--as-json T` programmanyň diňe maglumat bilen dolduryýan birini bridge edýär) we tipleri funksiýa saklaýan bahalar. [Bridge](/docs/concepts/bridge) sahypasyna serediň.

## TS-i bolşy ýaly ýazga almak

`record` TS çeşme faýllaryny göni işledýär: Node resolve hook-y bundler görnüşli importlara eýerýär, repozitoriýde hiç zat gurnalmadyk bolsa `--resolve-from` npm paketlerini berýär, öwürmegi talap edýän TS (enum-lar, parameter properties) bolsa `--experimental-transform-types` bilen täzeden synanyşylýar. `record --from-tests -- npx jest` islendik runner bilen asyl nusganyň öz test toplumynyň edýän her çagyryşyny ýazga alýar. Asyl nusganyň örtügi TS/JS üçin ölçenýär ([Örtük](/docs/concepts/coverage)).

## Portuň dogry etmeli zatlary

Hakyky kodda ýazga alnan ýagdaýlar arkaly tapyldy:

- `str.length`, `indexOf` we `slice` UTF-16 kod birliklerini sanaýar, şonuň üçin emoji iki orun tutýar. Döredilen `utf16` kömekçilerini ulanyň.
- `u` baýdagy bolmadyk `/.../i` diňe ASCII harplaryny deňleýär, Rust-yň `regex`-i bolsa Unicode-y deňleýär, şonuň üçin vercel/ms-de `"1 ſ"` (uzyn s) we Kelvin belgisi sekunt hökmünde parse ediljek ekeni.
- `Math.round` ýarymlary ýokary tegelekleýär (`-2.5` `-2` berýär); Rust-yň `round` funksiýasy noldan daşlaşýar. Döredilen `js.rs` faýlynda `js::math_round` bar.
- `${n}` `1e+21`, `5e-324` we `-0` üçin `0` çap edýär. `js::number_to_string` muny gaýtalaýar.
- `String.prototype.trim` Rust-yň `char::is_whitespace` funksiýasyny däl, JS boşluklaryny ulanýar.

## Tizlik

Her çagyryşda port her gezek bridge üçin töleýär, V8 bolsa kiçi funksiýalary nanosekuntlarda işledýär. vercel/ms ýazga alnan ýagdaýlarynda bir çagyryşda 0.56x işledi; x18n-iň ýadro portlary JS-den çagyrylanda TS-den haýal. Batch hökmünde aýlaw Rust-a göçýär:

| Iş ýüki | Asyl nusga | everything-rust bilen |
| --- | ---: | ---: |
| x18n gapdal panelindäki gözleg, 50,000 açar (Node) | bir soraga 10 ms | 0.5 ms: 20x has çalt |
| Şol gözleg brauzerde (wasm, bir akym) | 10 ms | 0.95 ms: 11.5x |
| Fuse.js, 50,000 setir | 91.2 ms | 2.8 ms: 32x |
| vercel/ms `parse` x100,000 | 11.8 ms | 0.65 ms: 18x (tabşyryşy hasaplanda 3.4x) |
| vercel/ms `format` x100,000 | 2.1 ms | tabşyryş bilen 2.5 ms: 0.8x |

Tejribe mysallary: [json-server](/docs/real-world/json-server), [Fuse.js](/docs/real-world/fuse-js), [vercel/ms](/docs/real-world/vercel-ms), [x18n](/docs/real-world/x18n).
