---
title: Ýagdaý
description: everything-rust-de häzir näme işleýär we her bölegiň soňky gezek nähili barlanandygy.
---

# Ýagdaý

Näme işleýär we her bölek soňky gezek nähili barlandy. Täzelenen senesi 2026-11-18.

**Barlanan**: aşakda ady agzalan, başga biriniň ýazan hakyky kodunda başdan-aýak işledildi (bridge,
record, port, diff, gate). **Synag edilen**: bu repozitoriýiň testleri bilen, olar üçin ýazylan
fixture-larda örtülýär, entek hakyky taslamada işledilmedi. **Preview**: beýan edilişi ýaly, işiň bir
böleginde işleýär.

## Birlikleri geçirmek

| Dil | Ýagdaý | Soňky hakyky işletme | Netije |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Barlanan | [Prettier](https://github.com/marprojector/everything-rust/blob/main/docs/real-world.md#prettier) (2026-11-18), [json-server](/docs/real-world/json-server) (2026-11-18), [Fuse.js](/docs/real-world/fuse-js) (2026-11-18), [vercel/ms](/docs/real-world/vercel-ms) (2026-11-18), x18n-iň `packages/core` | Prettier: `doc-printer-rs` 2,848/2,848 we `js-format-rs` 1,279/1,279, ikisinde hem gate geçýär; json-server: alty birlik, 3,399 ýagdaý, hersinde gate geçýär; Fuse.js, vercel/ms, x18n: 3,414/3,414, 975/975 we 9,000+ ýagdaý gabat gelýär |
| Python | Barlanan | CPython 3.13-den `colorsys.py` (2026-11-16) | 2,400/2,400; gate geçýär |
| Go | Barlanan | Go 1.24-den `path` (2026-11-16) | 2,642/2,642; gate geçýär |
| Ruby | Barlanan | Ruby 3.3-den `shellwords.rb` (2026-11-16) | 1,617/1,617; gate geçýär |
| Java | Barlanan | fasadyň arkasyndaky JDK 21 (2026-11-16) | 4,492/4,492; gate geçýär |
| C | Barlanan | fasadyň arkasyndaky glibc (2026-11-16) | kesgitlenmedik özüni alyp barşy bolan bir ýagdaý aýrylandan soň 1,859/1,859; gate geçýär |
| PHP | Preview | ýok | diňe `analyze` we tutuş programmalar; birlikleri entek bridge edip bolmaýar |
| C# | Preview | ýok | diňe `analyze` we tutuş programmalar; birlikleri entek bridge edip bolmaýar |

Her işletmäniň jikme-jiklikleri: [real-world.md](/docs/real-world/overview) (TS/JS) we
[real-world-languages.md](/docs/real-world/six-languages) (beýlekiler).

## Buýruklar

| Buýruk | Ýagdaý | Bellikler |
| --- | --- | --- |
| `analyze` | Barlanan | ýokardaky ähli diller, x18n-iň 142 birligi |
| `bridge`, `record`, `diff`, `gate`, `build` | Barlanan | ýokardaky işletmeler; json-server-iň `paginate` funksiýasynda `bridge --as-json` |
| `bench` | Barlanan | ýokardaky işletmeler; sanlar [real-world.md](/docs/real-world/overview) içinde |
| `record --from-tests` | Barlanan (TS/JS), Synag edilen (Python) | vercel/ms-iň Jest toplumy |
| `record` we `gate` içinde asyl nusganyň örtügi | Barlanan (TS/JS) | Fuse.js: ýagdaýlar onuň import edýän setirleriniň 81%-ini işledýär; beýleki diller ony entek ölçemeýär |
| `--batch` | Barlanan | Fuse.js, vercel/ms, x18n gözlegi |
| wasm bridge (`--kind wasm`) | Barlanan | x18n gözlegi |
| `shadow` | Barlanan | x18n-iň `packages/core` |
| `profile` | Synag edilen | |
| `app record`, `app replay` | Barlanan (serwerler we CLI-ler) | json-server: `--fresh`, `--arg`, `--pace` bilen 514 HTTP ýagdaýy; Prettier: `--inputs` bilen 3,080 CLI ýagdaýy; `tests/app.rs` içindäki CLI-ler |
| Front door (`front-door`), `wire` | Barlanan (TS serweri we CLI) | Prettier: bir birlik baglandy, 57.8% fallback bilen 3,080/3,080, glue `original::run_with_stdin` arkaly ýagdaý boýunça ret edýär; json-server: ilki okamalar baglandy (22.6% fallback), soňra ähli marşrutlar (0%); `tests/http.rs` içinde sekiz dilde serwerler |
| `lift` | Barlanan (TS serweri), Synag edilen (CLI-ler) | json-server: arassa Rust binar faýly, 514/514; `tests/app.rs` içinde ähli sekiz dilde CLI-ler |
| MCP serweri (`mcp`) | Synag edilen | `src/mcp.rs` içinde protokol testi |

## Testler nirede işleýär

CI macOS we Windows-da Node, Python, Go, Ruby we Java gurnaýar. Ol PHP-ni ýa-da .NET-i gurnamaýar,
şonuň üçin PHP we C# testleri ol ýerde geçirilip goýulýar we diňe olar gurnalan maşynda işleýär.
