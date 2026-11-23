---
title: Hissa qo'shish
description: Sozlash, testlar qayerga joylashadi, crate moslamasi yoki til qanday qo'shiladi va haqiqiy port bilan qanday ulashiladi.
---

# Hissa qo'shish

Yordamingiz uchun rahmat. Xato hisobotlari, tuzatishlar, yangi til qo'llab-quvvatlashi va haqiqiy portlar haqidagi yozuvlar
mamnuniyat bilan qabul qilinadi, xatoni tuzatish uchun pull request ochishdan oldin so'rashingiz shart emas.

## Boshlashdan oldin

- **Xato:** ishga tushirgan buyrug'ingiz, uning chiqishi, platformangiz va everything-rust versiyasi
  (`everything-rust --version`) bilan [issue](https://github.com/marprojector/everything-rust/issues/new/choose) oching.
  Kichik qayta ishlab chiqaruvchi loyiha eng ko'p yordam beradi.
- **Kattaroq o'zgarish** (yangi til, yangi buyruq, birlik yoki yozuv nima ekanini o'zgartirish):
  uni yozishdan oldin shakli bo'yicha kelishib olishimiz uchun avval issue oching. O'tgan
  tanlovlar sabablari [DECISIONS.md](https://github.com/marprojector/everything-rust/blob/main/DECISIONS.md) da.
- **Xavfsizlik muammosi:** ommaviy issue ochmang; [SECURITY.md](https://github.com/marprojector/everything-rust/blob/main/SECURITY.md) ga qarang.

## Sozlash

Sizga Rust kerak (stable, 1.97 yoki yangiroq). Testlar har bir qism ishlaydigan tilni ham ishga tushiradi va
runtime'i yo'q qismni o'tkazib yuboradi, shuning uchun o'zingiz tegadiganlarini o'rnating:

| Nima uchun | O'rnatish |
| --- | --- |
| TypeScript/JavaScript | Node 24 (`profile` dan tashqari 22.6+ ham ishlaydi) |
| Python | Python 3.9+ |
| C | C kompilyatori (`cc`) |
| Go | Go 1.22+ |
| Ruby | Ruby 3.x |
| Java | JDK (17+) |
| PHP | PHP 8 |
| C# | .NET SDK |
| wasm ko'priklari | `rustup target add wasm32-unknown-unknown` va mos `wasm-bindgen-cli` |

```sh
git clone https://github.com/marprojector/everything-rust
cd everything-rust
git config core.hooksPath .githooks
cargo test
```

`cargo run -- doctor` bu runtime'lardan qaysilari borligini, ularning versiyalarini va har birini o'rnatadigan yoki yangilaydigan
buyruqni ko'rsatadi. Yo'q runtime testlarini o'tkazib yuborish o'rniga muvaffaqiyatsiz qilish uchun
(CI shunday qiladi) uni `EVERYTHING_RUST_REQUIRE` da nomlang, masalan
`EVERYTHING_RUST_REQUIRE=node,python3,go,ruby,java cargo test`.

Hook `scripts/check-secrets.sh` ni ishga tushiradi, u commit qilayotgan narsangizda kalitlar, tokenlar va shaxsiy home papka
yo'llarini rad etadi. CI uni har bir faylda ishga tushiradi.

## O'zgarishni kiriting

- **Har bir tuzatish usiz muvaffaqiyatsiz bo'ladigan test bilan keladi.** Shunday ekaniga ishonch hosil qilish uchun testni bir marta eski kodda
  ishga tushiring.
- **Testlar qayerga joylashadi:**
  - `tests/analyze.rs`: tilni aniqlash, birliklar, xavflar, bog'liqliklar, `--whole-app`.
  - `tests/loop.rs`: har bir til uchun bridge, record, diff va shadow, build'lar o'rniga o'rinbosarlar bilan.
  - `tests/native.rs`: haqiqatan build qilingan, so'ng yuklangan va qayta ijro etilgan ko'priklar.
  - `tests/app.rs` va `tests/http.rs`: CLI'lar va serverlar uchun butun ilovani yozib olish, qayta ijro, front door va
    lift.
  - `tests/library.rs`: butun kutubxona portlari.
  - Unit testlar kod yonida joylashadi.
- **Fixture'lar** `tests/fixtures/` ostidagi kichik ilovalar. CLI va HTTP fixture'lari har bir tilda bir xil
  buyruqlar va marshrutlarni amalga oshiradi (`tcli upper|len|div|words|copy|now`, `/hello`, `/upper`,
  `/div/{a}/{b}`, `/now`), shuning uchun yangisini xatti-harakat bo'yicha boshqalari bilan bir xil saqlang.
- **Generatsiya qilinadigan kod** (everything-rust yozadigan ko'priklar, drayverlar va front door'lar) ham
  `clippy -D warnings` dan o'tishi kerak, chunki `gate` uni foydalanuvchilarning crate'larida ishga tushiradi.
- **Uslub:** atrofdagi kodga moslang. Izohlarni kod o'zi ayta olmaydigan narsalar uchun qoldiring,
  o'zgarish sabablarini esa pull request'ga yozing.

## Crate moslamasini qo'shish

`analyze` kod import qiladigan har bir kutubxona uchun Rust crate'ini `src/crates.toml` dan nomlaydi. Yangisini
qo'shish uchun uning tili ostiga yozuv qo'shing: `names` (aniq paket nomlari) yoki `prefixes` (nomning
boshlanishi, Java va C# uchun), `crates`, `fit` (`good`, `partial` yoki `none`) va moslik tushuntirishni
talab qilsa `note`. Birinchi mos kelgan yozuv yutadi, shuning uchun aniqlarini keng prefikslardan oldin qo'ying. Jadvalda
yo'q kutubxona `unknown` deb xabar qilinadi; hech qachon crate'ni taxmin qilmang.

## Til qo'shish

PHP va C# preview sifatida ikkita commit'da qo'shilgan, avval ularni o'qishga arziydi:
[e8ac243](https://github.com/marprojector/everything-rust/commit/e8ac243) (PHP) va
[caea59a](https://github.com/marprojector/everything-rust/commit/caea59a) (C#). Yangi til ham
xuddi shunday boshlanadi:

1. `src/lang/<name>.rs` dagi plugin: topish, skanerlash (importlar va xavflar), resolve va eksportlar,
   shuningdek uning kutubxonalari crate'lari `src/crates.toml` da `[[<name>]]` yozuvlari sifatida.
2. `src/lang/mod.rs` da aniqlash va yo'naltirish.
3. `src/whole_app.rs` da butun ilova qo'llab-quvvatlashi: kirish nuqtalari, serverlar, e'lon qilingan bog'liqliklar.
4. CLI va HTTP fixture'lari, kutubxona fixture'i hamda `tests/analyze.rs`, `tests/app.rs`
   va `tests/http.rs` dagi testlar.
5. Ko'prik paydo bo'lmaguncha `bridge` tilning birliklarini aniq xabar bilan rad etadi.

## README'ni tarjima qilish

Tarjimalar mamnuniyat bilan qabul qilinadi va hamjamiyat tomonidan yuritiladi. Yangi tarjima:

1. `docs/README_<CODE>.md` da joylashadi (masalan `docs/README_PT.md`); ildizdagi `README.md` inglizcha
   qoladi va hech qachon qayta nomlanmaydi yoki almashtirilmaydi.
2. `docs/` dan ishlaydigan yo'llardan foydalanadi: `assets/logo.svg`, `../LICENSE`, `../README.md`.
3. Boshqa tarjimalar bilan bir xil til qatoridan boshlanadi, o'z tili oddiy matn sifatida
   bo'ladi, undan keyin u hamjamiyat tomonidan yuritilishi va inglizcha README'dan ortda qolishi mumkinligi
   haqidagi izoh keladi (uni `docs/README_DE.md` dan ko'chirib, tarjima qiling).
4. Til qatorining **oxiriga**, `README.md` da va har bir tarjimada qo'shiladi. Allaqachon ro'yxatdagi
   tillar tartibi o'zgarmaydi.

## Hujjatlar sayti

Hujjatlar veb-sayti `website/` da joylashgan (Void, `@void/md`, React). Uning ko'plab sahifalari build vaqtida
`README.md`, `CONTRIBUTING.md` va `docs/*.md` bo'limlarini o'z ichiga oladi, shuning uchun o'sha fayllarni tahrirlang, sayt
ularga ergashadi. Uni `cd website && npm ci --ignore-scripts && npm run dev` bilan ishga tushiring va
`npm run build` bilan build qiling. CLI ma'lumotnomasi `website/content/cli-help.json` dagi `--help` snapshot'idan
olinadi; CLI'ni o'zgartirgandan keyin uni `cargo build --release` va
`website/` ichida `npm run cli-help` bilan yangilang.

## Pull request oching

Push qilishdan oldin:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

- Nima noto'g'ri bo'lganini, uni qanday takrorlash mumkinligini va o'zgarish uni qanday tuzatishini ayting.
- Issue'ni bog'lang (`Fixes #123`).
- Sinab ko'ra olmagan narsalaringizni qayd eting (boshqa OS, sizda yo'q runtime).
- Bitta o'zgarish bilan cheklaning: ikkita tuzatishni ikkita pull request sifatida ko'rib chiqish osonroq.

CI barcha runtime'lar o'rnatilgan Ubuntu va macOS'da ishlaydi. Birinchi marta hissa qo'shayotganlar uchun GitHub
maintainer CI ishga tushirilishini tasdiqlashini kutadi, shuning uchun tekshiruvlar boshlanishidan oldin qisqa kechikishni kuting.

Pull request'lar commit'lari saqlangan holda merge qilinadi, shunda tarixingiz sizniki bo'lib qoladi. Agar o'zgarish
kichik tuzatishlarni talab qilsa, maintainer yana bir aylanishni so'rash o'rniga ustiga commit qo'shishi mumkin.

## Haqiqiy port bilan ulashing

Haqiqiy loyihalar portlari nima ishlashi va nima buzilishining eng yaxshi dalilidir. Agar
everything-rust'ni haqiqiy narsada ishlatgan bo'lsangiz, uni
[docs/real-world.md](/docs/real-world/overview) ga qo'shadigan issue yoki pull request mamnuniyat bilan qabul qilinadi. Loyihani, ishga tushirgan
buyruqlaringizni, holatlar sonini va sizni hayratga solgan har qanday narsani kiriting.

## Xulq-atvor kodeksi

Ishtirok etayotgan har bir kishi [xulq-atvor kodeksi](https://github.com/marprojector/everything-rust/blob/main/CODE_OF_CONDUCT.md)ga amal qilishi kutiladi.
