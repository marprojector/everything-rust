---
title: 라이브러리
description: 라이브러리에는 재생할 프로세스가 없습니다. 공개 API가 프런트 도어이고, 각 유닛의 기록된 케이스가 증명입니다.
---

# 라이브러리

라이브러리에는 재생할 프로세스가 없습니다. 공개 API가 프런트 도어이고, 각 유닛의 기록된
케이스가 증명입니다.

```sh
everything-rust app front-door .        # rust/app: 공개 API를 가진 Rust crate
# 각 유닛은 루프를 거침: bridge, record, port.rs, build, diff, gate. 그다음:
everything-rust app wire . textkit-rs   # rust/app에서 re-export, 원본은 섀도를 import
everything-rust app replay .            # 연결된 모든 유닛의 케이스, 폴백 = 아직 원본에 있는 export
everything-rust app lift .
```

- `app front-door`는 저장소에 `main`도 서버도 없으면 공개 API(모든 유닛의 export)를 찾고
  (그렇지 않으면 `--library`), `rust/app`을 라이브러리 crate로 만듭니다. 연결된 각
  유닛은 그 모듈이 됩니다: `pub use textkit_rs::port as textkit;`.
- `app wire`는 `bridge`가 만든 crate를 받아 `gate`를 실행하고, 유닛의 섀도 대체 모듈을 작성합니다.
  원본 패키지는 유닛 대신 이를 import하므로, 호출하는 쪽은 원래 API를 유지한 채 Rust에
  도달합니다. `EVERYTHING_RUST_UNIT_<NAME>=original`은 유닛을 원본으로 되돌립니다(C: 섀도의
  플래그 없이 빌드).
- `app replay`는 연결된 모든 유닛의 기록된 케이스를 바인딩을 통해 실행합니다. 폴백 비율은
  Rust가 아직 응답하지 않는 공개 export의 비율입니다: 연결되지 않은 유닛, 그리고 `bridge`가
  원본 언어에 남겨 둔 export.
- `app lift`는 폴백 0과 모든 유닛의 `gate` 통과를 요구합니다. 유닛에서 바인딩, 래퍼,
  섀도를 걷어 내고 `rust/app`을 하나의 순수 Rust 라이브러리로 남깁니다. 어떤
  `port.rs`도 건드리지 않으므로 바인딩을 통해 일치했던 케이스가 여전히 그 동작을 설명합니다. 체크리스트는
  아직 섀도를 import하는 원본 파일이 있으면 경고합니다.

다섯 가지 모두 MCP 도구로도 제공됩니다: `app_record`, `app_replay`, `app_front_door`, `app_wire`,
`app_lift`.
