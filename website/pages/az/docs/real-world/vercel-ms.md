---
title: vercel/ms
description: vercel/ms başdan-başa port edilib. Qeydə alınmış hər halda düzgündür, çağırış başına daha yavaşdır, yalnız proqram bir dəfəyə çoxlu müddət parse edəndə daha sürətlidir.
---

# vercel/ms

[vercel/ms](https://github.com/vercel/ms) (5.5k ulduz, commit `4ff48ce`) `"2 days"`-i millisaniyələrə
və geri çevirir; əksər Node layihələri onu hansısa asılılıq vasitəsilə gətirir. everything-rust 1.5.1 ilə
başdan-başa port edilib; crate
[`demo/real-world/vercel-ms-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/vercel-ms-rs)-dədir.

| Addım | Nəticə |
| --- | --- |
| `analyze` | 1 vahid (`src/index.ts`), hazır, hazırlıq 96 |
| `bridge --kind napi` | `ms`, `parse`, `parseStrict` və `format` körpü ilə bağlandı, heç biri TS-də saxlanılmadı |
| `record --from-tests -- jest` | 975 hal: yaradılmışlar, tələlər və vercel/ms-in 167 Jest testindən tutulmuş 340 çağırış (ilk port onlardan əllə köçürülmüş 149 girişdən istifadə edirdi) |
| port | 184 sətir TS üçün 200 sətir Rust, `options`-u hər batch üçün bir dəfə oxuyan `FormatIndex` daxil olmaqla |
| `diff` | ilk build-də 784/784 uyğun gəldi; tutulmuş çağırışlarla 975/975 |
| `gate` | keçir (fmt, clippy, testlər, xəta yolları, ölçü, davranış) |
| `bench`, çağırış başına | qeydə alınmış hallar üzrə 0.56x (əvvəlcə 0.39x): normal string üzərində `parse` 79 ns çəkir, JS ilə bərabər, amma kənar hallar və atılan xətalar körpü vasitəsilə hələ də baha başa gəlir |
| batch, `parse` x100,000 | JS 11.8 ms, Rust 0.65 ms: **18x**; string-lərin birdəfəlik ötürülməsi daxil 3.4x (string-lər birləşdirilmiş keçməzdən əvvəl 2.1x) |
| batch, `format` x100,000 | JS 2.1 ms, ötürmə ilə Rust 2.5 ms: **0.8x** (əvvəlcə 0.2x); `format(x, { long: true })` 1.2x |

Portun düzgün etməli olduğu şeylər, hamısı qeydə alınmış hallar və ya tələlər tərəfindən tutulub:

- `u` bayrağı olmadan `/.../i` yalnız ASCII hərflərini qatlayır, Rust-un `regex`-i isə Unicode-u qatlayır, buna görə
  `"1 ſ"` (uzun s) və Kelvin işarəsi saniyə kimi parse edilərdi. Port vahidləri əllə, yalnız ASCII ilə
  uyğunlaşdırır.
- `Math.round` yarımları yuxarı yuvarlaqlaşdırır (`-2.5` `-2` verir); Rust-un `round` funksiyası sıfırdan uzağa yuvarlaqlaşdırır.
- `${n}` `1e+21`, `5e-324` və `-0` üçün `0` çap edir; port JS-in rəqəmdən string-ə çevirməsini təkrarlayır.
  Onun ilk versiyasında həm də ms-in tam ədəd çıxışlarının heç vaxt çatmadığı bərabərlik xətası var idi
  (`25.728225708007812` `...813` kimi çap olunurdu).
- `str.length` UTF-16 vahidlərini sayır, buna görə 100 simvol limiti emojini iki sayır.

Alətdə tapılıb və düzəldilib: strukturlaşdırılmış parametrli (`format`-ın `options`-u) napi batch metodu
kompilyasiya olunmurdu, çünki yaradılmış binding napi-nin `serde-json` xüsusiyyəti olmadan çevirə bilmədiyi
`serde_json::Value` qəbul edirdi. Batch sinifləri olan crate-lər indi onu aktivləşdirir.

Bu portdan alətə geri götürülənlər:

- Yaradılmış `js.rs`-də 20,000 təsadüfi double üzərində Node ilə yoxlanılmış `js::number_to_string` və
  `js::math_round` var; port öz surətləri əvəzinə onlardan istifadə edir. `number_to_string` rəqəmlərini
  serde_json-un printer-indən götürür: tam ədədlər üçün `format!`-dan 5x, kəsrlər üçün 3x daha sürətli.
- napi batch-ləri string elementlərini UTF-16 son offset-ləri ilə bir birləşdirilmiş string kimi, rəqəm elementlərini
  bir `Float64Array` kimi göndərir və string nəticələrini birləşdirilmiş qaytarır (`index.cjs` onları `slice` ilə ayırır):
  element başına bir JS dəyərindən string-lər üçün 2x, rəqəmlər üçün 20x daha sürətli. `new Batch(items)` və
  `run()` öz formasını saxlayır.
- `analyze` `Math.round`, `toFixed` və `s` və ya `k` hərfini çəkən `u`-suz `/i` regex-lərini
  hər biri üçün tövsiyə ilə işarələyir.
- Çağırışlar daha ucuz başa gəlir: napi-də hər sinxron funksiya napi arqumentləri olmadan binary wire üzərindən gedir
  (bir arqument təxminən 55 ns, bütün çağırış 22 ns tutur), funksiya başına iki yox, bir JS wrapper,
  `serde_json::Value` parametrləri istənilən formalı teqli dəyərlər kimi keçir, xətalar isə wire buferində
  qayıdır. Çağırış başına `parse` 121-dən 79 ns-ə düşdü.
- `record --from-tests` test girişlərini əllə köçürməyi əvəz edir: testlər işləyərkən dəstin etdiyi hər
  çağırışı qeydə alır, tutma kodu isə coverage-dən kənarda saxlanılır (ms 100% tələb edir).

Hökm: `ms` yaxşı port, amma pis sürət hədəfidir. Qeydə alınmış hər halda düzgündür, yalnız proqram
bir dəfəyə çoxlu müddət parse edəndə daha sürətlidir.
