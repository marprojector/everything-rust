---
title: "C# (프리뷰)"
description: "C#은 프리뷰 단계입니다. analyze는 .csproj 프로젝트를 읽고, C# CLI나 ASP.NET 서버를 기록하고 재생하며 Rust 프런트 도어 뒤에 둘 수 있습니다."
---

# C# (프리뷰)

| | |
| --- | --- |
| 유닛 | `public static` 메서드가 들어 있는 폴더 |
| 브리지 | 다음 단계: C ABI를 통한 P/Invoke |
| 현재 | 앱 전체: CLI와 ASP.NET 서버(`dotnet <app>.dll`)에 대한 `app record`, `app replay`, 프런트 도어 |
| 상태 | 프리뷰: `analyze`와 앱 전체만 지원, 유닛은 아직 브리지할 수 없음 |

`analyze`는 `.csproj` 프로젝트와 NuGet 패키지(crate로 매핑)를 읽고, `using` 지시문을 폴더로 해석하며, C#이 다르게 동작하는 부분을 표시합니다: `decimal`, `dynamic`, LINQ의 지연 쿼리, `Task`, `lock`, 리플렉션.

빌드된 앱을 기록합니다:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

P/Invoke를 통한 유닛 포팅이 다음 단계입니다. 그때까지 `bridge`는 명확한 메시지와 함께 C# 유닛을 거부합니다.

CI는 .NET을 설치하지 않으므로 C# 테스트는 .NET이 있는 머신에서만 실행됩니다. Kotlin과 Swift는 아직 지원하지 않습니다. 필요한 언어가 있으면 이슈를 열거나 기존 이슈에 추천을 눌러 주세요.
