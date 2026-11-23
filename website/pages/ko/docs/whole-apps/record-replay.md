---
title: 기록과 재생
description: app record는 문서화된 예제, 사용자 입력, 함정 케이스로 앱 전체를 실행하고, app replay는 어떤 빌드든 그 케이스에 대해 실행합니다.
---

# 기록과 재생

`record`와 `diff`는 유닛 하나를 증명하고, `app record`와 `app replay`는 앱을 증명합니다. 여섯 가지 언어 모두의 CLI와 서버를 지원합니다. 앱은 블랙박스로 다룹니다.

```sh
everything-rust app record . -- python3 cli.py          # 원본을 실행하는 명령
everything-rust app replay .                            # 원본은 100% 통과해야 함
everything-rust app replay . --build "target/release/mytool"
```

## 케이스의 출처

`app record`는 다음으로 원본을 실행합니다:

- 앱 자체의 문서화된 예제: Markdown과 셸 파일의 `$ mytool ...` 줄, 그리고 그 줄이 가리키는 파일
- 한 줄에 `{"name", "args", "stdin", "env", "files"}` 하나씩 담긴 `--inputs` JSONL 파일
- 위 케이스에서 파생한 의미상의 함정: ASCII가 아닌 문자열과 이모지, 음수 나눗셈과 나머지, 오버플로, 아주 크거나 작은 부동소수점, 맵 순서, 없는 파일

각 케이스는 두 번 실행되며, 두 실행 사이에 달라진 부분은 `[..]` 와일드카드가 됩니다. `--normalize timestamps|uuids|key-order` 또는 `--normalize name=regex`로 정규화 규칙을 추가할 수 있으며, 케이스와 함께 저장됩니다. `app record`는 달라진 부분을 종류별로 보고합니다: 값(이제 와일드카드), 순서(줄을 순서 무관하게 비교), 줄(무엇이든 일치하는 `...`). 하나하나 모두 확인하세요.

## 케이스는 평범한 파일입니다

케이스는 `rust/app-cases/`에 있는 [trycmd](https://docs.rs/trycmd) 파일입니다:

| 파일 | 내용 |
| --- | --- |
| `<case>.toml` | argv, 환경 변수, 종료 코드 |
| `<case>.stdin` | 표준 입력 |
| `<case>.stdout`, `<case>.stderr` | 원본이 출력한 내용 |
| `<case>.in/` | 입력 파일 |
| `<case>.out/` | 쓰인 파일 |

직접 읽고 편집할 수 있으며, 포트에 바이너리가 생기면 Rust 테스트에서 `trycmd`로 실행할 수 있습니다.

## 재생

- `app replay`는 어떤 빌드든 케이스에 대해 실행하고, `passed/total`을 출력하며, 실패한 케이스마다 diff를 담은 `rust/app-cases/replay-report.md`를 작성하고, 불일치가 하나라도 있으면 0이 아닌 코드로 종료합니다.
- [프런트 도어](/docs/whole-apps/front-door)처럼 `EVERYTHING_RUST_FALLBACK_LOG`가 가리키는 파일에 내용을 덧붙이는(원본으로 폴백할 때마다 한 줄) 빌드는 폴백 비율도 보고받습니다.
- `app replay --case <name>`은 보고서에 나온 이름으로 케이스 하나를 다시 실행합니다: 전체를 다시 실행하기 전에 불일치를 고치는 빠르고 반복 가능한 루프입니다.

## CI에서 재생하기

보고서에는 실행 시간이나 임시 경로가 없습니다. 같은 빌드를 두 번 재생하면 같은 파일이 작성됩니다. 커밋한 다음 CI에서 실행하세요:

```sh
everything-rust app replay .
git diff --exit-code rust/app-cases/replay-report.md
```

그러면 절반만 포팅된 앱은 통과 결과가 바뀔 때마다(좋아지든 나빠지든) 보고서가 갱신될 때까지 CI에서 실패합니다.

## 숫자 출력

`app record --float-tolerance 1e-9`는 숫자가 더 큰 값의 그 비율 이내에서 일치하도록 허용합니다. [부동소수점 허용 오차](/docs/concepts/float-tolerance)를 참고하세요.
