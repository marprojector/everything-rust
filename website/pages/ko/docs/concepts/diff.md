---
title: Diff와 알려진 차이
description: diff는 기록된 모든 케이스를 Rust에 대해 재생합니다. known-diffs.jsonl은 의도한 차이를 기록하며, 오래된 기록은 게이트를 실패시킵니다.
---

# Diff와 알려진 차이

```sh
everything-rust build <crate>
everything-rust diff <crate>                     # N/N이어야 함
everything-rust diff <crate> --json
```

`diff`는 빌드된 Rust 모듈을 원본의 호출자가 호출하는 방식 그대로 호출해 `cases.jsonl`을 재생하고, 불일치 목록을 보여 줍니다. 불일치는 동작 차이입니다. 기록이 아니라 포트를 고치세요.

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

## 알려진 차이

포트가 의도적으로 달라야 할 때도 있습니다. 사용자가 원본의 버그를 고치길 원하는 경우입니다. 먼저 물어본 다음, 케이스마다 한 줄씩 `<crate>/known-diffs.jsonl`에 추가하세요:

**known-diffs.jsonl**
```json
{"fn": "...", "args": [...], "actual": ..., "reason": "..."}
```

`fn`, `args`, `actual`은 `diff --json`에서 복사하세요. 그러면 해당 케이스는 정확히 그 결과일 때만 통과하고, 더 이상 차이가 나지 않으면 `gate`가 실패하므로 그때 그 줄을 지우세요.

## 오래된 기록

`record`는 원본 유닛 파일의 지문을 `recorded-from.json`에 저장합니다. 나중에 원본이 바뀌면 `diff`는 경고하고 `gate`는 다시 기록할 때까지 실패합니다. 원본의 예전 동작을 기준으로 통과하는 일은 없습니다. `gate`가 기록이 오래되었다고 하면, 포팅을 더 진행하기 전에 다시 기록하세요.

## 포팅이 정체될 때

`diff`는 빌드 사이의 진행 상황을 추적합니다. 포팅이 정체되었다고 하면(`trend.stalled`: `port.rs`를 연달아 세 번 바꿨는데 일치하는 케이스가 늘지 않음) 편집을 멈추세요. 남은 불일치에는 사용자만 아는 규칙이 필요하거나, 해당 함수들은 당분간 원본에 남겨 두는 편이 나을 수 있습니다.

## 부동소수점 허용 오차

crate를 `--float-tolerance`로 기록했다면 `diff`도 그것을 사용합니다. `diff`에 `--float-tolerance`를 주면 그 실행에서만 덮어씁니다. [부동소수점 허용 오차](/docs/concepts/float-tolerance)를 참고하세요.
