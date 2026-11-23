---
title: 기록
description: record는 생성된 엣지 케이스, 의미상의 함정, 원본 자체의 테스트, 사용자 입력으로 원본을 실행하고 결과를 cases.jsonl로 저장합니다.
---

# 기록

```sh
everything-rust record <crate>
everything-rust record <crate> --from-tests -- npx jest      # 또는: -- pytest
everything-rust record <crate> --inputs extra.jsonl
```

`record`는 원본 함수를 실행하고 각 함수가 반환하거나 발생시킨 것을 crate 옆의 `cases.jsonl`에 저장합니다. 이것이 포트가 맞춰야 할 동작입니다. 나중에 생기는 불일치는 동작 차이입니다. 기록이 아니라 포트를 고치세요.

## 케이스의 출처

- **생성된 엣지 케이스**: ASCII가 아닌 문자, 이모지, `-0`, `NaN`, 부동소수점 자리에 들어간 정수 등(`--count`로 함수당 개수 지정).
- 모든 매개변수에 대한 **의미상의 함정**: 문자열 길이와 대소문자, 부호 있는 나눗셈, 오버플로 경계, 부동소수점 출력, 맵과 정렬 순서.
- 정규식을 쓰는 파일의 모든 함수에 대한 **정규식 함정**: 터키어의 점 없는 `ı`, 켈빈 기호, 긴 s, 아랍-인도 숫자, 줄 구분자, 어말 시그마, zero-width joiner.
- **원본 자체의 테스트**: `--from-tests -- <test command>`는 테스트 스위트가 유닛의 export 함수에 하는 모든 호출을 기록합니다(TS/JS는 어떤 러너든, Python은 pytest나 unittest). 소스는 끝난 뒤 복원됩니다. vercel/ms에서는 Jest 테스트 167개에서 호출 340개를 수집했습니다.
- **사용자 입력**: 한 줄에 `{"fn": name, "args": [...]}` 하나씩 담긴 `--inputs cases.jsonl`. 실제 입력(프로덕션 샘플, 버그 리포트)을 추가하세요. 프로덕션을 망가뜨리는 사용자 입력은 생성기가 떠올리는 경우가 드뭅니다.

Python 호출자는 `1.0`만큼 `1`도 자주 넘기므로, 기록은 Python 케이스 네 개 중 하나에서 정수 값을 int로 넘깁니다.

## cases.jsonl의 마커

JSON은 `undefined`, NaN, Infinity, -0, 짝 없는 서로게이트를 표현할 수 없습니다. 구조화된 값은 생성된 `index.cjs`가 변환하는 마커를 담으며, 헬퍼는 crate의 `js.rs`에 있습니다:

| 마커 | 의미 |
| --- | --- |
| `{"$undefined": true}` | JavaScript `undefined` |
| `{"$num": "NaN"}` | JSON이 담을 수 없는 숫자(`NaN`, `Infinity`, `-0`) |
| `{"$utf16": [...]}` | 짝 없는 서로게이트를 위해 UTF-16 코드 유닛으로 표현한 문자열 |

함수의 계약을 벗어나는 C 입력은 원본을 크래시시킬 수 있습니다. C 드라이버는 각 케이스를 fork한 자식 프로세스에서 실행하고 `crash: SIGSEGV`를 기록합니다. C의 동작이 정의되지 않았으므로 `diff`는 그 경우 포트의 어떤 동작도 받아들입니다.

## 두 번의 실행과 달라지는 것

`record`는 원본을 두 번 실행하고 두 실행 사이에 달라진 것을 분류합니다: 바뀐 값(시간, ID)은 와일드카드가 되고, 같은 줄이 다른 순서로 나오면(해시 맵, 동시성) 순서 무관하게 비교하며, 줄 수가 달라지면 무엇이든 일치하는 `...`이 됩니다. `record`는 종류별로 목록을 보여 줍니다. 해시 순서나 동시성 차이는 포트에서 실제로 바뀔 수 있는 바로 그 부분이므로 하나하나 모두 확인하세요.

## 기록은 원본을 기억합니다

`record`는 유닛 파일의 지문(`recorded-from.json`)을 저장합니다. 나중에 원본이 바뀌면 `diff`는 경고하고 `gate`는 다시 기록할 때까지 실패합니다. 원본의 예전 동작을 기준으로 통과하는 일은 없습니다. [Diff](/docs/concepts/diff#stale-recordings)를 참고하세요.

## TypeScript 관련 사항

`record`는 TS 소스를 있는 그대로 실행합니다: Node resolve 훅이 번들러 스타일 import를 따라가고(`./errors`를 `./errors.ts`로), 저장소에 npm 패키지가 설치되어 있지 않으면 `--resolve-from`이 패키지를 제공하며, 변환이 필요한 TS(enum, 매개변수 프로퍼티)는 `--experimental-transform-types`로 다시 시도합니다. Node에서 직접 실행할 수 없는 TS는 `--module dist/x.mjs`로 빌드 결과물에서 기록하거나 `--node-arg=--import=tsx`로 기록할 수 있습니다.

## 수치 코드

부동소수점이 비트 단위로 일치하는 경우가 드문 코드라면 `--float-tolerance`로 기록하세요. [부동소수점 허용 오차](/docs/concepts/float-tolerance)를 참고하세요.
