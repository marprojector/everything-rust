---
title: アプリ全体の移植
description: 最初の記録から純粋な Rust バイナリまで。目標は、元の実装を参照として残したまま、元の実装と同じように振る舞う Rust アプリです。
---

# アプリ全体の移植

目標は、元の実装を参照として残したまま、元の実装と同じように振る舞う純粋な Rust アプリです。Rust を書くのはエージェントで、everything-rust は作業の順序を決め、雛形を生成し、すべてのステップを証明します。CLI、HTTP サーバー、ライブラリに対応しています。

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in    pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down proved by
 decisions to make      outside         original answers                                           replay
```

## 1. 計画する

```sh
everything-rust analyze . --whole-app
```

`--whole-app` は、アプリの形態（CLI、サーバー、ライブラリ）とエントリポイント、すべての外部依存関係（インポートされたものと宣言されたもの）とその Rust クレートと適合度、ピースとしての移植順序（互いにインポートし合うユニットはまとめて移動する）、そして推奨回答付きの番号付き決定事項（残すユニット、ブロックされているユニット、クレートがない、または部分的なクレートしかない依存関係）を追加します。

## 2. アプリ全体を記録し、エントリポイントを移す

1. [`app record` と `app replay`](/docs/whole-apps/record-replay) は、プログラムを外側からブラックボックスとして証明します。
2. [`app front-door`](/docs/whole-apps/front-door) は元の実装の前に Rust バイナリを置きます。すべてのコマンドはまだ元の実装を実行します。
3. 各ユニットは[移植ループ](/docs/concepts/the-loop)を通り、その後 `app wire` がそのコマンドを Rust に引き渡します。
4. [`app lift`](/docs/whole-apps/lift) は、リプレイがフォールバック率 0 で合格し、すべてのユニットが gate に合格した時点で元の実装を取り除きます。

[サーバー](/docs/whole-apps/servers)と[ライブラリ](/docs/whole-apps/libraries)も同じコマンドを使います。

## エージェントの進め方

[アプリ全体用のスキル](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md)は、あなたにしか下せない決定から始めて、エージェントを順に導きます:

- **質問攻め（grilling）**: エージェントは何回かに分けて質問し、それぞれに推奨回答を添えます。移植が依存するすべての決定が確定するまで続け、それぞれを ADR として `rust/DECISIONS.md` に書き、`rust/GLOSSARY.md` で元の実装の用語を Rust の名前に対応付けます。
- **調査**: バックグラウンドのエージェントが各依存関係の一次情報を読み、見つけたすべてのセマンティックトラップを含めて `rust/research/<topic>.md` を書きます。
- **セッションをまたぐ作業**: `rust/MAP.md` はすべてのユニットとその状態を一覧にします。セッションは開始前に 1 つのユニットを確保し、各チェックポイントで次のセッションのために `rust/HANDOFF.md` を残します。同じウェーブのユニットはサブエージェントが並行して移植できます。
- **切り替え**: エージェントはできることを行い、`rust/cutover.sh` を書きます。これはあなたにしかできないステップ（デプロイ、DNS、シークレット）のための対話式スクリプトで、各ステップに戻る手段があります。

質問攻め、調査、これらの手法は [Matt Pocock のスキル](https://github.com/mattpocock/skills)を基にしています。

## Rust が古い設計に縛られないか？

縛られるのは両方を並べて実行している間だけです。記録が確認するのはアプリの境界での挙動（CLI の出力、サーバーのレスポンス、モジュールがエクスポートする関数）であり、内部構造ではありません。ユニットがまだブリッジを越えている間は古い構造に合わせ、`app lift` が元の実装を取り除いたら Rust を自由に再構成してください。`app replay` は引き続き挙動を証明します。

## 実例

[json-server](/docs/real-world/json-server) はすべてのステップを通りました: 514 件の記録された HTTP ケース、フォールバック率 99.8% のフロントドア、最初に読み取りをワイヤリング（フォールバック率 22.6%）、次にすべてのルート（0%）、そして 514/514 に合格する純粋な Rust バイナリへのリフトです。
