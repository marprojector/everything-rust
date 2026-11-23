---
title: Китепканалар
description: Китепкананын кайра ойното турган процесси жок. Анын public API'си front door болуп саналат, ал эми ар бир бирдиктин жазылган кейстери далил.
---

# Китепканалар

Китепкананын кайра ойното турган процесси жок: анын public API'си front door болуп саналат, ал эми ар бир
бирдиктин жазылган кейстери далил.

```sh
everything-rust app front-door .        # rust/app: a Rust crate with the public API
# each unit through the loop: bridge, record, port.rs, build, diff, gate; then:
everything-rust app wire . textkit-rs   # re-exported from rust/app; the original imports its shadow
everything-rust app replay .            # every wired unit's cases; fallback = exports still in the original
everything-rust app lift .
```

- Репозиторийде `main` жана сервер жок болгондо (болбосо `--library`), `app front-door` public API'ни
  (ар бир бирдиктин экспорттору) таап, `rust/app`'ты китепкана crate'и катары даярдайт. Ар бир туташтырылган
  бирдик анын модулу болуп калат: `pub use textkit_rs::port as textkit;`.
- `app wire` `bridge` түзгөн crate'ти алат, `gate` иштетет жана бирдиктин shadow drop-in модулун жазат:
  түп нуска пакет аны бирдиктин ордуна импорттойт, ошентип анын чакыруучулары түп нуска API'син
  сактап, Rust'ка жетишет. `EVERYTHING_RUST_UNIT_<NAME>=original` бирдикти артка кайтарат (C: shadow'дун
  желектерисиз куруңуз).
- `app replay` ар бир туташтырылган бирдиктин жазылган кейстерин анын байланыштары аркылуу иштетет. Fallback rate
  Rust азырынча жооп бербеген public экспорттордун үлүшү: туташтырылбаган бирдиктер жана `bridge` түп
  тилде калтырган экспорттор.
- `app lift` 0 fallback жана ар бир бирдиктин `gate`'тен өтүшүн талап кылат; ал бирдиктерден
  байланыштарды, wrapper'лерди жана shadow'дорду алып салып, `rust/app`'ты бир таза Rust китепканасы катары
  калтырат. Ал эч бир `port.rs`'ке тийбейт, ошондуктан байланыштар аркылуу дал келген кейстер аны дагы эле
  сүрөттөйт; чек-лист shadow'дорду дагы эле импорттогон түп нуска файлдары жөнүндө эскертет.

Бешөө тең MCP куралдары да: `app_record`, `app_replay`, `app_front_door`, `app_wire` жана
`app_lift`.
