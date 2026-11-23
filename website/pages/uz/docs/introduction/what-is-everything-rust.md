---
title: everything-rust nima
description: everything-rust ilovangiz bugun nima qilayotganini yozib oladi va Rust'da qayta yozilgan har bir qismni shu yozuv bilan tekshiradi, toki ilova to'liq Rust bo'lguncha.
---

# everything-rust nima

<p class="lead">Butun ilovangizni Rust'da, qismma-qism qayta yozing. everything-rust Node.js (TypeScript/JavaScript), Python, C, Go, Ruby yoki Java ilovangiz bugun nima qilayotganini yozib oladi, so'ng Rust'da qayta yozilgan har bir qismni keyingisiga o'tishdan oldin shu yozuv bilan tekshiradi, toki ilova to'liq Rust bo'lguncha.</p>

U CLI'lar, HTTP serverlar va kutubxonalar bilan ishlaydi. Rust kodini kodlash agentingiz (Claude Code, Codex, Cursor, Gemini CLI) yoki o'zingiz yozasiz. PHP va C# [preview](/docs/languages/php) holatida.

## U nima qiladi va nima qilmaydi

everything-rust kodni tarjima qilmaydi. U agent ishonchli taxmin qila olmaydigan narsalarni beradi:

- **vaqt qayerga ketishi**: `profile` haqiqiy ish yukini ishga tushiradi va issiq funksiyalarni saralaydi;
- **avval nimani va qanday tartibda port qilish**: `analyze` birliklar, to'lqinlar, xavflar va crate'larni rejalashtiradi;
- **ko'prik**, shunda kodning qolgan qismi Rust'ni API'si o'zgarmagan holda chaqiradi (`bridge`);
- **port mos kelishi shart bo'lgan aniq xatti-harakat**: `record` asl nusxani chegaraviy holatlarda ishga tushiradi va u nima qaytarishi yoki qanday xato tashlashini saqlaydi;
- **uni chiqarishning xavfsiz yo'li**: `shadow` ikkalasini yonma-yon ishga tushiradi;
- **port qachon tayyor ekanini aytadigan gate**: `gate`.

Asl nusxa haqiqat manbai bo'lib qoladi. `diff` har bir yozib olingan holatni Rust'ga qarshi qayta ijro etadi va ularning hammasi mos kelmaguncha hamda `gate` o'tmaguncha hech narsa chiqarilmaydi.

## To'rt qadam

| Qadam | Nima bo'ladi |
| --- | --- |
| **Yozib olish** | `record` asl nusxani generatsiya qilingan chegaraviy holatlar, semantik tuzoqlar va o'z kirish ma'lumotlaringiz ustida ishga tushiradi hamda u nima qaytarishi yoki qanday xato tashlashini saqlaydi. |
| **Port** | `bridge` Rust crate'ini va wrapper'ni generatsiya qiladi, shunda chaqiruvchilar asl API'ni saqlab qoladi. Rust kodini siz yoki agentingiz yozadi. |
| **Tekshirish** | `diff` har bir yozib olingan holatni Rust'ga qarshi qayta ijro etadi va ularning hammasi mos kelmaguncha hamda `gate` o'tmaguncha hech narsa chiqarilmaydi. |
| **Lift** | Qayta ijro fallback'siz o'tganda va har bir birlik gate'dan o'tganda, `app lift` uni toza Rust ilovasiga aylantiradi. Asl nusxaga hech qachon tegilmaydi. |

## Rust har doim tezroqmi?

Yo'q, va everything-rust buni ochiq aytadi. Har bir element uchun bir marta chaqiriladigan kichik funksiya Rust'da ko'pincha sekinroq bo'ladi, chunki har bir chaqiruv ko'prikdan o'tadi; V8, JVM va Go bunday funksiyalarni nanosoniyalarda bajaradi. `profile` uning atrofidagi siklni topadi, `--batch` butun siklni ko'chiradi, `bench` esa chiqarishdan oldin hukm beradi. O'lchangan raqamlar, jumladan sekinlari uchun [Batch'lar va hot path'lar](/docs/concepts/batches) sahifasiga qarang.

## U faqat yozuv qamrab olgan narsani ushlay oladi

O'tish Rust yozib olingan har bir holatga mos kelishini anglatadi, boshqa hech narsani emas. Yashil gate'ga ishonishdan oldin [Nimalarni aniqlay olmaydi](/docs/concepts/limits) sahifasini o'qing.

::: warning Uni nusxada ishga tushiring
`record` va `app record` asl kodingizni ishga tushiradi, har bir funksiyani (yoki butun ilovani) ko'plab generatsiya qilingan kirish ma'lumotlari bilan chaqiradi, `app record` esa har bir holatni ikki marta ishga tushiradi. `diff`, `gate` va `app replay` xuddi shu holatlarni port yoki asl nusxada yana ishga tushiradi. Fayl yozadigan, ma'lumotlar bazasiga murojaat qiladigan yoki tarmoqqa chiqadigan kod buni har bir holat uchun bir marta qiladi. Ularni loyihaning nusxasida, test ma'lumotlar bazasi bilan va muhitda production hisob ma'lumotlarisiz ishga tushiring.
:::

## Keyingi qadamlar

- [Tezkor boshlash](/docs/introduction/quick-start): uni o'rnating va agentingizdan birinchi portni so'rang.
- [Sikl](/docs/concepts/the-loop): bitta birlik uchun har bir qadam.
- [Butun ilovani port qilish](/docs/whole-apps/overview): birinchi yozuvdan toza Rust binary'sigacha.
- [Haqiqiy portlar](/docs/real-world/overview): json-server, Fuse.js, vercel/ms va boshqalar, sekin natijalar tezlari bilan yonma-yon.
