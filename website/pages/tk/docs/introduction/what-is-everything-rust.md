---
title: everything-rust näme
description: everything-rust programmaňyzyň häzir näme edýändigini ýazga alýar we Rust-da täzeden ýazylan her bölegi şol ýazgy bilen barlaýar, programma arassa Rust bolýança.
---

# everything-rust näme

<p class="lead">Tutuş programmaňyzy Rust-da bölek-bölekden täzeden ýazyň. everything-rust siziň Node.js (TypeScript/JavaScript), Python, C, Go, Ruby ýa-da Java programmaňyzyň häzir näme edýändigini ýazga alýar, soňra Rust nusgasynyň her bölegini indiki bölege geçmezden öň şol ýazgy bilen deňeşdirip barlaýar, programma arassa Rust bolýança.</p>

Ol CLI-ler, HTTP serwerler we kitaphanalar bilen işleýär. Rust koduny siziň kodlaýyş agentiňiz (Claude Code, Codex, Cursor, Gemini CLI) ýa-da özüňiz ýazýarsyňyz. PHP we C# [preview](/docs/languages/php) ýagdaýynda.

## Ol näme edýär we näme etmeýär

everything-rust kody terjime etmeýär. Ol agentiň ygtybarly çaklap bilmeýän zatlaryny berýär:

- **wagt nirä gidýär**: `profile` hakyky iş ýüküni işledýär we gyzgyn funksiýalary tertipleşdirýär;
- **ilki näme we haýsy tertipde geçirmeli**: `analyze` birlikleri, tolkunlary, töwekgelçilikleri we crate-leri meýilleşdirýär;
- **bridge**, galan kod Rust-y API-sini üýtgetmezden çagyrar ýaly (`bridge`);
- **portuň gabat gelmeli takyk özüni alyp barşy**: `record` asyl nusgany çäk ýagdaýlarynda işledýär we onuň näme gaýtarýanyny ýa-da näme taşlaýanyny saklaýar;
- **ony çykarmagyň howpsuz ýoly**: `shadow` ikisini ýanaşyk işledýär;
- **portuň haçan taýýardygyny aýdýan gate**: `gate`.

Asyl nusga hakykatyň çeşmesi bolup galýar. `diff` ýazga alnan her ýagdaýy Rust-a garşy gaýtadan işledýär we olaryň hemmesi gabat gelýänçä we `gate` geçýänçä hiç zat çykarylmaýar.

## Dört ädim

| Ädim | Näme bolýar |
| --- | --- |
| **Ýazga almak** | `record` asyl nusgany döredilen çäk ýagdaýlarynda, semantik duzaklarda we siziň öz girişleriňizde işledýär we onuň näme gaýtarýanyny ýa-da näme taşlaýanyny saklaýar. |
| **Geçirmek** | `bridge` Rust crate-ini we wrapper döredýär, şonuň üçin çagyryjylar asyl API-ni saklaýar. Rust koduny siz ýa-da agentiňiz ýazýar. |
| **Barlamak** | `diff` ýazga alnan her ýagdaýy Rust-a garşy gaýtadan işledýär we olaryň hemmesi gabat gelýänçä we `gate` geçýänçä hiç zat çykarylmaýar. |
| **Lift** | Replay hiç hili fallback bolmazdan geçende we her birlik gate-den geçende, `app lift` ony arassa Rust programmasyna öwürýär. Asyl nusga hiç haçan üýtgedilmeýär. |

## Rust hemişe has çaltmy?

Ýok, everything-rust hem muny aýdýar. Her element üçin bir gezek çagyrylýan kiçijik funksiýa Rust-da köplenç has haýal bolýar, sebäbi her çagyryş bridge-den geçýär; V8, JVM we Go şeýle funksiýalary nanosekuntlarda işledýär. `profile` onuň daşyndaky aýlawy tapýar, `--batch` tutuş aýlawy göçürýär, `bench` bolsa çykarmazdan öň netije berýär. Ölçenen sanlar, haýal netijeler hem goşmak bilen, üçin [Batch-lar we hot paths](/docs/concepts/batches) sahypasyna serediň.

## Ol diňe ýazgynyň öz içine alýan zadyny tutup bilýär

Geçmek diňe Rust-yň ýazga alnan her ýagdaýa gabat gelýändigini aňladýar, ondan artyk zat däl. Ýaşyl gate-e ynanmazdan öň [Onuň tutup bilmeýän zatlary](/docs/concepts/limits) sahypasyny okaň.

::: warning Ony nusgada işlediň
`record` we `app record` siziň asyl kodyňyzy işledýär, her funksiýany (ýa-da tutuş programmany) köp sanly döredilen girişde çagyrýar, `app record` bolsa her ýagdaýy iki gezek işledýär. `diff`, `gate` we `app replay` şol ýagdaýlary portda ýa-da asyl nusgada ýene işledýär. Faýl ýazýan, maglumat binasy bilen işleýän ýa-da tora ýüz tutýan kod muny her ýagdaý üçin bir gezek edýär. Olary taslamanyň nusgasynda, synag maglumat binasy bilen we gurşawda production maglumatlary bolmazdan işlediň.
:::

## Indiki ädimler

- [Çalt başlangyç](/docs/introduction/quick-start): ony gurnaň we agentiňizden ilkinji porty soraň.
- [Aýlaw](/docs/concepts/the-loop): bir birlik üçin ähli ädimler.
- [Tutuş programmany geçirmek](/docs/whole-apps/overview): ilkinji ýazgydan arassa Rust binar faýlyna çenli.
- [Hakyky portlar](/docs/real-world/overview): json-server, Fuse.js, vercel/ms we başgalar, haýal netijeler çalt netijeleriň ýanynda.
