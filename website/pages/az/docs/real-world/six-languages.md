---
title: Altı dil
description: everything-rust Go, Ruby, Python, Java və C dillərində real kod üzərində başdan-başa işə salınıb, çağırış başına və batch sürəti, işləmələrin tapdığı xətalar.
---

# Altı dildə real kod

everything-rust dəstəklədiyi hər dildə real kod üzərində başdan-başa (analyze, bridge, record, port, diff, gate, bench)
işə salınıb. Kod artıq bu maşında olmalı idi: internetdən endirilmiş kod icra edilmədi. Buna görə
hədəflər quraşdırılmış toolchain-lərlə gələn standart kitabxana modullarıdır, uyğun kitabxana mənbəyi
diskdə olmayan Java və C üçün isə JDK və glibc özləri nazik fasad arxasında oracle rolunu oynayır.

Portlar, qeydə alınmış hallar və əlavə girişlər [`demo/real-world`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/README.md)-dədir.

| Dil | Kod | Funksiyalar | Qeydə alınmış hallar | Nəticə |
| --- | --- | ---: | ---: | --- |
| Go | Go 1.24-ün standart kitabxanasından `path` | 7 (+1 saxlanılıb) | 2,642 (542-si Go-nun öz test cədvəllərindən) | 2,642/2,642, gate keçir |
| Ruby | Ruby 3.3-dən `shellwords.rb` | 3 | 1,617 (446-sı raise edir) | 1,617/1,617, gate keçir |
| Python | CPython 3.13-dən `colorsys.py` | 6 | 2,400 | 2,400/2,400, gate keçir |
| Java | fasad arxasında JDK 21 (`Integer`, `Long`, `Math`, `Double.toString`, `URLEncoder`, `Base64`, `toUpperCase`) | 11 | 4,492 (226-sı xəta atır) | 4,492/4,492, gate keçir |
| C | fasad arxasında glibc (`strtol`, `atoi`, `strtod`, `strspn`, `strcmp`, ctype) | 9 | 1,860 (6-sı çökür) | bir qeyri-müəyyən davranış halı çıxarıldıqdan sonra 1,859/1,859, gate keçir |
| C | aklomp/base64 (mypyc-in runtime-ına daxil edilib) | 7-dən 0 | - | yalnız plan: hər funksiya çıxış buferləri və ya state struct-ı qəbul edir |

## Sürət

Python, Go, Ruby və Java üçün ilk körpülər hər arqumenti və nəticəni JSON mətni vasitəsilə göndərirdi,
bu da bu funksiyalardakı işdən 10-50x baha başa gəlirdi. Onlar dəyərləri birbaşa çevirmək üçün yenidən
quruldu. `bench` indi yalnız çağırışların vaxtını ölçür (arqumentlər bir dəfə, proses daxilində dekodlanır) və
buraxmağa dəyib-dəymədiyini deyir.

| Dil | Əvvəl çağırış başına | İndi çağırış başına | Rust orijinala qarşı (`bench`, həndəsi orta) | Hökm |
| --- | ---: | ---: | --- | --- |
| Ruby | 6.9-9.5 µs (Fiddle + JSON) | 0.36-1.15 µs (magnus genişlənməsi) | **4.4x daha sürətli** (funksiya başına 3.6-7x) | buraxın |
| Python | 11.7 µs (JSON) | təxminən 0.3 µs (birbaşa PyO3 çevirməsi) | **1.35x daha sürətli** (əksər çağırışlarda 1.5-1.9x) | buraxın |
| C | - | keçid yoxdur: birbaşa linklənib | 0.87x: ctype 1.1-1.4x daha sürətli, glibc-nin optimallaşdırılmış `strtod`, `strspn`, `strcmp` funksiyaları 0.5-0.7x | optimallaşdırın və ya yavaşları saxlayın |
| Java | təxminən 1.7 µs (JNI + JSON) | 30-800 ns (tipli JNI və binary wire) | 0.18x | daha böyük vahidləri port edin |
| Go | 2.6 µs (cgo + JSON) | 0.11-0.42 µs (tipli cgo və binary wire) | 0.04x | daha böyük vahidləri port edin |

Go və Java-da siyahılar, map-lər, struct-lar və bir neçə nəticə hər tip üçün yaradılmış binary wire kimi
keçir: Go-nun `WordFreq` (map) funksiyası 5.1-dən 1.0 µs-ə, `Midpoint` (struct) 7.2-dən 1.0 µs-ə düşdü;
Java-nın `frequencies` (`Map`) funksiyası 8.4-dən 1.0 µs-ə, `words` (`List<String>`) isə 6.0 µs-dən
0.57 µs-ə düşdü, Java-nın özündən 1.39x daha sürətli.

TypeScript (x18n core, doqquz vahid, eyni hallar üzərində 27 funksiya): binary wire körpü xərcini
orta hesabla üçdə bir azaltdı (`rangeContains` 3.6 -> 1.1 µs, `applyTextEdits` 10.4 -> 5.7
µs, `toWireRange` 5.0 -> 2.6 µs), `buildTree` isə bərabərdən TS-dən 2.0x daha sürətliyə keçdi. Digər
funksiyalar V8-in 15-600 ns-də işlətdiyi yarpaqlardır: Go və Java-da olduğu kimi, Rust-da hələ də daha yavaşdır.

Bunu həll edən amillər:

- **İnterpretasiya olunan orijinallar (Python, Ruby)**: Rust-a çağırış təxminən 100-300 ns tutur və
  Rust kodu interpretatordan xeyli sürətlidir, buna görə hətta kiçik funksiyalar da qazanır.
- **Kompilyasiya olunan orijinallar (Go, JIT-dən sonra Java)**: orijinal yarpaq funksiya 4-90 ns çəkir; tək bir
  cgo çağırışı təxminən 80 ns tutur, string-li JNI çağırışı isə daha çox. Heç bir körpü 5 ns-lik
  funksiyanı daha sürətli edə bilməz. Bu dillər yalnız bir çağırış çox iş görəndə qazanır: funksiyanı
  onu çağıranlarla birlikdə port edin. `bench` bunu deyir.
- **C**: körpü keçidi yoxdur; Rust kodu birbaşa C kodu ilə müqayisə olunur və nəticə
  portun xüsusiyyətidir.

### Batch-lər: x18n-in axtarışı

Yuxarıdakı yarpaq portları çağırış başına ölçülüb. x18n-in yan panel axtarışında (50,000 açar, hərf-hərf
yazılaraq) `everything-rust profile` `fuzzyScore`-u işləmənin təxminən dörddə biri kimi göstərdi, `matched` tərəfindən
hər açar üçün bir dəfə çağırılan, və `--batch fuzzyScore:text` təklif etdi. Yaradılmış `FuzzyScoreBatch`
açarları Rust-da saxlayır; `port.rs`-dəki `FuzzyScoreIndex` onları bir dəfə UTF-16-da kiçik hərflərə çevirir
və hər açardakı kod vahidlərinin 64 bitlik maskasını saxlayır, buna görə uyğun gələ bilməyən açarların çoxu
oxunmadan ötürülür. Nəticələr hər sorğu üçün TS pipeline-ı ilə eynidir:

| | Sorğu başına ms | TS-ə qarşı |
| --- | ---: | ---: |
| TS pipeline | 10 | 1x |
| Eyni daraltma ilə TS | 4.1-5.4 | 2x |
| Batch, standart indeks (element başına port, 4 nüvə) | 5.1 | 1.7x |
| `FuzzyScoreIndex` (maskalar) ilə batch, napi, 4 nüvə | 1.2 | 7x |
| Özü sıralayan `FuzzyScoreIndex` ilə batch, napi, 4 nüvə | 0.45-0.56 | 18-22x |
| Eynisi, wasm, 1 thread | 0.95 | 11.5x |

7x-dən 20x-ə addım: 1.2 ms-in 0.4 ms-i benchmark-ın öz `Array.from(order, i =>
keys[i])` ifadəsi idi (sadə dövr 0.02 ms tutur). Qalanı indeksin özünün sıralamasına icazə verməkdən gəldi:
yeni sorğu sonuncunu genişləndirəndə yalnız son sorğunun uyğunluqlarını yenidən qiymətləndirir (`s`-ə uyğun
gəlməyən açar `se`-yə uyğun gələ bilməz), bir hərfli sorğuları ilk mövqelər cədvəlindən oxuyur, ASCII
açarlarını bayt kimi qiymətləndirir və tam ədəd ballarını counting sort ilə sıralayır. Callgrind 30 baytlıq
açarlarda kitabxana çağırışlarının (`memchr`, `memcmp`) sadə dövrlərdən baha başa gəldiyini göstərdi. Qazancın bir hissəsi
alqoritmdir, buna görə benchmark eyni daraltma ilə TS-i də işə salır: Rust ondan 7-12x daha sürətlidir
(wasm 5x).

Eyni nümunə digər dillərdə, orijinallara qarşı (4 nüvə, eyni nəticələr və
xətalar):

| Dil | İş yükü | Orijinal | Çağırış başına port orijinala qarşı | Batch orijinala qarşı |
| --- | --- | ---: | ---: | ---: |
| Python | fixture `slugify`, 20,000 string | 98 ms | - | 17x |
| Ruby | fixture `slugify`, 20,000 string | 110 ms | - | 12.7x |
| Ruby | `Shellwords.shellescape`, 50,000 qarışıq dəyər | 88 ms | - | 11.8x |
| Go | fixture `Slugify`, 50,000 string | 49 ms | 0.93x | 3.3-4.1x |
| Java | fixture `slugify`, 50,000 string | 43 ms | 0.68x | 3.0x |

Go batch-i əvvəlcə bir thread-dən daha yavaş işlədi: port hər element üçün yaddaş ayırır, glibc-nin allocator-u isə
Go prosesinin içində thread-ləri ardıcıllaşdırırdı. Batch crate-ləri indi mimalloc istifadə edir. shellescape batch-i
həmçinin çağırış başına qeydin buraxdığı port xətasını tapdı: Ruby simvolları (`shellescape(:sym)`) öz JSON
işarəsi kimi qayıdırdı. Portda düzəldilib.

Sonra Go və Java 2.5x və 1.7x-dən 4x və 3x-ə qalxdı. Batch-in içində vaxtın yarısı iş deyildi:
nəticələr bir thread-də birləşdirilir və azad edilirdi, Go onları ikinci dəfə köçürürdü, Java isə
modified UTF-8 vasitəsilə hər string üçün bir JNI çağırışı edirdi. String nəticələri indi toplu şəkildə qayıdır,
bütün nüvələrdə Go-nun öz buferinə və ya bir Java `char[]`-ına yazılaraq. Digər yarısı portlar idi: bir
thread-də Rust `slugify` Go-nunkundan çətinliklə sürətli, Java-nın JIT-indən isə yavaş idi. ASCII-ni cədvəl
axtarışları və aralıq string olmadan kiçik hərflərə çevirmək bunu düzəltdi; Java portu son sigma qaydasını
yalnız `Σ` saxlayan boşluqla ayrılmış hissəni bütöv şəkildə kiçik hərflərə çevirməklə saxlayır.
Çağırış başına Go portu 0.73x-dən 0.93x-ə, Java portu isə 0.46x-dən 0.68x-ə qalxdı.

## İşləmələr nə tapdı

### everything-rust-dəki xətalar, düzəldilib

- **Bir ULP dəyişən float-lar** (Java işləməsi). `serde_json`-un standart float parser-i həmişə
  düzgün yuvarlaqlaşdırmır. everything-rust qeydə alınmış girişləri oxuyanda onları təhrif edirdi, yaradılmış crate-lər
  isə yenidən təhrif edirdi. Düzgün port üçün 43 `Double.toString` halı uğursuz oldu. everything-rust-də və onun yaratdığı
  hər crate-də (napi-rs, wasm və JSON dispatcher) `serde_json`-un `float_roundtrip` xüsusiyyəti ilə düzəldilib.
- **C çökmələri qeydi öldürürdü** (C işləməsi). C funksiyasının müqaviləsindən kənar girişlər (`isalpha(INT_MAX)`)
  glibc-də segfault verir. C drayveri indi hər halı fork edilmiş child prosesdə işə salır və
  `crash: SIGSEGV` qeydə alır; `diff` orada portun istənilən davranışını qəbul edir, çünki C-nin davranışı qeyri-müəyyəndir.
- **`module_function :name` görünmürdü** (Ruby işləməsi). `shellwords` modul funksiyalarını metodlardan sonra
  simvol arqumentləri ilə elan edir; exporter yalnız çılpaq `module_function` formasını bilirdi.
  İndi hər iki forma tapılır.
- **Tipsiz kod mənasız girişlər alırdı** (Python işləməsi). `colorsys`-də type hint-lər yoxdur, buna görə
  1,800 yaradılmış halın 1,709-u `TypeError` idi. Rəqəm kimi istifadə olunan annotasiyasız parametrlər (`r - g`,
  `max(r, g, b)`, `x < 1`) indi Python və Ruby-də `number` tipini alır: 94 xəta qalır, hamısı real
  (`ZeroDivisionError`).
- **Float kimi ötürülən int-lər heç vaxt qeydə alınmırdı** (Python işləməsi). Python çağıranları `1.0` qədər tez-tez
  `1` də ötürür, `colorsys` isə int-ləri olduğu kimi ötürür (`hsv_to_rgb(0, 0, 7)` `(7, 7, 7)`-dir). Qeyd indi
  hər dördüncü Python halında tam qiymətli rəqəmləri int kimi ötürür; bu, `f64` tipli portda 134 uyğunsuzluğu
  üzə çıxardı.
- **TypeScript dövründən qalma mesajlar**: "must stay in JS", "kept in TS", "characterization tests
  against the TS code" indi mənbə dilini adlandırır.
- **Yenidən açılmış core sinifləri** (`shellwords.rb`-də `class String`) indi monkeypatching riskidir.

### Portun uyğun gəlməli olduğu davranış, `diff` tərəfindən tapılıb

Hər sətir ilk portda qeydə alınmış hallar tərəfindən tutulmuş uyğunsuzluq idi. *fixture* işarəli sətirlər
hər dil üçün yazılmış `tests/fixtures/`-dəki kiçik test layihələrindən, qalanları isə yuxarıdakı
real işləmələrdən gəlir.

- Go `path.Clean`: Go-nun `lazybuf`-ı geri çəkildikdən sonra yazma indeksindəki baytı oxuyur, son
  saxlanılan baytı yox (9 hal, Go-nun öz test cədvəlindən).
- *fixture* Go `strings.ToLower` və Ruby `downcase` hər rune-u ayrıca kiçildir: `ΣΊΣΥΦΟΣ` `σίσυφοσ` olur,
  Rust-un `to_lowercase` funksiyası isə son sigma qaydasını tətbiq edir. Go həmçinin `İ`-ni `i`-yə çevirir, Rust isə
  `i̇` verir.
- *fixture* Go string-ləri baytlarla kəsir; rune-un ortasından keçən kəsik JSON-a hər bayt üçün bir U+FFFD kimi çatır.
- *fixture* Ruby `slugify` mənfi `max` üçün `NoMethodError` atırdı (`slug[0, -1]`
  `nil`-dir): portun saxlamalı olduğu və ya əvvəlcə çağıranların düzəldilməli olduğu orijinaldakı xəta.
- *fixture* Ruby `Array#sum` Kahan-Babuska cəmləməsindən istifadə edir və tam ədəd 0-dan başlayır, buna görə `[-0.0].sum` `0.0`-dır.
- Ruby `Float#to_s` 1e15-dən yuxarı eksponent formasından istifadə edir (`1e15.to_s` `1.0e+15`-dir, `1e14.to_s`
  `100000000000000.0`-dır); massivlərin içində `nil.inspect` `nil`-dir.
- *fixture* Java `\s` yalnız ASCII-dir; Rust-un `split_whitespace` funksiyası U+00A0-da da bölür.
- Python-un int və ya float nəticələri: `colorsys` `+`, `-` və `*` vasitəsilə int-ləri saxlayır, buna görə sadiq port
  `f64` əvəzinə kiçik int-or-float tipi ilə hesablayır (int-lər qeydə alınandan sonra 134 hal).
- glibc `strtod("-")` heç nə çevirmir və `-0.0` yox, `+0.0` qaytarır.

İlk portda emal edilib və qeydə alınmış hallarla təsdiqlənib:

- Java `Integer.parseInt` `Character.digit` vasitəsilə istənilən Unicode onluq rəqəmini (`١٢`, `２`) qəbul edir.
- Java `Double.toString` (JDK 19+) ən qısa onluq formanı çap edir, amma heç vaxt iki rəqəmdən az yox:
  `4.9E-324`, Rust-un ən qısa forması isə `5e-324`-dür.
- Python-un `max(nan, 1, 2)` `nan`-dır (sonrakı dəyər müqayisədə böyük olmasa, birinci dəyər qalib gəlir);
  `-0.0 % 1.0` `0.0`-dır.
- glibc `toupper(-128)` `128`-dir (onun cədvəlləri işarəli `char` dəyərlərini qəbul edir); 0 əsaslı `strtol`
  `077`-ni səkkizlik, hex rəqəmi olmayan `0x`-i isə `0` kimi oxuyur.

### Bu işləmələrin göstərdiyi məhdudiyyətlər

- **Bufer ötürən C API-ləri.** aklomp/base64-ün açıq funksiyaları `char *out, size_t *outlen`
  və ya `struct base64_state *` qəbul edir. C körpüsü yalnız skalyarları və `const char *` daşıyır, buna görə
  hamısını səbəbi göstərilməklə C-də saxlayır. Çıxış buferi nümunəsini dəstəkləmək C üçün növbəti addımdır.
- **C makroları.** base64-ün 51 faylından 43-ü funksiyaları makrolar vasitəsilə təyin edir
  (`BASE64_ENC_FUNCTION(arch)`); parser orada xəta bildirir və plan həmin vahidləri qismən kimi
  işarələyir. SIMD intrinsic-ləri hələ risk kimi göstərilmir.
- **Çökməyən qeyri-müəyyən davranış.** `ispunct(1000)` glibc-nin cədvəlindən kənarı oxuyur və
  1 qaytarır. Heç bir port bunu köçürməməlidir; həmin bir hal `cases.jsonl`-dən əllə silindi. Halı
  qeyri-müəyyən kimi işarələməyin hələ daxili yolu yoxdur.
- **Dinamik tipləşdirmə koda baha başa gəlir.** `shellwords` portu rəqəmlər, nil, massivlər və hash-lər üçün
  Ruby-nin `to_s` və `inspect` metodlarını yenidən həyata keçirir, çünki `shellescape` istənilən şeyi qəbul edir. `colorsys`
  portuna Python-un rəqəm iyerarxiyası lazımdır. Tipli port daha qısadır, amma bəzi çağıranlar üçün davranışı
  dəyişir; `diff` məhz hansıların olduğunu göstərir.
- **Fasadlar.** Java və C işləmələri real JDK və glibc davranışını sınayır, amma kitabxananın öz API-si ilə yox,
  işləmə üçün yazılmış fasadlar vasitəsilə.

## Hər dil necə işləyir

| | Qeyd | Diff | Körpü | Shadow |
| --- | --- | --- | --- | --- |
| Go | orijinala `replace` ilə öz modulu kimi qurulan yaradılmış drayver | cgo wrapper-i üzərində eyni drayver | JSON dispatcher + orijinal API-li cgo paketi | orijinal API-li paket, hər ikisini işə salır |
| Ruby | yaradılmış drayver orijinal faylları yükləyir | eyni drayver Fiddle wrapper-ini yükləyir | JSON dispatcher + eyni modulları təyin edən Fiddle wrapper-i | orijinaldan sonra require ediləcək fayl: hər ikisini `prepend` edir |
| Java | orijinal mənbələrlə kompilyasiya olunmuş reflection drayveri | JNI wrapper sinifləri ilə kompilyasiya olunmuş eyni drayver | JNI üzərində JSON dispatcher + orijinal statik metodlara malik wrapper sinifləri | `<Class>Shadow` sinifləri |
| Python | proses daxili drayver orijinalı import edir | eyni drayver PyO3 paketini import edir | PyO3 | orijinal API-li modul |
| C | orijinalın `cc -shared` ilə qurulması, ctypes ilə çağırılır, hər hal üçün bir fork | Rust `cdylib` üzərində ctypes | orijinal adlarla `extern "C"` funksiyaları və header | `-D` adlandırmaları ilə shadow `.c`; crate-in `shadow` xüsusiyyəti yalnız `everything_rust_` adlarını export edir |
