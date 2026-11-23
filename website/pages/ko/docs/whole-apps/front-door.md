---
title: 프런트 도어와 와이어
description: app front-door는 원본 앞에 Rust 바이너리를 두고, app wire는 유닛의 포트가 gate를 통과하면 명령을 그 포트에 넘깁니다.
---

# 프런트 도어와 와이어

먼저 진입점을 Rust로 옮긴 다음, 유닛 하나씩 명령을 넘깁니다:

```sh
everything-rust app front-door .                            # rust/app: 모든 명령이 원본을 실행
everything-rust app wire . textkit --command upper          # textkit-rs가 게이트를 통과한 후
# rust/app/src/units/textkit.rs에 글루 코드 작성, 그다음:
everything-rust app wire . textkit                          # 다시 100%, 폴백 비율은 낮아짐
```

## 프런트 도어

`app front-door`는 앱 이름을 딴 Rust 바이너리 `rust/app`의 뼈대를 만듭니다. 기록된 케이스에 나오는 명령과 플래그를 처리하는 clap 파서가 들어 있으며, 모든 명령은 같은 argv, stdin, 환경 변수로 원본을 실행하고 그 출력과 종료 코드를 그대로 전달합니다. 폴백은 하나하나 집계되므로, 갓 만든 프런트 도어를 재생하면 폴백 100%에서 100% 통과가 나옵니다(이를 보여 주기 위해 빌드하고 재생합니다). `rust/app`을 덮어쓰는 일은 없습니다. 그때부터 코드는 사용자의 것입니다.

## 유닛 연결하기

`app wire <repo> <unit> --command <name>`:

1. 유닛 crate에 `gate`를 실행합니다(방금 통과했다면 `--skip-gate`).
2. 그 crate를 라이브러리로 `rust/app`에 추가합니다.
3. 개발자나 에이전트가 채울 `src/units/<unit>.rs`를 한 번 작성합니다: 인자를 파싱하고, `<crate>::port`를 호출하고, 원본이 출력하는 것을 출력합니다.
4. 명령을 그 유닛으로 라우팅한 다음 빌드하고 재생합니다.

글루 코드를 작성한 뒤 다시 실행하세요: 100% 통과에 폴백 비율은 낮아집니다. `EVERYTHING_RUST_UNIT_<NAME>=original`은 리프트 전까지 연결된 유닛을 원본으로 되돌립니다.

명령을 지정하지 않은 argv(인자 없음, 알 수 없는 명령, 사용법 오류)를 위해 앱 자체 이름을 명령으로 연결하세요(`--command mytool`).

## 원본으로 되돌아 호출하는 것은 없습니다

프런트 도어는 원본을 별도 프로세스로 실행하거나(또는 HTTP를 원본으로 전달하고), 브리지된 유닛은 원본이 호출할 뿐 반대 방향으로는 호출하지 않으며, 콜백은 원본 언어에 남습니다.

서버는 명령 대신 라우트를 사용합니다: [서버](/docs/whole-apps/servers)를 참고하세요. 라이브러리는 공개 API를 사용합니다: [라이브러리](/docs/whole-apps/libraries)를 참고하세요.
