---
title: 실제 포팅 사례
description: everything-rust을 실제 코드에 적용했을 때 일어난 일. 빠른 결과 옆에 느린 결과도 함께 싣고, 그 결과로 도구에서 바뀐 점도 정리했습니다.
---

# 실제 포팅 사례

everything-rust을 실제 코드에 적용했을 때 일어난 일과, 그 결과로 도구에서 바뀐 점입니다. 아래의 모든 수치는 결과가 동일한지 먼저 확인하는 벤치마크에서 나왔으며, 느린 경우도 빠른 경우 옆에 함께 적었습니다.

| 포팅 대상 | 무엇인가 | 결과 |
| --- | --- | --- |
| [json-server](/docs/real-world/json-server) | `app record`, `front-door`, `wire`, `lift`로 처음부터 끝까지 포팅한 Node 서버 전체 | 기록된 HTTP 케이스 514/514를 통과하는 순수 Rust 바이너리, 유닛 6개가 `gate` 통과 |
| [Fuse.js](/docs/real-world/fuse-js) | 퍼지 검색, 검색이 시간을 쓰는 경로 | 3,414/3,414 케이스 일치, 문자열 50,000개에서 32배 빠름(코어 1개로는 5.3x) |
| [vercel/ms](/docs/real-world/vercel-ms) | 아주 작은 기간(duration) 파서, 처음부터 끝까지 | 975/975 케이스 일치, 호출당 0.56x, `parse` 배치 18x, `format` 배치 0.8x |
| [x18n](/docs/real-world/x18n) | 사례 연구 저장소 | `packages/core`에서 모든 게이트 통과, 브리지를 거치면 TS보다 느림, 검색은 배치로 18-22x |
| [여섯 가지 언어](/docs/real-world/six-languages) | 작은 파사드 뒤의 Go `path`, Ruby `shellwords`, Python `colorsys`, JDK와 glibc 동작 | 모든 포트가 게이트 통과, 기록된 케이스 총 13,010개 |
| [픽스처와 정적 실행](/docs/real-world/fixtures) | 도구를 시험하려고 작성한 픽스처, 그리고 분석만 하고 실행하지 않은 라이브러리 | |

실제 코드를 포팅해 보셨나요? 사례 글을 환영합니다: [기여하기](/docs/project/contributing#share-a-real-port)를 참고하세요.
