---
title: Profil və analiz
description: profile ilə vaxtın hara getdiyini tapın, sonra analyze ilə vahidləri, port ardıcıllığını, riskləri və crate-ləri planlaşdırın.
---

# Profil və analiz

## Profil: köçürməyə dəyəni tapın

Məqsəd sürətdirsə, nəyisə seçməzdən əvvəl ölçün:

```sh
everything-rust profile <repo> -- node bench.mjs      # və ya: python3 bench.py, ruby bench.rb,
                                                    #        java -cp out Main, go test -run X -bench . ./pkg
```

`profile` real iş yükünü dilin profiler-i altında işə salır (node, cProfile, `go test` pprof, JFR, Ruby tracer) və isti funksiyaları daxil olan vaxta görə, onları çağıranları və hansı dövrlərin batch-lənəcəyini siyahıya alır. Go yalnız `go test` vasitəsilə profil çıxarır; C dəstəklənmir.

- Hər element üçün bir dəfə çağırılan kiçik funksiyanı tək port etməyə **dəyməz**: Rust-a bir çağırış onun işindən baha başa gəlir, JIT-lər (V8, JVM, Go) isə onu nanosaniyələrdə işlədir. Onu çağıranları ilə birlikdə port edin və ya [batch edin](/docs/concepts/batches). Tövsiyə sətri hansının olduğunu dəqiq əmrlə deyir.
- Artıq kompilyasiya olunmuş kod (numba `@njit`, Cython, `@jax.jit`) maşın kodu kimi işləyir: Rust portu az sürət qazandırır. `profile` onu "Already compiled" altında siyahıya alır.

## Analiz: portu planlaşdırın

```sh
everything-rust analyze <repo> -o plan.md        # insanlar üçün
everything-rust analyze <repo> --json            # agentlər üçün: sxem everything-rust/analyze@1
everything-rust analyze <repo> --whole-app       # üstəgəl giriş nöqtələri, struktur, asılılıqlar, qərarlar
```

`analyze` vahidləri, port ardıcıllığını, `file:line` ilə riskləri, hər vahid üçün körpünü, hər asılılığın crate-ə nə dərəcədə uyğun gəldiyini və `start_here`-i, yəni ən yaxşı ilk portları siyahıya alır. `--whole-app` tətbiqin strukturunu (CLI, server və ya kitabxana) və giriş nöqtələrini, crate-i ilə birlikdə hər asılılığı (import edilmiş və ya elan edilmiş), hissələri (bir-birini import edən vahidlər birlikdə köçür) və tövsiyə olunan cavabla nömrələnmiş qərarları əlavə edir. Bax [Bütöv tətbiqi port etmək](/docs/whole-apps/overview).

- Paketlər (`package.json`) və mənbə faylları; `node_modules`, build çıxışı, fixture-lar,
  playground-lar və alət konfiqurasiyaları ötürülür. Vue/Svelte/Astro `<script>` blokları import-lar üçün parse edilir.
- **Vahidlər**: mənbə fayllarından ibarət qovluq və ya paketin mənbə kökündəki fayl.
- **Dalğalar**: vahidin import-ları əvvəlki dalğalarda gəlir; import dövrləri birlikdə port edilir.
- **Körpü**: vahidə və ya onun import etdiyi hər hansı şeyə Node lazım olduqda napi-rs; portativ kod üçün
  wasm-bindgen; UI və redaktor kodu üçün JS-də saxlamaq.
- **Hazırlıq** (nə qədər təhlükəsizdir) və **prioritet** (təhlükəsiz və dəyərlidir: dövrlər və string işi, testlər, ölçü).

| Risk | Ciddilik | Nümunə |
| --- | --- | --- |
| String offset-ləri | yüksək | `slice`, `indexOf`, `matchAll`, `d` bayrağı: UTF-16 və UTF-8 |
| Regex sintaksisi | yüksək | lookaround, backreference-lər, `y`: `regex` crate-ində yoxdur |
| Dinamik regex | yüksək | `new RegExp(userPattern)`: icra zamanı JS sintaksisi |
| Dinamik kod | yüksək | `eval`, `new Function`, hesablanmış `import()` |
| Dinamik tipləşdirmə | orta | `any`, `'key' in value` |
| Sıralı kolleksiyalar | orta | `Map`, `Set`, `Object.keys`: əlavə olunma ardıcıllığı |
| Async, generator-lar | orta | promise və generator sərhədləri |
| Sinif irsiliyi, reflection | orta | `extends`, `Proxy`, `Reflect`, `Symbol` |
| Callback formalı API-lər | orta | funksiya tipli parametrlər və metodlar: çox danışan körpülər |
| Exception-lar, rəqəmlər, JSON | aşağı | `throw`, bit əməliyyatları, `JSON.stringify` ardıcıllığı |
## Asılılıqlar üçün crate-lər

`analyze` kodun import etdiyi hər kitabxana üçün Rust crate-ini `good`, `partial` və ya `none` uyğunluğu ilə adlandırır. Cədvəlində olmayan kitabxana `unknown` kimi bildirilir; cədvəl heç vaxt crate təxmin etmir. Birini əlavə etmək üçün bax [Töhfə vermək](/docs/project/contributing#add-a-crate-mapping).
