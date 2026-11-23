---
title: 比對與已知差異
description: diff 會將每個已錄製的案例拿來重播比對 Rust 版本。known-diffs.jsonl 記錄刻意的差異，而過期的錄製內容會讓 gate 失敗。
---

# 比對與已知差異

```sh
everything-rust build <crate>
everything-rust diff <crate>                     # 必須是 N/N
everything-rust diff <crate> --json
```

`diff` 會以原始版本呼叫端的呼叫方式，將 `cases.jsonl` 拿來重播比對建置好的 Rust 模組，並列出不一致之處。不一致就是行為差異：請修正移植版本，絕對不要修改錄製內容。

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

## 已知差異

有時移植版本本來就應該不同：使用者希望它修正原始版本的錯誤。請先詢問，然後在 `<crate>/known-diffs.jsonl` 中為每個案例新增一行：

**known-diffs.jsonl**
```json
{"fn": "...", "args": [...], "actual": ..., "reason": "..."}
```

從 `diff --json` 複製 `fn`、`args` 與 `actual`。之後該案例只有在得到完全相同的結果時才會通過，而一旦它不再有差異，`gate` 就會失敗，這時請移除那一行。

## 過期的錄製內容

`record` 會將原始單元檔案的指紋儲存在 `recorded-from.json` 中。當原始版本之後有所變更，`diff` 會發出警告，而 `gate` 會失敗，直到你重新錄製為止，而不是拿原始版本過去的行為來判定通過。如果 `gate` 說錄製內容已過期，請在繼續移植之前重新錄製。

## 當移植停滯時

`diff` 會追蹤每次建置之間的進展。當它表示移植已停滯（其 `trend.stalled`：連續三次修改 `port.rs` 都沒有增加任何一致的案例）時，請停止編輯。剩下的不一致可能需要只有使用者才知道的規則，或者那些函式暫時應該留在原始版本中。

## 浮點容差

如果 crate 是以 `--float-tolerance` 錄製的，`diff` 會使用它；在 `diff` 上指定 `--float-tolerance` 會在該次執行中覆寫它。請參閱[浮點容差](/docs/concepts/float-tolerance)。
