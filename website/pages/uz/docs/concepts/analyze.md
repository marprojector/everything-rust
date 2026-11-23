---
title: Profil va tahlil
description: profile bilan vaqt qayerga ketishini toping, so'ng analyze bilan birliklar, port tartibi, xavflar va crate'larni rejalashtiring.
---

# Profil va tahlil

## Profile: ko'chirishga arziydigan narsani toping

Maqsad tezlik bo'lsa, biror narsani tanlashdan oldin o'lchang:

```sh
everything-rust profile <repo> -- node bench.mjs      # yoki: python3 bench.py, ruby bench.rb,
                                                    #     java -cp out Main, go test -run X -bench . ./pkg
```

`profile` haqiqiy ish yukini tilning profiler'i ostida ishga tushiradi (node, cProfile, `go test` pprof, JFR, Ruby tracer'i) va issiq funksiyalarni inklyuziv vaqt bo'yicha, ularning chaqiruvchilari va qaysi sikllarni batch qilish kerakligi bilan ro'yxatlaydi. Go faqat `go test` orqali profil qilinadi; C qo'llab-quvvatlanmaydi.

- Har bir element uchun bir marta chaqiriladigan kichik funksiyani alohida port qilishga **arzimaydi**: Rust'ga bitta chaqiruv uning ishidan qimmatroq, JIT'lar (V8, JVM, Go) esa uni nanosoniyalarda bajaradi. Uni chaqiruvchilari bilan birga port qiling yoki [batch qiling](/docs/concepts/batches). Maslahat qatori qaysi birini aniq buyruq bilan aytadi.
- Allaqachon kompilyatsiya qilingan kod (numba `@njit`, Cython, `@jax.jit`) mashina kodi sifatida ishlaydi: Rust porti tezlikda kam yutadi. `profile` uni "Already compiled" ostida ko'rsatadi.

## Analyze: portni rejalashtiring

```sh
everything-rust analyze <repo> -o plan.md        # odamlar uchun
everything-rust analyze <repo> --json            # agentlar uchun: everything-rust/analyze@1 sxemasi
everything-rust analyze <repo> --whole-app       # qo'shimcha: kirish nuqtalari, shakl, bog'liqliklar, qarorlar
```

`analyze` birliklarni, port tartibini, `file:line` bilan xavflarni, har bir birlik uchun ko'prikni, har bir bog'liqlik crate'ga qanchalik yaxshi moslashishini va `start_here` ni ro'yxatlaydi: eng yaxshi birinchi portlar. `--whole-app` ilovaning shaklini (CLI, server yoki kutubxona) va kirish nuqtalarini, har bir bog'liqlikni (import qilingan yoki e'lon qilingan) uning crate'i bilan, qismlarni (bir-birini import qiladigan birliklar birga ko'chadi) va tavsiya etilgan javobli raqamlangan qarorlarni qo'shadi. [Butun ilovani port qilish](/docs/whole-apps/overview) sahifasiga qarang.

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
## Bog'liqliklar uchun crate'lar

`analyze` kod import qiladigan har bir kutubxona uchun Rust crate'ini `good`, `partial` yoki `none` moslik darajasi bilan nomlaydi. Jadvalda yo'q kutubxona `unknown` deb xabar qilinadi; jadval hech qachon crate'ni taxmin qilmaydi. Yangisini qo'shish uchun [Hissa qo'shish](/docs/project/contributing#add-a-crate-mapping) sahifasiga qarang.
