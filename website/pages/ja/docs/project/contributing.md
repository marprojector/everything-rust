---
title: コントリビュート
description: セットアップ、テストの置き場所、クレートの対応付けや言語の追加方法、実際の移植の共有方法。
---

# コントリビュート

ご協力ありがとうございます。バグ報告、修正、新しい言語のサポート、実際の移植の記録はどれも
歓迎します。バグ修正の Pull Request は事前に相談せずに開いて構いません。

## 始める前に

- **バグ:** [Issue](https://github.com/marprojector/everything-rust/issues/new/choose) を開き、
  実行したコマンド、その出力、プラットフォーム、everything-rust のバージョン
  （`everything-rust --version`）を添えてください。小さな再現用プロジェクトがあると最も助かります。
- **大きめの変更**（新しい言語、新しいコマンド、ユニットや記録の定義の変更）:
  書き始める前に形について合意できるよう、まず Issue を開いてください。過去の
  選択の理由は [DECISIONS.md](https://github.com/marprojector/everything-rust/blob/main/DECISIONS.md) にあります。
- **セキュリティ上の問題:** 公開の Issue は開かないでください。[SECURITY.md](https://github.com/marprojector/everything-rust/blob/main/SECURITY.md) を参照してください。

## セットアップ

Rust（stable、1.97 以降）が必要です。テストは各部分が扱う言語も実行し、
ランタイムがない部分はスキップするので、触る部分のランタイムをインストールしてください:

| 対象 | インストールするもの |
| --- | --- |
| TypeScript/JavaScript | Node 24（`profile` 以外は 22.6+ でも動作） |
| Python | Python 3.9+ |
| C | C コンパイラ（`cc`） |
| Go | Go 1.22+ |
| Ruby | Ruby 3.x |
| Java | JDK（17+） |
| PHP | PHP 8 |
| C# | .NET SDK |
| wasm ブリッジ | `rustup target add wasm32-unknown-unknown` と、対応するバージョンの `wasm-bindgen-cli` |

```sh
git clone https://github.com/marprojector/everything-rust
cd everything-rust
git config core.hooksPath .githooks
cargo test
```

`cargo run -- doctor` は、これらのランタイムのうち手元にあるもの、そのバージョン、それぞれを
インストールまたは更新するコマンドを一覧表示します。ランタイムがない場合にテストをスキップせず失敗させるには（CI はそうしています）、
`EVERYTHING_RUST_REQUIRE` にその名前を指定します。例:
`EVERYTHING_RUST_REQUIRE=node,python3,go,ruby,java cargo test`。

フックは `scripts/check-secrets.sh` を実行し、コミット内容に含まれるキー、トークン、個人のホームフォルダの
パスを拒否します。CI はすべてのファイルに対してこれを実行します。

## 変更を加える

- **すべての修正には、その修正なしでは失敗するテストを付けてください。** 確実にそうなるよう、古いコードに対して一度
  テストを実行してください。
- **テストの置き場所:**
  - `tests/analyze.rs`: 言語の検出、ユニット、リスク、依存関係、`--whole-app`。
  - `tests/loop.rs`: 言語ごとの bridge、record、diff、shadow。ビルドの代わりにスタンドインを使用。
  - `tests/native.rs`: 実際にビルドしたブリッジを読み込んでリプレイ。
  - `tests/app.rs` と `tests/http.rs`: CLI とサーバーのアプリ全体の record、replay、フロントドア、lift。
  - `tests/library.rs`: ライブラリ全体の移植。
  - ユニットテストはコードの隣に置きます。
- **フィクスチャ**は `tests/fixtures/` 配下の小さなアプリです。CLI と HTTP のフィクスチャは、すべての言語で同じ
  コマンドとルート（`tcli upper|len|div|words|copy|now`、`/hello`、`/upper`、
  `/div/{a}/{b}`、`/now`）を実装しているので、新しいものも他と挙動を同一に保ってください。
- **生成コード**（everything-rust が書くブリッジ、ドライバー、フロントドア）も
  `clippy -D warnings` に合格しなければなりません。`gate` がユーザーのクレートに対してそれを実行するからです。
- **スタイル:** 周りのコードに合わせてください。コメントはコードだけでは伝えられないことに限り、
  変更の理由は Pull Request に書いてください。

## クレートの対応付けを追加する

`analyze` は、コードがインポートする各ライブラリに対応する Rust クレートを `src/crates.toml` から示します。追加するには、
その言語の下にエントリを追記します: `names`（パッケージ名の完全一致）または `prefixes`（名前の先頭部分。
Java と C# 用）、`crates`、`fit`（`good`、`partial`、`none`）、そして適合度の説明が必要な場合は `note` です。
最初に一致したエントリが優先されるので、具体的なものを広いプレフィックスより前に置いてください。
テーブルにないライブラリは `unknown` と報告されます。クレートを推測してはいけません。

## 言語を追加する

PHP と C# はプレビューとして 2 つのコミットで追加されました。最初に読んでおく価値があります:
[e8ac243](https://github.com/marprojector/everything-rust/commit/e8ac243)（PHP）と
[caea59a](https://github.com/marprojector/everything-rust/commit/caea59a)（C#）。新しい言語も
同じように始めます:

1. `src/lang/<name>.rs` のプラグイン: 検出、スキャン（インポートとリスク）、解決とエクスポート、
   さらにそのライブラリのクレートを `src/crates.toml` の `[[<name>]]` エントリとして追加。
2. `src/lang/mod.rs` での検出とディスパッチ。
3. `src/whole_app.rs` でのアプリ全体のサポート: エントリポイント、サーバー、宣言された依存関係。
4. CLI と HTTP のフィクスチャ、ライブラリのフィクスチャ、そして `tests/analyze.rs`、`tests/app.rs`、
   `tests/http.rs` のテスト。
5. ブリッジができるまで、`bridge` は明確なメッセージとともにその言語のユニットを拒否します。

## README を翻訳する

翻訳は歓迎しており、コミュニティによって管理されています。新しい翻訳は:

1. `docs/README_<CODE>.md`（例: `docs/README_PT.md`）に置きます。ルートの `README.md` は英語のままで、
   名前を変えたり置き換えたりすることはありません。
2. `docs/` から機能するパスを使います: `assets/logo.svg`、`../LICENSE`、`../README.md`。
3. 他の翻訳と同じ言語の行で始め、自身の言語はプレーンテキストにし、その後に
   コミュニティによって管理されており英語版 README より古い場合があるという注記を続けます
   （`docs/README_DE.md` からコピーして翻訳してください）。
4. `README.md` とすべての翻訳で、言語の行の**末尾**に追加します。すでに並んでいる
   言語の順序は変えません。

## ドキュメントサイト

ドキュメントのウェブサイトは `website/` にあります（Void、`@void/md`、React）。多くのページはビルド時に
`README.md`、`CONTRIBUTING.md`、`docs/*.md` のセクションを取り込むので、それらのファイルを編集すれば
サイトも追従します。`cd website && npm ci --ignore-scripts && npm run dev` で実行し、
`npm run build` でビルドします。CLI リファレンスは `website/content/cli-help.json` にある `--help` のスナップショットから作られます。
CLI を変更した後は、`website/` で `cargo build --release` と
`npm run cli-help` を実行して更新してください。

## Pull Request を開く

プッシュする前に:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

- 何が問題だったか、どう再現するか、変更がどう直すかを書いてください。
- Issue をリンクしてください（`Fixes #123`）。
- テストできなかったこと（別の OS、手元にないランタイム）を記してください。
- 1 つの変更にとどめてください: 2 つの修正は 2 つの Pull Request に分けた方がレビューしやすくなります。

CI はすべてのランタイムをインストールした Ubuntu と macOS で実行されます。初めてのコントリビューターの場合、GitHub は
メンテナーが CI の実行を承認するのを待つので、チェックが始まるまで少し時間がかかります。

Pull Request はコミットを保持したままマージされるので、あなたの履歴はあなたのものとして残ります。変更に
小さな調整が必要な場合、メンテナーはもう一度のやり取りを求める代わりに、上にコミットを追加することがあります。

## 実際の移植を共有する

実プロジェクトの移植は、何が動き何が壊れるかを示す最良の証拠です。実際のものに
everything-rust を使ったら、[docs/real-world.md](/docs/real-world/overview) に追加する Issue や Pull Request を
大歓迎します。プロジェクト、実行した
コマンド、ケース数、そして驚いたことを含めてください。

## 行動規範

参加するすべての人は[行動規範](https://github.com/marprojector/everything-rust/blob/main/CODE_OF_CONDUCT.md)に従うことが求められます。
