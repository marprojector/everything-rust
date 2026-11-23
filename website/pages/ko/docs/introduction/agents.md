---
title: 에이전트 설정
description: everything-rust setup은 MCP 서버와 워크플로 스킬을 Claude Code, Codex, Cursor, Gemini CLI에 등록합니다.
---

# 에이전트 설정

```sh
everything-rust setup
```

`setup`은 찾아낸 각 에이전트에 MCP 서버를 추가하고, 에이전트가 스킬을 읽는 위치에 워크플로 스킬을 설치합니다. 기존의 다른 설정은 그대로 유지되고, 여러 번 다시 실행해도 안전하며, `--remove`로 되돌릴 수 있습니다.

## 지원 에이전트

| 에이전트 | `everything-rust setup`이 추가하는 항목 |
| --- | --- |
| Claude Code | `~/.claude.json`의 MCP 서버, `~/.claude/skills/`의 스킬 |
| Codex | `~/.codex/config.toml`의 MCP 서버, `~/.codex/skills/`의 스킬 |
| Cursor | `~/.cursor/mcp.json`의 MCP 서버 |
| Gemini CLI | `~/.gemini/settings.json`의 MCP 서버 |
| 기타 MCP 클라이언트 | `everything-rust mcp`를 stdio 서버로 직접 추가 (아래 참고) |

이미 있는 파일을 수정하기 전에 `setup`은 같은 위치에 복사본(`<file>.everything-rust.bak`)을 저장합니다. `setup --remove`는 everything-rust 항목만 제거하며, 저장해 둔 복사본은 직접 삭제할 때까지 남아 있습니다.

## MCP 서버

`everything-rust mcp`는 모든 단계를 stdio를 통한 MCP 도구로 제공합니다: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate`, 그리고 앱 전체용 `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. 각 도구와 매개변수, 도구가 건드릴 수 있는 범위는 [MCP 도구](/docs/reference/mcp-tools)를 참고하세요. 서버는 스킬을 바탕으로 만든 프롬프트 두 개(`port_unit`, `port_whole_app`)도 제공합니다.

대신 프로젝트 하나에만 MCP 서버를 직접 추가하려면:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## 스킬

워크플로 스킬([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md))은 에이전트에게 루프를 실행하는 방법을 알려 줍니다: 프로파일링하고, 계획을 세운 뒤 승인을 기다리고, 유닛 하나를 포팅하고, 검증하고, 섀도 뒤에서 배포하며, `gate`를 통과했을 때만 완료로 간주합니다. 앱 전체의 경우에는 [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md)를 따릅니다. [앱 전체 포팅](/docs/whole-apps/overview)을 참고하세요.

## Claude Code 플러그인

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

플러그인에는 MCP 서버와 스킬이 포함되며, CLI는 `PATH`에 있어야 합니다.
