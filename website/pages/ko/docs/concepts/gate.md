---
title: 게이트
description: gate는 포팅 완료의 정의입니다. gate가 실행하는 모든 검사와, 실패한 검사를 고치는 방법.
---

# 게이트

```sh
everything-rust gate <crate>
```

완료란 `gate`가 통과한다는 뜻입니다. 검사 항목은 다음과 같습니다:

- `cargo fmt`, `-D warnings`를 적용한 clippy, cargo 테스트
- `unsafe`는 `// SAFETY:` 주석이 있을 때만 허용, `unimplemented!`는 없어야 함
- 원본이 의도적으로 실패하는 모든 곳(`throw`, `raise`, `error` 결과)마다 기록된 실패 케이스. 게이트가 이를 지적하면 `record --inputs`로 추가하세요.
- 원본이 오류를 반환하는 함수에는 `unwrap`, `expect`, `panic!`이 없어야 함. `Err`를 반환하세요. 원본도 불가능하다고 보는 상태라면 `// INFALLIBLE: <why>` 주석을 답니다.
- 포트는 원본 줄 수의 최대 3배(import와 시그니처용 50줄 추가 허용)
- 모듈이 빌드되고, 모든 export가 존재함
- [알려진 차이](/docs/concepts/diff#known-diffs)를 포함해 기록된 모든 케이스가 일치하고, 기록이 [오래되지](/docs/concepts/diff#stale-recordings) 않았음
- TS/JS의 경우 [커버리지](/docs/concepts/coverage) 줄: 케이스가 원본을 얼마나 실행했는지

실패한 검사마다 `fix`가 함께 나옵니다. 그것부터 실행하세요.

## 그다음 보고하고 기다리기

게이트가 통과하면 워크플로 스킬은 에이전트에게 다음을 보고하게 합니다: 바뀐 것(파일, 함수, 브리지, 배치), 게이트 출력, diff 수치와 벤치 수치, 남은 동작 차이나 위험 요소, 계획에 남은 작업. 그리고 다음 유닛으로 넘어가기 전에 승인을 기다리게 합니다.
