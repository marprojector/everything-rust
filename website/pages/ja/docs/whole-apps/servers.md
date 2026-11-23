---
title: サーバー
description: 同じコマンドで HTTP サーバーも証明できます。--http、--fresh、--arg、--pace、そしてどのユニットも応答しないルートをすべてプロキシする axum のフロントドア。
---

# サーバー

同じコマンドで HTTP サーバーも証明できます。`--http` は、コマンドがサーバーを起動することを示します。ポートは
`PORT` またはコマンド内の `{port}` から取得します（常に同じポートで待ち受けるサーバーには `--port`）:

```sh
everything-rust app record . --http -- node server.js
everything-rust app replay .
everything-rust app front-door .                                    # axum: すべてのルートをプロキシ
everything-rust app wire . api --route 'GET /users/{id}' --route '*'
everything-rust app lift .
```

- リクエストは、アプリのドキュメントにある `curl` の行（`curl -X POST -d ... http://localhost:8080/x`）、
  `--inputs`（`{"name", "method", "path", "headers", "body"}`）、そしてトラップから来ます: パスのセグメントとクエリの
  値を CLI の引数と同じトラップに置き換えたもの、ボディをテキストと JSON のトラップに置き換えたもの、そして
  存在しないルートです。各実行ではサーバーを新たに起動し、リクエストを名前順に送るので、
  リクエスト間の状態は保持されます。
- ファイルに状態を保持するサーバーには、起動のたびに新しいコピーが渡されます: `--fresh db.json`（ファイル
  またはフォルダ、複数指定可）は `http/files/` にスナップショットされ、元の実装、フロントドア、ビルドのいずれかが
  起動するたびに新しい作業ディレクトリにコピーされます。`--arg db.json` は起動のたびにコマンドの後ろに付く引数なので、
  Rust バイナリも元の実装と同じように起動されます。
  `--pace 50` は各レスポンスの後に 50 ms 待ちます。サーバーが応答後に行う処理（書き込んだばかりのものを再読み込みする
  ファイル監視など）のためです。
- ケースは `rust/app-cases/http/<case>.toml`（リクエスト、ステータス、重要なヘッダ）
  と `<case>.body`（レスポンスボディ。2 回の実行で異なった部分は `[..]`）です。Date、Server などの
  レスポンスごとに変わるヘッダは除外されます。気にしないヘッダの行は削除してください。
- フロントドアは axum のサーバーで、元の実装を自身のポートで起動し、どのユニットも応答しないルートを
  すべてプロキシします。`wire --route` は axum のルートを受け取ります: `METHOD /path/{param}`、任意のメソッドには `/path`、
  それ以外すべてには `*` です。グルーコードはルート、そのパラメータ、リクエストを受け取ります。
  `lift` はプロキシを取り除き、その後 HTTP ケースは CI で `app replay` によって実行されます。
## 実例

[json-server](/docs/real-world/json-server) には `--fresh`、`--arg`、`--pace` が必要でした: 書き込みのたびにデータベースを書き直し、ファイル監視が直後の書き込みを元に戻してしまうことがあるからです。
