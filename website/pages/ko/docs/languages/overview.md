---
title: 언어
description: "TypeScript/JavaScript, Python, C, Go, Ruby, Java, 그리고 프리뷰 단계의 PHP와 C#. 언어별 유닛, 브리지, 기록 방식."
---

# 언어

여섯 가지 언어가 전체 루프를 지원합니다: Node.js(TypeScript/JavaScript), Python, C, Go, Ruby, Java. PHP와 C#은 프리뷰 단계로, 현재는 `analyze`와 앱 전체를 지원하며 유닛 브리지는 다음 단계입니다.

| 언어 | 유닛 | 브리지 | 기록과 재생 담당 |
| --- | --- | --- | --- |
| TypeScript/JavaScript | 폴더 또는 파일 | 생성된 바이너리 와이어를 쓰는 napi-rs 또는 wasm-bindgen, 클래스와 async 포함 | `.ts` 소스를 실행하는 Node |
| Python | 패키지 또는 모듈 | PyO3 (abi3), 직접 변환, 타입이 있는 예외 | 원본을 import하는 Python |
| C | `.c` 파일 | 원래 이름을 가진 `extern "C"` 함수와 헤더 | 공유 라이브러리로 컴파일한 원본을 ctypes로 호출, 케이스마다 fork 하나 |
| Go | 패키지 | 원래 API를 가진 패키지 안의, 타입이 있는 cgo 호출과 생성된 바이너리 와이어 | 독립 모듈로 빌드되는, 생성된 Go 드라이버 |
| Ruby | 모듈 함수가 든 파일 또는 폴더 | 원래 모듈 아래의 magnus 확장 | 생성된 Ruby 드라이버 |
| Java | `public static` 메서드 패키지 | 원래 메서드를 가진 래퍼 클래스 안의, 타입이 있는 JNI와 생성된 바이너리 와이어 | 생성된 리플렉션 드라이버 |
| PHP (프리뷰) | 함수와 `public static` 메서드가 든 폴더 | 다음 단계: C ABI를 통한 PHP FFI | 현재는 앱 전체: CLI와 서버(`php -S`)에 대한 `app record`, `app replay`, 프런트 도어 |
| C# (프리뷰) | `public static` 메서드가 든 폴더 | 다음 단계: C ABI를 통한 P/Invoke | 현재는 앱 전체: CLI와 ASP.NET 서버(`dotnet <app>.dll`)에 대한 `app record`, `app replay`, 프런트 도어 |

C를 제외한 모든 언어는 배치 클래스(`--batch`)도 지원합니다: 항목이 경계를 한 번만 넘어
Rust 안에 머물고, 각 호출은 모든 코어에서 전체 항목을 처리합니다.

언어는 저장소에서 감지합니다(`go.mod`, `pom.xml`, `.csproj` 또는 `.sln`,
`pyproject.toml`, `Gemfile`, `composer.json`, `package.json`, 없으면 가장 많은 소스 파일 기준).
`--lang`으로 덮어쓸 수 있습니다. 타입은 소스에서 가져옵니다: TS 타입, Python 타입 힌트, C, Go, Java, C#
시그니처, Ruby YARD 태그, PHP 타입과 phpdoc. 타입이 없지만 숫자로 쓰이는 매개변수는 숫자로
추론합니다.

PHP는 프리뷰 단계입니다: `analyze`는 PHP 앱을 계획하고(Composer 패키지와 PSR-4 오토로딩, 느슨한
비교나 정수가 부동소수점으로 넘치는 것 같은 위험 요소, crate로 매핑된 Composer 패키지),
PHP CLI나 서버를 기록하고 재생하며 Rust 프런트 도어 뒤에 둘 수 있습니다. PHP의 FFI를 통해
유닛을 Rust로 포팅하는 것이 다음 단계입니다([issue #2](https://github.com/marprojector/everything-rust/issues/2)).

C#도 같은 방식으로 프리뷰 단계입니다: `analyze`는 `.csproj` 프로젝트와 NuGet 패키지
(crate로 매핑)를 읽고, `using` 지시문을 폴더로 해석하며, C#이 다르게 동작하는 부분을 표시합니다
(`decimal`, `dynamic`, LINQ의 지연 쿼리, `Task`, `lock`, 리플렉션). C# CLI나 ASP.NET
서버를 기록하고 재생하며 Rust 프런트 도어 뒤에 둘 수 있고, P/Invoke를 통한 유닛 포팅은
다음 단계입니다.
## 언어별 상태

| 언어 | 상태 | 마지막 실제 실행 |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | 검증됨 | json-server, Fuse.js, vercel/ms, x18n의 `packages/core` |
| [Python](/docs/languages/python) | 검증됨 | CPython 3.13의 `colorsys.py`: 2,400/2,400 |
| [Go](/docs/languages/go) | 검증됨 | Go 1.24의 `path`: 2,642/2,642 |
| [Ruby](/docs/languages/ruby) | 검증됨 | Ruby 3.3의 `shellwords.rb`: 1,617/1,617 |
| [Java](/docs/languages/java) | 검증됨 | 파사드 뒤의 JDK 21: 4,492/4,492 |
| [C](/docs/languages/c) | 검증됨 | 파사드 뒤의 glibc: 정의되지 않은 동작 케이스 하나를 제외하고 1,859/1,859 |
| [PHP](/docs/languages/php) | 프리뷰 | `analyze`와 앱 전체만 지원 |
| [C#](/docs/languages/csharp) | 프리뷰 | `analyze`와 앱 전체만 지원 |

명령어가 포함된 전체 표는 [현황](/docs/project/status) 페이지에 있습니다.
