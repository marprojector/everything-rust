---
title: 앱 전체 포팅
description: 첫 기록부터 순수 Rust 바이너리까지. 목표는 원본을 참조용으로 남겨 둔 채, 원본처럼 동작하는 Rust 앱입니다.
---

# 앱 전체 포팅

목표는 원본을 참조용으로 남겨 둔 채, 원본처럼 동작하는 순수 Rust 앱입니다. Rust 코드는 에이전트가 작성하고, everything-rust은 작업 순서를 정하고, 뼈대를 만들고, 모든 단계를 증명합니다. CLI, HTTP 서버, 라이브러리를 지원합니다.

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in    pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down proved by
 decisions to make      outside         original answers                                           replay
```

## 1. 계획

```sh
everything-rust analyze . --whole-app
```

`--whole-app`은 다음을 추가합니다: 앱의 형태(CLI, 서버 또는 라이브러리)와 진입점, 모든 외부 의존성(import했거나 선언한 것)과 그에 맞는 Rust crate 및 적합도, 조각 단위의 포팅 순서(서로 import하는 유닛은 함께 옮김), 그리고 권장 답이 붙은 번호 매긴 결정 사항: 유지할 유닛, 막힌 유닛, crate가 없거나 일부만 있는 의존성.

## 2. 앱 전체를 기록한 다음 진입점 옮기기

1. [`app record`와 `app replay`](/docs/whole-apps/record-replay)는 프로그램을 블랙박스로 보고 바깥에서 증명합니다.
2. [`app front-door`](/docs/whole-apps/front-door)는 원본 앞에 Rust 바이너리를 둡니다. 모든 명령은 여전히 원본을 실행합니다.
3. 각 유닛은 [작업 루프](/docs/concepts/the-loop)를 거친 다음, `app wire`가 그 명령을 Rust에 넘깁니다.
4. [`app lift`](/docs/whole-apps/lift)는 재생이 폴백 비율 0으로 통과하고 모든 유닛이 게이트를 통과하면 원본을 제거합니다.

[서버](/docs/whole-apps/servers)와 [라이브러리](/docs/whole-apps/libraries)도 같은 명령을 사용합니다.

## 에이전트가 진행하는 방식

[whole-app 스킬](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md)이 에이전트를 안내하며, 개발자만 내릴 수 있는 결정부터 시작합니다:

- **집중 질문(Grilling)**: 에이전트가 여러 차례에 걸쳐 질문하며, 각 질문에는 권장 답이 붙습니다. 포팅이 의존하는 모든 결정이 정해질 때까지 계속하고, 각 결정을 `rust/DECISIONS.md`에 ADR로 기록하며, `rust/GLOSSARY.md`에 원본의 용어를 Rust 이름으로 매핑합니다.
- **조사**: 백그라운드 에이전트가 각 의존성의 1차 자료를 읽고, 발견한 의미상의 함정을 모두 담아 `rust/research/<topic>.md`를 작성합니다.
- **세션 간 작업**: `rust/MAP.md`에 모든 유닛과 그 상태가 나열됩니다. 세션은 시작 전에 유닛 하나를 맡겠다고 표시하고, 각 체크포인트마다 다음 세션을 위해 `rust/HANDOFF.md`를 남깁니다. 같은 웨이브의 유닛은 서브에이전트가 병렬로 포팅할 수 있습니다.
- **전환**: 에이전트가 할 수 있는 일을 하고, 개발자만 할 수 있는 단계(배포, DNS, 시크릿)를 위한 대화형 스크립트 `rust/cutover.sh`를 작성합니다. 각 단계마다 되돌아갈 방법이 있습니다.

집중 질문, 조사, 그리고 이런 방식들은 [Matt Pocock의 스킬](https://github.com/mattpocock/skills)에서 가져와 다듬었습니다.

## Rust가 옛 설계에 묶이지 않을까요?

둘을 나란히 실행하는 동안에만 그렇습니다. 기록은 앱의 내부가 아니라 경계(CLI의 출력, 서버의 응답, 모듈이 export하는 함수)에서 동작을 검사합니다. 유닛이 아직 브리지를 건너는 동안에는 기존 구조를 따르세요. `app lift`가 원본을 제거하고 나면 Rust를 자유롭게 재구성해도 `app replay`가 여전히 동작을 증명합니다.

## 실제 사례

[json-server](/docs/real-world/json-server)는 모든 단계를 거쳤습니다: 기록된 HTTP 케이스 514개, 폴백 99.8%의 프런트 도어, 읽기를 먼저 연결(폴백 22.6%), 그다음 모든 라우트(0%), 그리고 514/514를 통과하는 순수 Rust 바이너리로의 리프트.
