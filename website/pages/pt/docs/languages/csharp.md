---
title: "C# (pré-visualização)"
description: "O C# está em pré-visualização. O analyze lê projetos .csproj, e uma CLI ou um servidor ASP.NET em C# pode ser registado, reproduzido e colocado atrás de uma front door em Rust."
---

# C# (pré-visualização)

| | |
| --- | --- |
| Unidade | uma pasta de métodos `public static` |
| Bridge | em breve: P/Invoke sobre uma ABI de C |
| Hoje | aplicações completas: `app record`, `app replay` e a front door, para CLIs e servidores ASP.NET (`dotnet <app>.dll`) |
| Estado | Pré-visualização: apenas `analyze` e aplicações completas; ainda não é possível criar bridges para unidades |

O `analyze` lê projetos `.csproj` e os respetivos pacotes NuGet (mapeados para crates), resolve as diretivas `using` para pastas e assinala o que o C# faz de forma diferente: `decimal`, `dynamic`, as consultas diferidas do LINQ, `Task`, `lock`, reflexão.

Registe uma aplicação compilada:

```sh
everything-rust app record . -- dotnet out/app.dll
everything-rust app record . --http -- dotnet out/app.dll --urls http://127.0.0.1:{port}
everything-rust app replay .
everything-rust app front-door .
```

A migração de unidades através de P/Invoke é o próximo passo. Até lá, o `bridge` recusa unidades C# com uma mensagem clara.

A CI não instala o .NET, por isso os testes de C# só correm numa máquina que o tenha. Kotlin e Swift ainda não são suportados; abra uma issue (ou vote numa existente) para a linguagem de que precisa.
