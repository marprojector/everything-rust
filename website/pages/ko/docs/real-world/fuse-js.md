---
title: Fuse.js
description: Rust로 옮긴 Fuse.js 문자열 목록 검색. 기록된 케이스 3,414/3,414 일치, 바이트 단위로 동일한 결과, 문자열 50,000개에서 32배 빠름.
---

# Fuse.js

[Fuse.js](https://github.com/krisk/Fuse)(스타 20k, 주간 npm 다운로드 약 2천만 회,
커밋 `edf2fb6`)는 수많은 검색창 뒤에 있는 퍼지 검색 라이브러리입니다. everything-rust 1.6(미출시)으로
포팅했으며, crate는 [`demo/real-world/fuse-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/fuse-rs)에 있습니다.

`analyze`는 `src/`의 모든 폴더를 하나의 import 순환에 넣었으므로, Fuse는 파일 단위로 포팅할 수 없습니다.
대신 포트는 검색이 시간을 쓰는 경로를 택합니다: 54줄짜리 어댑터
(`adapter/relevance.ts`)가 문자열 목록의 문자열 하나에 대한 Fuse 자체 점수를 export하며, 이는
Fuse의 `BitapSearch`, field norm, `computeScoreSingle`로 만들어집니다. 문자열 50,000개를 검색하면
이 함수가 50,000번 실행됩니다. `bridge --batch relevance:text`는 문자열을 Rust 안에 두고
그 전체를 대상으로 모든 코어에서 검색을 실행합니다.

| 단계 | 결과 |
| --- | --- |
| 어댑터 | Rust 이전에, 쿼리와 옵션 조합 81개에서 어댑터의 순위가 `new Fuse(list).search(q)`와 같음 |
| `record --inputs` | 케이스 3,414개: Fuse 자체 픽스처의 제목, ASCII가 아닌 문자, 이모지, 32 유닛보다 긴 패턴, 지원하는 모든 옵션, 잘못된 타입의 옵션(거부됨, 149개 케이스) |
| 포트 | 대체하는 Fuse 코드 839줄에 대해 Rust 673줄 |
| `diff` | 첫 빌드에서 3,414/3,414 일치 |
| `gate` | 통과 |
| 대체 모듈 | `withRust(Fuse)`는 문자열 목록 검색을 Rust로 처리하고 나머지는 모두 Fuse에 넘깁니다. 검색 360회(옵션 조합 12개, 쿼리 15개, `limit` 유무)에서 Fuse와 바이트 단위로 동일한 결과를 반환합니다 |

쿼리당 검색 시간, Apple M4(10코어), Node 24, 시간 측정 전에 결과가 동일한지 확인:

| 목록 | Fuse.js | Rust | 속도 향상 |
| ---: | ---: | ---: | ---: |
| 문자열 1,000개 | 1.9 ms | 0.1 ms | 18x |
| 문자열 10,000개 | 19.1 ms | 0.7 ms | 27 to 30x |
| 문자열 50,000개 | 91.2 ms | 2.8 ms | **32x** |
| 문자열 50,000개, 코어 1개 | 92.0 ms | 17.5 ms | 5.3x |

포트가 정확히 맞춰야 했던 부분:

- Bitap은 32비트 JS 정수로 동작합니다: `1 << 31`은 음수이고 `(1 << 31) - 1`은 double이 되어
  다음 시프트에서 `2147483647`이 됩니다. 포트는 이 값들을 JS와 똑같이 유지합니다.
- 텍스트는 UTF-16 코드 유닛으로 인덱싱되므로, Fuse에서처럼 이모지 하나가 두 위치를 차지합니다.
- `ignoreDiacritics`는 NFD에 결합 문자 범위 목록과 직접 지정한 몇몇 글자
  (`ł`, `ß`, `ı`)를 더한 것입니다. 포트도 같은 목록을 사용합니다.
- Fuse가 어떤 문자열을 건너뛸지는 `String.prototype.trim`이 결정합니다: Rust의
  `char::is_whitespace`(U+0085를 포함하고 U+FEFF는 제외)가 아니라 JS 공백 정의입니다.

도구에서 발견해 고친 것: 항목은 문자열이지만 결과는 문자열이 아닌 배치 crate에
사용하지 않는 헬퍼가 생성되어 `clippy -D warnings`에 실패했습니다. 그리고 `gate`의 크기 검사가 어댑터가 import하는
Fuse 파일 대신 어댑터만(54줄, 그래서 12.5배 "너무 큼") 셌습니다.

다루지 않는 것: `keys`가 있는 객체 목록, 확장 검색과 토큰 검색, 논리 쿼리,
`includeMatches`, 사용자 정의 `sortFn`이나 `getFn`. 대체 모듈은 이런 경우를 그대로 Fuse에 넘깁니다.
속도 향상을 얻으려면 검색 사이에 목록이 Rust 안에 머물러야 합니다. 목록을 만드는 비용은
Fuse 자체 인덱스와 비슷합니다(문자열 50,000개 기준 3.4 ms 대 2.8 ms).
