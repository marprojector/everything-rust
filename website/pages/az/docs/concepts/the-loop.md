---
title: Dövr
description: profile, analyze, bridge, record, port, diff, bench, shadow və gate. Hər vahidin keçdiyi addımlar.
---

# Dövr

Hər port, bir isti funksiyadan bütöv tətbiqə qədər, eyni addımlardan vahid-vahid keçir. Port yalnız `gate` keçəndə hazırdır.

everything-rust portu necə yoxlayır, onun əmrləri nəyə qərar verir və məhdudiyyətləri haradadır. [README](/)-yə qayıt.

## Necə işləyir

```
profile ──> analyze ──> bridge ──> record ──> port.rs ──> diff ──> bench ──> shadow ──> gate
 where the    what to     the Rust   what the    you or      every     faster?    both side   done means
 time goes    port first  crate and  original    your agent  recorded  the        by side in  this passes
                          wrapper    does        write it    case      verdict    production
```

Orijinal həqiqət mənbəyi olaraq qalır: `record` onu kənar hallarla (və sizin girişlərinizlə) işə salır və
nə qaytardığını və ya hansı xətanı atdığını saxlayır; `diff` hər halı Rust-a qarşı təkrar oynadır; hamısı
uyğun gəlib `gate` keçənə qədər heç nə buraxılmır. Çağıranlar bütün müddət ərzində orijinal API-dən istifadə edir.

Bütöv tətbiq də eyni dövrdən vahid-vahid keçir, bütün proqramı sübut edən çərçivənin içində
([Bütöv tətbiqi port etmək](/docs/whole-apps/overview)):

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop above, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in         pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down      proved by
 decisions to make      outside         original answers                                                replay
```

## Dövr

| Addım | Əmr | Nə edir |
| --- | --- | --- |
| 0. Profil | `everything-rust profile <repo> -- node bench.mjs` | Real iş yükünü dilin profiler-i altında işə salır (node, cProfile, `go test` pprof, JFR, Ruby tracer): daxil olan vaxta görə isti funksiyalar, onları çağıranlar və hansı dövrlərin batch-lənəcəyi; artıq kompilyasiya olunmuş kodu (numba, Cython, JAX) işarələyir, çünki orada Rust az qazandırır |
| 1. Plan | `everything-rust analyze <repo>` | Vahidlər, port ardıcıllığı, `file:line` ilə risklər, hər vahid üçün körpü, asılılıq-crate uyğunluğu, ən yaxşı ilk portlar; `--whole-app` giriş nöqtələrini və strukturu, crate-i ilə birlikdə hər asılılığı (import edilmiş və ya elan edilmiş), hissələri (bir-birini import edən vahidlər birlikdə köçür) və tövsiyə olunan cavabla nömrələnmiş qərarları əlavə edir |
| 2. Körpü | `everything-rust bridge <repo> <unit>` | Binding-ləri olan crate (`lib.rs` və mənbə dilin yüklədiyi wrapper) və port ediləcək stub-lar (`port.rs`); `--batch fn:param` çoxlu `param` dəyərini Rust-da saxlayan sinif əlavə edir; `--as-json T` tətbiqin yalnız məlumatla doldurduğu tip parametrini JSON dəyərləri kimi körpü ilə bağlayır |
| 3. Qeyd | `everything-rust record <crate>` | Orijinalı yaradılmış kənar hallarla (ASCII olmayan, emoji, `-0`, `NaN`, float gözlənilən yerdə int, ...) və hər parametrdəki hər semantik tələ ilə (string uzunluğu və hərf registri, işarəli bölmə, daşma sərhədləri, float çapı, map və sıralama ardıcıllığı) işə salır və `cases.jsonl`-i saxlayır; `--from-tests -- npx jest` (və ya `-- pytest`) həmçinin orijinalın öz test dəstinin etdiyi hər çağırışı qeydə alır; `--float-tolerance 1e-9` rəqəmsal kod üçün float-ların bu pay daxilində uyğun gəlməsinə icazə verir |
| 3. Qurma | `everything-rust build <crate>` | Rust tərəfini qurur: `.node`, wasm paketi, Python genişlənməsi və ya paylaşılan kitabxana |
| 3. Diff | `everything-rust diff <crate>` | Qeydə alınmış halları Rust-a qarşı, orijinalı çağıranların çağırdığı kimi təkrar oynadır; uyğunsuzluqları siyahıya alır |
| 3. Bench | `everything-rust bench <crate>` | Qeydə alınmış halların orijinalda və Rust-da, körpü daxil, vaxtını ölçür |
| 4. Shadow | `everything-rust shadow <crate>` | Hər ikisini işə salan, orijinalın nəticəsini qaytaran və uyğunsuzluqları bildirən drop-in; `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. Gate | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, testlər, `unsafe` auditi, port edilməmiş heç nə qalmaması, orijinalın hər xəta yolu üçün qeydə alınmış uğursuz hal, orijinalın xəta qaytardığı yerdə `unwrap`/`expect`/`panic!` olmaması (`// INFALLIBLE: <why>` ilə işarələnməyibsə), qurma, API, davranış |
| Agentlər | `everything-rust setup` | MCP serveri (`everything-rust mcp`, hər addım alət kimi) və iş axını skill-ini ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) kodlaşdırma agentlərinizdə qeydiyyatdan keçirir |

## Hot path-lar: məlumatı Rust-da saxlayın

Hər element üçün bir dəfə çağırılan port körpünün haqqını hər element üçün bir dəfə ödəyir və JIT-ə uduzur. Qazandıran şey
dövrü köçürməkdir: `profile` onu tapır, `--batch` isə elementləri bir dəfə qəbul edən
və funksiyanı hər çağırışda hamısının üzərində bütün nüvələrdə, typed array nəticələri ilə işlədən sinif yaradır.

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore: işləmənin dörddə biri, `matched` tərəfindən hər açar üçün çağırılır: batch edin
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

`port.rs`-dəki `FuzzyScoreIndex` struct-ı standart dövrü elementləri bir dəfə hazırlayan dövrlə əvəz edir
və özü sıralaya bilir (`rank`): açarları kiçik hərflərlə, hər birini simvol maskası ilə saxlayır,
hər açarın hər ümumi simvolunun ilk mövqeyinin cədvəlini (bir hərfli sorğu bir sütundur) və son
sorğunun uyğunluqlarını saxlayır (`s`-dən sonra `se` yazmaq yalnız `s`-in uyğun gəldiklərini yenidən qiymətləndirir).
x18n-in 50,000 açar üzrə yan panel axtarışında, hərf-hərf yazılaraq, eyni nəticələrlə:

| | Sorğu başına | TS-ə qarşı |
| --- | ---: | ---: |
| TS: `map(fuzzyScore)`, filter, sort | 10 ms | 1x |
| İndekslə eyni daraltma ilə TS | 4.1-5.4 ms | 2x |
| Eyni port, hər açar üçün çağırılır | - | 0.2-0.4x |
| `FuzzyScoreBatch`, standart indeks (4 nüvə) | 5.1 ms | 1.7x |
| `FuzzyScoreIndex` ilə `FuzzyScoreBatch` (napi, 4 nüvə) | 0.45-0.56 ms | **18-22x** |
| Eynisi wasm ilə (bir thread) | 0.95 ms | **11.5x** |

Eyni daraltmanı edən TS-ə qarşı Rust yenə də 7-12x daha sürətlidir (wasm 5x). Qaytarılan
indeksləri sadə dövrlə açarlara çevirin: `Array.from(order, i => keys[i])` 20x baha başa gəldi, 0.4 ms.

Hər dil batch-lər alır və onlar hətta çağırış başına portların uduzduğu yerdə orijinalı ötür.
4 nüvədə eyni iş, eyni nəticələr və xətalar (benchmark-lar hər demo qovluğundadır):

| Dil | İş yükü | Orijinal | Çağırış başına port orijinala qarşı | Batch orijinala qarşı |
| --- | --- | ---: | ---: | ---: |
| TS | x18n axtarışı, 50,000 açar | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`, 20,000 string | 98 ms | - | **17x** |
| Ruby | `slugify`, 20,000 string | 110 ms | - | **12.7x** |
| Ruby | `shellescape` (stdlib), 50,000 qarışıq dəyər | 88 ms | - | **11.8x** |
| Go | `Slugify`, 50,000 string | 49 ms | 0.93x | **4x** |
| Java | `slugify`, 50,000 string | 43 ms | 0.68x | **3x** |

Kompilyasiya olunan orijinallar daha az qazanır: element başına iş artıq sürətlidir, buna görə batch nüvələrin sayı
qədər qazanır. Batch crate-ləri mimalloc ilə yaddaş ayırır: sistem allocator-u ilə Go batch-i bir
thread-dən daha yavaş idi. Demo: [`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md).

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

## `analyze` nəyə baxır

- Paketlər (`package.json`) və mənbə faylları; `node_modules`, build çıxışı, fixture-lar,
  playground-lar və alət konfiqurasiyaları ötürülür. Vue/Svelte/Astro `<script>` blokları import-lar üçün parse edilir.
- **Vahidlər**: mənbə fayllarından ibarət qovluq və ya paketin mənbə kökündəki fayl.
- **Dalğalar**: vahidin import-ları əvvəlki dalğalarda gəlir; import dövrləri birlikdə port edilir.
- **Körpü**: vahidə və ya onun import etdiyi hər hansı şeyə Node lazım olduqda napi-rs; portativ kod üçün
  wasm-bindgen; UI və redaktor kodu üçün JS-də saxlamaq.
- **Hazırlıq** (nə qədər təhlükəsizdir) və **prioritet** (təhlükəsiz və dəyərlidir: dövrlər və string işi, testlər, ölçü).

| Risk | Ciddilik | Nümunə |
| --- | --- | --- |
| String offset-ləri | yüksək | `slice`, `indexOf`, `matchAll`, `d` bayrağı: UTF-16 və UTF-8 |
| Regex sintaksisi | yüksək | lookaround, backreference-lər, `y`: `regex` crate-ində yoxdur |
| Dinamik regex | yüksək | `new RegExp(userPattern)`: icra zamanı JS sintaksisi |
| Dinamik kod | yüksək | `eval`, `new Function`, hesablanmış `import()` |
| Dinamik tipləşdirmə | orta | `any`, `'key' in value` |
| Sıralı kolleksiyalar | orta | `Map`, `Set`, `Object.keys`: əlavə olunma ardıcıllığı |
| Async, generator-lar | orta | promise və generator sərhədləri |
| Sinif irsiliyi, reflection | orta | `extends`, `Proxy`, `Reflect`, `Symbol` |
| Callback formalı API-lər | orta | funksiya tipli parametrlər və metodlar: çox danışan körpülər |
| Exception-lar, rəqəmlər, JSON | aşağı | `throw`, bit əməliyyatları, `JSON.stringify` ardıcıllığı |

## Nəyi tuta bilmir

Keçmək Rust-un qeydə alınmış hər hala uyğun gəldiyi deməkdir, artıq heç nə. Yenidən yazmalara
zərər verən şey adətən qeydin xaricində olur, buna görə sərhədlərin harada olduğunu bilin:

- **Heç kimin qeydə almadığı girişlər.** Yaradılmış hallar, tələlər, orijinalın test dəsti
  (`record --from-tests`) və sizin öz `--inputs` girişləriniz tavandır. Real girişlər əlavə edin (production
  nümunələri, xəta hesabatları); prod-u sındıran istifadəçi girişi nadir hallarda generatorun ağlına gələn olur.
- **İrəliləmiş orijinal.** `record` vahidin fayllarının barmaq izini saxlayır
  (`recorded-from.json`); orijinal sonradan dəyişəndə `diff` xəbərdarlıq edir, `gate` isə siz yenidən
  qeydə alana qədər uğursuz olur, orijinalın əvvəllər etdiyinə qarşı keçmək əvəzinə.
- **Regex-lər.** `regex` crate-ində lookaround və backreference yoxdur, hətta kompilyasiya olunan
  pattern belə ASCII olmayan mətndə `\w`, `\b`, `\d` və hərf registrindən asılı olmayan uyğunluğu
  JS, Python və ya Java-dan fərqli işləyə bilər. `fancy-regex`-ə keçmək və ya pattern-i yenidən yazmaq qeydə alınmış
  hər haldan keçə və yenə də türk nöqtəsiz `ı`-sında və ya zero-width joiner-də fərqlənə bilər. `record`
  regex istifadə edən fayldakı hər funksiyaya regex tələləri (bunlar, Kelvin işarəsi, uzun s, ərəb-hind rəqəmləri,
  sətir ayırıcıları, son sigma) əlavə edir, amma real pattern-lərinizi real mətnə qarşı da fuzz edin.
- **İşləmələr arasında dəyişənlər.** `record` orijinalı iki dəfə işə salır və fərqlənəni ayırır:
  dəyişən dəyərlər (vaxtlar, id-lər) wildcard-a çevrilir; başqa ardıcıllıqdakı eyni sətirlər (hash map-lər,
  paralellik) istənilən ardıcıllıqla müqayisə olunur, buna görə port yenə də məhz həmin sətirləri çıxarmalıdır;
  fərqli sayda sətir isə istənilən şeyə uyğun gələn `...`-a çevrilir. `record` hər növü siyahıya alır:
  hər birinə baxın, çünki hash ardıcıllığı və ya paralellik fərqi məhz portda həqiqətən dəyişə bilən şeydir.
- **Float-lar.** Rəqəmdən mətnə formatlama dillər arasında fərqlənir (`1e21`, `-0`, rəqəm sayları).
  Tələlər bu dəyərləri əhatə edir və onlar dəqiq uyğun gəlməlidir; `--float-tolerance` yalnız
  yuvarlaqlaşdırmanı bağışlayır, formatlamanı heç vaxt.
- **Çıxışda olmayan hər şey.** Vaxt, yaddaş, yük altında paralellik və stdout, stderr, çıxış kodları,
  yazılmış fayllar və HTTP cavablarından başqa yan təsirlər (şəbəkə çağırışları, verilənlər bazaları,
  siqnallar) müqayisə olunmur.
- **Digər platformalar.** Qeyd davranışı yalnız onu işə salan maşında sübut edir. Nəticələri tip ölçülərindən,
  endianness-dən və ya kompilyatordan asılı olan C kodu hədəfdə fərqli davrana bilər; `analyze`
  bu səbəbdən aparatla işləyən C kodunu C-də saxlayır.
## Hər addım

| Addım | Əmr | Nə edir |
| --- | --- | --- |
| 0. Profil | `everything-rust profile <repo> -- node bench.mjs` | Real iş yükünü dilin profiler-i altında işə salır (node, cProfile, `go test` pprof, JFR, Ruby tracer): daxil olan vaxta görə isti funksiyalar, onları çağıranlar və hansı dövrlərin batch-lənəcəyi; artıq kompilyasiya olunmuş kodu (numba, Cython, JAX) işarələyir, çünki orada Rust az qazandırır |
| 1. Plan | `everything-rust analyze <repo>` | Vahidlər, port ardıcıllığı, `file:line` ilə risklər, hər vahid üçün körpü, asılılıq-crate uyğunluğu, ən yaxşı ilk portlar; `--whole-app` giriş nöqtələrini və strukturu, crate-i ilə birlikdə hər asılılığı (import edilmiş və ya elan edilmiş), hissələri (bir-birini import edən vahidlər birlikdə köçür) və tövsiyə olunan cavabla nömrələnmiş qərarları əlavə edir |
| 2. Körpü | `everything-rust bridge <repo> <unit>` | Binding-ləri olan crate (`lib.rs` və mənbə dilin yüklədiyi wrapper) və port ediləcək stub-lar (`port.rs`); `--batch fn:param` çoxlu `param` dəyərini Rust-da saxlayan sinif əlavə edir; `--as-json T` tətbiqin yalnız məlumatla doldurduğu tip parametrini JSON dəyərləri kimi körpü ilə bağlayır |
| 3. Qeyd | `everything-rust record <crate>` | Orijinalı yaradılmış kənar hallarla (ASCII olmayan, emoji, `-0`, `NaN`, float gözlənilən yerdə int, ...) və hər parametrdəki hər semantik tələ ilə (string uzunluğu və hərf registri, işarəli bölmə, daşma sərhədləri, float çapı, map və sıralama ardıcıllığı) işə salır və `cases.jsonl`-i saxlayır; `--from-tests -- npx jest` (və ya `-- pytest`) həmçinin orijinalın öz test dəstinin etdiyi hər çağırışı qeydə alır; `--float-tolerance 1e-9` rəqəmsal kod üçün float-ların bu pay daxilində uyğun gəlməsinə icazə verir |
| 3. Qurma | `everything-rust build <crate>` | Rust tərəfini qurur: `.node`, wasm paketi, Python genişlənməsi və ya paylaşılan kitabxana |
| 3. Diff | `everything-rust diff <crate>` | Qeydə alınmış halları Rust-a qarşı, orijinalı çağıranların çağırdığı kimi təkrar oynadır; uyğunsuzluqları siyahıya alır |
| 3. Bench | `everything-rust bench <crate>` | Qeydə alınmış halların orijinalda və Rust-da, körpü daxil, vaxtını ölçür |
| 4. Shadow | `everything-rust shadow <crate>` | Hər ikisini işə salan, orijinalın nəticəsini qaytaran və uyğunsuzluqları bildirən drop-in; `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. Gate | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, testlər, `unsafe` auditi, port edilməmiş heç nə qalmaması, orijinalın hər xəta yolu üçün qeydə alınmış uğursuz hal, orijinalın xəta qaytardığı yerdə `unwrap`/`expect`/`panic!` olmaması (`// INFALLIBLE: <why>` ilə işarələnməyibsə), qurma, API, davranış |
| Agentlər | `everything-rust setup` | MCP serveri (`everything-rust mcp`, hər addım alət kimi) və iş axını skill-ini ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) kodlaşdırma agentlərinizdə qeydiyyatdan keçirir |
## Növbəti

Hər addımın öz səhifəsi var: [Profil və analiz](/docs/concepts/analyze), [Körpü](/docs/concepts/bridge), [Qeyd](/docs/concepts/record), [Port](/docs/concepts/port), [Diff](/docs/concepts/diff), [Bench](/docs/concepts/bench), [Shadow](/docs/concepts/shadow) və [Gate](/docs/concepts/gate).
