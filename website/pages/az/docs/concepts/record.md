---
title: Qeyd
description: record orijinalı yaradılmış kənar hallar, semantik tələlər, öz testləri və sizin girişlərinizlə işə salır və nəticələri cases.jsonl kimi saxlayır.
---

# Qeyd

```sh
everything-rust record <crate>
everything-rust record <crate> --from-tests -- npx jest      # və ya: -- pytest
everything-rust record <crate> --inputs extra.jsonl
```

`record` orijinal funksiyaları işə salır və hər birinin nə qaytardığını və ya hansı xətanı atdığını crate-in yanında `cases.jsonl`-də saxlayır: portun uyğun gəlməli olduğu davranış. Sonradan uyğunsuzluq davranış fərqidir: qeydi yox, portu düzəldin.

## Hallar haradan gəlir

- **Yaradılmış kənar hallar**: ASCII olmayan, emoji, `-0`, `NaN`, float gözlənilən yerdə int və s. (`--count` hər funksiya üçün neçə olduğunu təyin edir).
- Hər parametrdə **semantik tələlər**: string uzunluğu və hərf registri, işarəli bölmə, daşma sərhədləri, float çapı, map və sıralama ardıcıllığı.
- Regex istifadə edən fayldakı hər funksiya üçün **regex tələləri**: türk nöqtəsiz `ı`-sı, Kelvin işarəsi, uzun s, ərəb-hind rəqəmləri, sətir ayırıcıları, son sigma, zero-width joiner-lər.
- **Orijinalın öz testləri**: `--from-tests -- <test command>` test dəstinin vahidin export edilmiş funksiyalarına etdiyi hər çağırışı qeydə alır (istənilən runner ilə TS/JS, pytest və ya unittest ilə Python). Sonra mənbə bərpa olunur. vercel/ms-də bu, 167 Jest testindən 340 çağırış tutdu.
- **Sizin girişləriniz**: hər sətirdə bir `{"fn": name, "args": [...]}` olan `--inputs cases.jsonl`. Real girişlər əlavə edin (production nümunələri, xəta hesabatları); prod-u sındıran istifadəçi girişi nadir hallarda generatorun ağlına gələn olur.

Python çağıranları `1.0` qədər tez-tez `1` də ötürür, buna görə qeyd hər dördüncü Python halında tam qiymətli rəqəmləri int kimi ötürür.

## cases.jsonl-dəki işarələr

JSON `undefined`, NaN, Infinity, -0 və ya tək surrogate-ləri ifadə edə bilmir. Strukturlaşdırılmış dəyərlər yaradılmış `index.cjs`-in tərcümə etdiyi işarələr daşıyır, köməkçilər isə crate-in `js.rs` faylındadır:

| İşarə | Mənası |
| --- | --- |
| `{"$undefined": true}` | JavaScript `undefined` |
| `{"$num": "NaN"}` | JSON-un saxlaya bilmədiyi rəqəm (`NaN`, `Infinity`, `-0`) |
| `{"$utf16": [...]}` | tək surrogate-lər üçün UTF-16 kod vahidləri ilə verilmiş string |

Funksiyanın müqaviləsindən kənar C girişləri orijinalı çökdürə bilər. C drayveri hər halı fork edilmiş child prosesdə işə salır və `crash: SIGSEGV` qeydə alır; `diff` orada portun istənilən davranışını qəbul edir, çünki C-nin davranışı qeyri-müəyyəndir.

## İki işləmə və fərqlənənlər

`record` orijinalı iki dəfə işə salır və işləmələr arasında fərqlənəni ayırır: dəyişən dəyərlər (vaxtlar, id-lər) wildcard-a çevrilir; başqa ardıcıllıqdakı eyni sətirlər (hash map-lər, paralellik) istənilən ardıcıllıqla müqayisə olunur; fərqli sayda sətir isə istənilən şeyə uyğun gələn `...`-a çevrilir. `record` hər növü siyahıya alır. Hər birinə baxın, çünki hash ardıcıllığı və ya paralellik fərqi məhz portda həqiqətən dəyişə bilən şeydir.

## Qeyd öz orijinalını xatırlayır

`record` vahidin fayllarının barmaq izini saxlayır (`recorded-from.json`). Orijinal sonradan dəyişəndə `diff` xəbərdarlıq edir, `gate` isə siz yenidən qeydə alana qədər uğursuz olur, orijinalın əvvəllər etdiyinə qarşı keçmək əvəzinə. Bax [Diff](/docs/concepts/diff#stale-recordings).

## TypeScript xüsusiyyətləri

`record` TS mənbələrini olduğu kimi işə salır: Node resolve hook-u bundler üslublu import-ları izləyir (`./errors`-dan `./errors.ts`-ə), repoda heç bir paket quraşdırılmayanda `--resolve-from` npm paketlərini təmin edir, çevrilmə tələb edən TS (enum-lar, parameter properties) isə `--experimental-transform-types` ilə yenidən cəhd edilir. Node-da birbaşa işləyə bilməyən TS `--module dist/x.mjs` ilə build çıxışından və ya `--node-arg=--import=tsx` ilə qeydə alına bilər.

## Rəqəmsal kod

Float-ların nadir hallarda bit-bit uyğun gəldiyi kod üçün `--float-tolerance` ilə qeydə alın. Bax [Float tolerantlığı](/docs/concepts/float-tolerance).
