---
title: Servidores
description: Os mesmos comandos comprovam servidores HTTP. --http, --fresh, --arg e --pace, e uma front door axum que faz proxy de todas as rotas a que nenhuma unidade responde.
---

# Servidores

Os mesmos comandos comprovam servidores HTTP. `--http` indica que o comando inicia um servidor; este obtém a sua
porta de `PORT` ou de `{port}` no comando (`--port` para um servidor que escuta sempre na mesma
porta):

```sh
everything-rust app record . --http -- node server.js
everything-rust app replay .
everything-rust app front-door .                                    # axum: todas as rotas com proxy
everything-rust app wire . api --route 'GET /users/{id}' --route '*'
everything-rust app lift .
```

- Os pedidos vêm das linhas `curl` na documentação da aplicação (`curl -X POST -d ... http://localhost:8080/x`),
  de `--inputs` (`{"name", "method", "path", "headers", "body"}`) e de armadilhas: segmentos do caminho e valores
  da query trocados pelas mesmas armadilhas usadas nos argumentos da CLI, corpos por armadilhas de texto e JSON, e uma
  rota desconhecida. Cada execução inicia o servidor de raiz e envia os pedidos por ordem de nome, por isso o estado
  entre pedidos é mantido.
- Um servidor que guarda estado em ficheiros recebe uma cópia nova a cada arranque: `--fresh db.json` (um ficheiro
  ou pasta, pode repetir-se) é capturado para `http/files/` e copiado para um novo diretório de trabalho
  sempre que o original, a front door ou uma build arranca; `--arg db.json` é um
  argumento que cada arranque recebe depois do seu comando, para que o binário Rust seja iniciado tal como o original.
  `--pace 50` espera 50 ms depois de cada resposta, para trabalho que um servidor faz depois de responder (um observador
  de ficheiros a recarregar o que acabou de escrever).
- Os casos são `rust/app-cases/http/<case>.toml` (o pedido, o estado e os headers relevantes)
  e `<case>.body` (o corpo da resposta, com `[..]` onde duas execuções diferiram). Date, server e
  outros headers específicos de cada resposta são deixados de fora; apague qualquer linha de header que não lhe interesse.
- A front door é um servidor axum que inicia o original numa porta própria e faz proxy de
  todas as rotas a que nenhuma unidade responde. `wire --route` aceita rotas axum: `METHOD /path/{param}`, `/path`
  para qualquer método, `*` para tudo o resto; a ligação recebe a rota, os seus parâmetros e o
  pedido. O `lift` remove o proxy; os casos HTTP passam então a correr na CI com `app replay`.

## Um caso real

O [json-server](/docs/real-world/json-server) precisou de `--fresh`, `--arg` e `--pace`: reescreve a sua base de dados a cada escrita, e o seu observador de ficheiros pode desfazer uma escrita que se segue rapidamente.
