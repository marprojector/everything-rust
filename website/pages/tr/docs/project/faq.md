---
title: SSS
description: Kodumu otomatik olarak yeniden yazar mı? Rust her zaman daha mı hızlı? Float'larım eşleşir mi? Sık sorulan soruların yanıtları.
---

# SSS

everything-rust hakkında sorulan sorular. [README](/)'ye dön.

## SSS

**Kodumu otomatik olarak yeniden yazar mı?** Hayır. Neyin taşınacağını bulur, köprüyü üretir,
orijinalin ne yaptığını kaydeder ve Rust'ı buna karşı kontrol eder. Rust'ı `port.rs` içinde siz ya da
ajanınız yazar; ne zaman doğru olduğuna `diff` ve `gate` karar verir.

**Uygulamamın API'si değişecek mi?** Hayır. Çağıranlar aynı fonksiyonları import eder (ya da siz güven
kazanırken ikisini de çalıştırıp orijinalin sonucunu döndüren shadow modülünü).

**Rust her zaman daha mı hızlı?** Hayır, ve everything-rust bunu söyler. Her öğe için bir kez çağrılan
küçük bir fonksiyon Rust'ta çoğu zaman daha yavaştır, çünkü her çağrı bir köprüyü geçer; V8, JVM ve Go
bu tür fonksiyonları nanosaniyeler içinde çalıştırır. `profile` onu çevreleyen döngüyü bulur, `--batch`
döngünün tamamını taşır ve `bench` siz yayına almadan önce bir karar verir. numba, Cython veya JAX ile
zaten derlenmiş kod da makine kodu olarak çalışır: `profile` ve `analyze` bunu işaretler, çünkü onu port
etmek pek hız kazandırmaz.

**Kodum sayısal; float'lar tam olarak eşleşir mi?** Çoğu zaman hayır: işlem sırası, fused
multiply-add ve matematik kütüphanesi derleyiciden derleyiciye değişir. `--float-tolerance 1e-9` ile
kaydedin (`record` ya da `app record`); float'lar o zaman büyük değerin bu oranı içinde eşleşir (1'in
altında mutlak değer olarak). Tam sayılar, metin ve yapı yine tam olarak eşleşmelidir.

**Elimde yalnızca APK, JAR ya da .exe olarak bulunan bir uygulamayı port edebilir mi?** Hayır. Kaynak
kodu port eder: fonksiyonları ayrıştırır, her birinin ne yaptığını kaydeder ve Rust'ı çalışan orijinale
karşı kontrol eder. Derlenmiş bir uygulamada bunların hiçbiri yoktur ve `analyze` bunu söyler. Bir
binary'nin görünür davranışından yeni bir Rust uygulaması yazmak ayrı bir projedir.

**Sahada sınanmış C kodunu port etmeli miyim?** Çoğu zaman hayır ve `analyze` nerede olduğunu söyler.
Donanım yazmaçlarına dokunan, derleyicinin tip boyutlarına veya bellek düzenine dayanan ya da satır içi
assembly kullanan (`volatile`, bitfield'lar, packed struct'lar, `__asm__`, adrese dönüştürülen tam
sayılar) C kodu "C'de tut" olarak işaretlenir: sizin makinenizdeki bir kayıt hedef hakkında hiçbir şey
kanıtlamaz ve kararlı sürücü ya da firmware kodu yeniden yazımdan pek kazanç sağlamaz. İyi taşınan,
onu çevreleyen taşınabilir mantıktır (ayrıştırma, protokoller, veri işleme).

**Electron uygulamamı port edebilir mi?** Mantığını evet; pencereler web sayfası olarak kalır. `analyze
--whole-app` Electron'u fark eder ve renderer olduğu gibi kalırken main process'i parça parça planlar.
UI'ı Tauri'ye taşımak (bir Rust backend üzerinde aynı web UI) mantık Rust olduktan sonra ayrı bir adımdır.

**Peki C#, Kotlin veya Swift?** C# önizlemededir (bkz. [Diller](/docs/languages/overview)); Kotlin ve
Swift henüz desteklenmiyor. Her dil, hepsi uçtan uca test edilmiş bir ayrıştırıcı, bir kaydedici, bir
köprü ve bir yeniden oynatma sürücüsü gerektirir. İhtiyacınız olan dil için bir issue açın (ya da var olana oy verin).

**Rust kodu orijinalden çok daha büyük olmaz mı?** Olmamalı. `gate` portun satır sayısını orijinalinkiyle
karşılaştırır ve 3x'in üzerinde başarısız olur (import'lar ve imzalar için artı 50 satır): bu genellikle
bir crate'in zaten yaptığını elle yazmak ya da vakaları tek tek ele almak demektir. `analyze` kullanılacak
crate'leri listeler.

**Parça parça port etmek Rust'ı eski uygulamanın tasarımına kilitlemez mi?** Yalnızca ikisi yan yana
çalıştığı sürece. Kayıtlar davranışı uygulamanın iç yapısında değil, kenarlarında kontrol eder (bir
CLI'ın çıktısı, bir sunucunun yanıtları, bir modülün export edilen fonksiyonları). Birimler hâlâ köprüyü
geçerken eski yapıyı yansıtın; `app lift` orijinali kaldırdıktan sonra Rust'ı özgürce yeniden
yapılandırın, `app replay` davranışı hâlâ kanıtlar. Hiçbir şey orijinale geri çağrı da yapmaz: front door
onu ayrı bir süreç olarak çalıştırır (ya da HTTP'yi ona iletir), köprülenmiş birimler orijinal tarafından
çağrılır, asla tersi değil, ve callback'ler kaynak dilde kalır.

**Kodum makinemden çıkar mı?** Hayır: everything-rust yerel olarak çalışır ve kodunuzun hiçbir kısmını
hiçbir yere göndermez. Cargo crate'leri her zamanki gibi indirir ve ajanınızın model sağlayıcısı, kendi
politikası çerçevesinde ajanınızın ona gönderdiklerini görür.

**Neyin kurulu olması gerekir?** Rust, artı port ettiğiniz dilin runtime'ı (bkz.
[Kurulum](/docs/introduction/install)).

**Bir şey bozuldu.** Komutu, çıktısını ve platformunuzu içeren bir issue açın.
