---
title: json-server
description: Başdan-başa port edilmiş ilk bütöv tətbiq. Node serverinin cavab verdiyini cavablandıran bir Rust binary-si, qeydə alınmış 514/514 HTTP halı.
---

# json-server

[json-server](https://github.com/typicode/json-server) (75k ulduz, MIT, commit `89a34a4`,
1.0.0-beta.15) JSON faylından REST API təqdim edir: Node, tinyhttp və lowdb. Bu, bütöv tətbiq axını
(`app record`, `front-door`, `wire`, `lift`) ilə başdan-başa port edilmiş ilk bütöv tətbiqdir:
nəticə Node serverinin cavab verdiyini cavablandıran bir Rust binary-sidir. Hər şey
[`demo/real-world/json-server-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/json-server-rs)-dədir.

| Addım | Nəticə |
| --- | --- |
| `analyze --whole-app` | server, 9 vahid (7 hazır, 2 ehtiyat), əvəz ediləcək 13 asılılıq (tinyhttp, lowdb, sirv, eta, milliparsec, dot-prop, sort-on, inflection...) |
| `app record --http` | 514 hal: `--inputs` ilə verilmiş 268 sorğu (hər route və metod, where operatorları, sort, pagination, embed-lər, CORS preflight, statik fayllar və diapazonlar, 29 səhv formalı JSON gövdəsi, `/__proto__` və digər prototype adları, səhv escape-lər) və 245 tələ; hər biri iki dəfə qeydə alınıb, təsadüfi id-lər `[..]` kimi saxlanılıb |
| `front-door` | 99.8% fallback ilə 513/514; uyğunsuzluq URL-dəki xam `ü`-dür, Node ona 400 ilə cavab verir, proxy isə onu yenidən kodlaşdırır |
| vahidlər | `where-operators` 207, `parse-where` 289, `matches-where` 1,346, `paginate` 853 hal (json-server-in öz testlərindən çağırışlar daxil); `service` 461 və `app` 243, Service çağırışlarını və real HTTP sorğularını işlədən iki kiçik adapter vasitəsilə; altısının hamısında `gate` keçir |
| `wire` | əvvəlcə oxumalar: 22.6% fallback ilə 514/514, yazmalara Node cavab verir, Rust isə Node-un yazdığı faylı yenidən yükləyir; sonra hər route: 0% ilə 514/514 |
| `lift` | təmiz Rust binary-si (5.3 MB), 514/514; `bin.ts`-in komanda sətri əllə port edilib (help, version, fayl yoxlamaları, başlanğıc banneri) |

Lift-dən sonra vahidlərin binding-ləri yoxdur, buna görə hər crate-in `tests/recorded.rs` faylı onun
qeydə alınmış hallarını port üzərində təkrar oynadır: 3,399 halın hamısı `cargo test` ilə işləyir.

Portun düzgün etməli olduğu şeylər, hamısı qeydə alınmış hallar tərəfindən tutulub:

- tinyhttp-in router-i bir dəfə ötürmür: hər `next()`-dən sonra bütün middleware-ləri və
  route-ları yenidən, sonuncu middleware-in yenidən yazdığı URL üzərində uyğunlaşdırır və `HEAD` üçün hər route-u işə salır. Buna görə
  `HEAD /posts` GET handler-ini, sonra POST-unkunu işə salır və POST-un 400-ü ilə cavab verir; `//posts` CORS
  başlıqlarını və body parser-i ötürür. Port bu dövrü olduğu kimi işlədir.
- milliparsec hər POST, PUT, PATCH və DELETE gövdəsini tipindən asılı olmayaraq JSON kimi parse edir, səhv
  olanı isə V8-in mesajı ilə 500-dür: `Expected double-quoted property name in JSON at position 7
  (line 1 column 8)`, `Unexpected token 'x', ..."bbbbbbb": x}" is not valid JSON`. Port
  V8-in parser xətalarını, UTF-16 vahidlərindəki mövqelər daxil olmaqla, təkrarlayır; 3,806 səhv formalı
  giriş üzərində Node-a qarşı yoxlanılıb.
- Sıralama `localeCompare` (ICU) istifadə edir, ICU4X-in collator-u ilə edilib və 3,000 cüt üzərində yoxlanılıb;
  `Number()`, `JSON.stringify` (`1e+21`, əvvəlcə tam ədəd açarları), Node-un iki query-string parser-i
  (`é%E9`-ni WHATWG-dən fərqli dekodlayır), xəta mesajları ilə dot-prop və embed-lər üçün inflection
  (`person` və `people`) hər biri `json-server-js`-də Node-a qarşı yoxlanılıb.
- `db.data[name]` `Object.prototype`-u oxuyur: `/__proto__` `{}`, `/constructor` boş
  200 ilə cavab verir, `PATCH /__proto__` isə verilənlər bazasının prototype-unu əvəz edir və sonrakı axtarışlar bunu görür.
- Node-un HTTP parser-i xam qeyri-ASCII hədəflərə və naməlum metodlara 400 ilə cavab verir və bağlantını bağlayır.

json-server-də tapılıb: öz yazmalarının hər birindən sonra onun fayl izləyicisi faylı yenidən yükləyir. Növbəti
sorğu bu yenidən yükləmə bitməzdən əvvəl yazanda, yenidən yükləmə həmin yazmanı silə bilər: server cavab verdiyi
sürətlə göndərilmiş ilk qeyddə 201 ilə cavablanmış, amma sonrakı oxumaların göstərmədiyi POST var idi.
`app record --pace 50` sorğular arasında gözləyir və hallar bu şəkildə qeydə alınıb. Port
faylı yalnız başqası onu dəyişəndə yenidən yükləyir, buna görə həmin yazmanı itirmir: qəsdən edilmiş
fərq.

Benchmark: Apple M4 (10 nüvə, 16 GB), macOS 27.3, `tsc`-nin npm üçün qurduğu kimi `lib/bin.js`-i işlədən
Node 24.15.0, `--release` ilə qurulmuş Rust binary-si. Hər ikisi loopback üzərində eyni verilənlər bazasının
surətini təqdim edir; `ab -k` isinmədən sonra eyni sorğuları göndərir, hər sətir üçün 20,000 (böyük
verilənlər bazasında yavaş embed üçün 40). İki işləmə, hər biri 1 dəqiqəlik orta yük 3-dən aşağı olanda başladılıb;
xanalar saniyədə sorğu ilə 1-ci işləmə / 2-ci işləməni göstərir. Skript [`bench/bench.py`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/json-server-rs/bench/bench.py)-dir.

Fixture verilənlər bazası (6 post, 3 şərh):

| Sorğu | Node, 1 bağlantı | Rust, 1 bağlantı | Node, 50 bağlantı | Rust, 50 bağlantı |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts` | 11.1k / 10.8k | 27.3k / 23.9k | 19.7k / 20.4k | 115.0k / 110.7k |
| `GET /posts/1` | 11.0k / 11.3k | 27.7k / 25.4k | 21.9k / 22.6k | 119.9k / 99.0k |
| filter, sort, səhifə | 10.5k / 10.0k | 25.2k / 21.5k | 18.3k / 16.4k | 107.5k / 96.3k |
| `?_embed=comments` | 8.7k / 6.6k | 22.5k / 21.8k | 13.8k / 11.0k | 98.3k / 100.6k |
| `GET /` (index səhifəsi) | 7.3k / 6.3k | 27.6k / 21.7k | 10.2k / 9.5k | 135.1k / 119.3k |
| `GET /test.html` (statik) | 12.0k / 11.0k | 25.8k / 25.9k | 30.3k / 26.1k | 103.9k / 104.4k |
| `POST /comments`, 10 bağlantı | | | 2,973 / 2,467 | 5,821 / 5,976 |

Yaradılmış verilənlər bazası (10,000 post, 20,000 şərh, 3.6 MB):

| Sorğu | Node, 1 bağlantı | Rust, 1 bağlantı | Node, 50 (10) bağlantı | Rust, 50 (10) bağlantı |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts/5000` | 8.4k / 9.2k | 14.6k / 14.8k | 15.5k / 16.1k | 69.5k / 67.4k |
| `?_page=3&_per_page=10` | 4.6k / 4.2k | 8.8k / 8.4k | 6.5k / 6.8k | 47.4k / 46.2k |
| `title:contains`, sıralanmış | 431 / 396 | 600 / 560 | 465 / 453 | 2,516 / 2,329 |
| `?_embed=comments`, 5-lik səhifə (10 bağlantı) | 2.4 / 2.4 | 81 / 103 | 2.4 / 2.2 | 253 / 216 |
| `GET /posts`, bütöv siyahı (10 bağlantı) | 328 / 319 | 399 / 407 | 381 / 387 | 1,514 / 1,657 |
| `POST /comments` (10 bağlantı) | | | 236 / 241 | 292 / 264 |

| | Node | Rust |
| --- | ---: | ---: |
| Başlanğıcdan ilk cavaba qədər, fixture verilənlər bazası | 85 / 79 ms | 9.9 / 7.9 ms |
| Başlanğıcdan ilk cavaba qədər, 10,000 post | 95 / 91 ms | 48 / 47 ms |
| Boş vaxtda yaddaş (RSS), fixture verilənlər bazası | 74 MB | 3.5 MB |
| Boş vaxtda yaddaş (RSS), 10,000 post | 94 MB | 41 MB |
| İşləmələr zamanı pik yaddaş (RSS), fixture verilənlər bazası | 424 / 390 MB | 18 MB |
| İşləmələr zamanı pik yaddaş (RSS), 10,000 post | 462 / 485 MB | 289 / 291 MB |

Fərqin kiçik olduğu yerlər: bütöv 10,000 postluq siyahını oxuyan bir bağlantı (1.2x: hər ikisi vaxtı
1.5 MB JSON çap etməyə sərf edir), böyük verilənlər bazasına yazmalar (1.1-dən 1.2x-ə qədər: hər ikisi hər
yazmada bütöv 3.6 MB faylı yenidən yazır, Rust növbədəki yazmaları lowdb kimi birləşdirir) və bir bağlantıda
sıralanmış başlıq axtarışı (1.4x). Böyük qazanclar Node prosesində olmayan paralellikdən və
json-server-in hər post üçün hər şərhi filtrlədiyi embed-dən gəlir (port şərhləri hər sorğuda bir dəfə
indeksləyir, nəticə eynidir).

İlk lift edilmiş binary böyük verilənlər bazasında Node-dan daha yavaş idi (bütöv siyahı üçün saniyədə 298-ə qarşı
33 sorğu, yazmalar üçün 217-yə qarşı 39). Profil çıxarmaq ardıcıllıqla bunları tapdı:
`TCP_NODELAY` yox idi (hər böyük cavab gecikmiş ACK üçün təxminən 40 ms gözləyirdi), hər yazmadan əvvəl `$schema` əlavə etmək
üçün bütöv verilənlər bazası klonlanırdı, çap etməzdən əvvəl siyahıdakı hər element klonlanırdı, bütün sorğular üçün
bir kilid var idi və 10,000 elementin hər biri üçün `item.id` SipHash ilə hash edilirdi. Hamısı düzəldilib; 3,399
vahid halı və 514 HTTP halı hər dəyişiklikdən sonra keçir.

Alətdə tapılıb və düzəldilib:

- Vəziyyətini yazan server eyni başlanğıcdan iki dəfə qeydə alına bilmirdi:
  `--fresh <path>` indi hər başlanğıc üçün verilənlər bazasını (və `public/`-u) hallarla birlikdə saxlanılan
  snapshot-dan təzə iş qovluğuna köçürür; `--arg` hər build-ə orijinalın arqumentlərini verir;
  `--pace` yuxarıdadır.
- `bridge` `paginate<T>(items: T[])`-i TS-də saxlayırdı (istənilən şey üzərində generic, funksiyalar daxil);
  `--as-json T` onu JSON dəyərləri kimi körpü ilə bağlayır.
- Front door-un parametrli route-ları `/posts/%E0` üçün vahid sorğunu görməzdən əvvəl axum-un öz 400-ü ilə
  cavab verirdi; parametrlər indi URI-dən oxunur.
- `lift` yalnız qoşulmuş vahidi hazırlayırdı, yol ilə asılı olduğu crate-ləri yox, və `cargo fmt --check`-in
  rədd etdiyi `lib.rs` yazırdı; hər ikisi düzəldilib.
- Onun yaratdığı serverlər `TCP_NODELAY` təyin etmirdi: iki hissədə yazılmış cavab müştərinin gecikmiş
  ACK-ını, təxminən 40 ms, gözləyirdi. İndi onu təyin edirlər.

Nəyi əhatə etmir: JSON5 verilənlər bazaları (binary bunu deyir və çıxır), 100 KiB gövdə limiti
(Node-unku gövdənin 64 KiB-lik oxumalarla necə gəldiyindən asılıdır; port tam oxumaları fərz edir), tək
surrogate-li string-lər (U+FFFD-yə çevrilir) və `1e400` verilənlər bazası dəyəri (Node növbəti yenidən yükləməyə
qədər yaddaşda `Infinity` saxlayır; port faylın aldığı `null`-u saxlayır).
