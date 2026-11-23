---
title: 서버
description: 같은 명령으로 HTTP 서버도 증명합니다. --http, --fresh, --arg, --pace, 그리고 유닛이 응답하지 않는 모든 라우트를 프록시하는 axum 프런트 도어.
---

# 서버

같은 명령으로 HTTP 서버도 증명합니다. `--http`는 명령이 서버를 시작한다는 뜻입니다. 포트는
`PORT`나 명령 안의 `{port}`에서 가져옵니다(항상 같은 포트에서 대기하는 서버에는 `--port`):

```sh
everything-rust app record . --http -- node server.js
everything-rust app replay .
everything-rust app front-door .                                    # axum: 모든 라우트를 프록시
everything-rust app wire . api --route 'GET /users/{id}' --route '*'
everything-rust app lift .
```

- 요청은 앱 문서의 `curl` 줄(`curl -X POST -d ... http://localhost:8080/x`),
  `--inputs`(`{"name", "method", "path", "headers", "body"}`), 함정 케이스에서 가져옵니다: 경로 세그먼트와 쿼리
  값은 CLI 인자와 같은 함정으로 바꾸고, 본문은 텍스트와 JSON 함정으로 바꾸며, 알 수 없는
  라우트도 하나 넣습니다. 각 실행은 서버를 새로 시작하고 요청을 이름 순서로 보내므로 요청
  사이의 상태가 유지됩니다.
- 상태를 파일에 저장하는 서버는 시작할 때마다 새 복사본을 받습니다: `--fresh db.json`(파일
  또는 폴더, 반복 가능)은 `http/files/`에 스냅숏으로 저장되고, 원본, 프런트 도어, 빌드가
  시작할 때마다 새 작업 디렉터리로 복사됩니다. `--arg db.json`은 매번 시작할 때 명령 뒤에 붙는
  인자이므로 Rust 바이너리도 원본과 같은 방식으로 시작됩니다.
  `--pace 50`은 서버가 응답 후에 하는 작업(방금 쓴 내용을 다시 읽는 파일 감시자 등)을 위해
  응답마다 50 ms를 기다립니다.
- 케이스는 `rust/app-cases/http/<case>.toml`(요청, 상태 코드, 중요한 헤더)과
  `<case>.body`(응답 본문, 두 실행에서 달랐던 부분은 `[..]`)입니다. Date, server 등
  응답마다 달라지는 헤더는 제외됩니다. 신경 쓰지 않는 헤더 줄은 삭제하세요.
- 프런트 도어는 자체 포트에서 원본을 시작하고 유닛이 응답하지 않는 모든 라우트를 프록시하는
  axum 서버입니다. `wire --route`는 axum 라우트를 받습니다: `METHOD /path/{param}`, 모든 메서드에
  대한 `/path`, 나머지 전부에 대한 `*`. 글루 코드는 라우트, 매개변수, 요청을 받습니다.
  `lift`가 프록시를 제거하며, 이후 HTTP 케이스는 CI에서 `app replay`로 실행됩니다.
## 실제 사례

[json-server](/docs/real-world/json-server)에는 `--fresh`, `--arg`, `--pace`가 필요했습니다: 쓰기마다 데이터베이스를 다시 쓰고, 파일 감시자가 곧바로 뒤따르는 쓰기를 되돌릴 수 있기 때문입니다.
