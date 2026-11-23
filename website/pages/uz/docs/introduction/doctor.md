---
title: Doctor
description: everything-rust doctor til runtime'larini, agent sozlamalarini va MCP serverni tekshiradi hamda har bir muammoni tuzatadigan buyruqni ko'rsatadi.
---

# Doctor

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor` til runtime'larini, agent sozlamalarini va MCP serverni tekshiradi hamda har bir muammoni tuzatadigan buyruqni chiqaradi. Kerakli narsa yetishmasa, u 1 kodi bilan tugaydi.

Runtime, agent sozlamalari yoki MCP server yo'qligi yoki eskirganligi sababli biror qadam muvaffaqiyatsiz bo'lsa, uni ishga tushiring. Workflow skill'i agentlarga ham shunday qilishni aytadi.

## Har bir tilga nima kerak

`record`, `diff` va `bench` asl nusxani o'z runtime'ida ishga tushiradi: TS uchun Node 22.6+ (u `.ts` manbalarini to'g'ridan-to'g'ri ishga tushiradi), Python va C (ctypes) uchun Python 3.9+, Go 1.21+, Ruby 3+, JDK 17+ (`javac` va `java`). Node'da `profile` uchun Node 24+ kerak, u `NODE_OPTIONS` ichida `--cpu-prof` ga ruxsat beradi. C uchun C kompilyatori ham kerak. wasm ko'priklari uchun yana `rustup target add wasm32-unknown-unknown` va mos `wasm-bindgen-cli` kerak; aniq versiyani `build` aytadi.

## Hissa qo'shuvchilar uchun

`cargo run -- doctor` qaysi runtime'lar borligini, ularning versiyalarini va har birini o'rnatadigan yoki yangilaydigan buyruqni ko'rsatadi. [Hissa qo'shish](/docs/project/contributing) sahifasiga qarang.
