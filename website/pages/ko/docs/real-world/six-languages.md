---
title: 여섯 가지 언어
description: Go, Ruby, Python, Java, C의 실제 코드에 everything-rust을 처음부터 끝까지 적용한 결과. 호출당 속도와 배치 속도, 그리고 실행 중에 발견한 버그.
---

# 여섯 가지 언어의 실제 코드

everything-rust이 지원하는 각 언어의 실제 코드에 처음부터 끝까지(analyze, bridge, record, port, diff, gate, bench)
실행해 보았습니다. 코드는 이 머신에 이미 있는 것이어야 했습니다: 인터넷에서 내려받은 코드는
실행하지 않았습니다. 그래서 대상은 설치된 툴체인에 포함된 표준 라이브러리 모듈이며, 적절한 라이브러리 소스가
디스크에 없었던 Java와 C는 JDK와 glibc 자체를 얇은 파사드 뒤에 두고 정답(oracle)으로 삼았습니다.

포트, 기록된 케이스, 추가 입력은 [`demo/real-world`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/README.md)에 있습니다.

| 언어 | 코드 | 함수 | 기록된 케이스 | 결과 |
| --- | --- | ---: | ---: | --- |
| Go | Go 1.24 표준 라이브러리의 `path` | 7 (+1 유지) | 2,642 (542개는 Go 자체 테스트 테이블에서) | 2,642/2,642, 게이트 통과 |
| Ruby | Ruby 3.3의 `shellwords.rb` | 3 | 1,617 (446개는 예외 발생) | 1,617/1,617, 게이트 통과 |
| Python | CPython 3.13의 `colorsys.py` | 6 | 2,400 | 2,400/2,400, 게이트 통과 |
| Java | 파사드 뒤의 JDK 21 (`Integer`, `Long`, `Math`, `Double.toString`, `URLEncoder`, `Base64`, `toUpperCase`) | 11 | 4,492 (226개는 예외 발생) | 4,492/4,492, 게이트 통과 |
| C | 파사드 뒤의 glibc (`strtol`, `atoi`, `strtod`, `strspn`, `strcmp`, ctype) | 9 | 1,860 (6개는 크래시) | 정의되지 않은 동작 케이스 하나를 제외하고 1,859/1,859, 게이트 통과 |
| C | aklomp/base64 (mypyc 런타임에 포함된 사본) | 7개 중 0개 | - | 계획만: 모든 함수가 출력 버퍼나 상태 구조체를 받음 |

## 속도

Python, Go, Ruby, Java의 첫 브리지는 모든 인자와 결과를 JSON 텍스트로 보냈고, 그 비용이
이 함수들이 하는 일의 10-50배였습니다. 그래서 값을 직접 변환하도록 다시 만들었습니다. 이제 `bench`는
호출만 측정하고(인자는 프로세스 안에서 한 번만 디코딩) 배포 여부를 알려 줍니다.

| 언어 | 이전 호출당 | 현재 호출당 | 원본 대비 Rust (`bench`, 기하 평균) | 판정 |
| --- | ---: | ---: | --- | --- |
| Ruby | 6.9-9.5 µs (Fiddle + JSON) | 0.36-1.15 µs (magnus 확장) | **4.4배 빠름** (함수별 3.6-7x) | 배포 |
| Python | 11.7 µs (JSON) | 약 0.3 µs (PyO3 직접 변환) | **1.35배 빠름** (대부분의 호출에서 1.5-1.9x) | 배포 |
| C | - | 경계 없음: 직접 링크 | 0.87x: ctype은 1.1-1.4x 빠름, glibc가 최적화한 `strtod`, `strspn`, `strcmp`는 0.5-0.7x | 최적화하거나 느린 것은 유지 |
| Java | 약 1.7 µs (JNI + JSON) | 30-800 ns (타입이 있는 JNI와 바이너리 와이어) | 0.18x | 더 큰 유닛 포팅 |
| Go | 2.6 µs (cgo + JSON) | 0.11-0.42 µs (타입이 있는 cgo와 바이너리 와이어) | 0.04x | 더 큰 유닛 포팅 |

Go와 Java에서 리스트, 맵, 구조체, 여러 개의 결과는 타입별로 생성된 바이너리 와이어로 건너갑니다:
Go의 `WordFreq`(맵)는 5.1 µs에서 1.0 µs로, `Midpoint`(구조체)는 7.2 µs에서 1.0 µs로 줄었고,
Java의 `frequencies`(`Map`)는 8.4 µs에서 1.0 µs로, `words`(`List<String>`)는 6.0 µs에서
0.57 µs로 줄어 Java 자체보다 1.39배 빨라졌습니다.

TypeScript(x18n core, 유닛 9개, 같은 케이스에서 함수 27개): 바이너리 와이어가 브리지 비용을 평균
3분의 1 줄였고(`rangeContains` 3.6 -> 1.1 µs, `applyTextEdits` 10.4 -> 5.7
µs, `toWireRange` 5.0 -> 2.6 µs), `buildTree`는 TS와 비슷한 수준에서 2.0배 빠른 수준이 되었습니다. 나머지
함수는 V8이 15-600 ns에 실행하는 리프 함수로, Go와 Java처럼 Rust에서 여전히 더 느립니다.

결과를 가르는 요인:

- **인터프리터 언어 원본(Python, Ruby)**: Rust 호출 비용은 약 100-300 ns이고
  Rust 코드는 인터프리터보다 훨씬 빠르므로, 작은 함수도 이득을 봅니다.
- **컴파일 언어 원본(Go, JIT 이후의 Java)**: 원본 리프 함수는 4-90 ns가 걸립니다. cgo 호출
  하나만으로도 약 80 ns가 들고, 문자열을 넘기는 JNI 호출은 그보다 더 듭니다. 어떤 브리지도 5 ns짜리
  함수를 더 빠르게 만들 수는 없습니다. 이런 언어는 한 번의 호출이 많은 일을 할 때만 이득을 봅니다: 함수를
  호출자와 함께 포팅하세요. `bench`가 그렇게 알려 줍니다.
- **C**: 경계를 넘는 비용이 없습니다. Rust 코드를 C 코드와 직접 비교하며, 결과는
  포트 자체의 특성입니다.

### 배치: x18n의 검색

위의 리프 포트는 호출당으로 측정했습니다. x18n의 사이드바 검색(키 50,000개, 한 글자씩
입력)에서 `everything-rust profile`은 `fuzzyScore`가 실행 시간의 약 4분의 1을 차지하며 `matched`가 키마다 한 번씩
호출한다는 것을 보여 주었고, `--batch fuzzyScore:text`를 제안했습니다. 생성된 `FuzzyScoreBatch`는
키를 Rust 안에 유지합니다. `port.rs`의 `FuzzyScoreIndex`는 키를 한 번 UTF-16 소문자로 바꾸고, 각 키에 들어 있는
코드 유닛의 64비트 마스크를 보관하므로, 일치할 수 없는 키 대부분은 읽지도 않고 건너뜁니다. 결과는
모든 쿼리에서 TS 파이프라인과 동일합니다:

| | 쿼리당 ms | TS 대비 |
| --- | ---: | ---: |
| TS 파이프라인 | 10 | 1x |
| 같은 범위 좁히기를 적용한 TS | 4.1-5.4 | 2x |
| 배치, 기본 인덱스 (항목마다 포트 호출, 4코어) | 5.1 | 1.7x |
| `FuzzyScoreIndex`(마스크)를 쓴 배치, napi, 4코어 | 1.2 | 7x |
| `FuzzyScoreIndex`가 직접 순위를 매기는 배치, napi, 4코어 | 0.45-0.56 | 18-22x |
| 같은 것, wasm, 스레드 1개 | 0.95 | 11.5x |

7x에서 20x로 넘어간 과정: 1.2 ms 중 0.4 ms는 벤치마크 자체의 `Array.from(order, i =>
keys[i])`였습니다(평범한 루프는 0.02 ms). 나머지는 인덱스가 직접 순위를 매기게 해서 얻었습니다:
새 쿼리가 이전 쿼리를 확장하면 이전 쿼리의 일치 결과만 다시 점수를 매기고(`s`와 일치하지 않는 키는 `se`와도
일치할 수 없음), 한 글자 쿼리는 첫 위치 표에서 읽으며, ASCII 키는 바이트로 점수를 매기고,
정수 점수는 계수 정렬로 정렬합니다. Callgrind는 30바이트 키에서 라이브러리 호출(`memchr`, `memcmp`)이
평범한 루프보다 더 비싸다는 것을 보여 주었습니다. 이득의 일부는 알고리즘에서 오므로, 벤치마크는 같은 범위 좁히기를 적용한
TS도 실행합니다: Rust는 그보다도 7-12x 빠릅니다(wasm은 5x).

다른 언어에서 같은 패턴을 원본과 비교한 결과(4코어, 결과와 오류 동일):

| 언어 | 워크로드 | 원본 | 호출 단위 포트 대 원본 | 배치 대 원본 |
| --- | --- | ---: | ---: | ---: |
| Python | 픽스처 `slugify`, 문자열 20,000개 | 98 ms | - | 17x |
| Ruby | 픽스처 `slugify`, 문자열 20,000개 | 110 ms | - | 12.7x |
| Ruby | `Shellwords.shellescape`, 섞인 값 50,000개 | 88 ms | - | 11.8x |
| Go | 픽스처 `Slugify`, 문자열 50,000개 | 49 ms | 0.93x | 3.3-4.1x |
| Java | 픽스처 `slugify`, 문자열 50,000개 | 43 ms | 0.68x | 3.0x |

Go 배치는 처음에 단일 스레드보다 느렸습니다: 포트가 항목마다 메모리를 할당했고 glibc의 할당자가
Go 프로세스 안에서 스레드들을 직렬화했기 때문입니다. 이제 배치 crate는 mimalloc을 사용합니다. shellescape 배치는
호출 단위 기록이 놓친 포트 버그도 찾았습니다: Ruby 심볼(`shellescape(:sym)`)이 JSON 마커 형태로
돌아왔습니다. 포트에서 고쳤습니다.

그다음 Go와 Java는 2.5x와 1.7x에서 4x와 3x로 올라갔습니다. 배치 안에서 시간의 절반은 실제 작업이
아니었습니다: 결과를 스레드 하나에서 이어 붙이고 해제했으며, Go는 결과를 한 번 더 복사했고, Java는
수정된 UTF-8을 거쳐 문자열마다 JNI 호출을 한 번씩 했습니다. 이제 문자열 결과는 한꺼번에 돌아가며,
모든 코어에서 Go 자체 버퍼나 Java `char[]` 하나에 기록됩니다. 나머지 절반은 포트였습니다: 스레드 하나에서
Rust `slugify`는 Go보다 아주 조금 빠를 뿐이었고 Java의 JIT보다는 느렸습니다. ASCII를 표 조회 없이,
중간 문자열 없이 소문자로 바꾸자 해결되었습니다. Java 포트는 `Σ`가 든 공백 구분 조각만 통째로 소문자로 바꿔
어말 시그마 규칙을 유지합니다.
호출당으로는 Go 포트가 0.73x에서 0.93x로, Java 포트가 0.46x에서 0.68x로 올라갔습니다.

## 실행에서 발견한 것

### everything-rust의 버그, 수정 완료

- **부동소수점 값이 1 ULP만큼 바뀜**(Java 실행). `serde_json`의 기본 부동소수점 파서는 항상
  정확히 반올림하지는 않습니다. everything-rust이 기록된 입력을 읽을 때 값을 미세하게 바꿨고, 생성된 crate가
  다시 한 번 바꿨습니다. 올바른 포트인데도 `Double.toString` 케이스 43개가 실패했습니다. everything-rust과
  everything-rust이 생성하는 모든 crate(napi-rs, wasm, JSON 디스패처)에서 `serde_json`의 `float_roundtrip` 기능을 써서 고쳤습니다.
- **C 크래시가 기록을 중단시킴**(C 실행). C 함수의 계약을 벗어나는 입력(`isalpha(INT_MAX)`)은
  glibc에서 세그폴트를 일으킵니다. 이제 C 드라이버는 각 케이스를 fork한 자식 프로세스에서 실행하고
  `crash: SIGSEGV`를 기록합니다. C의 동작이 정의되지 않았으므로 `diff`는 그 경우 포트의 어떤 동작도 받아들입니다.
- **`module_function :name`을 인식하지 못함**(Ruby 실행). `shellwords`는 메서드 뒤에 심볼 인자로
  모듈 함수를 선언합니다. export 탐지기는 인자 없는 `module_function`
  형태만 알았습니다. 이제 두 형태 모두 찾습니다.
- **타입 없는 코드가 쓰레기 입력을 받음**(Python 실행). `colorsys`에는 타입 힌트가 없어서, 생성된 케이스
  1,800개 중 1,709개가 `TypeError`였습니다. 이제 Python과 Ruby에서 주석이 없지만 숫자로 쓰이는 매개변수(`r - g`,
  `max(r, g, b)`, `x < 1`)는 `number` 타입이 됩니다: 남은 오류 94개는 모두 실제 오류입니다
  (`ZeroDivisionError`).
- **float 자리에 넘긴 int가 기록되지 않음**(Python 실행). Python 호출자는 `1.0`만큼 `1`도 자주 넘기며,
  `colorsys`는 int를 그대로 전달합니다(`hsv_to_rgb(0, 0, 7)`은 `(7, 7, 7)`). 이제 기록은
  Python 케이스 네 개 중 하나에서 정수 값을 int로 넘깁니다. 그 결과 `f64`로 타입을 지정한 포트에서 불일치 134개가
  드러났습니다.
- **TypeScript 시절의 메시지**: "must stay in JS", "kept in TS", "characterization tests
  against the TS code"가 이제 원본 언어 이름을 씁니다.
- **다시 연 코어 클래스**(`shellwords.rb`의 `class String`)는 이제 몽키패칭 위험으로 표시됩니다.

### `diff`가 찾아낸, 포트가 맞춰야 할 동작

각 줄은 첫 포트에서 기록된 케이스가 잡아낸 불일치였습니다. *fixture* 표시가 붙은 줄은
언어마다 작성한 `tests/fixtures/`의 작은 테스트 프로젝트에서, 나머지는 위의
실제 코드 실행에서 나왔습니다.

- Go `path.Clean`: Go의 `lazybuf`는 쓰기 인덱스가 뒤로 이동한 뒤 그 위치의 바이트를 읽으며, 마지막으로
  남긴 바이트를 읽지 않습니다(Go 자체 테스트 테이블의 9개 케이스).
- *fixture* Go `strings.ToLower`와 Ruby `downcase`는 각 룬을 개별적으로 소문자로 바꿉니다: `ΣΊΣΥΦΟΣ`는 `σίσυφοσ`가 되지만,
  Rust의 `to_lowercase`는 어말 시그마 규칙을 적용합니다. Go는 `İ`도 `i`로 바꾸지만 Rust는
  `i̇`를 냅니다.
- *fixture* Go는 문자열을 바이트 단위로 자릅니다. 룬 중간을 자르면 JSON에서 바이트마다 U+FFFD 하나가 됩니다.
- *fixture* Ruby `slugify`는 음수 `max`에 대해 `NoMethodError`를 발생시켰습니다(`slug[0, -1]`
  은 `nil`): 포트가 그대로 유지하거나, 먼저 호출하는 쪽을 고쳐야 하는 원본의 버그입니다.
- *fixture* Ruby `Array#sum`은 Kahan-Babuska 합산을 사용하고 정수 0에서 시작하므로 `[-0.0].sum`은 `0.0`입니다.
- Ruby `Float#to_s`는 1e15부터 지수 형식을 씁니다(`1e15.to_s`는 `1.0e+15`, `1e14.to_s`는
  `100000000000000.0`). 배열 안의 `nil.inspect`는 `nil`입니다.
- *fixture* Java `\s`는 ASCII만 해당합니다. Rust의 `split_whitespace`는 U+00A0에서도 나눕니다.
- Python의 int-or-float 결과: `colorsys`는 `+`, `-`, `*`를 거쳐도 int를 유지하므로, 충실한 포트는
  `f64` 대신 작은 int-or-float 타입으로 계산합니다(int가 기록되자 134개 케이스).
- glibc `strtod("-")`는 아무것도 변환하지 않고 `-0.0`이 아니라 `+0.0`을 반환합니다.

첫 포트에서 처리했고, 기록된 케이스로 확인한 것:

- Java `Integer.parseInt`는 `Character.digit`을 통해 모든 유니코드 십진 숫자(`١٢`, `２`)를 받아들입니다.
- Java `Double.toString`(JDK 19+)은 가장 짧은 십진수를 출력하지만 두 자리보다 적게 출력하지는 않습니다:
  `4.9E-324`이며, Rust의 가장 짧은 형식은 `5e-324`입니다.
- Python의 `max(nan, 1, 2)`는 `nan`입니다(이후 값이 더 크다고 비교되지 않는 한 첫 번째 값이 이김).
  `-0.0 % 1.0`은 `0.0`입니다.
- glibc `toupper(-128)`은 `128`입니다(테이블이 부호 있는 `char` 값을 받아들임). 기수 0인 `strtol`은
  `077`을 8진수로, 16진 숫자가 없는 `0x`를 `0`으로 읽습니다.

### 이번 실행에서 드러난 한계

- **버퍼를 주고받는 C API.** aklomp/base64의 공개 함수는 `char *out, size_t *outlen`
  또는 `struct base64_state *`를 받습니다. C 브리지는 스칼라와 `const char *`만 전달하므로
  이유와 함께 모두 C에 남깁니다. 출력 버퍼 패턴 지원이 C의 다음 단계입니다.
- **C 매크로.** base64의 파일 51개 중 43개가 매크로로 함수를 정의합니다
  (`BASE64_ENC_FUNCTION(arch)`). 파서는 거기서 오류를 보고하고 계획은 해당 유닛을
  부분(partial)으로 표시합니다. SIMD intrinsic은 아직 위험 요소로 표시하지 않습니다.
- **크래시하지 않는 정의되지 않은 동작.** `ispunct(1000)`은 glibc 테이블 범위 밖을 읽고
  1을 반환합니다. 어떤 포트도 이를 따라 하면 안 됩니다. 해당 케이스 하나는 `cases.jsonl`에서 직접 제거했습니다.
  케이스를 정의되지 않은 동작으로 표시하는 내장 방법은 아직 없습니다.
- **동적 타이핑은 코드 비용을 늘립니다.** `shellescape`가 무엇이든 받아들이기 때문에 `shellwords` 포트는 숫자, nil, 배열,
  해시에 대한 Ruby의 `to_s`와 `inspect`를 다시 구현합니다. `colorsys`
  포트에는 Python의 숫자 타워가 필요합니다. 타입이 있는 포트는 더 짧지만 일부 호출자에게는
  동작이 바뀝니다. `diff`가 정확히 어떤 경우인지 보여 줍니다.
- **파사드.** Java와 C 실행은 실제 JDK와 glibc 동작을 검사하지만, 라이브러리 자체의 API가 아니라
  이번 실행을 위해 작성한 파사드를 거칩니다.

## 언어별 실행 방식

| | 기록 | Diff | 브리지 | 섀도 |
| --- | --- | --- | --- | --- |
| Go | 원본을 가리키는 `replace`와 함께 독립 모듈로 빌드되는 생성된 드라이버 | 같은 드라이버를 cgo 래퍼 위에서 실행 | JSON 디스패처 + 원래 API를 가진 cgo 패키지 | 원래 API를 가진 패키지, 둘 다 실행 |
| Ruby | 생성된 드라이버가 원본 파일을 불러옴 | 같은 드라이버가 Fiddle 래퍼를 불러옴 | JSON 디스패처 + 같은 모듈을 정의하는 Fiddle 래퍼 | 원본 다음에 require하는 파일: 둘 다 `prepend` |
| Java | 원본 소스와 함께 컴파일되는 리플렉션 드라이버 | 같은 드라이버를 JNI 래퍼 클래스와 함께 컴파일 | JNI 위의 JSON 디스패처 + 원래 static 메서드를 가진 래퍼 클래스 | `<Class>Shadow` 클래스 |
| Python | 프로세스 내 드라이버가 원본을 import | 같은 드라이버가 PyO3 패키지를 import | PyO3 | 원래 API를 가진 모듈 |
| C | 원본을 `cc -shared`로 빌드해 ctypes로 호출, 케이스마다 fork 하나 | Rust `cdylib` 위의 ctypes | 원래 이름을 가진 `extern "C"` 함수와 헤더 | `-D` 이름 변경을 쓰는 섀도 `.c`, crate의 `shadow` 기능은 `everything_rust_` 이름만 export |
