---
title: Ýazga almak we gaýtadan işletmek
description: app record tutuş programmany onuň resminamalaşdyrylan mysallarynda, siziň girişleriňizde we duzaklarda işledýär; app replay islendik gurluşy ýagdaýlara garşy işledýär.
---

# Ýazga almak we gaýtadan işletmek

`record` we `diff` bir birligi subut edýär; `app record` we `app replay` bolsa programmany subut edýär, alty diliň islendiginde CLI-ler we serwerler üçin. Programma gara guty hökmünde seredilýär.

```sh
everything-rust app record . -- python3 cli.py          # asyl nusgany işledýän buýruk
everything-rust app replay .                            # asyl nusga 100% geçmeli
everything-rust app replay . --build "target/release/mytool"
```

## Ýagdaýlar nireden gelýär

`app record` asyl nusgany şularda işledýär:

- programmanyň öz resminamalaşdyrylan mysallary: onuň Markdown we shell faýllaryndaky `$ mytool ...` setirleri, olaryň görkezýän faýllary bilen;
- `--inputs` JSONL faýly, her setirde bir `{"name", "args", "stdin", "env", "files"}`;
- şol ýagdaýlardan alnan semantik duzaklar: ASCII däl we emoji setirler, otrisatel bölmek we modul, doly geçme, uly we kiçijik float-lar, map tertibi, ýok faýllar.

Her ýagdaý iki gezek işleýär; iki işletmäniň arasynda tapawutlanýan zat `[..]` wildcard bolýar. `--normalize timestamps|uuids|key-order` ýa-da `--normalize name=regex` ýagdaýlar bilen saklanýan normalizatorlary goşýar. `app record` nämäniň tapawutlanandygyny görnüşi boýunça habar berýär: bahalar (indi wildcard-lar), tertip (setirler islendik tertipde deňeşdirilýär) we setirler (islendik zada gabat gelýän `...`). Olaryň her birine serediň.

## Ýagdaýlar ýönekeý faýllar

Ýagdaýlar `rust/app-cases/` içindäki [trycmd](https://docs.rs/trycmd) faýllary:

| Faýl | Saklaýar |
| --- | --- |
| `<case>.toml` | argv, gurşaw, çykyş kody |
| `<case>.stdin` | standart giriş |
| `<case>.stdout`, `<case>.stderr` | asyl nusganyň çap eden zady |
| `<case>.in/` | giriş faýllary |
| `<case>.out/` | ýazylan faýllar |

Olary el bilen okaň we üýtgediň; portuň binar faýly bolandan soň Rust testi olary `trycmd` bilen işledip biler.

## Gaýtadan işletmek

- `app replay` islendik gurluşy ýagdaýlara garşy işledýär, `passed/total` çap edýär, şowsuz her ýagdaý üçin diff bilen `rust/app-cases/replay-report.md` ýazýar we islendik gabat gelmezlikde noldan tapawutly kod bilen çykýar.
- [Front door](/docs/whole-apps/front-door) ýaly `EVERYTHING_RUST_FALLBACK_LOG` bilen görkezilen faýla ýazgy goşýan gurluş (asyl nusga her fallback üçin bir setir) öz fallback paýyny hem habar berýär.
- `app replay --case <name>` hasabatyň beren ady boýunça bir ýagdaýy täzeden işledýär: hemmesini ýene işletmezden öň gabat gelmezligi düzetmek üçin çalt we gaýtalanýan aýlaw.

## CI-de gaýtadan işletmek

Hasabatda wagt ölçegleri ýa-da wagtlaýyn ýollar ýok: şol bir gurluşyň iki replay-i şol bir faýly ýazýar. Ony commit ediň, soňra CI-de işlediň:

```sh
everything-rust app replay .
git diff --exit-code rust/app-cases/replay-report.md
```

Şonda ýarym geçirilen programma geçýän zatlaryň islendik üýtgemesinde, gowulyga ýa-da erbetlige, hasabat oňa görä täzelenýänçä CI-den geçmeýär.

## Sanly çykyş

`app record --float-tolerance 1e-9` sanlara has uly bahanyň şol paýynyň çäginde gabat gelmäge rugsat berýär. [Float rugsady](/docs/concepts/float-tolerance) sahypasyna serediň.
