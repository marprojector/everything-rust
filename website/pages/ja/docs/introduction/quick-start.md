---
title: クイックスタート
description: everything-rust をインストールし、コーディングエージェントに登録して、最初のユニットでループを回します。
---

# クイックスタート

## エージェントにインストールさせる

Claude Code、Codex、Cursor、または Gemini CLI に以下を貼り付けてください:

```text
everything-rust をインストールして: `cargo install everything-rust` を実行し、次に `everything-rust setup` を実行してください。
再起動が必要なタイミングを教えてください。
```

## または自分でインストールする

::: tabs
== cargo
```sh
cargo install everything-rust   # crates.io からインストール
everything-rust setup           # Claude Code、Codex、Cursor、Gemini CLI に登録
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

Rust ツールチェーンをまだ導入していない場合は、各 [GitHub リリース](https://github.com/marprojector/everything-rust/releases)に macOS、Linux、Windows 向けのビルド済みバイナリが添付されています。JavaScript プロジェクトでは npm が同じバイナリをインストールします。

`setup` は見つかった各エージェントに MCP サーバーを追加し、エージェントがスキルを読み込む場所にワークフロースキルをインストールします。何が変更されるかは[エージェントの設定](/docs/introduction/agents)を参照してください。

## エージェントに依頼する

- *"everything-rust を使ってこの CLI をエンドツーエンドで Rust に書き直し、元の実装と同じ挙動であることを証明して。"*
- *"この検索が遅く感じる。everything-rust でプロファイルを取って、ホットループを Rust に移して。"*
- *"everything-rust で `src/parser/` の Rust 移行計画を立てて。最初のユニットを移植して検証して。"*
- *"`rust/text-rs` の Rust 移植版と元の実装をベンチマーク比較して、出荷すべきか教えて。"*

## または自分でステップを実行する

x18n の `fuzzyScore` での手順: `profile` がホットループを見つけ、`bridge --batch` がクレートを生成し、`gate` に合格し、ベンチマークで TypeScript と比較します。

```sh
everything-rust profile . -- node bench.mjs              # どこがホットで、誰が呼んでいるか
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs を書く（またはエージェントに書かせる）。その後:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # 記録されたすべてのケースが一致する必要がある
everything-rust bench rust/fuzzy-rs                      # 速くなったか？判定が教えてくれる
everything-rust gate rust/fuzzy-rs                       # これに合格すれば完了
```

各ステップは[移植ループ](/docs/concepts/the-loop)で説明しています。

::: tip 何かが足りないとき
ランタイム、エージェントの設定、または MCP サーバーがない、あるいは古いためにステップが失敗した場合は、[`everything-rust doctor`](/docs/introduction/doctor) を実行してください。
:::
