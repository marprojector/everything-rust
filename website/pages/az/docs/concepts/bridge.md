---
title: Körpü
description: bridge bir vahid üçün binding-lər və wrapper ilə Rust crate-inin karkasını qurur ki, çağıranlar orijinal API-ni saxlasın.
---

# Körpü

```sh
everything-rust bridge <repo> <unit-id>          # crate <repo>/rust/<name>-də; heç vaxt üzərinə yazmır
everything-rust refresh <crate>                  # port.rs-də imzanı dəyişdikdən sonra
```

`bridge` binding-ləri (`lib.rs`, üstəgəl mənbə dilin yüklədiyi wrapper) və port ediləcək stub-ları (`port.rs`) olan crate yaradır. Portu yalnız `src/port.rs`-də yazırsınız; `src/lib.rs` və wrapper-lər yaradılır, `refresh` isə onları `port.rs`-dəki imzalardan yenidən yaradır və yeni batch siniflərini əlavə edir. O, `port.rs`-ə heç vaxt toxunmur.

Seçimlər:

- `--batch fn:param` çoxlu `param` dəyərini Rust-da saxlayan sinif əlavə edir. Bax [Batch-lər](/docs/concepts/batches).
- `--as-json T` tətbiqin yalnız məlumatla doldurduğu tip parametrini (`paginate<T>(items: T[])`-dəki `T`) JSON dəyərləri kimi körpü ilə bağlayır.
- `--keep "name: reason"` daha çox export-u mənbə dilində saxlayır, məsələn `Intl` məlumatından asılı kodu.
- `--kind` planın seçdiyi körpünü əvəz edir (TypeScript üçün napi-rs və ya wasm).

## Dillər üzrə körpü

| Dil | Körpü |
| --- | --- |
| TypeScript/JavaScript | yaradılmış binary wire ilə napi-rs və ya wasm-bindgen; siniflər və async daxil |
| Python | PyO3 (abi3), birbaşa çevirmə, tipli exception-lar |
| C | orijinal adlarla `extern "C"` funksiyaları və header |
| Go | orijinal API-li paketdə tipli cgo çağırışları və yaradılmış binary wire |
| Ruby | orijinal modulların altında magnus genişlənməsi |
| Java | orijinal metodlara malik wrapper siniflərdə tipli JNI və yaradılmış binary wire |

PHP və C# vahidləri hələ körpü ilə bağlana bilmir; bax [PHP](/docs/languages/php) və [C#](/docs/languages/csharp).

## `bridge` nəyə qərar verir

- Körpünü keçə bilməyən export-lar səbəbi göstərilməklə mənbə dilində qalır və shadow
  modulu orijinalı yenidən export edir. TS üçün:
  funksiya qəbul edən və ya qaytaran funksiyalar, generator-lar və iterator-lar, RegExp, `instanceof` yoxlamaları,
  generic-lər (`--as-json T` tətbiqin yalnız məlumatla doldurduğu birini körpü ilə bağlayır) və tipləri (import-lar, re-export-lar və `extends` vasitəsilə izlənməklə)
  funksiya saxlayan dəyərlər. `--keep "name: reason"` daha çoxunu saxlayır, məsələn `Intl` məlumatından asılı kodu.
- TS tipləri Rust tiplərinə uyğunlaşdırılır, tip alias-ları import-lar vasitəsilə həll olunur, parametr tipləri isə
  standart dəyərlərdən müəyyən edilir; strukturlaşdırılmış dəyərlər yaradılmış binary wire kimi keçir (JSON yalnız
  wire-ın modelləşdirmədiyi formalar üçün fallback kimi).
- JSON `undefined`, NaN, Infinity, -0 və ya tək surrogate-ləri ifadə edə bilmir: strukturlaşdırılmış dəyərlər
  yaradılmış `index.cjs`-in tərcümə etdiyi işarələr (`{"$undefined": true}`, `{"$num": "NaN"}`, `{"$utf16": [...]}`) daşıyır,
  köməkçilər isə crate-in `js.rs` faylındadır.

`record` TS mənbələrini olduğu kimi işə salır: Node resolve hook-u bundler üslublu import-ları izləyir
(`./errors`-dan `./errors.ts`-ə), repoda heç bir paket quraşdırılmayanda `--resolve-from` npm paketlərini
təmin edir, çevrilmə tələb edən TS (enum-lar, parameter properties) isə
`--experimental-transform-types` ilə yenidən cəhd edilir.
