---
title: 섀도
description: shadow는 원본과 Rust를 나란히 실행하고 불일치를 보고하는 대체 모듈을 생성합니다.
---

# 섀도

```sh
everything-rust shadow <crate>
```

`shadow`는 원본의 API를 그대로 가진 대체 모듈을 생성합니다: TS 또는 Python 모듈, Go 패키지, Ruby 파일, Java 클래스, C 파일. 앱이 원본을 불러오던 곳에서 이 모듈을 불러오세요. 둘 다 실행해 원본의 결과를 반환하고, 프로덕션에서 신뢰를 쌓는 동안 Rust의 불일치를 보고합니다.

| `EVERYTHING_RUST_MODE` | 동작 |
| --- | --- |
| `shadow` (기본값) | 둘 다 실행하고, 원본의 결과를 반환하며, 불일치를 보고 |
| `new` | Rust로 전환 |
| `old` | Rust를 끔 |

공개 API는 바뀌지 않습니다. 호출하는 쪽은 같은 함수를 import하거나, 그 자리에 섀도 모듈을 import합니다.
