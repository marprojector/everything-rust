---
title: ステータス
description: everything-rust で現在動作するもの、そして各部分が最後にどう確認されたか。
---

# ステータス

動作するもの、そして各部分が最後にどう確認されたか。2026-11-18 更新。

**検証済み（Verified）**: 下記に挙げた、他の人が書いた実際のコードでエンドツーエンド（bridge、record、port、diff、gate）に実行済み。
**テスト済み（Tested）**: このリポジトリのテストで、そのために書いたフィクスチャ上でカバーされているが、まだ
実プロジェクトでは実行していない。**プレビュー（Preview）**: 説明のとおり、フローの一部で動作する。

## ユニットの移植

| 言語 | ステータス | 最後の実プロジェクトでの実行 | 結果 |
| --- | --- | --- | --- |
| TypeScript/JavaScript | 検証済み | [Prettier](https://github.com/marprojector/everything-rust/blob/main/docs/real-world.md#prettier)（2026-11-18）、[json-server](/docs/real-world/json-server)（2026-11-18）、[Fuse.js](/docs/real-world/fuse-js)（2026-11-18）、[vercel/ms](/docs/real-world/vercel-ms)（2026-11-18）、x18n の `packages/core` | Prettier: `doc-printer-rs` 2,848/2,848 と `js-format-rs` 1,279/1,279、両方とも gate に合格。json-server: 6 ユニット、3,399 ケース、それぞれ gate に合格。Fuse.js、vercel/ms、x18n: 3,414/3,414、975/975、9,000 件以上のケースが一致 |
| Python | 検証済み | CPython 3.13 の `colorsys.py`（2026-11-16） | 2,400/2,400、gate に合格 |
| Go | 検証済み | Go 1.24 の `path`（2026-11-16） | 2,642/2,642、gate に合格 |
| Ruby | 検証済み | Ruby 3.3 の `shellwords.rb`（2026-11-16） | 1,617/1,617、gate に合格 |
| Java | 検証済み | ファサード経由の JDK 21（2026-11-16） | 4,492/4,492、gate に合格 |
| C | 検証済み | ファサード経由の glibc（2026-11-16） | 未定義動作のケースを 1 件除外して 1,859/1,859、gate に合格 |
| PHP | プレビュー | なし | `analyze` とアプリ全体のみ。ユニットはまだブリッジできない |
| C# | プレビュー | なし | `analyze` とアプリ全体のみ。ユニットはまだブリッジできない |

各実行の詳細: [real-world.md](/docs/real-world/overview)（TS/JS）と
[real-world-languages.md](/docs/real-world/six-languages)（その他）。

## コマンド

| コマンド | ステータス | 備考 |
| --- | --- | --- |
| `analyze` | 検証済み | 上記のすべての言語、x18n の 142 ユニット |
| `bridge`, `record`, `diff`, `gate`, `build` | 検証済み | 上記の実行。json-server の `paginate` で `bridge --as-json` |
| `bench` | 検証済み | 上記の実行。数値は [real-world.md](/docs/real-world/overview) |
| `record --from-tests` | 検証済み（TS/JS）、テスト済み（Python） | vercel/ms の Jest スイート |
| `record` と `gate` での元の実装のカバレッジ | 検証済み（TS/JS） | Fuse.js: ケースはインポートする行の 81% を実行。他の言語ではまだ計測していない |
| `--batch` | 検証済み | Fuse.js、vercel/ms、x18n の検索 |
| wasm ブリッジ（`--kind wasm`） | 検証済み | x18n の検索 |
| `shadow` | 検証済み | x18n の `packages/core` |
| `profile` | テスト済み | |
| `app record`, `app replay` | 検証済み（サーバーと CLI） | json-server: `--fresh`、`--arg`、`--pace` で 514 件の HTTP ケース。Prettier: `--inputs` で 3,080 件の CLI ケース。`tests/app.rs` の CLI |
| フロントドア（`front-door`）、`wire` | 検証済み（TS のサーバーと CLI） | Prettier: 1 ユニットをワイヤリングし、フォールバック率 57.8% で 3,080/3,080、グルーコードは `original::run_with_stdin` でケースごとに処理を辞退。json-server: 最初に読み取りをワイヤリング（フォールバック率 22.6%）、次にすべてのルート（0%）。`tests/http.rs` に 8 言語のサーバー |
| `lift` | 検証済み（TS のサーバー）、テスト済み（CLI） | json-server: 純粋な Rust バイナリ、514/514。`tests/app.rs` に 8 言語すべての CLI |
| MCP サーバー（`mcp`） | テスト済み | `src/mcp.rs` のプロトコルテスト |

## テストが実行される場所

CI は macOS と Windows に Node、Python、Go、Ruby、Java をインストールします。PHP と
.NET はインストールしないので、PHP と C# のテストはそこではスキップされ、それらがインストールされたマシンでのみ実行されます。
