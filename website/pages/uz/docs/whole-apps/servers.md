---
title: Serverlar
description: Xuddi shu buyruqlar HTTP serverlarni isbotlaydi. --http, --fresh, --arg va --pace, shuningdek hech bir birlik javob bermaydigan har bir marshrutni proksi qiladigan axum front door.
---

# Serverlar

Xuddi shu buyruqlar HTTP serverlarni isbotlaydi. `--http` buyruq serverni ishga tushirishini bildiradi; u portini
`PORT` dan yoki buyruqdagi `{port}` dan oladi (har doim bir xil portda tinglaydigan server uchun `--port`):

```sh
everything-rust app record . --http -- node server.js
everything-rust app replay .
everything-rust app front-door .                                    # axum: har bir marshrut proksi qilinadi
everything-rust app wire . api --route 'GET /users/{id}' --route '*'
everything-rust app lift .
```

- So'rovlar ilova hujjatlaridagi `curl` qatorlaridan (`curl -X POST -d ... http://localhost:8080/x`),
  `--inputs` dan (`{"name", "method", "path", "headers", "body"}`) va tuzoqlardan keladi: yo'l segmentlari va query
  qiymatlari CLI argumentlari bilan bir xil tuzoqlarga, body'lar esa matn va JSON tuzoqlariga almashtiriladi, shuningdek
  noma'lum marshrut. Har bir ishga tushirish serverni yangidan boshlaydi va so'rovlarni nom tartibida yuboradi, shuning uchun
  so'rovlar orasidagi holat saqlanadi.
- Holatni fayllarda saqlaydigan server har bir ishga tushishda yangi nusxa oladi: `--fresh db.json` (fayl
  yoki papka, takrorlanishi mumkin) `http/files/` ga snapshot qilinadi va har safar asl nusxa, front door yoki build
  ishga tushganda yangi ishchi papkaga ko'chiriladi; `--arg db.json` har bir ishga tushirish buyruqdan keyin
  oladigan argument, shunda Rust binary'si asl nusxa kabi ishga tushiriladi.
  `--pace 50` serverning javobdan keyin bajaradigan ishi uchun (o'zi hozirgina yozgan narsani qayta yuklaydigan fayl
  kuzatuvchisi) har bir javobdan keyin 50 ms kutadi.
- Holatlar `rust/app-cases/http/<case>.toml` (so'rov, status va muhim header'lar)
  va `<case>.body` (javob body'si, ikki ishga tushirish farq qilgan joylarda `[..]` bilan). Date, server va
  boshqa har bir javobga xos header'lar tashlab qoldiriladi; sizni qiziqtirmaydigan istalgan header qatorini o'chiring.
- Front door bu asl nusxani o'zining alohida portida ishga tushiradigan va hech bir birlik javob bermaydigan har bir marshrutni
  proksi qiladigan axum serveri. `wire --route` axum marshrutlarini qabul qiladi: `METHOD /path/{param}`, istalgan metod uchun `/path`,
  qolgan hamma narsa uchun `*`; bog'lovchi kod marshrutni, uning parametrlarini va
  so'rovni oladi. `lift` proksini olib tashlaydi; shundan keyin HTTP holatlari CI'da `app replay` bilan ishlaydi.
## Haqiqiy misol

[json-server](/docs/real-world/json-server) uchun `--fresh`, `--arg` va `--pace` kerak bo'ldi: u har bir yozuvda o'z ma'lumotlar bazasini qayta yozadi, uning fayl kuzatuvchisi esa tez ketidan keladigan yozuvni bekor qilib yuborishi mumkin.
