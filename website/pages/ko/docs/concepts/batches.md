---
title: 배치와 핫 패스
description: 항목마다 한 번씩 호출되는 포트는 JIT에 집니다. 배치는 루프 전체를 Rust로 옮겨 모든 코어에서 실행하며, 호출 단위 포트가 지는 곳에서도 이깁니다.
---

# 배치와 핫 패스

```sh
everything-rust bridge <repo> <unit> --batch fn:param    # 또는: everything-rust refresh <crate> --batch fn:param
```

`--batch`는 `param` 값들을 한 번 받아 Rust 안에 유지하는 클래스를 생성합니다: `new FnBatch(items)`(TS, Python, Java), `Mod::FnBatch.new(items)`(Ruby), `NewFnBatch(items)`(Go). 쿼리마다 `run(<other args>)`은 모든 결과를 하나의 배열로 반환하고, `rank(<other args>, min)`은 항목 인덱스를 좋은 순서로 반환합니다. 모든 코어에서 실행되며, 오류는 원본과 같습니다. C를 제외한 모든 언어가 배치를 지원합니다.

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
## 빠른 인덱스 작성하기

- 호출하는 쪽이 배치를 쿼리마다가 아니라 한 번만 만들게 하세요. 반환된 인덱스는 평범한 루프로 항목에 다시 매핑하세요.
- 항목을 한 번 준비하려면(소문자 변환, 마스크, 위치 표) `port.rs`에 `new(items)`와 `run(<other args>)`을 가진 `pub struct FnIndex`를 정의하세요. 항목별로 함수와 같은 결과를 내야 합니다.
- 순위 매기기에서 작업을 건너뛸 수 있다면 인덱스에 자체 `rank(<other args>, min)`을 주세요. 함수 결과로 순위를 매긴 것과 일치해야 합니다: 점수는 높은 순, 동점은 항목 순서.
- 배치는 포트의 항목당 속도에 코어 수를 곱합니다. 컴파일 언어 원본(Go, Java)과 겨루려면 포트 자체가 빨라야 합니다: ASCII는 유니코드 표 없이 처리하고, 각 결과는 미리 크기를 정한 `String` 하나에 만들고, 중간 복사를 피하세요.
- Go와 Java에서는 다 쓰면 배치를 닫으세요(finalizer는 폴백일 뿐입니다).
