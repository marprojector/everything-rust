---
title: Портування
description: Як писати port.rs, єдиний файл, який пишете ви або ваш агент, і правила, що тримають його у відповідності до оригіналу.
---

# Портування

Порт у `src/port.rs` пишете ви або ваш агент. Усе інше в крейті генерується. Після зміни сигнатури в `port.rs` запустіть `everything-rust refresh <crate>`.

## Правила, що заощаджують раунд diff

- Перш ніж писати код, прочитайте кожен ризик, який план вказує для модуля, за його `file:line`.
- **Зміщення в рядках**: JS і Java індексують кодові одиниці UTF-16, Rust байти UTF-8. Використовуйте згенерований `src/utf16.rs`.
- **Помилки**: повертайте `Err("TypeName: message")`. Це стає відповідним винятком у Python, Ruby і Java і текстом `error` у Go. Паніка Rust ніколи не збігається із записаною помилкою.
- **Числа в Python і Ruby**: int, які оригінал пропускає далі, мають повертатися як int. У портах Python використовуйте `crate::pynum::Num` для int-або-float, а не `serde_json::Value`.
- **C**: вхідні дані, записані як `crash:`, є невизначеною поведінкою; проходить будь-який результат.
- **Впорядковані мапи і ключі об'єктів**: `indexmap` там, де порядок потрапляє у вивід.
- **Lookaround чи зворотні посилання в регулярних виразах**: `fancy-regex`, а не `regex`. У будь-якому разі `\w`, `\b`, `\d` і пошук без урахування регістру обробляють не-ASCII текст інакше, ніж рушій мови оригіналу. Встановіть `unicode(false)` або випишіть класи ASCII (`[A-Za-z0-9_]`) там, де `\w` в оригіналі охоплює лише ASCII (JS без `u`, Java).
- **Жодного `unsafe`.** Якщо він справді потрібен: мінімальний, ізольований, з коментарем `// SAFETY:`.
- **Тримайте хелпери приватними.** Лише експорти модуля є `pub`; хелпер із `pub` стає частиною API за мостом.
- Додавайте модульні тести Rust у `port.rs` для випадків, які ви обмірковуєте.

Згенерований `js.rs` містить хелпери, що відтворюють JavaScript там, де Rust відрізняється, як-от `js::number_to_string` (`String(n)` у JS) і `js::math_round` (`Math.round`).

## Приклад: емодзі, що зсунув позицію

`packages/core/src/fuzzy.ts` з [x18n](https://github.com/x18n-dev/x18n), портований обома способами в
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo):

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: готовий, wasm-bindgen (core працює в браузерах)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# напишіть src/port.rs
everything-rust gate demo/fuzzy-rs
```

Перший порт, що індексував за Rust `char`, пройшов прості тести, але провалив `diff`:

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

`indexOf` у JS рахує кодові одиниці UTF-16, тож емодзі займає дві позиції. Порт у
`demo/*/src/port.rs` використовує згенеровані хелпери `utf16` і проходить 2000/2000 випадків і gate,
як napi-rs (`demo/fuzzy-napi`) і як wasm-bindgen (`demo/fuzzy-rs`).

План, який everything-rust пише для всього x18n: [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md).
## Яким великим він має бути?

`gate` порівнює кількість рядків порту з оригіналом і падає, якщо порт більший ніж у 3x (плюс 50 рядків на імпорти та сигнатури). Перевищення зазвичай означає, що вручну написано те, що вже робить крейт, або випадки обробляються по одному. `analyze` перелічує крейти, які варто використати.
