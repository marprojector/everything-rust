---
title: TypeScript/JavaScript
description: Node.js kodunu, TypeScript və ya JavaScript, napi-rs və ya wasm-bindgen vasitəsilə, orijinal API saxlanılmaqla Rust-a port edin.
---

# TypeScript/JavaScript

| | |
| --- | --- |
| Vahid | qovluq və ya fayl |
| Körpü | yaradılmış binary wire ilə napi-rs və ya wasm-bindgen; siniflər və async daxil |
| Qeydə alan və təkrar oynadan | `.ts` mənbələrini işlədən Node |
| Tələb olunur | Node 22.6+ (`profile` üçün Node 24+ lazımdır) |
| Vəziyyət | Yoxlanılıb |

## Körpünün seçilməsi

`analyze` vahidə və ya onun import etdiyi hər hansı şeyə Node lazım olduqda napi-rs, portativ kod üçün wasm-bindgen seçir (o, brauzerlərdə də işləyir), UI və redaktor kodunu isə JS-də saxlayır. `bridge --kind napi|wasm` bunu əvəz edir.

Körpünü keçə bilməyən export-lar səbəbi göstərilməklə TS-də qalır və shadow modulu orijinalı yenidən export edir: funksiya qəbul edən və ya qaytaran funksiyalar, generator-lar və iterator-lar, RegExp, `instanceof` yoxlamaları, generic-lər (`--as-json T` tətbiqin yalnız məlumatla doldurduğu birini körpü ilə bağlayır) və tipləri funksiya saxlayan dəyərlər. Bax [Körpü](/docs/concepts/bridge).

## TS-i olduğu kimi qeydə almaq

`record` TS mənbələrini birbaşa işə salır: Node resolve hook-u bundler üslublu import-ları izləyir, repoda heç bir paket quraşdırılmayanda `--resolve-from` npm paketlərini təmin edir, çevrilmə tələb edən TS (enum-lar, parameter properties) isə `--experimental-transform-types` ilə yenidən cəhd edilir. `record --from-tests -- npx jest` orijinalın öz test dəstinin etdiyi hər çağırışı istənilən runner ilə qeydə alır. Orijinalın əhatəsi TS/JS üçün ölçülür ([Əhatə](/docs/concepts/coverage)).

## Portun düzgün etməli olduğu şeylər

Real kodda qeydə alınmış hallarla tapılıb:

- `str.length`, `indexOf` və `slice` UTF-16 kod vahidlərini sayır, buna görə emoji iki mövqe tutur. Yaradılmış `utf16` köməkçilərindən istifadə edin.
- `u` bayrağı olmadan `/.../i` yalnız ASCII hərflərini qatlayır, Rust-un `regex`-i isə Unicode-u qatlayır, buna görə vercel/ms-də `"1 ſ"` (uzun s) və Kelvin işarəsi saniyə kimi parse edilərdi.
- `Math.round` yarımları yuxarı yuvarlaqlaşdırır (`-2.5` `-2` verir); Rust-un `round` funksiyası sıfırdan uzağa yuvarlaqlaşdırır. Yaradılmış `js.rs`-də `js::math_round` var.
- `${n}` `1e+21`, `5e-324` və `-0` üçün `0` çap edir. `js::number_to_string` bunu təkrarlayır.
- `String.prototype.trim` Rust-un `char::is_whitespace`-ini yox, JS boşluq simvollarını istifadə edir.

## Sürət

Çağırış başına port hər dəfə körpünün haqqını ödəyir, V8 isə kiçik funksiyaları nanosaniyələrdə işlədir. vercel/ms qeydə alınmış halları üzrə çağırış başına 0.56x işlədi; x18n-in core portları JS-dən çağırıldıqda TS-dən daha yavaşdır. Batch-lər kimi dövr Rust-a köçür:

| İş yükü | Orijinal | everything-rust ilə |
| --- | ---: | ---: |
| x18n-in yan panel axtarışı, 50,000 açar (Node) | sorğu başına 10 ms | 0.5 ms: 20x daha sürətli |
| Eyni axtarış brauzerdə (wasm, bir thread) | 10 ms | 0.95 ms: 11.5x |
| Fuse.js, 50,000 string | 91.2 ms | 2.8 ms: 32x |
| vercel/ms `parse` x100,000 | 11.8 ms | 0.65 ms: 18x (ötürmə daxil 3.4x) |
| vercel/ms `format` x100,000 | 2.1 ms | ötürmə ilə 2.5 ms: 0.8x |

Nümunə araşdırmaları: [json-server](/docs/real-world/json-server), [Fuse.js](/docs/real-world/fuse-js), [vercel/ms](/docs/real-world/vercel-ms), [x18n](/docs/real-world/x18n).
