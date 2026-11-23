---
title: Goşant goşmak
description: Gurşawy sazlamak, testleriň nirede ýerleşýändigi, crate laýyklygyny ýa-da dili nähili goşmaly we hakyky port bilen nähili paýlaşmaly.
---

# Goşant goşmak

Kömegiňiz üçin sag boluň. Ýalňyşlyk habarlary, düzedişler, täze dil goldawy we hakyky portlar barada
ýazgylar hoş görülýär, ýalňyşlygy düzedýän pull request açmazdan öň rugsat soramagyňyz gerek däl.

## Başlamazdan öň

- **Ýalňyşlyk:** işleden buýrugyňyz, onuň çykyşy, platformaňyz we everything-rust wersiýasy
  (`everything-rust --version`) bilen [issue](https://github.com/marprojector/everything-rust/issues/new/choose)
  açyň. Ýalňyşlygy gaýtalaýan kiçijik taslama iň köp kömek edýär.
- **Has uly üýtgeşme** (täze dil, täze buýruk, birligiň ýa-da ýazgynyň näme bolýandygyna üýtgeşme):
  ony ýazmazdan öň görnüşi barada ylalaşmak üçin ilki issue açyň. Öňki kararlaryň sebäpleri
  [DECISIONS.md](https://github.com/marprojector/everything-rust/blob/main/DECISIONS.md) içinde.
- **Howpsuzlyk meselesi:** açyk issue açmaň; [SECURITY.md](https://github.com/marprojector/everything-rust/blob/main/SECURITY.md) faýlyna serediň.

## Gurşawy sazlamak

Size Rust gerek (stable, 1.97 ýa-da täzesi). Testler her bölegiň işleýän dilini hem işledýär we
runtime-y ýok bölegi geçip gidýär, şonuň üçin degýän dilleriňiziňkini gurnaň:

| Näme üçin | Gurnamaly |
| --- | --- |
| TypeScript/JavaScript | Node 24 (`profile`-den başga ýerde 22.6+ hem işleýär) |
| Python | Python 3.9+ |
| C | C kompilýatory (`cc`) |
| Go | Go 1.22+ |
| Ruby | Ruby 3.x |
| Java | JDK (17+) |
| PHP | PHP 8 |
| C# | .NET SDK |
| wasm bridge-leri | `rustup target add wasm32-unknown-unknown` we laýyk `wasm-bindgen-cli` |

```sh
git clone https://github.com/marprojector/everything-rust
cd everything-rust
git config core.hooksPath .githooks
cargo test
```

`cargo run -- doctor` bu runtime-laryň haýsylarynyň sizde bardygyny, olaryň wersiýalaryny we hersini
gurnaýan ýa-da täzeleýän buýrugy sanaýar. Ýok runtime-yň testlerini geçip gitmegiň ýerine şowsuz
etmek üçin (CI şeýle edýär), ony `EVERYTHING_RUST_REQUIRE` içinde görkeziň, meselem
`EVERYTHING_RUST_REQUIRE=node,python3,go,ruby,java cargo test`.

Hook `scripts/check-secrets.sh` işledýär, ol commit edýän zadyňyzdaky açarlary, token-leri we şahsy
öý bukjasynyň ýollaryny ret edýär. CI ony her faýlda işledýär.

## Üýtgeşme girizmek

- **Her düzediş onsuz şowsuz bolýan test bilen gelýär.** Şeýle bolýandygyna göz ýetirmek üçin testi
  bir gezek köne kodda işlediň.
- **Testler nirede ýerleşýär:**
  - `tests/analyze.rs`: dili kesgitlemek, birlikler, töwekgelçilikler, garaşlylyklar, `--whole-app`.
  - `tests/loop.rs`: her dil üçin bridge, record, diff we shadow, gurluşlaryň ýerine çalşyryjylar bilen.
  - `tests/native.rs`: hakykatdan gurlan, soňra ýüklenen we gaýtadan işledilen bridge-ler.
  - `tests/app.rs` we `tests/http.rs`: CLI-ler we serwerler üçin tutuş programmany ýazga almak,
    gaýtadan işletmek, front door we lift.
  - `tests/library.rs`: tutuş kitaphanalaryň portlary.
  - Birlik testleri kodyň ýanynda ýerleşýär.
- **Fixture-lar** `tests/fixtures/` içindäki kiçijik programmalar. CLI we HTTP fixture-lary her dilde
  şol bir buýruklary we marşrutlary durmuşa geçirýär (`tcli upper|len|div|words|copy|now`, `/hello`,
  `/upper`, `/div/{a}/{b}`, `/now`), şonuň üçin täzesini özüni alyp barşy boýunça beýlekiler bilen
  birmeňzeş saklaň.
- **Döredilen kod** (everything-rust-iň ýazýan bridge-leri, draýwerleri we front door-lary) hem
  `clippy -D warnings`-den geçmeli, sebäbi `gate` ony ulanyjylaryň crate-lerinde işledýär.
- **Stil:** töweregiňizdäki koda laýyk geliň. Teswirleri kodyň özi aýdyp bilmeýän zatlar üçin
  galdyryň, üýtgeşmäniň sebäbini bolsa pull request-e ýazyň.

## Crate laýyklygyny goşmak

`analyze` kodyň import edýän her kitaphanasy üçin Rust crate-ini `src/crates.toml`-dan görkezýär.
Täzesini goşmak üçin, onuň diliniň aşagyna ýazgy goşuň: `names` (takyk paket atlary) ýa-da `prefixes`
(Java we C# üçin adyň başy), `crates`, `fit` (`good`, `partial` ýa-da `none`) we laýyklygy düşündirmek
gerek bolsa `note`. Ilkinji gabat gelýän ýazgy ýeňýär, şonuň üçin anyk ýazgylary giň prefikslerden öň
goýuň. Tablisada ýok kitaphana `unknown` diýip habar berilýär; crate-i hiç haçan çaklamaň.

## Dil goşmak

PHP we C# preview hökmünde iki commit-de goşuldy, olary ilki okamak peýdaly:
[e8ac243](https://github.com/marprojector/everything-rust/commit/e8ac243) (PHP) we
[caea59a](https://github.com/marprojector/everything-rust/commit/caea59a) (C#). Täze dil hem şeýle başlaýar:

1. `src/lang/<name>.rs` içinde plugin: tapmak, skan etmek (importlar we töwekgelçilikler), çözmek we
   eksportlar, üstesine onuň kitaphanalarynyň crate-leri `src/crates.toml` içinde `[[<name>]]`
   ýazgylary hökmünde.
2. `src/lang/mod.rs` içinde kesgitlemek we ugratmak.
3. `src/whole_app.rs` içinde tutuş programma goldawy: giriş nokatlary, serwerler, yglan edilen
   garaşlylyklar.
4. CLI we HTTP fixture-lary, kitaphana fixture-y we `tests/analyze.rs`, `tests/app.rs` we
   `tests/http.rs` içindäki testler.
5. Bridge bolýança `bridge` diliň birliklerini düşnükli habar bilen ret edýär.

## README-ni terjime etmek

Terjimeler hoş görülýär we olary jemgyýetçilik goldaýar. Täze terjime:

1. `docs/README_<CODE>.md` içinde ýerleşýär (meselem `docs/README_PT.md`); kökdäki `README.md` iňlisçe
   galýar we hiç haçan adyny üýtgetmeýär ýa-da çalşyrylmaýar.
2. `docs/` içinden işleýän ýollary ulanýar: `assets/logo.svg`, `../LICENSE`, `../README.md`.
3. Beýleki terjimeler ýaly şol bir dil setiri bilen başlaýar, öz dili ýönekeý tekst hökmünde, soňra
   onuň jemgyýetçilik tarapyndan goldanýandygy we iňlisçe README-den yzda galyp biljekdigi barada
   bellik gelýär (ony `docs/README_DE.md`-den göçürip terjime ediň).
4. Dil setiriniň **ahyryna** goşulýar, `README.md`-de hem, her terjimede hem. Eýýäm sanalan dilleriň
   tertibi üýtgemeýär.

## Resminamalar saýty

Resminamalar web saýty `website/` içinde ýerleşýär (Void, `@void/md`, React). Onuň köp sahypasy gurluş
wagtynda `README.md`, `CONTRIBUTING.md` we `docs/*.md` faýllarynyň bölümlerini öz içine alýar, şonuň
üçin şol faýllary üýtgediň, saýt hem yzyndan gelýär. Ony `cd website && npm ci --ignore-scripts && npm run dev`
bilen işlediň we `npm run build` bilen guruň. CLI gollanmasy `website/content/cli-help.json` içindäki
`--help` suratyndan alynýar; CLI üýtgedileninden soň, ony `website/` içinde `cargo build --release` we
`npm run cli-help` bilen täzeläň.

## Pull request açmak

Push etmezden öň:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

- Näme nädogrudygyny, ony nähili gaýtalap bolýandygyny we üýtgeşmäniň ony nähili düzedýändigini aýdyň.
- Issue-ny baglaň (`Fixes #123`).
- Synag edip bilmedik zatlaryňyzy belläň (başga OS, sizde ýok runtime).
- Bir üýtgeşme bilen çäklendiriň: iki düzedişi iki pull request hökmünde gözden geçirmek has aňsat.

CI ähli runtime-lar gurnalan Ubuntu we macOS-da işleýär. Ilkinji gezek goşant goşýanlar üçin GitHub
CI işletmesini tassyklamak üçin maintainer-e garaşýar, şonuň üçin barlaglar başlamazdan öň gysga
gijikmä garaşyň.

Pull request-ler commit-leri saklanyp birleşdirilýär, şonuň üçin taryhyňyz siziňki bolup galýar.
Üýtgeşmä kiçi düzedişler gerek bolsa, maintainer täze tapgyr soramagyň ýerine üstüne commit goşup biler.

## Hakyky port bilen paýlaşmak

Hakyky taslamalaryň portlary nämäniň işleýändigine we nämäniň döwülýändigine iň gowy subutnama.
Eger everything-rust-i hakyky bir zatda ulanan bolsaňyz, ony [docs/real-world.md](/docs/real-world/overview)
faýlyna goşýan issue ýa-da pull request örän hoş görülýär. Taslamany, işleden buýruklaryňyzy, ýagdaý
sanlaryny we sizi geň galdyran islendik zady görkeziň.

## Özüňi alyp barş kodeksi

Gatnaşýanlaryň hemmesinden [özüňi alyp barş kodeksine](https://github.com/marprojector/everything-rust/blob/main/CODE_OF_CONDUCT.md) eýermek garaşylýar.
