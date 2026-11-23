---
title: Doctor
description: everything-rust doctor는 언어 런타임, 에이전트 설정, MCP 서버를 점검하고 각 문제를 해결하는 명령어를 알려 줍니다.
---

# Doctor

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor`는 언어 런타임, 에이전트 설정, MCP 서버를 점검하고 각 문제를 해결하는 명령어를 출력합니다. 필수 항목이 없으면 종료 코드 1로 끝납니다.

런타임, 에이전트 설정, MCP 서버가 없거나 오래되어 어떤 단계가 실패할 때 실행하세요. 워크플로 스킬도 에이전트에게 똑같이 하라고 안내합니다.

## 언어별 요구 사항

`record`, `diff`, `bench`는 원본을 해당 언어의 런타임에서 실행합니다: TS는 Node 22.6+(`.ts` 소스를 직접 실행), Python과 C(ctypes)는 Python 3.9+, Go 1.21+, Ruby 3+, JDK 17+(`javac`와 `java`). Node에서 `profile`을 쓰려면 `NODE_OPTIONS`에 `--cpu-prof`를 허용하는 Node 24+가 필요합니다. C는 C 컴파일러도 필요합니다. wasm 브리지는 `rustup target add wasm32-unknown-unknown`과 버전이 맞는 `wasm-bindgen-cli`도 필요하며, 정확한 버전은 `build`가 알려 줍니다.

## 기여자를 위한 안내

`cargo run -- doctor`는 설치된 런타임과 버전, 각 런타임을 설치하거나 업데이트하는 명령어를 보여 줍니다. [기여하기](/docs/project/contributing)를 참고하세요.
