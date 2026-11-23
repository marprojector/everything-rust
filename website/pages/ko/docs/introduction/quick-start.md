---
title: 빠른 시작
description: everything-rust을 설치하고 코딩 에이전트에 등록한 뒤, 첫 번째 유닛으로 루프를 실행합니다.
---

# 빠른 시작

## 에이전트에게 설치 맡기기

아래 내용을 Claude Code, Codex, Cursor 또는 Gemini CLI에 붙여 넣으세요:

```text
Install everything-rust: run `cargo install everything-rust`, then `everything-rust setup`.
Tell me when to restart you.
```

## 직접 설치하기

::: tabs
== cargo
```sh
cargo install everything-rust   # crates.io에서 설치
everything-rust setup           # Claude Code, Codex, Cursor, Gemini CLI에 등록
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

아직 Rust 툴체인이 없나요? macOS, Linux, Windows용 사전 빌드 바이너리가 각 [GitHub 릴리스](https://github.com/marprojector/everything-rust/releases)에 첨부되어 있으며, JavaScript 프로젝트라면 npm으로 동일한 바이너리를 설치할 수 있습니다.

`setup`은 찾아낸 각 에이전트에 MCP 서버를 추가하고, 에이전트가 스킬을 읽는 위치에 워크플로 스킬을 설치합니다. 무엇이 바뀌는지는 [에이전트 설정](/docs/introduction/agents)을 참고하세요.

## 에이전트에게 요청하기

- *"everything-rust으로 이 CLI를 처음부터 끝까지 Rust로 다시 작성하고, 똑같이 동작한다는 걸 증명해 줘."*
- *"이 검색이 느린 것 같아. everything-rust으로 프로파일링하고 핫 루프를 Rust로 옮겨 줘."*
- *"everything-rust으로 `src/parser/`를 Rust로 마이그레이션하는 계획을 세우고, 첫 번째 유닛을 포팅해서 검증해 줘."*
- *"`rust/text-rs`의 Rust 포트를 원본과 벤치마킹하고 배포해도 될지 알려 줘."*

## 단계를 직접 실행하기

x18n의 `fuzzyScore`에 적용한 단계입니다: `profile`이 핫 루프를 찾고, `bridge --batch`가 crate를 생성하고, `gate`가 통과하며, 벤치마크가 TypeScript와 비교합니다.

```sh
everything-rust profile . -- node bench.mjs              # 어디가 핫하고, 누가 호출하는지
everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs 작성 (또는 에이전트에게 맡기기), 그다음:
everything-rust record rust/fuzzy-rs && everything-rust build rust/fuzzy-rs
everything-rust diff rust/fuzzy-rs                       # 기록된 모든 케이스가 일치해야 함
everything-rust bench rust/fuzzy-rs                      # 더 빠른가? 판정이 알려 줌
everything-rust gate rust/fuzzy-rs                       # 이것이 통과해야 완료
```

각 단계는 [작업 루프](/docs/concepts/the-loop)에서 설명합니다.

::: tip 무언가 빠져 있을 때
런타임, 에이전트 설정, MCP 서버가 없거나 오래되어 어떤 단계가 실패하면 [`everything-rust doctor`](/docs/introduction/doctor)를 실행하세요.
:::
