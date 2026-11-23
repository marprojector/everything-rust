---
title: Sikl
description: profile, analyze, bridge, record, port, diff, bench, shadow va gate. Har bir birlik o'tadigan qadamlar.
---

# Sikl

Har bir port, bitta issiq funksiyadan butun ilovagacha, birma-bir birlik bo'yicha bir xil qadamlardan o'tadi. Port faqat `gate` o'tgandagina tayyor hisoblanadi.

everything-rust portni qanday tekshiradi, uning buyruqlari nimani hal qiladi va cheklovlari qayerda. [README](/) ga qaytish.

## Qanday ishlaydi

```
profile ──> analyze ──> bridge ──> record ──> port.rs ──> diff ──> bench ──> shadow ──> gate
 where the    what to     the Rust   what the    you or      every     faster?    both side   done means
 time goes    port first  crate and  original    your agent  recorded  the        by side in  this passes
                          wrapper    does        write it    case      verdict    production
```

Asl nusxa haqiqat manbai bo'lib qoladi: `record` uni chegaraviy holatlarda (va sizning kirish ma'lumotlaringizda) ishga tushiradi va
u nima qaytarishi yoki qanday xato tashlashini saqlaydi; `diff` har bir holatni Rust'ga qarshi qayta ijro etadi; ularning hammasi
mos kelmaguncha va `gate` o'tmaguncha hech narsa chiqarilmaydi. Chaqiruvchilar butun jarayon davomida asl API'ni saqlaydi.

Butun ilova ham xuddi shu sikldan birlikma-birlik, butun dasturni isbotlaydigan
ramka ichida o'tadi ([Butun ilovani port qilish](/docs/whole-apps/overview)):

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop above, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in         pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down      proved by
 decisions to make      outside         original answers                                                replay
```

## Sikl

| Qadam | Buyruq | Nima qiladi |
| --- | --- | --- |
| 0. Profil | `everything-rust profile <repo> -- node bench.mjs` | Haqiqiy ish yukini tilning profiler'i ostida ishga tushiradi (node, cProfile, `go test` pprof, JFR, Ruby tracer'i): inklyuziv vaqt bo'yicha issiq funksiyalar, ularning chaqiruvchilari va qaysi sikllarni batch qilish kerakligi; Rust kam yutadigan, allaqachon kompilyatsiya qilingan kodni (numba, Cython, JAX) belgilaydi |
| 1. Reja | `everything-rust analyze <repo>` | Birliklar, port tartibi, `file:line` bilan xavflar, har bir birlik uchun ko'prik, bog'liqlikning crate'ga mosligi, eng yaxshi birinchi portlar; `--whole-app` kirish nuqtalari va shaklni, har bir bog'liqlikni (import qilingan yoki e'lon qilingan) uning crate'i bilan, qismlarni (bir-birini import qiladigan birliklar birga ko'chadi) va tavsiya etilgan javobli raqamlangan qarorlarni qo'shadi |
| 2. Ko'prik | `everything-rust bridge <repo> <unit>` | Binding'lar (`lib.rs` va manba tili yuklaydigan wrapper) hamda port qilinadigan stub'lar (`port.rs`) bilan crate; `--batch fn:param` ko'plab `param` qiymatlarini Rust'da saqlaydigan klass qo'shadi; `--as-json T` ilova faqat ma'lumot bilan to'ldiradigan tip parametrini JSON qiymatlari sifatida ko'prik orqali o'tkazadi |
| 3. Yozib olish | `everything-rust record <crate>` | Asl nusxani generatsiya qilingan chegaraviy holatlar (ASCII bo'lmagan belgilar, emoji, `-0`, `NaN`, float kutilgan joyda int'lar, ...) va har bir parametrdagi har bir semantik tuzoq (string uzunligi va registri, ishorali bo'lish, to'lib ketish chegaralari, float'ni chiqarish, map va saralash tartibi) ustida ishga tushiradi hamda `cases.jsonl` ni saqlaydi; `--from-tests -- npx jest` (yoki `-- pytest`) asl nusxaning o'z test to'plami qiladigan har bir chaqiruvni ham yozib oladi; `--float-tolerance 1e-9` sonli kod uchun float'larga shu ulush doirasida mos kelishga ruxsat beradi |
| 3. Build | `everything-rust build <crate>` | Rust tomonini build qiladi: `.node`, wasm paketi, Python kengaytmasi yoki umumiy kutubxona |
| 3. Diff | `everything-rust diff <crate>` | Yozib olingan holatlarni Rust'ga qarshi, uni asl nusxa chaqiruvchilari chaqirgandek chaqirib, qayta ijro etadi; nomuvofiqliklarni ro'yxatlaydi |
| 3. Bench | `everything-rust bench <crate>` | Yozib olingan holatlarni asl nusxada va Rust'da, ko'prik bilan birga, vaqt bo'yicha o'lchaydi |
| 4. Shadow | `everything-rust shadow <crate>` | Ikkalasini ishga tushiradigan, asl nusxa natijasini qaytaradigan va nomuvofiqliklarni xabar qiladigan o'rinbosar; `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. Gate | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, testlar, `unsafe` auditi, port qilinmagan hech narsa qolmagan, asl nusxaning har bir xato yo'li uchun yozib olingan xatoli holat, asl nusxa xato qaytaradigan joyda `unwrap`/`expect`/`panic!` yo'q (`// INFALLIBLE: <why>` bilan belgilanmagan bo'lsa), build, API, xatti-harakat |
| Agentlar | `everything-rust setup` | MCP serverni (`everything-rust mcp`, har bir qadam vosita sifatida) va workflow skill'ini ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) kodlash agentlaringizda ro'yxatdan o'tkazadi |

## Hot path'lar: ma'lumotni Rust'da saqlang

Har bir element uchun bir marta chaqiriladigan port ko'prik narxini har bir element uchun bir marta to'laydi va JIT'dan yutqazadi. Yutadigan narsa
siklni ko'chirish: `profile` uni topadi, `--batch` esa elementlarni bir marta qabul qiladigan
va har bir chaqiruvda funksiyani ularning hammasi ustida, barcha yadrolarda, tiplangan massiv natijalari bilan bajaradigan klassni generatsiya qiladi.

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore: ishning to'rtdan biri, `matched` tomonidan har bir kalit uchun chaqiriladi: batch qiling
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

`port.rs` dagi `FuzzyScoreIndex` struct'i standart siklni elementlarni bir marta tayyorlaydigan
sikl bilan almashtiradi va o'zi saralay oladi (`rank`): u kalitlarni har biri belgi niqobi bilan
kichik harflarda, har bir kalitda har bir keng tarqalgan belgining birinchi pozitsiyasi jadvalini (bir harfli so'rov bitta
ustun) va oxirgi so'rov mosliklarini saqlaydi (`s` dan keyin `se` yozilsa, faqat `s` mos kelganlar qayta baholanadi). x18n'ning
50,000 kalit bo'yicha yon panel qidiruvida, harfma-harf yozilganda, bir xil natijalar bilan:

| | Har bir so'rovga | TS'ga nisbatan |
| --- | ---: | ---: |
| TS: `map(fuzzyScore)`, filter, sort | 10 ms | 1x |
| Indeks bilan bir xil toraytirishli TS | 4.1-5.4 ms | 2x |
| Xuddi shu port, har bir kalit uchun chaqirilgan | - | 0.2-0.4x |
| `FuzzyScoreBatch`, standart indeks (4 yadro) | 5.1 ms | 1.7x |
| `FuzzyScoreIndex` bilan `FuzzyScoreBatch` (napi, 4 yadro) | 0.45-0.56 ms | **18-22x** |
| Xuddi shu wasm bilan (bitta thread) | 0.95 ms | **11.5x** |

Xuddi shunday toraytirishni bajaradigan TS'ga qarshi ham Rust 7-12x tezroq (wasm 5x). Qaytarilgan
indekslarni oddiy sikl bilan kalitlarga aylantiring: `Array.from(order, i => keys[i])` 20x qimmatroq, 0.4 ms turdi.

Har bir til batch'larni oladi va ular har bir chaqiruvli portlar yutqazadigan joyda ham asl nusxadan o'zib ketadi.
4 yadroda bir xil ish, bir xil natijalar va xatolar (benchmark'lar har bir demo papkasida):

| Til | Ish yuki | Asl nusxa | Har bir chaqiruvli port asl nusxaga nisbatan | Batch asl nusxaga nisbatan |
| --- | --- | ---: | ---: | ---: |
| TS | x18n qidiruvi, 50,000 kalit | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`, 20,000 string | 98 ms | - | **17x** |
| Ruby | `slugify`, 20,000 string | 110 ms | - | **12.7x** |
| Ruby | `shellescape` (stdlib), 50,000 aralash qiymat | 88 ms | - | **11.8x** |
| Go | `Slugify`, 50,000 string | 49 ms | 0.93x | **4x** |
| Java | `slugify`, 50,000 string | 43 ms | 0.68x | **3x** |

Kompilyatsiya qilinadigan asl nusxalar kamroq yutadi: har bir elementdagi ish allaqachon tez, shuning uchun batch yadrolar
soni hisobiga yutadi. Batch crate'lari mimalloc bilan xotira ajratadi: tizim allokatori bilan Go batch'i bitta
thread'dan sekinroq edi. Demo: [`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md).

## `bridge` nimani hal qiladi

- Ko'prikdan o'ta olmaydigan eksportlar sababi ko'rsatilgan holda manba tilida qoladi, shadow
  moduli esa asl nusxani qayta eksport qiladi. TS uchun:
  funksiya qabul qiladigan yoki qaytaradigan funksiyalar, generator va iteratorlar, RegExp, `instanceof` tekshiruvlari,
  generic'lar (`--as-json T` ilova faqat ma'lumot bilan to'ldiradigan bittasini ko'prik orqali o'tkazadi) va tiplari (importlar, qayta eksportlar va `extends` orqali kuzatilgan)
  funksiya saqlaydigan qiymatlar. `--keep "name: reason"` qo'shimchalarini qoldiradi, masalan `Intl` ma'lumotlariga bog'liq kodni.
- TS tiplari Rust tiplariga moslanadi, tip taxalluslari importlar orqali aniqlanadi, parametr tiplari esa
  standart qiymatlardan chiqariladi; tuzilmali qiymatlar generatsiya qilingan binary wire sifatida o'tadi (JSON faqat
  wire modellashtirmaydigan shakllar uchun fallback sifatida).
- JSON `undefined`, NaN, Infinity, -0 yoki yolg'iz surrogatlarni ifodalay olmaydi: tuzilmali qiymatlar
  generatsiya qilingan `index.cjs` tarjima qiladigan belgilarni (`{"$undefined": true}`, `{"$num": "NaN"}`, `{"$utf16": [...]}`) olib yuradi,
  yordamchilar crate'ning `js.rs` ida.

`record` TS manbalarini o'z holicha ishga tushiradi: Node resolve hook'i bundler uslubidagi importlarga amal qiladi
(`./errors` dan `./errors.ts` ga), repoda npm paketlari o'rnatilmagan bo'lsa `--resolve-from` ularni
yetkazib beradi, transformatsiya talab qiladigan TS (enum'lar, parametr xossalari) esa
`--experimental-transform-types` bilan qayta urinib ko'riladi.

## `analyze` nimalarga qaraydi

- Paketlar (`package.json`) va manba fayllari; `node_modules`, build natijalari, fixture'lar,
  playground'lar, vosita konfiguratsiyalari o'tkazib yuboriladi. Vue/Svelte/Astro `<script>` bloklari importlar uchun parse qilinadi.
- **Birliklar**: manba fayllari papkasi yoki paketning manba ildizidagi fayl.
- **To'lqinlar**: birlikning importlari oldingi to'lqinlarda keladi; import sikllari birga port qilinadi.
- **Ko'prik**: birlik yoki u import qiladigan biror narsa Node'ni talab qilsa napi-rs; ko'chiriladigan
  kod uchun wasm-bindgen; UI va muharrir kodi uchun JS'da qoldirish.
- **Tayyorlik** (qanchalik xavfsiz) va **ustuvorlik** (xavfsiz va arziydi: sikllar va string bilan ishlash, testlar, hajm).

| Xavf | Jiddiylik | Misol |
| --- | --- | --- |
| String ofsetlari | yuqori | `slice`, `indexOf`, `matchAll`, `d` bayrog'i: UTF-16 va UTF-8 |
| Regex sintaksisi | yuqori | lookaround, backreference'lar, `y`: `regex` crate'ida yo'q |
| Dinamik regex | yuqori | `new RegExp(userPattern)`: ishga tushish vaqtida JS sintaksisi |
| Dinamik kod | yuqori | `eval`, `new Function`, hisoblangan `import()` |
| Dinamik tiplash | o'rta | `any`, `'key' in value` |
| Tartiblangan to'plamlar | o'rta | `Map`, `Set`, `Object.keys`: qo'shilish tartibi |
| Async, generatorlar | o'rta | promise va generator chegaralari |
| Klass merosi, reflection | o'rta | `extends`, `Proxy`, `Reflect`, `Symbol` |
| Callback ko'rinishidagi API'lar | o'rta | funksiya tipidagi parametrlar va metodlar: sergap ko'priklar |
| Exception'lar, sonlar, JSON | past | `throw`, bit amallari, `JSON.stringify` tartibi |

## Nimalarni aniqlay olmaydi

O'tish Rust yozib olingan har bir holatga mos kelishini anglatadi, boshqa hech narsani emas. Qayta yozishlarni
odatda yozuvdan tashqaridagi narsalar qiynaydi, shuning uchun chegaralar qayerda ekanini biling:

- **Hech kim yozib olmagan kirish ma'lumotlari.** Generatsiya qilingan holatlar, tuzoqlar, asl nusxaning test to'plami
  (`record --from-tests`) va o'zingizning `--inputs` lar shift hisoblanadi. Haqiqiy kirish ma'lumotlarini qo'shing (production
  namunalari, xato hisobotlari); prod'ni buzadigan foydalanuvchi kiritmasi kamdan-kam hollarda generator o'ylab topadigan narsa bo'ladi.
- **O'zgarib ketgan asl nusxa.** `record` birlik fayllarining barmoq izini saqlaydi
  (`recorded-from.json`); asl nusxa keyinroq o'zgarsa, asl nusxa avval nima qilganiga qarshi o'tib ketish o'rniga,
  siz qayta yozib olmaguningizcha `diff` ogohlantiradi va `gate` muvaffaqiyatsiz bo'ladi.
- **Regex'lar.** `regex` crate'ida lookaround yoki backreference yo'q, hatto kompilyatsiya bo'ladigan
  pattern ham ASCII bo'lmagan matnda `\w`, `\b`, `\d` va registrdan mustaqil moslashtirishni JS, Python yoki Java'dan
  boshqacha talqin qilishi mumkin. `fancy-regex` ga almashtirish yoki pattern'ni qayta yozish har bir
  yozib olingan holatdan o'tishi va baribir turkcha nuqtasiz `ı` yoki zero-width joiner'da farq qilishi mumkin. `record`
  regex'lar ishlatiladigan fayldagi har bir funksiyaga regex tuzoqlarini (yuqoridagilar, Kelvin belgisi, uzun s, arab-hind raqamlari, qator ajratgichlari, so'z oxiridagi sigma)
  qo'shadi, lekin haqiqiy pattern'laringizni haqiqiy matnga qarshi fuzz ham qiling.
- **Ishga tushirishlar orasida o'zgaradigan narsalar.** `record` asl nusxani ikki marta ishga tushiradi va nima farq qilganini ajratadi:
  o'zgargan qiymatlar (vaqtlar, id'lar) wildcard'larga aylanadi; boshqa tartibdagi bir xil qatorlar (hash map'lar,
  parallellik) istalgan tartibda solishtiriladi, shuning uchun port baribir aynan o'sha qatorlarni chiqarishi kerak;
  qatorlar sonining farqi istalgan narsaga mos keladigan `...` ga aylanadi. `record` har bir turni ro'yxatlaydi:
  har birini ko'rib chiqing, chunki hash tartibi yoki parallellik farqi aynan portda haqiqatan o'zgarishi
  mumkin bo'lgan narsa.
- **Float'lar.** Sonni matnga formatlash tillar orasida farq qiladi (`1e21`, `-0`, raqamlar soni).
  Tuzoqlar bu qiymatlarni o'z ichiga oladi va ular aynan mos kelishi kerak; `--float-tolerance` faqat
  yaxlitlashni kechiradi, formatlashni hech qachon.
- **Chiqishda bo'lmagan har qanday narsa.** Vaqt, xotira, yuklama ostidagi parallellik va stdout, stderr, exit kodlari,
  yozilgan fayllar va HTTP javoblaridan boshqa yon ta'sirlar (tarmoq chaqiruvlari, ma'lumotlar bazalari,
  signallar) solishtirilmaydi.
- **Boshqa platformalar.** Yozuv xatti-harakatni faqat uni ishga tushirgan mashinada isbotlaydi. Natijalari
  tip o'lchamlari, bayt tartibi yoki kompilyatorga bog'liq bo'lgan C kodi target'da boshqacha ishlashi mumkin; shu sababli `analyze`
  apparatga yaqin C kodini C'da qoldiradi.
## Har bir qadam

| Qadam | Buyruq | Nima qiladi |
| --- | --- | --- |
| 0. Profil | `everything-rust profile <repo> -- node bench.mjs` | Haqiqiy ish yukini tilning profiler'i ostida ishga tushiradi (node, cProfile, `go test` pprof, JFR, Ruby tracer'i): inklyuziv vaqt bo'yicha issiq funksiyalar, ularning chaqiruvchilari va qaysi sikllarni batch qilish kerakligi; Rust kam yutadigan, allaqachon kompilyatsiya qilingan kodni (numba, Cython, JAX) belgilaydi |
| 1. Reja | `everything-rust analyze <repo>` | Birliklar, port tartibi, `file:line` bilan xavflar, har bir birlik uchun ko'prik, bog'liqlikning crate'ga mosligi, eng yaxshi birinchi portlar; `--whole-app` kirish nuqtalari va shaklni, har bir bog'liqlikni (import qilingan yoki e'lon qilingan) uning crate'i bilan, qismlarni (bir-birini import qiladigan birliklar birga ko'chadi) va tavsiya etilgan javobli raqamlangan qarorlarni qo'shadi |
| 2. Ko'prik | `everything-rust bridge <repo> <unit>` | Binding'lar (`lib.rs` va manba tili yuklaydigan wrapper) hamda port qilinadigan stub'lar (`port.rs`) bilan crate; `--batch fn:param` ko'plab `param` qiymatlarini Rust'da saqlaydigan klass qo'shadi; `--as-json T` ilova faqat ma'lumot bilan to'ldiradigan tip parametrini JSON qiymatlari sifatida ko'prik orqali o'tkazadi |
| 3. Yozib olish | `everything-rust record <crate>` | Asl nusxani generatsiya qilingan chegaraviy holatlar (ASCII bo'lmagan belgilar, emoji, `-0`, `NaN`, float kutilgan joyda int'lar, ...) va har bir parametrdagi har bir semantik tuzoq (string uzunligi va registri, ishorali bo'lish, to'lib ketish chegaralari, float'ni chiqarish, map va saralash tartibi) ustida ishga tushiradi hamda `cases.jsonl` ni saqlaydi; `--from-tests -- npx jest` (yoki `-- pytest`) asl nusxaning o'z test to'plami qiladigan har bir chaqiruvni ham yozib oladi; `--float-tolerance 1e-9` sonli kod uchun float'larga shu ulush doirasida mos kelishga ruxsat beradi |
| 3. Build | `everything-rust build <crate>` | Rust tomonini build qiladi: `.node`, wasm paketi, Python kengaytmasi yoki umumiy kutubxona |
| 3. Diff | `everything-rust diff <crate>` | Yozib olingan holatlarni Rust'ga qarshi, uni asl nusxa chaqiruvchilari chaqirgandek chaqirib, qayta ijro etadi; nomuvofiqliklarni ro'yxatlaydi |
| 3. Bench | `everything-rust bench <crate>` | Yozib olingan holatlarni asl nusxada va Rust'da, ko'prik bilan birga, vaqt bo'yicha o'lchaydi |
| 4. Shadow | `everything-rust shadow <crate>` | Ikkalasini ishga tushiradigan, asl nusxa natijasini qaytaradigan va nomuvofiqliklarni xabar qiladigan o'rinbosar; `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. Gate | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, testlar, `unsafe` auditi, port qilinmagan hech narsa qolmagan, asl nusxaning har bir xato yo'li uchun yozib olingan xatoli holat, asl nusxa xato qaytaradigan joyda `unwrap`/`expect`/`panic!` yo'q (`// INFALLIBLE: <why>` bilan belgilanmagan bo'lsa), build, API, xatti-harakat |
| Agentlar | `everything-rust setup` | MCP serverni (`everything-rust mcp`, har bir qadam vosita sifatida) va workflow skill'ini ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) kodlash agentlaringizda ro'yxatdan o'tkazadi |
## Keyingi

Har bir qadamning o'z sahifasi bor: [Profil va tahlil](/docs/concepts/analyze), [Bridge](/docs/concepts/bridge), [Record](/docs/concepts/record), [Port](/docs/concepts/port), [Diff](/docs/concepts/diff), [Bench](/docs/concepts/bench), [Shadow](/docs/concepts/shadow) va [Gate](/docs/concepts/gate).
