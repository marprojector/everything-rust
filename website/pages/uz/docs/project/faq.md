---
title: Ko'p so'raladigan savollar
description: U kodimni avtomatik qayta yozadimi? Rust har doim tezroqmi? Float'larim mos keladimi? Keng tarqalgan savollarga javoblar.
---

# Ko'p so'raladigan savollar

everything-rust haqida odamlar beradigan savollar. [README](/) ga qaytish.

## Ko'p so'raladigan savollar

**U kodimni avtomatik qayta yozadimi?** Yo'q. U nimani ko'chirish kerakligini topadi, ko'prikni generatsiya qiladi,
asl nusxa nima qilishini yozib oladi va Rust'ni unga qarshi tekshiradi. Rust kodini `port.rs` da siz yoki agentingiz
yozadi; u qachon to'g'ri ekanini `diff` va `gate` hal qiladi.

**Ilovamning API'si o'zgaradimi?** Yo'q. Chaqiruvchilar xuddi shu funksiyalarni (yoki ishonch hosil qilayotganingizda
ikkalasini ishga tushirib, asl nusxa natijasini qaytaradigan shadow modulini) import qiladi.

**Rust har doim tezroqmi?** Yo'q, va everything-rust buni ochiq aytadi. Har bir element uchun bir marta chaqiriladigan kichik funksiya
Rust'da ko'pincha sekinroq bo'ladi, chunki har bir chaqiruv ko'prikdan o'tadi; V8, JVM va Go bunday funksiyalarni
nanosoniyalarda bajaradi. `profile` uning atrofidagi siklni topadi, `--batch` butun siklni ko'chiradi, `bench` esa
chiqarishdan oldin hukm beradi. numba, Cython yoki JAX tomonidan allaqachon kompilyatsiya qilingan kod ham mashina
kodi sifatida ishlaydi: `profile` va `analyze` uni belgilaydi, chunki uni port qilish tezlikka kam foyda beradi.

**Kodim sonli; float'lar aynan mos keladimi?** Ko'pincha yo'q: amallar tartibi, fused
multiply-add va matematik kutubxona kompilyatorlar orasida farq qiladi. `--float-tolerance 1e-9` bilan yozib oling
(`record` yoki `app record`), shunda float'lar kattaroq qiymatning shu ulushi doirasida mos keladi (1 dan kichik qiymatlar
uchun mutlaq). Butun sonlar, matn va tuzilma baribir aynan mos kelishi kerak.

**Faqat APK, JAR yoki .exe ko'rinishida bor ilovani port qila oladimi?** Yo'q. U manba kodini port qiladi: u
funksiyalarni parse qiladi, har biri nima qilishini yozib oladi va Rust'ni ishlab turgan asl nusxaga
qarshi tekshiradi. Kompilyatsiya qilingan ilovada bularning hech biri yo'q, va `analyze` buni aytadi. Binary'ning ko'rinadigan
xatti-harakatidan yangi Rust ilovasini yozish boshqa loyiha.

**Sinovdan o'tgan ishonchli C kodini port qilishim kerakmi?** Ko'pincha yo'q, va `analyze` qayerda ekanini aytadi. Apparat
registrlariga tegadigan, kompilyatorining tip o'lchamlari yoki xotira joylashuviga tayanadigan yoki inline assembly ishlatadigan
C kodi (`volatile`, bitfield'lar, packed struct'lar, `__asm__`, manzilga cast qilingan butun sonlar) "C'da
qoldirish" deb belgilanadi: sizning mashinangizdagi yozuv target haqida hech narsani isbotlamaydi, barqaror drayver yoki firmware
kodi esa qayta yozishdan kam foyda oladi. Uning atrofidagi ko'chiriladigan mantiq (parsing, protokollar, ma'lumotlar bilan ishlash)
yaxshi ko'chadi.

**Electron ilovamni port qila oladimi?** Mantiqni ha; oynalar veb sahifa bo'lib qoladi. `analyze
--whole-app` Electron'ni aniqlaydi va renderer o'z holicha qolgan holda main jarayonni qismma-qism
rejalashtiradi. UI'ni Tauri'ga (Rust backend ustidagi o'sha veb UI) ko'chirish mantiq Rust'ga o'tgandan keyingi
alohida qadam.

**C#, Kotlin yoki Swift-chi?** C# preview holatida ([Tillar](/docs/languages/overview) sahifasiga qarang); Kotlin va
Swift hozircha qo'llab-quvvatlanmaydi. Har bir til uchun parser, yozib oluvchi, ko'prik va qayta ijro
drayveri kerak, hammasi boshidan oxirigacha sinovdan o'tgan bo'lishi lozim. Sizga kerak bo'lgan til uchun issue oching (yoki mavjudiga ovoz bering).

**Rust kodi asl nusxadan ancha katta bo'lib ketmaydimi?** Bo'lmasligi kerak. `gate` port
qatorlarini asl nusxa qatorlari bilan solishtiradi va 3x dan oshsa (importlar va signaturalar uchun yana 50 qator) muvaffaqiyatsiz bo'ladi: bu odatda crate allaqachon
qiladigan ishni qo'lda yozish yoki holatlarni birma-bir ko'rib chiqishni anglatadi. `analyze` ishlatiladigan crate'larni ro'yxatlaydi.

**Qismma-qism port qilish Rust'ni eski ilova dizayniga bog'lab qo'ymaydimi?** Faqat ikkalasi yonma-yon
ishlayotgan paytda. Yozuvlar xatti-harakatni ilovaning ichki qismida emas, chegaralarida tekshiradi (CLI chiqishi, server
javoblari, modulning eksport qilingan funksiyalari). Birliklar hali ko'prikdan o'tayotganda eski tuzilmani
aks ettiring; `app lift` asl nusxani olib tashlagach, Rust'ni erkin qayta tuzing, `app replay` esa xatti-harakatni
baribir isbotlaydi. Hech narsa asl nusxaga qayta murojaat ham qilmaydi:
front door uni alohida jarayon sifatida ishga tushiradi (yoki HTTP'ni unga uzatadi), ko'prik orqali ulangan birliklarni
asl nusxa chaqiradi va hech qachon aksincha emas, callback'lar esa manba tilida qoladi.

**Kodim mashinamdan tashqariga chiqadimi?** Yo'q: everything-rust lokal ishlaydi va kodingizning hech bir qismini
hech qayerga yubormaydi. Cargo crate'larni odatdagidek yuklab oladi, agentingiz model provayderi esa agentingiz unga
yuborgan narsani o'z siyosatiga ko'ra ko'radi.

**Nimalar o'rnatilgan bo'lishi kerak?** Rust va siz port qilayotgan tilning runtime'i
([O'rnatish](/docs/introduction/install) sahifasiga qarang).

**Nimadir buzildi.** Buyruq, uning chiqishi va platformangizni ko'rsatib issue oching.
