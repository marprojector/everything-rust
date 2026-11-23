---
title: TypeScript/JavaScript
description: 원래 API를 유지하며, napi-rs나 wasm-bindgen을 통해 Node.js 코드(TypeScript 또는 JavaScript)를 Rust로 포팅합니다.
---

# TypeScript/JavaScript

| | |
| --- | --- |
| 유닛 | 폴더 또는 파일 |
| 브리지 | 생성된 바이너리 와이어를 쓰는 napi-rs 또는 wasm-bindgen, 클래스와 async 포함 |
| 기록과 재생 담당 | `.ts` 소스를 실행하는 Node |
| 필요 조건 | Node 22.6+ (`profile`은 Node 24+ 필요) |
| 상태 | 검증됨 |

## 브리지 선택

`analyze`는 유닛이나 유닛이 import하는 무언가가 Node를 필요로 하면 napi-rs를, 이식 가능한 코드에는 wasm-bindgen(브라우저에서도 실행됨)을 고르며, UI와 에디터 코드는 JS에 남깁니다. `bridge --kind napi|wasm`으로 덮어쓸 수 있습니다.

브리지를 건널 수 없는 export는 이유와 함께 TS에 남고, 섀도 모듈이 원본을 다시 export합니다: 함수를 받거나 반환하는 함수, 제너레이터와 이터레이터, RegExp, `instanceof` 검사, 제네릭(`--as-json T`는 앱이 데이터로만 채우는 제네릭을 브리지합니다), 함수를 담는 타입의 값. [브리지](/docs/concepts/bridge)를 참고하세요.

## TS를 있는 그대로 기록하기

`record`는 TS 소스를 직접 실행합니다: Node resolve 훅이 번들러 스타일 import를 따라가고, 저장소에 npm 패키지가 설치되어 있지 않으면 `--resolve-from`이 패키지를 제공하며, 변환이 필요한 TS(enum, 매개변수 프로퍼티)는 `--experimental-transform-types`로 다시 시도합니다. `record --from-tests -- npx jest`는 어떤 러너를 쓰든 원본의 자체 테스트 스위트가 하는 모든 호출을 기록합니다. 원본의 커버리지는 TS/JS에 대해 측정합니다([커버리지](/docs/concepts/coverage)).

## 포트가 정확히 맞춰야 할 부분

실제 코드에서 기록된 케이스로 발견한 것들입니다:

- `str.length`, `indexOf`, `slice`는 UTF-16 코드 유닛을 세므로 이모지 하나가 두 위치를 차지합니다. 생성된 `utf16` 헬퍼를 사용하세요.
- `u` 플래그가 없는 `/.../i`는 ASCII 문자만 대소문자를 무시하지만 Rust의 `regex`는 유니코드 전체를 무시합니다. 그래서 vercel/ms에서 `"1 ſ"`(긴 s)와 켈빈 기호가 초로 파싱될 뻔했습니다.
- `Math.round`는 0.5를 올림합니다(`-2.5`는 `-2`). Rust의 `round`는 0에서 먼 쪽으로 반올림합니다. 생성된 `js.rs`에 `js::math_round`가 있습니다.
- `${n}`은 `1e+21`, `5e-324`를 출력하고 `-0`은 `0`으로 출력합니다. `js::number_to_string`이 이를 재현합니다.
- `String.prototype.trim`은 Rust의 `char::is_whitespace`가 아니라 JS 공백 정의를 씁니다.

## 속도

호출 단위로는 포트가 매번 브리지 비용을 치르고, V8은 작은 함수를 나노초 단위로 실행합니다. vercel/ms는 기록된 케이스에서 호출당 0.56x였고, x18n의 core 포트는 JS에서 호출할 때 TS보다 느립니다. 배치로 하면 루프가 Rust로 옮겨집니다:

| 워크로드 | 원본 | everything-rust 적용 |
| --- | ---: | ---: |
| x18n 사이드바 검색, 키 50,000개 (Node) | 쿼리당 10 ms | 0.5 ms: 20배 빠름 |
| 브라우저에서 같은 검색 (wasm, 스레드 1개) | 10 ms | 0.95 ms: 11.5x |
| Fuse.js, 문자열 50,000개 | 91.2 ms | 2.8 ms: 32x |
| vercel/ms `parse` x100,000 | 11.8 ms | 0.65 ms: 18x (전달 비용 포함 시 3.4x) |
| vercel/ms `format` x100,000 | 2.1 ms | 전달 비용 포함 2.5 ms: 0.8x |

사례 연구: [json-server](/docs/real-world/json-server), [Fuse.js](/docs/real-world/fuse-js), [vercel/ms](/docs/real-world/vercel-ms), [x18n](/docs/real-world/x18n).
