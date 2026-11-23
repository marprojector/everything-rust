---
title: everything-rust nədir
description: everything-rust tətbiqinizin bu gün nə etdiyini qeydə alır və Rust-da yenidən yazılmış hər hissəni bu qeydlə yoxlayır, tətbiq təmiz Rust olana qədər.
---

# everything-rust nədir

<p class="lead">Bütün tətbiqinizi Rust-da, hissə-hissə yenidən yazın. everything-rust Node.js (TypeScript/JavaScript), Python, C, Go, Ruby və ya Java tətbiqinizin bu gün nə etdiyini qeydə alır, sonra Rust-da yenidən yazılmış hər hissəni növbətiyə keçməzdən əvvəl bu qeydlə yoxlayır, tətbiq təmiz Rust olana qədər.</p>

CLI-lar, HTTP serverləri və kitabxanalar üzərində işləyir. Rust-u kodlaşdırma agentiniz (Claude Code, Codex, Cursor, Gemini CLI) və ya siz yazırsınız. PHP və C# [önizləmədədir](/docs/languages/php).

## Nə edir, nə etmir

everything-rust kodu tərcümə etmir. Agentin etibarlı şəkildə təxmin edə bilmədiyini təmin edir:

- **vaxtın hara getdiyini**: `profile` real iş yükünü işə salır və isti funksiyaları sıralayır;
- **əvvəlcə nəyi və hansı ardıcıllıqla port etməyi**: `analyze` vahidləri, dalğaları, riskləri və crate-ləri planlaşdırır;
- kodun qalan hissəsinin Rust-u API-si dəyişmədən çağırması üçün **körpünü** (`bridge`);
- **portun uyğun gəlməli olduğu dəqiq davranışı**: `record` orijinalı kənar hallarla işə salır və nə qaytardığını və ya hansı xətanı atdığını saxlayır;
- **onu buraxmağın təhlükəsiz yolunu**: `shadow` hər ikisini yan-yana işə salır;
- **portun nə vaxt hazır olduğunu deyən gate-i**: `gate`.

Orijinal həqiqət mənbəyi olaraq qalır. `diff` hər qeydə alınmış halı Rust-a qarşı təkrar oynadır və hamısı uyğun gəlib `gate` keçənə qədər heç nə buraxılmır.

## Dörd addım

| Addım | Nə baş verir |
| --- | --- |
| **Qeyd** | `record` orijinalı yaradılmış kənar hallar, semantik tələlər və sizin öz girişlərinizlə işə salır və nə qaytardığını və ya hansı xətanı atdığını saxlayır. |
| **Port** | `bridge` Rust crate-ini və wrapper yaradır ki, çağıranlar orijinal API-ni saxlasın. Rust-u siz və ya agentiniz yazır. |
| **Yoxlama** | `diff` hər qeydə alınmış halı Rust-a qarşı təkrar oynadır və hamısı uyğun gəlib `gate` keçənə qədər heç nə buraxılmır. |
| **Lift** | Təkrar oynatma fallback olmadan keçəndə və hər vahid gate-dən keçəndə, `app lift` onu təmiz Rust tətbiqinə çevirir. Orijinala heç vaxt toxunulmur. |

## Rust həmişə daha sürətlidirmi?

Xeyr, və everything-rust bunu deyir. Hər element üçün bir dəfə çağırılan kiçik funksiya Rust-da çox vaxt daha yavaş olur, çünki hər çağırış körpünü keçir; V8, JVM və Go belə funksiyaları nanosaniyələrdə işlədir. `profile` onun ətrafındakı dövrü tapır, `--batch` bütöv dövrü köçürür, `bench` isə siz buraxmazdan əvvəl hökm verir. Ölçülmüş rəqəmlər, yavaşlar da daxil olmaqla, üçün bax [Batch-lər və hot path-lar](/docs/concepts/batches).

## Yalnız qeydin əhatə etdiyini tuta bilər

Keçmək Rust-un qeydə alınmış hər hala uyğun gəldiyi deməkdir, artıq heç nə. Yaşıl gate-ə etibar etməzdən əvvəl [Nəyi tuta bilmir](/docs/concepts/limits) səhifəsini oxuyun.

::: warning Onu surət üzərində işə salın
`record` və `app record` orijinal kodunuzu işə salır, hər funksiyanı (və ya bütöv tətbiqi) çoxlu yaradılmış girişlə çağırır, `app record` isə hər halı iki dəfə işlədir. `diff`, `gate` və `app replay` eyni halları port və ya orijinal üzərində yenidən işə salır. Fayl yazan, verilənlər bazası ilə danışan və ya şəbəkəyə müraciət edən kod bunu hər hal üçün bir dəfə edir. Onları layihənin surətində, test verilənlər bazası ilə və mühitdə production etimadnamələri olmadan işə salın.
:::

## Növbəti addımlar

- [Sürətli başlanğıc](/docs/introduction/quick-start): onu quraşdırın və agentinizdən ilk portu istəyin.
- [Dövr](/docs/concepts/the-loop): bir vahid üçün hər addım.
- [Bütöv tətbiqi port etmək](/docs/whole-apps/overview): ilk qeyddən təmiz Rust binary-sinə qədər.
- [Real portlar](/docs/real-world/overview): json-server, Fuse.js, vercel/ms və daha çoxu, yavaş nəticələr sürətlilərin yanında.
