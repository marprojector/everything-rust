---
title: 포팅
description: 개발자나 에이전트가 작성하는 유일한 파일인 port.rs 작성법과, 원본과 일치하게 유지하는 규칙.
---

# 포팅

포트는 개발자나 에이전트가 `src/port.rs`에 작성합니다. crate의 나머지는 모두 생성됩니다. `port.rs`에서 시그니처를 바꾼 뒤에는 `everything-rust refresh <crate>`를 실행하세요.

## diff 한 바퀴를 아껴 주는 규칙

- 코드를 쓰기 전에 계획이 해당 유닛에 대해 나열한 위험 요소를 `file:line` 위치에서 하나씩 읽으세요.
- **문자열 오프셋**: JS와 Java는 UTF-16 코드 유닛으로, Rust는 UTF-8 바이트로 인덱싱합니다. 생성된 `src/utf16.rs`를 사용하세요.
- **오류**: `Err("TypeName: message")`를 반환하세요. Python, Ruby, Java에서는 해당 예외가 되고, Go에서는 `error` 텍스트가 됩니다. Rust의 panic은 기록된 오류와 절대 일치하지 않습니다.
- **Python과 Ruby의 숫자**: 원본이 그대로 전달하는 int는 int로 돌아와야 합니다. Python 포트에서는 int-or-float에 `serde_json::Value`가 아니라 `crate::pynum::Num`을 사용하세요.
- **C**: `crash:`로 기록된 입력은 정의되지 않은 동작이므로 어떤 결과든 통과합니다.
- **순서 있는 맵과 객체 키**: 순서가 출력에 영향을 주면 `indexmap`을 사용하세요.
- **정규식 전후방 탐색이나 역참조**: `regex`가 아니라 `fancy-regex`를 사용하세요. 어느 쪽이든 `\w`, `\b`, `\d`와 대소문자 무시 매칭은 ASCII가 아닌 텍스트를 원본 언어의 엔진과 다르게 처리합니다. 원본의 `\w`가 ASCII 전용인 곳(`u` 없는 JS, Java)에서는 `unicode(false)`를 설정하거나 ASCII 클래스(`[A-Za-z0-9_]`)를 직접 쓰세요.
- **`unsafe` 금지.** 정말 필요하다면 최소한으로, 격리해서, `// SAFETY:` 주석과 함께 쓰세요.
- **헬퍼는 비공개로.** 유닛의 export만 `pub`입니다. `pub` 헬퍼는 브리지된 API의 일부가 됩니다.
- 직접 따져 본 케이스는 `port.rs`에 Rust 유닛 테스트로 추가하세요.

생성된 `js.rs`에는 Rust가 JavaScript와 다르게 동작하는 부분을 재현하는 헬퍼가 들어 있습니다. 예: `js::number_to_string`(JS `String(n)`), `js::math_round`(`Math.round`).

## 예시: 위치가 한 칸 밀린 이모지

[x18n](https://github.com/x18n-dev/x18n)의 `packages/core/src/fuzzy.ts`를
[`demo/`](https://github.com/marprojector/everything-rust/tree/main/demo)에서 두 방식으로 포팅했습니다:

```sh
everything-rust analyze ../x18n                                      # fuzzy.ts: ready, wasm-bindgen (core runs in browsers)
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --out demo/fuzzy-rs
everything-rust record demo/fuzzy-rs --count 2000
# src/port.rs 작성
everything-rust gate demo/fuzzy-rs
```

Rust `char` 단위로 인덱싱한 첫 번째 포트는 간단한 테스트는 통과했지만 `diff`에서 실패했습니다:

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

JS의 `indexOf`는 UTF-16 코드 유닛을 세므로 이모지 하나가 두 위치를 차지합니다.
`demo/*/src/port.rs`의 포트는 생성된 `utf16` 헬퍼를 사용해 2000/2000 케이스와 게이트를 통과합니다.
napi-rs(`demo/fuzzy-napi`)와 wasm-bindgen(`demo/fuzzy-rs`) 양쪽 모두입니다.

everything-rust이 x18n 전체에 대해 작성한 계획: [`docs/examples/x18n.md`](https://github.com/marprojector/everything-rust/blob/main/docs/examples/x18n.md).
## 얼마나 커야 할까요?

`gate`는 포트의 줄 수를 원본과 비교해 3배(import와 시그니처용 50줄 추가 허용)를 넘으면 실패합니다. 이를 넘는다면 대개 crate가 이미 하는 일을 손으로 다시 쓰고 있거나, 케이스를 하나하나 따로 처리하고 있다는 뜻입니다. `analyze`가 사용할 crate를 알려 줍니다.
