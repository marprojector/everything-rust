---
title: 브리지
description: bridge는 유닛 하나를 위한 Rust crate의 뼈대를 바인딩, 래퍼와 함께 만들어, 호출하는 쪽이 원래 API를 그대로 쓰게 합니다.
---

# 브리지

```sh
everything-rust bridge <repo> <unit-id>          # <repo>/rust/<name>에 crate 생성, 덮어쓰지 않음
everything-rust refresh <crate>                  # port.rs의 시그니처를 바꾼 후
```

`bridge`는 바인딩(`lib.rs`와 원본 언어가 불러오는 래퍼)과 포팅할 스텁(`port.rs`)이 담긴 crate를 만듭니다. 포트는 `src/port.rs`에만 작성합니다. `src/lib.rs`와 래퍼는 생성되며, `refresh`가 `port.rs`의 시그니처로부터 이를 다시 생성하고 새 배치 클래스도 추가합니다. `port.rs`는 절대 건드리지 않습니다.

옵션:

- `--batch fn:param`은 많은 `param` 값을 Rust 안에 유지하는 클래스를 추가합니다. [배치](/docs/concepts/batches)를 참고하세요.
- `--as-json T`는 앱이 데이터로만 채우는 타입 매개변수(`paginate<T>(items: T[])`의 `T`)를 JSON 값으로 브리지합니다.
- `--keep "name: reason"`은 더 많은 export를 원본 언어에 남깁니다. 예: `Intl` 데이터에 의존하는 코드.
- `--kind`는 계획이 고른 브리지를 덮어씁니다(TypeScript에서 napi-rs 또는 wasm).

## 언어별 브리지

| 언어 | 브리지 |
| --- | --- |
| TypeScript/JavaScript | 생성된 바이너리 와이어를 쓰는 napi-rs 또는 wasm-bindgen, 클래스와 async 포함 |
| Python | PyO3 (abi3), 직접 변환, 타입이 있는 예외 |
| C | 원래 이름을 가진 `extern "C"` 함수와 헤더 |
| Go | 원래 API를 가진 패키지 안의, 타입이 있는 cgo 호출과 생성된 바이너리 와이어 |
| Ruby | 원래 모듈 아래의 magnus 확장 |
| Java | 원래 메서드를 가진 래퍼 클래스 안의, 타입이 있는 JNI와 생성된 바이너리 와이어 |

PHP와 C# 유닛은 아직 브리지할 수 없습니다. [PHP](/docs/languages/php)와 [C#](/docs/languages/csharp)을 참고하세요.

## bridge가 결정하는 것

- 브리지를 건널 수 없는 export는 이유와 함께 원본 언어에 남고, 섀도
  모듈이 원본을 다시 export합니다. TS의 경우:
  함수를 받거나 반환하는 함수, 제너레이터와 이터레이터, RegExp, `instanceof` 검사,
  제네릭(`--as-json T`는 앱이 데이터로만 채우는 제네릭을 브리지합니다), 그리고 타입(import, re-export, `extends`를 따라가며 확인)이
  함수를 담는 값. `--keep "name: reason"`으로 더 남길 수 있습니다. 예: `Intl` 데이터에 의존하는 코드.
- TS 타입은 Rust 타입으로 매핑되며, 타입 별칭은 import를 따라 해석되고 매개변수 타입은
  기본값에서 추론합니다. 구조화된 값은 생성된 바이너리 와이어로 건너갑니다(와이어가 모델링하지 않는
  형태에 한해 JSON으로 폴백).
- JSON은 `undefined`, NaN, Infinity, -0, 짝 없는 서로게이트를 표현할 수 없습니다. 구조화된 값은
  생성된 `index.cjs`가 변환하는 마커(`{"$undefined": true}`, `{"$num": "NaN"}`, `{"$utf16": [...]}`)를 담으며,
  헬퍼는 crate의 `js.rs`에 있습니다.

`record`는 TS 소스를 있는 그대로 실행합니다: Node resolve 훅이 번들러 스타일 import를 따라가고
(`./errors`를 `./errors.ts`로), 저장소에 npm 패키지가 설치되어 있지 않으면 `--resolve-from`이 패키지를
제공하며, 변환이 필요한 TS(enum, 매개변수 프로퍼티)는
`--experimental-transform-types`로 다시 시도합니다.
