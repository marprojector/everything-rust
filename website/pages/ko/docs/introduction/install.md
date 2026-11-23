---
title: 설치
description: crates.io, npm 또는 릴리스 바이너리로 everything-rust을 설치하고, 언어별로 필요한 런타임을 준비합니다.
---

# 설치

## crates.io, npm 또는 릴리스에서 설치

::: tabs
== cargo
```sh
cargo install everything-rust
everything-rust setup
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

npm은 JavaScript 프로젝트를 위해 동일한 바이너리를 설치합니다.

macOS, Linux, Windows용 사전 빌드 바이너리는 각 [GitHub 릴리스](https://github.com/marprojector/everything-rust/releases)에 첨부되어 있습니다. Claude Code에서는 플러그인(MCP 서버와 스킬 포함, CLI는 `PATH`에 있어야 함)으로도 설치할 수 있습니다:

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## 소스에서 설치

```sh
cargo install --path .
```

`record`, `diff`, `bench`는 원본을 해당 언어의 런타임에서 실행합니다: TS는 Node 22.6+(`.ts`
소스를 직접 실행), Python과 C(ctypes)는 Python 3.9+, Go 1.21+, Ruby 3+, JDK 17+
(`javac`와 `java`). Node에서 `profile`을 쓰려면 `NODE_OPTIONS`에 `--cpu-prof`를 허용하는 Node 24+가 필요합니다. C는 C 컴파일러도 필요합니다. wasm 브리지는 `rustup target add wasm32-unknown-unknown`과
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked`도 필요하며, 정확한 버전은 `build`가
알려 줍니다.

## Windows

네이티브 Windows CI는 `x86_64-pc-windows-msvc` Rust 타깃을 사용합니다. stable MSVC
툴체인과 **Desktop development with C++** 워크로드가 포함된 Visual Studio Build Tools를 설치하세요.
C 컴파일러와 링커는 C 브리지와 네이티브 Rust 확장에 필요합니다. 마이그레이션할 원본 언어의
런타임도 설치하세요. Python은 `python3` 또는 `python`으로 제공될 수 있으며,
everything-rust은 둘 다 감지합니다. GNU Windows 타깃은 CI에서 검증하지 않습니다.

네이티브 브리지 통합 테스트 스위트는 Developer PowerShell 프롬프트에서 실행하세요:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

이 스위트는 실제 DLL 기반 Node 및 Python 확장과, `PATH`에 있는 각 런타임에서 지원되는 네이티브
브리지를 빌드하고 로드합니다. 없는 선택적 런타임은 건너뜀(skipped)으로 보고됩니다.
Cargo, 활성화된 Rust 표준 라이브러리, MSVC 링커는 필수입니다. Cargo가 `std`를 찾을 수 없다고
보고하면 `rustup target remove x86_64-pc-windows-msvc`를 실행한 다음
`rustup target add x86_64-pc-windows-msvc`를 실행해 활성 타깃을 다시 설치하세요. 링크에 실패하면 Visual Studio Developer
PowerShell에서 실행하고 `where.exe link`가 MSVC 링커를 찾는지 확인하세요. 테스트는 공백과 유니코드 문자가 들어간 경로를 포함해
임시 폴더를 사용하며, 실행이 성공하면 생성된 아티팩트를 삭제합니다.

에이전트용으로는 `everything-rust setup`이 모든 것을 등록합니다(빠른 시작 참고). 대신 프로젝트 하나에만
MCP 서버를 직접 추가하려면(`.mcp.json`):

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

각 도구와 매개변수, 도구가 건드릴 수 있는 범위: [MCP 도구](/docs/reference/mcp-tools). 서버는 스킬을 바탕으로 만든
프롬프트 두 개(`port_unit`, `port_whole_app`)도 제공합니다.
## 업데이트 또는 제거

```sh
cargo install everything-rust                                    # 업데이트 (Cargo)
npm install -g everything-rust@latest                            # 업데이트 (npm)
everything-rust setup                                            # 에이전트 설정 갱신
everything-rust setup --remove && cargo uninstall everything-rust  # 제거 (Cargo)
everything-rust setup --remove && npm uninstall -g everything-rust # 제거 (npm)
```

`setup --remove`는 everything-rust 항목만 제거하며, 저장해 둔 복사본
(`<file>.everything-rust.bak`)은 직접 삭제할 때까지 남아 있습니다.
