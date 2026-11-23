---
title: Doctor
description: everything-rust doctor は言語ランタイム、エージェントの設定、MCP サーバーをチェックし、問題ごとにそれを直すコマンドを示します。
---

# Doctor

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor` は言語ランタイム、エージェントの設定、MCP サーバーをチェックし、問題ごとにそれを直すコマンドを表示します。必要なものが欠けている場合は終了コード 1 で終了します。

ランタイム、エージェントの設定、または MCP サーバーがない、あるいは古いためにステップが失敗したときに実行してください。ワークフロースキルもエージェントに同じことを指示します。

## 各言語に必要なもの

`record`、`diff`、`bench` は元の実装をそれ自身のランタイムで実行します: TS には Node 22.6+（`.ts` ソースを直接実行します）、Python と C（ctypes）には Python 3.9+、Go 1.21+、Ruby 3+、JDK 17+（`javac` と `java`）。Node での `profile` には、`NODE_OPTIONS` で `--cpu-prof` を使える Node 24+ が必要です。C には C コンパイラも必要です。wasm ブリッジには `rustup target add wasm32-unknown-unknown` と、対応するバージョンの `wasm-bindgen-cli` も必要です。正確なバージョンは `build` が示します。

## コントリビューター向け

`cargo run -- doctor` は、手元にあるランタイム、そのバージョン、それぞれをインストールまたは更新するコマンドを一覧表示します。[コントリビュート](/docs/project/contributing)を参照してください。
