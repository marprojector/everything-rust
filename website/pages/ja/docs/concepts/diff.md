---
title: 差分と既知の差分
description: diff は記録されたすべてのケースを Rust に対してリプレイします。known-diffs.jsonl は意図した差分を記録し、古くなった記録はゲートで失敗します。
---

# 差分と既知の差分

```sh
everything-rust build <crate>
everything-rust diff <crate>                     # N/N でなければならない
everything-rust diff <crate> --json
```

`diff` は `cases.jsonl` を、ビルドした Rust モジュールに対して、元の実装の呼び出し元と同じ呼び方でリプレイし、不一致を一覧表示します。不一致は挙動の違いです: 直すのは移植版であり、記録ではありません。

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

## 既知の差分

移植版が意図的に異なる場合もあります: ユーザーが元の実装のバグを直したい場合です。まず確認してから、ケースごとに 1 行を `<crate>/known-diffs.jsonl` に追加します:

**known-diffs.jsonl**
```json
{"fn": "...", "args": [...], "actual": ..., "reason": "..."}
```

`fn`、`args`、`actual` は `diff --json` からコピーします。そのケースは、まさにその結果の場合にのみ合格し、差がなくなると `gate` が失敗するので、そのときに行を削除してください。

## 古くなった記録

`record` は元のユニットのファイルのフィンガープリントを `recorded-from.json` に保存します。後で元の実装が変わると、元の実装の以前の挙動に対して合格させるのではなく、`diff` が警告し、再度記録するまで `gate` が失敗します。`gate` が記録が古いと言ったら、移植を進める前に再度記録してください。

## 移植が行き詰まったとき

`diff` はビルドをまたいで進捗を追跡します。移植が行き詰まっている（`trend.stalled`: `port.rs` への 3 回連続の変更で一致するケースが増えなかった）と言われたら、編集をやめてください。残りの不一致にはユーザーしか知らないルールが必要かもしれませんし、それらの関数は当面元の実装に残しておくべきかもしれません。

## 浮動小数点の許容誤差

クレートが `--float-tolerance` 付きで記録されていれば、`diff` はそれを使います。`diff` に `--float-tolerance` を付けると、その 1 回の実行だけ上書きされます。[浮動小数点の許容誤差](/docs/concepts/float-tolerance)を参照してください。
