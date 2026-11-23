---
title: 프로파일링과 분석
description: profile로 시간이 어디서 쓰이는지 찾은 다음, analyze로 유닛, 포팅 순서, 위험 요소, crate를 계획합니다.
---

# 프로파일링과 분석

## Profile: 옮길 가치가 있는 것 찾기

목표가 속도라면 무엇을 고르기 전에 먼저 측정하세요:

```sh
everything-rust profile <repo> -- node bench.mjs      # or: python3 bench.py, ruby bench.rb,
                                                    #     java -cp out Main, go test -run X -bench . ./pkg
```

`profile`은 실제 워크로드를 언어별 프로파일러(node, cProfile, `go test` pprof, JFR, Ruby 트레이서)로 실행하고, 핫 함수를 포함 시간 기준으로 나열하며, 그 호출자와 어떤 루프를 배치로 만들지 보여 줍니다. Go는 `go test`를 통해서만 프로파일링하며, C는 지원하지 않습니다.

- 항목마다 한 번씩 호출되는 작은 함수는 단독으로 포팅할 가치가 **없습니다**. Rust 호출 한 번의 비용이 그 함수가 하는 일보다 크고, JIT(V8, JVM, Go)는 이런 함수를 나노초 단위로 실행합니다. 호출자와 함께 포팅하거나 [배치로 만드세요](/docs/concepts/batches). 조언 줄이 어느 쪽인지 정확한 명령어와 함께 알려 줍니다.
- 이미 컴파일된 코드(numba `@njit`, Cython, `@jax.jit`)는 기계어로 실행되므로 Rust 포트로 얻는 속도 이득이 적습니다. `profile`은 이를 "Already compiled" 아래에 나열합니다.

## Analyze: 포팅 계획 세우기

```sh
everything-rust analyze <repo> -o plan.md        # for people
everything-rust analyze <repo> --json            # for agents: schema everything-rust/analyze@1
everything-rust analyze <repo> --whole-app       # plus entry points, shape, dependencies, decisions
```

`analyze`는 유닛, 포팅 순서, `file:line`이 붙은 위험 요소, 유닛별 브리지, 각 의존성이 crate에 얼마나 잘 매핑되는지, 그리고 가장 좋은 첫 포팅 대상인 `start_here`를 나열합니다. `--whole-app`은 앱의 형태(CLI, 서버 또는 라이브러리)와 진입점, 모든 의존성(import했거나 선언한 것)과 그 crate, 조각(서로 import하는 유닛은 함께 옮김), 권장 답이 붙은 번호 매긴 결정 사항을 추가합니다. [앱 전체 포팅](/docs/whole-apps/overview)을 참고하세요.

- 패키지(`package.json`)와 소스 파일을 읽고, `node_modules`, 빌드 출력, 픽스처,
  플레이그라운드, 도구 설정은 건너뜁니다. Vue/Svelte/Astro의 `<script>` 블록도 import를 위해 파싱합니다.
- **유닛**: 소스 파일이 든 폴더, 또는 패키지 소스 루트에 있는 파일.
- **웨이브**: 유닛이 import하는 것은 더 앞선 웨이브에 옵니다. import 순환은 함께 포팅합니다.
- **브리지**: 유닛이나 유닛이 import하는 무언가가 Node를 필요로 하면 napi-rs, 이식 가능한
  코드에는 wasm-bindgen, UI와 에디터 코드는 JS에 유지.
- **준비도(readiness)**(얼마나 안전한가)와 **우선순위(priority)**(안전하고 가치가 있는가: 루프와 문자열 작업, 테스트, 크기).

| 위험 요소 | 심각도 | 예시 |
| --- | --- | --- |
| 문자열 오프셋 | high | `slice`, `indexOf`, `matchAll`, `d` 플래그: UTF-16 대 UTF-8 |
| 정규식 문법 | high | 전후방 탐색, 역참조, `y`: `regex` crate에 없음 |
| 동적 정규식 | high | `new RegExp(userPattern)`: 런타임의 JS 문법 |
| 동적 코드 | high | `eval`, `new Function`, 계산된 `import()` |
| 동적 타이핑 | medium | `any`, `'key' in value` |
| 순서 있는 컬렉션 | medium | `Map`, `Set`, `Object.keys`: 삽입 순서 |
| async, 제너레이터 | medium | promise와 제너레이터 경계 |
| 클래스 상속, 리플렉션 | medium | `extends`, `Proxy`, `Reflect`, `Symbol` |
| 콜백 형태의 API | medium | 함수 타입 매개변수와 메서드: 호출이 잦은 브리지 |
| 예외, 숫자, JSON | low | `throw`, 비트 연산, `JSON.stringify` 순서 |
## 의존성에 맞는 crate

`analyze`는 코드가 import하는 각 라이브러리에 맞는 Rust crate를 `good`, `partial`, `none` 적합도와 함께 알려 줍니다. 표에 없는 라이브러리는 `unknown`으로 보고하며, 표는 crate를 추측하지 않습니다. 추가하려면 [기여하기](/docs/project/contributing#add-a-crate-mapping)를 참고하세요.
