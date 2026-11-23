---
title: PHP (프리뷰)
description: PHP는 프리뷰 단계입니다. analyze는 PHP 앱을 계획하고, PHP CLI나 서버를 기록하고 재생하며 Rust 프런트 도어 뒤에 둘 수 있습니다.
---

# PHP (프리뷰)

| | |
| --- | --- |
| 유닛 | 함수와 `public static` 메서드가 든 폴더 |
| 브리지 | 다음 단계: C ABI를 통한 PHP FFI |
| 현재 | 앱 전체: CLI와 서버(`php -S`)에 대한 `app record`, `app replay`, 프런트 도어 |
| 상태 | 프리뷰: `analyze`와 앱 전체만 지원, 유닛은 아직 브리지할 수 없음 |

`analyze`는 PHP 앱을 계획합니다: Composer 패키지와 PSR-4 오토로딩, 느슨한 비교나 정수가 부동소수점으로 넘치는 것 같은 위험 요소, crate로 매핑된 Composer 패키지. 타입은 PHP 타입과 phpdoc에서 가져옵니다.

PHP CLI나 서버를 기록하고 재생하며 Rust 프런트 도어 뒤에 둘 수 있습니다:

```sh
everything-rust app record . -- php cli.php
everything-rust app record . --http -- php -S 127.0.0.1:{port} router.php
everything-rust app replay .
everything-rust app front-door .
```

PHP의 FFI를 통해 유닛을 Rust로 포팅하는 것이 다음 단계입니다([issue #2](https://github.com/marprojector/everything-rust/issues/2)). 그때까지 `bridge`는 명확한 메시지와 함께 PHP 유닛을 거부합니다.

CI는 PHP를 설치하지 않으므로 PHP 테스트는 PHP가 있는 머신에서만 실행됩니다.
