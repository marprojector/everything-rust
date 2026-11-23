---
title: json-server
description: 처음부터 끝까지 포팅한 첫 번째 앱 전체. Node 서버가 응답하는 대로 응답하는 Rust 바이너리 하나, 기록된 HTTP 케이스 514/514.
---

# json-server

[json-server](https://github.com/typicode/json-server)(스타 75k, MIT, 커밋 `89a34a4`,
1.0.0-beta.15)는 JSON 파일에서 REST API를 제공합니다: Node, tinyhttp, lowdb. 앱 전체 흐름
(`app record`, `front-door`, `wire`, `lift`)으로 처음부터 끝까지 포팅한 첫 번째 앱 전체이며,
결과는 Node 서버가 응답하는 대로 응답하는 Rust 바이너리 하나입니다. 모든 것이
[`demo/real-world/json-server-rs`](https://github.com/marprojector/everything-rust/tree/main/demo/real-world/json-server-rs)에 있습니다.

| 단계 | 결과 |
| --- | --- |
| `analyze --whole-app` | 서버, 유닛 9개(ready 7개, caution 2개), 대체할 의존성 13개(tinyhttp, lowdb, sirv, eta, milliparsec, dot-prop, sort-on, inflection...) |
| `app record --http` | 케이스 514개: `--inputs`로 준 요청 268개(모든 라우트와 메서드, where 연산자, 정렬, 페이지네이션, embed, CORS preflight, 정적 파일과 range, 잘못된 JSON 본문 29개, `/__proto__`와 기타 프로토타입 이름, 잘못된 이스케이프)와 함정 케이스 245개. 각각 두 번 기록했으며, 무작위 id는 `[..]`로 남김 |
| `front-door` | 폴백 99.8%에서 513/514. 실패한 하나는 URL에 들어간 원시 `ü`로, Node는 400으로 응답하고 프록시는 다시 인코딩함 |
| 유닛 | `where-operators` 207개, `parse-where` 289개, `matches-where` 1,346개, `paginate` 853개 케이스(json-server 자체 테스트의 호출 포함). `service` 461개와 `app` 243개는 Service 호출과 실제 HTTP 요청을 실행하는 작은 어댑터 두 개를 거침. 여섯 개 모두 `gate` 통과 |
| `wire` | 읽기를 먼저: 폴백 22.6%에서 514/514, 쓰기는 Node가 응답하고 Rust가 Node가 쓴 파일을 다시 읽음. 그다음 모든 라우트: 폴백 0%에서 514/514 |
| `lift` | 순수 Rust 바이너리(5.3 MB), 514/514. `bin.ts`의 명령줄(도움말, 버전, 파일 검사, 시작 배너)은 손으로 포팅 |

리프트 후 유닛에는 바인딩이 없으므로, 각 crate의 `tests/recorded.rs`가 기록된
케이스를 포트에서 재생합니다: 3,399개 모두 `cargo test`로 실행됩니다.

포트가 정확히 맞춰야 했던 부분, 모두 기록된 케이스가 잡아냈습니다:

- tinyhttp의 라우터는 한 번만 디스패치하지 않습니다: `next()`가 호출될 때마다 마지막 미들웨어가 다시 쓴 URL로
  모든 미들웨어와 라우트를 다시 매칭하며, `HEAD`에 대해서는 모든 라우트를 실행합니다. 그래서
  `HEAD /posts`는 GET 핸들러를 실행한 다음 POST 핸들러를 실행하고 POST의 400으로 응답합니다. `//posts`는 CORS
  헤더와 본문 파서를 건너뜁니다. 포트는 이 루프를 그대로 실행합니다.
- milliparsec은 모든 POST, PUT, PATCH, DELETE 본문을 타입과 상관없이 JSON으로 파싱하며, 잘못된
  본문은 V8의 메시지와 함께 500이 됩니다: `Expected double-quoted property name in JSON at position 7
  (line 1 column 8)`, `Unexpected token 'x', ..."bbbbbbb": x}" is not valid JSON`. 포트는
  UTF-16 유닛 기준 위치를 포함해 V8 파서의 오류를 재현하며, 잘못된 입력 3,806개로 Node와 대조해
  검증했습니다.
- 정렬은 `localeCompare`(ICU)를 사용하며, ICU4X의 collator로 구현하고 3,000쌍으로 검증했습니다.
  `Number()`, `JSON.stringify`(`1e+21`, 정수 키 먼저), Node의 두 가지 쿼리 문자열 파서
  (`é%E9`를 WHATWG와 다르게 디코딩), 오류 메시지를 포함한 dot-prop, embed용 inflection
  (`person`과 `people`)도 각각 `json-server-js`에서 Node와 대조해 검증했습니다.
- `db.data[name]`은 `Object.prototype`을 읽습니다: `/__proto__`는 `{}`로 응답하고, `/constructor`는 빈
  200으로 응답하며, `PATCH /__proto__`는 데이터베이스의 프로토타입을 바꾸고 이후 조회에서 그것이 보입니다.
- Node의 HTTP 파서는 원시 비 ASCII 대상과 알 수 없는 메서드에 대해 400으로 응답하고 연결을 닫습니다.

json-server에서 발견한 것: 자체 쓰기가 끝날 때마다 파일 감시자가 파일을 다시 읽습니다. 그 다시 읽기가
끝나기 전에 다음 요청이 쓰기를 하면, 다시 읽기가 그 쓰기를 지워 버릴 수 있습니다: 서버가 응답하는 속도 그대로
보낸 첫 기록에서는 201로 응답한 POST가 이후 읽기에서 보이지 않았습니다. `app record --pace 50`은 요청 사이에 기다리며,
케이스는 그렇게 기록되었습니다. 포트는 다른 누군가가 파일을 바꿨을 때만 다시 읽으므로 그 쓰기를 잃지 않습니다:
의도된 차이입니다.

벤치마크: Apple M4(10코어, 16 GB), macOS 27.3, npm용으로 `tsc`가 빌드한 `lib/bin.js`를 실행하는 Node 24.15.0,
`--release`로 빌드한 Rust 바이너리. 둘 다 같은 데이터베이스의 복사본을 루프백에서 제공하며,
`ab -k`가 워밍업 후 같은 요청을 행마다 20,000개 보냅니다(큰 데이터베이스의 느린 embed는 40개). 두 번 실행했고,
각 실행은 1분 평균 부하가 3 미만일 때 시작했습니다. 각 칸은 1회차 / 2회차의 초당 요청 수입니다. [`bench/bench.py`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/json-server-rs/bench/bench.py)가
그 스크립트입니다.

픽스처 데이터베이스(게시물 6개, 댓글 3개):

| 요청 | Node, 연결 1개 | Rust, 연결 1개 | Node, 연결 50개 | Rust, 연결 50개 |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts` | 11.1k / 10.8k | 27.3k / 23.9k | 19.7k / 20.4k | 115.0k / 110.7k |
| `GET /posts/1` | 11.0k / 11.3k | 27.7k / 25.4k | 21.9k / 22.6k | 119.9k / 99.0k |
| 필터, 정렬, 페이지 | 10.5k / 10.0k | 25.2k / 21.5k | 18.3k / 16.4k | 107.5k / 96.3k |
| `?_embed=comments` | 8.7k / 6.6k | 22.5k / 21.8k | 13.8k / 11.0k | 98.3k / 100.6k |
| `GET /` (인덱스 페이지) | 7.3k / 6.3k | 27.6k / 21.7k | 10.2k / 9.5k | 135.1k / 119.3k |
| `GET /test.html` (정적) | 12.0k / 11.0k | 25.8k / 25.9k | 30.3k / 26.1k | 103.9k / 104.4k |
| `POST /comments`, 연결 10개 | | | 2,973 / 2,467 | 5,821 / 5,976 |

생성한 데이터베이스(게시물 10,000개, 댓글 20,000개, 3.6 MB):

| 요청 | Node, 연결 1개 | Rust, 연결 1개 | Node, 연결 50 (10)개 | Rust, 연결 50 (10)개 |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts/5000` | 8.4k / 9.2k | 14.6k / 14.8k | 15.5k / 16.1k | 69.5k / 67.4k |
| `?_page=3&_per_page=10` | 4.6k / 4.2k | 8.8k / 8.4k | 6.5k / 6.8k | 47.4k / 46.2k |
| `title:contains`, 정렬 | 431 / 396 | 600 / 560 | 465 / 453 | 2,516 / 2,329 |
| `?_embed=comments`, 5개짜리 페이지 (연결 10개) | 2.4 / 2.4 | 81 / 103 | 2.4 / 2.2 | 253 / 216 |
| `GET /posts`, 전체 목록 (연결 10개) | 328 / 319 | 399 / 407 | 381 / 387 | 1,514 / 1,657 |
| `POST /comments` (연결 10개) | | | 236 / 241 | 292 / 264 |

| | Node | Rust |
| --- | ---: | ---: |
| 시작부터 첫 응답까지, 픽스처 데이터베이스 | 85 / 79 ms | 9.9 / 7.9 ms |
| 시작부터 첫 응답까지, 게시물 10,000개 | 95 / 91 ms | 48 / 47 ms |
| 유휴 메모리(RSS), 픽스처 데이터베이스 | 74 MB | 3.5 MB |
| 유휴 메모리(RSS), 게시물 10,000개 | 94 MB | 41 MB |
| 실행 중 최대 메모리(RSS), 픽스처 데이터베이스 | 424 / 390 MB | 18 MB |
| 실행 중 최대 메모리(RSS), 게시물 10,000개 | 462 / 485 MB | 289 / 291 MB |

차이가 작은 경우: 연결 하나로 게시물 10,000개 전체 목록을 읽을 때(1.2x: 둘 다 1.5 MB의 JSON을
출력하는 데 시간을 씀), 큰 데이터베이스에 쓸 때(1.1 to 1.2x: 둘 다 쓰기마다 3.6 MB 파일 전체를 다시 쓰며,
Rust는 lowdb처럼 대기 중인 쓰기를 합침), 연결 하나로 제목을 정렬해 검색할 때(1.4x). 큰 이득은
Node 프로세스에는 없는 동시성에서, 그리고 json-server가 게시물마다 모든 댓글을 필터링하는 embed에서 옵니다
(포트는 요청마다 댓글을 한 번 인덱싱하며 결과는 같음).

처음 리프트한 바이너리는 큰 데이터베이스에서 Node보다 느렸습니다(전체 목록은 초당 요청 33 대 298,
쓰기는 39 대 217). 프로파일링으로 다음을 순서대로 찾았습니다: `TCP_NODELAY` 없음(큰 응답마다 지연된 ACK를
약 40 ms 기다림), 쓰기마다 `$schema`를 추가하려고 데이터베이스 전체를 복제, 출력 전에 나열된 항목마다 복제,
모든 요청에 대한 잠금 하나, 항목 10,000개마다 `item.id`를 SipHash로 해시. 모두 고쳤으며, 변경할 때마다 유닛 케이스
3,399개와 HTTP 케이스 514개가 통과합니다.

도구에서 발견해 고친 것:

- 상태를 쓰는 서버는 같은 시작 상태에서 두 번 기록할 수 없었습니다:
  이제 `--fresh <path>`가 케이스와 함께 보관한 스냅숏에서 데이터베이스(와 `public/`)를 시작할 때마다
  새 작업 디렉터리로 복사합니다. `--arg`는 모든 빌드에 원본의 인자를 줍니다.
  `--pace`는 위에 설명했습니다.
- `bridge`가 `paginate<T>(items: T[])`를 TS에 남겼습니다(함수를 포함한 무엇에든 제네릭).
  `--as-json T`가 이를 JSON 값으로 브리지합니다.
- 매개변수가 있는 프런트 도어 라우트가 유닛이 요청을 보기 전에 `/posts/%E0`에 대해 axum 자체의 400으로
  응답했습니다. 이제 매개변수는 URI에서 읽습니다.
- `lift`가 연결된 유닛만 스테이징하고 그 유닛이 경로로 의존하는 crate는 스테이징하지 않았으며, `cargo fmt --check`가
  거부하는 `lib.rs`를 작성했습니다. 둘 다 고쳤습니다.
- 생성된 서버가 `TCP_NODELAY`를 설정하지 않았습니다: 두 부분으로 나눠 쓴 응답이 클라이언트의
  지연된 ACK를 약 40 ms 기다렸습니다. 이제 설정합니다.

다루지 않는 것: JSON5 데이터베이스(바이너리가 그렇다고 알리고 종료), 100 KiB 본문 제한
(Node는 본문이 64 KiB 단위 읽기로 어떻게 도착하는지에 따라 달라지며, 포트는 전체 읽기를 가정), 짝 없는
서로게이트가 든 문자열(U+FFFD가 됨), 그리고 데이터베이스 값 `1e400`(Node는 다음 다시 읽기까지 메모리에 `Infinity`를
유지하고, 포트는 파일에 들어가는 값인 `null`을 저장).
