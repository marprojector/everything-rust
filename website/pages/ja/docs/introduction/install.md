---
title: インストール
description: crates.io、npm、またはリリースバイナリから everything-rust をインストールする方法と、各言語に必要なランタイム。
---

# インストール

## crates.io、npm、またはリリースから

::: tabs
== cargo
```sh
cargo install everything-rust
everything-rust setup
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

npm は JavaScript プロジェクト向けに同じバイナリをインストールします。

macOS、Linux、Windows 向けのビルド済みバイナリは、各 [GitHub リリース](https://github.com/marprojector/everything-rust/releases)に添付されています。Claude Code ではプラグインとしてインストールすることもできます（MCP サーバーとスキル。CLI が `PATH` に通っている必要があります）:

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## ソースから

```sh
cargo install --path .
```

`record`、`diff`、`bench` は元の実装をそれ自身のランタイムで実行します: TS には Node 22.6+（`.ts`
ソースを直接実行します）、Python と C（ctypes）には Python 3.9+、Go 1.21+、Ruby 3+、JDK 17+
（`javac` と `java`）。Node での `profile` には、`NODE_OPTIONS` で `--cpu-prof` を使える Node 24+ が必要です。C には C コンパイラも必要です。wasm ブリッジには `rustup target add wasm32-unknown-unknown` と
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked` も必要です。正確なバージョンは `build` が
示します。

## Windows

ネイティブ Windows の CI は `x86_64-pc-windows-msvc` の Rust ターゲットを使います。stable の MSVC
ツールチェーンと、**Desktop development with C++** ワークロード付きの Visual Studio Build Tools をインストールしてください。
C コンパイラとリンカーは C ブリッジとネイティブ Rust 拡張に必要です。移行する元の言語の
ランタイムもインストールしてください。Python は `python3` と
`python` のどちらでも構いません。everything-rust は両方を検出します。GNU の Windows ターゲットは CI では検証していません。

ネイティブブリッジの統合テストスイートは Developer PowerShell のプロンプトから実行します:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

このスイートは、実際の DLL ベースの Node と Python の拡張、および `PATH` 上にある各ランタイムでサポートされる
ネイティブブリッジをビルドして読み込みます。任意のランタイムがない場合はスキップとして報告されます。
Cargo、有効な Rust 標準ライブラリ、MSVC リンカーは必須です。Cargo が
`std` を見つけられないと報告した場合は、
`rustup target remove x86_64-pc-windows-msvc` に続けて
`rustup target add x86_64-pc-windows-msvc` を実行して、有効なターゲットを再インストールしてください。リンクに失敗する場合は、Visual Studio Developer
PowerShell から実行し、`where.exe link` が MSVC リンカーを見つけることを確認してください。テストは一時フォルダ
（スペースと Unicode 文字を含むパスを含む）を使い、成功した実行の後に生成物を
削除します。

エージェント向けには、`everything-rust setup` がすべてを登録します（クイックスタートを参照）。代わりに 1 つのプロジェクトに手動で MCP サーバーを
追加するには（`.mcp.json`）:

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

各ツール、そのパラメータ、触れる可能性のあるもの: [docs/tools.md](/docs/reference/mcp-tools)。サーバーは
スキルから作られた 2 つのプロンプト、`port_unit` と `port_whole_app` も提供します。
## 更新または削除

```sh
cargo install everything-rust                                    # 更新（Cargo）
npm install -g everything-rust@latest                            # 更新（npm）
everything-rust setup                                            # エージェントの設定を更新
everything-rust setup --remove && cargo uninstall everything-rust  # 削除（Cargo）
everything-rust setup --remove && npm uninstall -g everything-rust # 削除（npm）
```

`setup --remove` は everything-rust のエントリだけを取り除きます。保存したコピー
（`<file>.everything-rust.bak`）は、あなたが削除するまで残ります。
