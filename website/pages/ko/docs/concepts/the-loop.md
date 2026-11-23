---
title: 작업 루프
description: profile, analyze, bridge, record, port, diff, bench, shadow, gate. 모든 유닛이 거치는 단계.
---

# 작업 루프

핫 함수 하나부터 앱 전체까지, 모든 포팅은 유닛 하나씩 같은 단계를 거칩니다. 포팅은 `gate`가 통과해야만 완료됩니다.

everything-rust이 포트를 검증하는 방법, 각 명령이 결정하는 것, 그리고 한계가 어디인지 설명합니다. [README](/)로 돌아가기.

## 동작 방식

```
profile ──> analyze ──> bridge ──> record ──> port.rs ──> diff ──> bench ──> shadow ──> gate
 where the    what to     the Rust   what the    you or      every     faster?    both side   done means
 time goes    port first  crate and  original    your agent  recorded  the        by side in  this passes
                          wrapper    does        write it    case      verdict    production
```

원본이 계속 기준(source of truth)으로 남습니다: `record`는 엣지 케이스(그리고 사용자가 준 입력)로 원본을 실행하고
반환값이나 발생한 예외를 저장하며, `diff`는 모든 케이스를 Rust에 대해 재생하고, 모두 일치하고
`gate`를 통과하기 전까지는 아무것도 배포되지 않습니다. 호출하는 쪽은 처음부터 끝까지 원래 API를 그대로 씁니다.

앱 전체도 같은 루프를 유닛 하나씩 거치며, 그 바깥을 프로그램 전체를 증명하는 틀이
감쌉니다([앱 전체 포팅](/docs/whole-apps/overview)):

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop above, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in         pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down      proved by
 decisions to make      outside         original answers                                                replay
```

## 작업 루프

| 단계 | 명령 | 하는 일 |
| --- | --- | --- |
| 0. 프로파일링 | `everything-rust profile <repo> -- node bench.mjs` | 실제 워크로드를 언어별 프로파일러(node, cProfile, `go test` pprof, JFR, Ruby 트레이서)로 실행: 포함 시간 기준 핫 함수, 그 호출자, 배치로 만들 루프. Rust로 얻는 것이 적은 이미 컴파일된 코드(numba, Cython, JAX)를 표시 |
| 1. 계획 | `everything-rust analyze <repo>` | 유닛, 포팅 순서, `file:line`이 붙은 위험 요소, 유닛별 브리지, 의존성과 crate의 적합도, 가장 좋은 첫 포팅 대상. `--whole-app`은 진입점과 형태, 모든 의존성(import했거나 선언한 것)과 그 crate, 조각(서로 import하는 유닛은 함께 옮김), 권장 답이 붙은 번호 매긴 결정 사항을 추가 |
| 2. 브리지 | `everything-rust bridge <repo> <unit>` | 바인딩(`lib.rs`와 원본 언어가 불러오는 래퍼)과 포팅할 스텁(`port.rs`)이 담긴 crate. `--batch fn:param`은 많은 `param` 값을 Rust 안에 유지하는 클래스를 추가. `--as-json T`는 앱이 데이터로만 채우는 타입 매개변수를 JSON 값으로 브리지 |
| 3. 기록 | `everything-rust record <crate>` | 생성된 엣지 케이스(ASCII가 아닌 문자, 이모지, `-0`, `NaN`, 부동소수점 자리에 들어간 정수 등)와 모든 매개변수에 대한 모든 의미상의 함정(문자열 길이와 대소문자, 부호 있는 나눗셈, 오버플로 경계, 부동소수점 출력, 맵과 정렬 순서)으로 원본을 실행하고 `cases.jsonl`을 저장. `--from-tests -- npx jest`(또는 `-- pytest`)는 원본의 자체 테스트 스위트가 하는 모든 호출도 기록. `--float-tolerance 1e-9`는 수치 코드에서 부동소수점이 그 비율 이내로 일치하도록 허용 |
| 3. 빌드 | `everything-rust build <crate>` | Rust 쪽을 빌드: `.node`, wasm 패키지, Python 확장 또는 공유 라이브러리 |
| 3. Diff | `everything-rust diff <crate>` | 원본의 호출자가 호출하는 방식 그대로 Rust를 호출해 기록된 케이스를 재생하고, 불일치를 나열 |
| 3. 벤치 | `everything-rust bench <crate>` | 브리지 비용을 포함해 원본과 Rust에서 기록된 케이스의 실행 시간 측정 |
| 4. 섀도 | `everything-rust shadow <crate>` | 둘 다 실행해 원본의 결과를 반환하고 불일치를 보고하는 대체 모듈. `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. 게이트 | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, 테스트, `unsafe` 감사, 포팅되지 않은 것 없음, 원본의 모든 오류 경로에 대한 기록된 실패 케이스, 원본이 오류를 반환하는 곳에 `unwrap`/`expect`/`panic!` 없음(`// INFALLIBLE: <why>` 표시가 없는 한), 빌드, API, 동작 |
| 에이전트 | `everything-rust setup` | MCP 서버(`everything-rust mcp`, 모든 단계를 도구로 제공)와 워크플로 스킬([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md))을 코딩 에이전트에 등록 |

## 핫 패스: 데이터를 Rust 안에 두기

항목마다 한 번씩 호출되는 포트는 항목마다 브리지 비용을 치르고 JIT에 집니다. 이기는 방법은
루프를 옮기는 것입니다: `profile`이 루프를 찾고, `--batch`는 항목을 한 번 받아
호출마다 모든 항목에 함수를 실행하는 클래스를 생성합니다. 모든 코어에서 실행되며 결과는 typed array로 돌려줍니다.

```sh
everything-rust profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore: a quarter of the run, called per key by `matched`: batch it
everything-rust bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

`port.rs`의 `FuzzyScoreIndex` 구조체는 기본 루프를 항목을 한 번 준비해 두는 루프로
대체하며, 스스로 순위를 매길 수도 있습니다(`rank`). 키를 소문자로 바꿔 각각 문자 마스크와 함께 보관하고,
각 키에서 모든 흔한 문자가 처음 나오는 위치의 표(한 글자 쿼리는 열 하나)와,
마지막 쿼리의 일치 결과(`s` 다음에 `se`를 입력하면 `s`가 일치한 것만 다시 점수를 매김)를 보관합니다.
x18n 사이드바에서 키 50,000개를 한 글자씩 입력하며 검색했을 때, 결과는 동일했습니다:

| | 쿼리당 | TS 대비 |
| --- | ---: | ---: |
| TS: `map(fuzzyScore)`, filter, sort | 10 ms | 1x |
| 인덱스와 같은 범위 좁히기를 적용한 TS | 4.1-5.4 ms | 2x |
| 같은 포트를 키마다 호출 | - | 0.2-0.4x |
| `FuzzyScoreBatch`, 기본 인덱스 (4코어) | 5.1 ms | 1.7x |
| `FuzzyScoreIndex`를 쓴 `FuzzyScoreBatch` (napi, 4코어) | 0.45-0.56 ms | **18-22x** |
| 같은 것을 wasm으로 (스레드 1개) | 0.95 ms | **11.5x** |

같은 범위 좁히기를 하는 TS와 비교해도 Rust가 여전히 7-12x 빠릅니다(wasm은 5x). 반환된
인덱스는 평범한 루프로 키로 바꾸세요: `Array.from(order, i => keys[i])`는 20배 더 비싼 0.4 ms가 걸렸습니다.

모든 언어가 배치를 지원하며, 호출 단위 포트가 지는 곳에서도 원본을 이깁니다. 4코어에서
같은 작업을 하고 결과와 오류는 동일했습니다(벤치마크는 각 demo 폴더에 있음):

| 언어 | 워크로드 | 원본 | 호출 단위 포트 대 원본 | 배치 대 원본 |
| --- | --- | ---: | ---: | ---: |
| TS | x18n 검색, 키 50,000개 | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`, 문자열 20,000개 | 98 ms | - | **17x** |
| Ruby | `slugify`, 문자열 20,000개 | 110 ms | - | **12.7x** |
| Ruby | `shellescape` (표준 라이브러리), 섞인 값 50,000개 | 88 ms | - | **11.8x** |
| Go | `Slugify`, 문자열 50,000개 | 49 ms | 0.93x | **4x** |
| Java | `slugify`, 문자열 50,000개 | 43 ms | 0.68x | **3x** |

컴파일 언어 원본은 이득이 적습니다. 항목당 작업이 이미 빠르므로 배치는 코어 수만큼 이깁니다.
배치 crate는 mimalloc으로 메모리를 할당합니다. 시스템 할당자를 쓰면 Go 배치가 단일 스레드보다
느렸습니다. 데모: [`demo/x18n-search`](https://github.com/marprojector/everything-rust/blob/main/demo/x18n-search/README.md).

## `bridge`가 결정하는 것

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

## `analyze`가 살펴보는 것

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

## 잡아내지 못하는 것

통과했다는 것은 Rust가 기록된 모든 케이스와 일치한다는 뜻일 뿐, 그 이상은 아닙니다. 재작성을 괴롭히는 문제는
대개 기록 바깥에 있으므로, 경계가 어디인지 알아 두세요:

- **아무도 기록하지 않은 입력.** 생성된 케이스, 함정 케이스, 원본의 테스트 스위트
  (`record --from-tests`), 직접 넣은 `--inputs`가 한계입니다. 실제 입력(프로덕션
  샘플, 버그 리포트)을 추가하세요. 프로덕션을 망가뜨리는 사용자 입력은 생성기가 떠올리는 경우가 드뭅니다.
- **그 사이 바뀐 원본.** `record`는 유닛 파일의 지문(`recorded-from.json`)을 저장합니다.
  나중에 원본이 바뀌면 `diff`는 경고하고 `gate`는 다시 기록할 때까지 실패합니다. 원본의 예전
  동작을 기준으로 통과하는 일은 없습니다.
- **정규식.** `regex` crate에는 전후방 탐색(lookaround)과 역참조가 없으며, 컴파일되는 패턴이라도
  ASCII가 아닌 텍스트에서 `\w`, `\b`, `\d`와 대소문자 무시 매칭을 JS, Python, Java와 다르게
  처리할 수 있습니다. `fancy-regex`로 바꾸거나 패턴을 다시 쓰면 기록된 모든 케이스를 통과하면서도
  터키어의 점 없는 `ı`나 zero-width joiner에서 여전히 다를 수 있습니다. `record`는 정규식을
  쓰는 파일의 모든 함수에 정규식 함정(위의 것들, 켈빈 기호, 긴 s, 아랍-인도 숫자, 줄 구분자, 어말 시그마)을
  추가하지만, 실제 패턴도 실제 텍스트로 퍼징하세요.
- **실행마다 바뀌는 것.** `record`는 원본을 두 번 실행하고 달라진 것을 분류합니다:
  바뀐 값(시간, ID)은 와일드카드가 되고, 같은 줄이 다른 순서로 나오면(해시 맵,
  동시성) 순서 무관하게 비교하므로 포트는 여전히 정확히 그 줄들을 만들어야 합니다. 줄 수가
  달라지면 무엇이든 일치하는 `...`이 됩니다. `record`는 종류별로 목록을 보여 줍니다.
  해시 순서나 동시성 차이는 포트에서 실제로 바뀔 수 있는 바로 그 부분이므로 하나하나 모두 확인하세요.
- **부동소수점.** 숫자를 텍스트로 바꾸는 형식은 언어마다 다릅니다(`1e21`, `-0`, 자릿수).
  함정 케이스에 이런 값들이 포함되며 정확히 일치해야 합니다. `--float-tolerance`는
  반올림만 허용할 뿐 형식 차이는 절대 허용하지 않습니다.
- **출력에 나타나지 않는 것.** 실행 시간, 메모리, 부하 상황의 동시성, 그리고 stdout, stderr,
  종료 코드, 쓴 파일, HTTP 응답 이외의 부수 효과(네트워크 호출, 데이터베이스,
  시그널)는 비교하지 않습니다.
- **다른 플랫폼.** 기록은 그것을 실행한 머신에서의 동작을 증명합니다. 결과가 타입 크기, 엔디언,
  컴파일러에 좌우되는 C 코드는 타깃에서 다르게 동작할 수 있습니다. 그래서 `analyze`는
  하드웨어를 다루는 C 코드를 C로 유지합니다.
## 모든 단계

| 단계 | 명령 | 하는 일 |
| --- | --- | --- |
| 0. 프로파일링 | `everything-rust profile <repo> -- node bench.mjs` | 실제 워크로드를 언어별 프로파일러(node, cProfile, `go test` pprof, JFR, Ruby 트레이서)로 실행: 포함 시간 기준 핫 함수, 그 호출자, 배치로 만들 루프. Rust로 얻는 것이 적은 이미 컴파일된 코드(numba, Cython, JAX)를 표시 |
| 1. 계획 | `everything-rust analyze <repo>` | 유닛, 포팅 순서, `file:line`이 붙은 위험 요소, 유닛별 브리지, 의존성과 crate의 적합도, 가장 좋은 첫 포팅 대상. `--whole-app`은 진입점과 형태, 모든 의존성(import했거나 선언한 것)과 그 crate, 조각(서로 import하는 유닛은 함께 옮김), 권장 답이 붙은 번호 매긴 결정 사항을 추가 |
| 2. 브리지 | `everything-rust bridge <repo> <unit>` | 바인딩(`lib.rs`와 원본 언어가 불러오는 래퍼)과 포팅할 스텁(`port.rs`)이 담긴 crate. `--batch fn:param`은 많은 `param` 값을 Rust 안에 유지하는 클래스를 추가. `--as-json T`는 앱이 데이터로만 채우는 타입 매개변수를 JSON 값으로 브리지 |
| 3. 기록 | `everything-rust record <crate>` | 생성된 엣지 케이스(ASCII가 아닌 문자, 이모지, `-0`, `NaN`, 부동소수점 자리에 들어간 정수 등)와 모든 매개변수에 대한 모든 의미상의 함정(문자열 길이와 대소문자, 부호 있는 나눗셈, 오버플로 경계, 부동소수점 출력, 맵과 정렬 순서)으로 원본을 실행하고 `cases.jsonl`을 저장. `--from-tests -- npx jest`(또는 `-- pytest`)는 원본의 자체 테스트 스위트가 하는 모든 호출도 기록. `--float-tolerance 1e-9`는 수치 코드에서 부동소수점이 그 비율 이내로 일치하도록 허용 |
| 3. 빌드 | `everything-rust build <crate>` | Rust 쪽을 빌드: `.node`, wasm 패키지, Python 확장 또는 공유 라이브러리 |
| 3. Diff | `everything-rust diff <crate>` | 원본의 호출자가 호출하는 방식 그대로 Rust를 호출해 기록된 케이스를 재생하고, 불일치를 나열 |
| 3. 벤치 | `everything-rust bench <crate>` | 브리지 비용을 포함해 원본과 Rust에서 기록된 케이스의 실행 시간 측정 |
| 4. 섀도 | `everything-rust shadow <crate>` | 둘 다 실행해 원본의 결과를 반환하고 불일치를 보고하는 대체 모듈. `EVERYTHING_RUST_MODE=old\|shadow\|new` |
| 5. 게이트 | `everything-rust gate <crate>` | fmt, clippy `-D warnings`, 테스트, `unsafe` 감사, 포팅되지 않은 것 없음, 원본의 모든 오류 경로에 대한 기록된 실패 케이스, 원본이 오류를 반환하는 곳에 `unwrap`/`expect`/`panic!` 없음(`// INFALLIBLE: <why>` 표시가 없는 한), 빌드, API, 동작 |
| 에이전트 | `everything-rust setup` | MCP 서버(`everything-rust mcp`, 모든 단계를 도구로 제공)와 워크플로 스킬([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md))을 코딩 에이전트에 등록 |
## 다음

각 단계마다 별도 페이지가 있습니다: [프로파일링과 분석](/docs/concepts/analyze), [브리지](/docs/concepts/bridge), [기록](/docs/concepts/record), [포팅](/docs/concepts/port), [Diff](/docs/concepts/diff), [벤치](/docs/concepts/bench), [섀도](/docs/concepts/shadow), [게이트](/docs/concepts/gate).
