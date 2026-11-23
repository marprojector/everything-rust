---
title: 実プロジェクトの移植
description: everything-rust が実際のコードに出会ったときに起きたこと。遅い結果も速い結果と並べて掲載し、それを受けてツールで何が変わったかも記します。
---

# 実プロジェクトの移植

everything-rust が実際のコードに出会ったときに起きたこと、そしてそれを受けてツールで何が変わったか。以下の数値はすべて、まず結果が同一であることを確認するベンチマークから得たもので、遅いケースも速いケースと並べて掲載しています。

| 移植 | 内容 | 結果 |
| --- | --- | --- |
| [json-server](/docs/real-world/json-server) | Node サーバー全体を、`app record`、`front-door`、`wire`、`lift` でエンドツーエンドに移植 | 記録された 514/514 件の HTTP ケースに合格する純粋な Rust バイナリ。6 つのユニットが `gate` に合格 |
| [Fuse.js](/docs/real-world/fuse-js) | あいまい検索の、検索が時間を費やすパス | 3,414/3,414 ケースが一致。50,000 文字列で 32 倍高速（1 コアでは 5.3 倍） |
| [vercel/ms](/docs/real-world/vercel-ms) | 小さな期間パーサーをエンドツーエンドで | 975/975 ケースが一致。呼び出しごとに 0.56 倍、`parse` のバッチで 18 倍、`format` のバッチで 0.8 倍 |
| [x18n](/docs/real-world/x18n) | ケーススタディ用のリポジトリ | `packages/core` ですべてのゲートに合格、ブリッジ経由では TS より遅い。検索はバッチで 18-22 倍 |
| [6 つの言語](/docs/real-world/six-languages) | Go の `path`、Ruby の `shellwords`、Python の `colorsys`、小さなファサード経由の JDK と glibc の挙動 | すべての移植版がゲートに合格、記録ケースは合計 13,010 件 |
| [フィクスチャと静的実行](/docs/real-world/fixtures) | ツールに負荷をかけるために書いたフィクスチャと、分析はしたが実行していないライブラリ | |

実際のものを移植しましたか？記録を歓迎します: [コントリビュート](/docs/project/contributing#share-a-real-port)を参照してください。
