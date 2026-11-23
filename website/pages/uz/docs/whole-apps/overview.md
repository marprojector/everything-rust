---
title: Butun ilovani port qilish
description: Birinchi yozuvdan toza Rust binary'sigacha. Maqsad asl nusxa kabi ishlaydigan Rust ilovasi, asl nusxa esa namuna sifatida saqlanadi.
---

# Butun ilovani port qilish

Maqsad asl nusxa kabi ishlaydigan toza Rust ilovasi, asl nusxa esa namuna sifatida saqlanadi. Rust kodini agentingiz yozadi; everything-rust ishni tartibga soladi, karkasini yaratadi va har bir qadamni isbotlaydi. CLI'lar, HTTP serverlar va kutubxonalar.

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in    pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down proved by
 decisions to make      outside         original answers                                           replay
```

## 1. Rejalashtirish

```sh
everything-rust analyze . --whole-app
```

`--whole-app` ilovaning shaklini (CLI, server yoki kutubxona) va kirish nuqtalarini, har bir tashqi bog'liqlikni (import qilingan yoki e'lon qilingan) uning Rust crate'i va mosligi bilan, port tartibini qismlar sifatida (bir-birini import qiladigan birliklar birga ko'chadi) va tavsiya etilgan javobli raqamlangan qarorlarni qo'shadi: qoldiriladigan birliklar, bloklangan birliklar, crate'i yo'q yoki qisman bo'lgan bog'liqliklar.

## 2. Butun ilovani yozib oling, so'ng kirish nuqtasini ko'chiring

1. [`app record` va `app replay`](/docs/whole-apps/record-replay) dasturni tashqaridan, qora quti sifatida isbotlaydi.
2. [`app front-door`](/docs/whole-apps/front-door) asl nusxaning oldiga Rust binary'sini qo'yadi; har bir buyruq hali ham asl nusxani ishga tushiradi.
3. Har bir birlik [sikl](/docs/concepts/the-loop)dan o'tadi, so'ng `app wire` uning buyruqlarini Rust'ga topshiradi.
4. Qayta ijro 0 fallback ulushi bilan o'tgach va har bir birlik gate'dan o'tgach, [`app lift`](/docs/whole-apps/lift) asl nusxani olib tashlaydi.

[Serverlar](/docs/whole-apps/servers) va [kutubxonalar](/docs/whole-apps/libraries) xuddi shu buyruqlardan foydalanadi.

## Agent buni qanday bajaradi

[Butun ilova skill'i](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md) agentni bosqichma-bosqich olib boradi, faqat siz qabul qila oladigan qarorlardan boshlab:

- **Grilling (savolga tutish)**: agent bosqichma-bosqich, har bir savolni tavsiya etilgan javobi bilan so'raydi, toki port bog'liq bo'lgan har bir qaror hal bo'lmaguncha, va har birini `rust/DECISIONS.md` ga ADR sifatida yozadi, `rust/GLOSSARY.md` esa asl nusxa atamalarini Rust nomlariga moslaydi.
- **Tadqiqot**: fon agentlari har bir bog'liqlikning asosiy manbalarini o'qiydi va topgan har bir semantik tuzoq bilan `rust/research/<topic>.md` ni yozadi.
- **Sessiyalar bo'ylab**: `rust/MAP.md` har bir birlikni holati bilan ro'yxatlaydi; sessiya boshlanishidan oldin bitta birlikni o'ziga oladi va har bir nazorat nuqtasi keyingi sessiya uchun `rust/HANDOFF.md` qoldiradi. Bitta to'lqindagi birliklarni subagent'lar parallel ravishda port qilishi mumkin.
- **O'tish (cutover)**: agent qo'lidan kelganini qiladi va `rust/cutover.sh` ni yozadi, bu faqat siz bajara oladigan qadamlar (deploy'lar, DNS, maxfiy kalitlar) uchun interaktiv skript bo'lib, har bir qadamda orqaga qaytish yo'li bor.

Grilling, tadqiqot va bu amaliyotlar [Matt Pocock'ning skill'laridan](https://github.com/mattpocock/skills) moslashtirilgan.

## Bu Rust'ni eski dizaynga bog'lab qo'ymaydimi?

Faqat ikkalasi yonma-yon ishlayotgan paytda. Yozuvlar xatti-harakatni ilovaning ichki qismida emas, chegaralarida tekshiradi (CLI chiqishi, server javoblari, modulning eksport qilingan funksiyalari). Birliklar hali ko'prikdan o'tayotganda eski tuzilmani aks ettiring; `app lift` asl nusxani olib tashlagach, Rust'ni erkin qayta tuzing, `app replay` esa xatti-harakatni baribir isbotlaydi.

## Haqiqiy misol

[json-server](/docs/real-world/json-server) har bir qadamdan o'tdi: 514 ta yozib olingan HTTP holat, 99.8% fallback bilan front door, avval o'qishlar ulandi (22.6% fallback), so'ng har bir marshrut (0%), va 514/514 dan o'tadigan toza Rust binary'siga lift.
