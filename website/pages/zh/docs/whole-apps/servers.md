---
title: 服务端
description: 同样的命令也能验证 HTTP 服务端。--http、--fresh、--arg 和 --pace，以及一个把尚无单元接管的路由全部代理出去的 axum 前门。
---

# 服务端

同样的命令也能验证 HTTP 服务端。`--http` 表示该命令会启动一个服务端；它从 `PORT` 或命令中的
`{port}` 获取端口（如果服务端总是监听同一个端口，使用 `--port`）：

```sh
everything-rust app record . --http -- node server.js
everything-rust app replay .
everything-rust app front-door .                                    # axum：所有路由都被代理
everything-rust app wire . api --route 'GET /users/{id}' --route '*'
everything-rust app lift .
```

- 请求来自应用文档中的 `curl` 命令（`curl -X POST -d ... http://localhost:8080/x`）、
  `--inputs`（`{"name", "method", "path", "headers", "body"}`）以及语义陷阱：路径段和查询
  参数会替换为与 CLI 参数相同的陷阱值，请求体替换为文本和 JSON 陷阱，另外还有一个
  未知路由。每次运行都会全新启动服务端，并按名称顺序发送请求，因此请求之间的
  状态会被保留。
- 把状态保存在文件中的服务端，每次启动都会拿到一份全新的副本：`--fresh db.json`（文件
  或文件夹，可重复指定）会被快照到 `http/files/`，每次启动原实现、前门或某个构建产物时，都会被复制到一个新的工作
  目录中；`--arg db.json` 是每次启动时追加在命令之后的参数，这样 Rust 二进制就能以与原实现相同的方式启动。
  `--pace 50` 在每次响应后等待 50 ms，用于服务端在响应之后还要做的工作（例如文件
  监视器重新加载它刚写入的内容）。
- 用例保存为 `rust/app-cases/http/<case>.toml`（请求、状态码和需要关心的响应头）
  与 `<case>.body`（响应体，两次运行不同的地方替换为 `[..]`）。Date、Server 和
  其他每次响应都会变化的头会被剔除；你不关心的任何响应头行都可以删除。
- 前门是一个 axum 服务端，它在自己的端口上启动原实现，并把尚无单元接管的
  路由全部代理过去。`wire --route` 接受 axum 路由写法：`METHOD /path/{param}`、匹配任意方法的 `/path`，
  以及匹配其余一切的 `*`；胶水代码会拿到路由、路径参数和
  请求。`lift` 会移除代理；之后 HTTP 用例就可以在 CI 中用 `app replay` 运行。
## 一个真实案例

[json-server](/docs/real-world/json-server) 需要 `--fresh`、`--arg` 和 `--pace`：它每次写入都会重写数据库，而它的文件监视器可能会撤销紧随其后的写入。
