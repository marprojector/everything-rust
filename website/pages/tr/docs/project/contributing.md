---
title: Katkıda bulunma
description: Kurulum, testlerin nereye gittiği, bir crate eşlemesinin ya da bir dilin nasıl ekleneceği ve gerçek bir portun nasıl paylaşılacağı.
---

# Katkıda bulunma

Yardımınız için teşekkürler. Hata raporları, düzeltmeler, yeni dil desteği ve gerçek portların
yazılı anlatımları memnuniyetle karşılanır; bir hata düzeltmesi için pull request açmadan önce sormanız gerekmez.

## Başlamadan önce

- **Bir hata:** çalıştırdığınız komutu, çıktısını, platformunuzu ve everything-rust sürümünü
  (`everything-rust --version`) içeren bir [issue](https://github.com/marprojector/everything-rust/issues/new/choose) açın.
  En çok küçük bir yeniden üretim projesi yardımcı olur.
- **Daha büyük bir değişiklik** (yeni bir dil, yeni bir komut, bir birimin ya da kaydın ne olduğuna dair bir değişiklik):
  siz yazmadan önce biçimi üzerinde anlaşabilmemiz için önce bir issue açın. Geçmiş seçimlerin
  gerekçeleri [DECISIONS.md](https://github.com/marprojector/everything-rust/blob/main/DECISIONS.md) içindedir.
- **Bir güvenlik sorunu:** herkese açık bir issue açmayın; bkz. [SECURITY.md](https://github.com/marprojector/everything-rust/blob/main/SECURITY.md).

## Kurulum

Rust gerekir (stable, 1.97 veya daha yeni). Testler ayrıca her parçanın ele aldığı dili çalıştırır ve
runtime'ı eksik olan parçayı atlar, bu yüzden dokunduğunuz dillerinkini kurun:

| Ne için | Kurulacak |
| --- | --- |
| TypeScript/JavaScript | Node 24 (`profile` dışında 22.6+ çalışır) |
| Python | Python 3.9+ |
| C | bir C derleyicisi (`cc`) |
| Go | Go 1.22+ |
| Ruby | Ruby 3.x |
| Java | bir JDK (17+) |
| PHP | PHP 8 |
| C# | .NET SDK |
| wasm köprüleri | `rustup target add wasm32-unknown-unknown` ve buna uyan `wasm-bindgen-cli` |

```sh
git clone https://github.com/marprojector/everything-rust
cd everything-rust
git config core.hooksPath .githooks
cargo test
```

`cargo run -- doctor`, bu runtime'lardan hangilerine sahip olduğunuzu, sürümlerini ve her birini
kuran ya da güncelleyen komutu listeler. Eksik bir runtime'ın testlerini atlamak yerine başarısız
kılmak için (CI bunu yapar) onu `EVERYTHING_RUST_REQUIRE` içinde adlandırın, örneğin
`EVERYTHING_RUST_REQUIRE=node,python3,go,ruby,java cargo test`.

Hook, commit'lediklerinizde anahtarları, token'ları ve kişisel home klasörü yollarını reddeden
`scripts/check-secrets.sh`'i çalıştırır. CI bunu her dosyada çalıştırır.

## Değişikliği yapın

- **Her düzeltme, o düzeltme olmadan başarısız olan bir testle gelir.** Başarısız olduğundan emin
  olmak için testi bir kez eski koda karşı çalıştırın.
- **Testler nereye gider:**
  - `tests/analyze.rs`: dil algılama, birimler, riskler, bağımlılıklar, `--whole-app`.
  - `tests/loop.rs`: dil başına bridge, record, diff ve shadow; derlemeler yerine yer tutucularla.
  - `tests/native.rs`: gerçekten derlenen, ardından yüklenip yeniden oynatılan köprüler.
  - `tests/app.rs` ve `tests/http.rs`: CLI'lar ve sunucular için tüm uygulama record, replay, front door ve
    lift.
  - `tests/library.rs`: tüm kütüphane portları.
  - Birim testleri kodun yanında yaşar.
- **Fixture'lar**, `tests/fixtures/` altındaki küçük uygulamalardır. CLI ve HTTP fixture'ları her dilde
  aynı komutları ve route'ları uygular (`tcli upper|len|div|words|copy|now`, `/hello`, `/upper`,
  `/div/{a}/{b}`, `/now`), bu yüzden yenisini davranış olarak diğerleriyle aynı tutun.
- **Üretilen kod** (everything-rust'in yazdığı köprüler, sürücüler ve front door'lar) da
  `clippy -D warnings`'i geçmelidir, çünkü `gate` bunu kullanıcıların crate'lerinde çalıştırır.
- **Stil:** çevrenizdeki koda uyun. Yorumları kodun kendi başına söyleyemediği şeyler için saklayın
  ve bir değişikliğin gerekçesini bunun yerine pull request'e yazın.

## Bir crate eşlemesi ekleyin

`analyze`, kodun import ettiği her kütüphane için Rust crate'ini `src/crates.toml`'dan adlandırır. Bir
tane eklemek için kendi dilinin altına bir girdi ekleyin: `names` (tam paket adları) ya da `prefixes`
(Java ve C# için bir adın başlangıcı), `crates`, `fit` (`good`, `partial` veya `none`) ve uyumun
açıklanması gerekiyorsa bir `note`. Eşleşen ilk girdi kazanır, bu yüzden özel olanları geniş
öneklerden önce koyun. Tabloda olmayan bir kütüphane `unknown` olarak raporlanır; asla crate tahmin etmeyin.

## Bir dil ekleyin

PHP ve C# önizleme olarak, önce okunmaya değer iki commit'te eklendi:
[e8ac243](https://github.com/marprojector/everything-rust/commit/e8ac243) (PHP) ve
[caea59a](https://github.com/marprojector/everything-rust/commit/caea59a) (C#). Yeni bir dil de aynı
şekilde başlar:

1. `src/lang/<name>.rs` içinde bir plugin: keşif, tarama (import'lar ve riskler), çözümleme ve export'lar;
   ayrıca kütüphanelerinin crate'leri `src/crates.toml` içinde `[[<name>]]` girdileri olarak.
2. `src/lang/mod.rs` içinde algılama ve yönlendirme.
3. `src/whole_app.rs` içinde tüm uygulama desteği: giriş noktaları, sunucular, bildirilen bağımlılıklar.
4. CLI ve HTTP fixture'ları, bir kütüphane fixture'ı ve `tests/analyze.rs`, `tests/app.rs`
   ile `tests/http.rs` içinde testler.
5. Bir köprü var olana kadar `bridge` dilin birimlerini açık bir mesajla reddeder.

## README'yi çevirin

Çeviriler memnuniyetle karşılanır ve topluluk tarafından sürdürülür. Yeni bir çeviri:

1. `docs/README_<CODE>.md` içinde yaşar (örneğin `docs/README_PT.md`); kökteki `README.md` İngilizce
   kalır ve asla yeniden adlandırılmaz ya da değiştirilmez.
2. `docs/` içinden çalışan yollar kullanır: `assets/logo.svg`, `../LICENSE`, `../README.md`.
3. Diğer çevirilerle aynı dil satırıyla başlar, kendi dili düz metin olarak yazılır; ardından
   topluluk tarafından sürdürüldüğünü ve İngilizce README'nin gerisinde kalabileceğini belirten not
   gelir (`docs/README_DE.md`'den kopyalayıp çevirin).
4. Dil satırının **sonuna** eklenir; hem `README.md`'de hem de her çeviride. Zaten listelenmiş
   dillerin sırası değişmez.

## Dokümantasyon sitesi

Dokümantasyon sitesi `website/` içinde yaşar (Void, `@void/md`, React). Sayfalarının çoğu derleme
sırasında `README.md`, `CONTRIBUTING.md` ve `docs/*.md` dosyalarının bölümlerini içerir, bu yüzden o
dosyaları düzenleyin, site onları izler. `cd website && npm ci --ignore-scripts && npm run dev` ile
çalıştırın ve `npm run build` ile derleyin. CLI başvurusu, `website/content/cli-help.json` içindeki bir
`--help` anlık görüntüsünden gelir; CLI'ı değiştirdikten sonra `website/` içinde `cargo build --release`
ve `npm run cli-help` ile yenileyin.

## Pull request'i açın

Push'lamadan önce:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

- Neyin yanlış olduğunu, nasıl yeniden üretileceğini ve değişikliğin onu nasıl düzelttiğini söyleyin.
- Issue'yu bağlayın (`Fixes #123`).
- Test edemediğiniz her şeyi not edin (başka bir işletim sistemi, sizde olmayan bir runtime).
- Tek bir değişiklikle sınırlı tutun: iki düzeltmeyi iki pull request olarak incelemek daha kolaydır.

CI, her runtime kurulu olarak Ubuntu ve macOS üzerinde çalışır. İlk kez katkıda bulunanlar için GitHub,
CI çalıştırmasını bir maintainer'ın onaylamasını bekler, bu yüzden kontroller başlamadan önce kısa bir gecikme bekleyin.

Pull request'ler commit'leri korunarak birleştirilir, böylece geçmişiniz sizin kalır. Bir değişikliğin
küçük düzeltmelere ihtiyacı varsa, bir maintainer yeni bir tur istemek yerine üstüne bir commit ekleyebilir.

## Gerçek bir portu paylaşın

Gerçek projelerin portları, neyin çalışıp neyin bozulduğunun en iyi kanıtıdır. everything-rust'i gerçek
bir şey üzerinde kullandıysanız, onu [docs/real-world.md](/docs/real-world/overview)'ye ekleyen bir issue
ya da pull request çok memnuniyetle karşılanır. Projeyi, çalıştırdığınız komutları, vaka sayılarını ve
sizi şaşırtan her şeyi ekleyin.

## Davranış kuralları

Katılan herkesin [davranış kurallarına](https://github.com/marprojector/everything-rust/blob/main/CODE_OF_CONDUCT.md) uyması beklenir.
