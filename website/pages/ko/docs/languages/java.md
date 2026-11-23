---
title: Java
description: 타입이 있는 JNI와 생성된 바이너리 와이어를 통해, 원래 메서드를 가진 래퍼 클래스 안에서 public static 메서드 패키지를 Rust로 포팅합니다.
---

# Java

| | |
| --- | --- |
| 유닛 | `public static` 메서드 패키지 |
| 브리지 | 원래 메서드를 가진 래퍼 클래스 안의, 타입이 있는 JNI와 생성된 바이너리 와이어 |
| 기록과 재생 담당 | 생성된 리플렉션 드라이버 |
| 필요 조건 | JDK 17+ (`javac`와 `java`) |
| 상태 | 파사드 뒤의 JDK 21에서 검증: 4,492/4,492 (226개는 예외 발생), 게이트 통과 |

배치는 `new FnBatch(items)`이며, 다 쓰면 닫아야 합니다. `profile`은 JFR을 사용합니다.

## 포트가 정확히 맞춰야 할 부분

- `Integer.parseInt`는 `Character.digit`을 통해 모든 유니코드 십진 숫자(`١٢`, `２`)를 받아들입니다.
- `Double.toString`(JDK 19+)은 가장 짧은 십진수를 출력하지만 두 자리보다 적게 출력하지는 않습니다: `4.9E-324`이며, Rust의 가장 짧은 형식은 `5e-324`입니다.
- `\s`는 ASCII만 해당합니다. Rust의 `split_whitespace`는 U+00A0에서도 나눕니다.
- 부동소수점 값은 왕복 변환을 거쳐도 유지되어야 합니다. `serde_json`의 기본 부동소수점 파서는 항상 정확히 반올림하지는 않으므로, everything-rust과 everything-rust이 생성하는 모든 crate는 `float_roundtrip` 기능을 사용합니다.

## 속도

JIT 이후 리프 함수는 몇 나노초가 걸리고, 문자열을 넘기는 JNI 호출은 그보다 비쌉니다. JDK 파사드에서 `bench`는 포트를 호출당 0.18x로 측정했습니다. 더 큰 유닛을 포팅하세요. 4코어에서 배치로 실행하면 문자열 50,000개에 대한 `slugify`(Java에서 43 ms)가 **3배 더 빨랐습니다**(호출당 포트: 0.68x).

자세한 내용: [여섯 가지 언어](/docs/real-world/six-languages).
