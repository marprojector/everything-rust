---
title: تېز باشلاش
description: everything-rust نى ئورنىتىڭ، ئۇنى كود ۋاكالەتچىڭىزگە تىزىملىتىڭ، ھەمدە تۇنجى بىرلىكتە ئايلانمىنى ئىجرا قىلىڭ.
---

# تېز باشلاش

## ۋاكالەتچىڭىزگە ئورناتقۇزۇڭ

بۇنى Claude Code، Codex، Cursor ياكى Gemini CLI غا چاپلاڭ:

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## ياكى ئۆزىڭىز ئورنىتىڭ

::: tabs
== cargo
```sh
cargo install everything-rust   # crates.io دىن
everything-rust setup           # Claude Code، Codex، Cursor ۋە Gemini CLI غا تىزىملىتىدۇ
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

تېخى Rust قورال زەنجىرىڭىز يوقمۇ؟ macOS، Linux ۋە Windows ئۈچۈن ئالدىن تۈزۈلگەن ئىككىلىك ھۆججەتلەر ھەر بىر [GitHub release](https://github.com/marprojector/everything-rust/releases) غا قوشۇلغان، JavaScript تۈرلىرى ئۈچۈن بولسا npm ئوخشاش ئىككىلىك ھۆججەتنى ئورنىتىدۇ.

`setup` تاپقان ھەر بىر ۋاكالەتچىگە MCP مۇلازىمېتىرىنى قوشىدۇ ۋە خىزمەت ئېقىمى ماھارىتىنى ۋاكالەتچى ماھارەتلەرنى ئوقۇيدىغان جايغا ئورنىتىدۇ. ئۇنىڭ نېمىلەرنى ئۆزگەرتىدىغانلىقى ئۈچۈن [ۋاكالەتچىڭىزنى تەڭشەش](/docs/introduction/agents) نى كۆرۈڭ.

## ۋاكالەتچىڭىزدىن سوراڭ

- *"بۇ CLI نى everything-rust بىلەن باشتىن-ئاخىر Rust دا قايتا يېز، ھەمدە ئۇنىڭ ئوخشاش ئىشلەيدىغانلىقىنى ئىسپاتلا."*
- *"بۇ ئىزدەش ئاستا تۇيۇلىدۇ. ئۇنى everything-rust بىلەن ئانالىز قىلىپ، قىزىق ئايلانمىنى Rust قا يۆتكە."*
- *"`src/parser/` نى everything-rust بىلەن Rust قا يۆتكەشنى پىلانلا؛ تۇنجى بىرلىكنى كۆچۈرۈپ ئىسپاتلا."*
- *"`rust/text-rs` دىكى Rust كۆچۈرمىسىنى ئەسلى نۇسخىغا سېلىشتۇرۇپ سىنا، ئېلان قىلىش كېرەكمۇ يوق ماڭا ئېيت."*

## ياكى قەدەملەرنى ئۆزىڭىز ئىجرا قىلىڭ

x18n نىڭ `fuzzyScore` ىدىكى قەدەملەر: `profile` قىزىق ئايلانمىنى تاپىدۇ، `bridge --batch` crate نى ھاسىل قىلىدۇ، `gate` ئۆتىدۇ، سىناق ئۆلچىمى ئۇنى TypeScript بىلەن سېلىشتۇرىدۇ.

```sh
everything-rust profile . -- node bench.mjs              # نېمە قىزىق، ئۇنى كىم چاقىرىدۇ
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs نى يېزىڭ (ياكى ۋاكالەتچىڭىزگە يازدۇرۇڭ)، ئاندىن:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # خاتىرىلەنگەن ھەر بىر ئەھۋال ماس كېلىشى كېرەك
everything-rust bench rust/fuzzy-rs                      # تېزرەكمۇ؟ ھۆكۈم ئېيتىدۇ
everything-rust gate rust/fuzzy-rs                       # تامام دېگەن بۇنىڭ ئۆتۈشى
```

ھەر بىر قەدەم [ئايلانما](/docs/concepts/the-loop) دا چۈشەندۈرۈلگەن.

::: tip بىر نەرسە كەم بولغاندا
ئەگەر بىر قەدەم ئىجرا مۇھىتى، ۋاكالەتچى تەڭشىكى ياكى MCP مۇلازىمېتىرى كەم ياكى كونىراپ كەتكەنلىكى سەۋەبىدىن مەغلۇپ بولسا، [`everything-rust doctor`](/docs/introduction/doctor) نى ئىجرا قىلىڭ.
:::
