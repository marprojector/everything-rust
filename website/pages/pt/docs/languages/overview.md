---
title: Linguagens
description: "TypeScript/JavaScript, Python, C, Go, Ruby e Java, com PHP e C# em pré-visualização. A unidade, a bridge e o mecanismo de registo de cada uma."
---

# Linguagens

Seis linguagens passam pelo ciclo completo: Node.js (TypeScript/JavaScript), Python, C, Go, Ruby e Java. PHP e C# estão em pré-visualização: hoje `analyze` e aplicações completas, as bridges de unidades vêm a seguir.

| Linguagem | Unidade | Bridge | Registado e reproduzido por |
| --- | --- | --- | --- |
| TypeScript/JavaScript | pasta ou ficheiro | napi-rs ou wasm-bindgen com um formato binário gerado; classes e async incluídos | Node, a executar os ficheiros `.ts` |
| Python | pacote ou módulo | PyO3 (abi3), conversão direta, exceções tipadas | Python, a importar o original |
| C | ficheiro `.c` | funções `extern "C"` com os nomes originais e um header | o original compilado como biblioteca partilhada, chamado através de ctypes, um fork por caso |
| Go | pacote | chamadas cgo tipadas e um formato binário gerado, num pacote com a API original | um driver Go gerado, compilado como módulo próprio |
| Ruby | ficheiro ou pasta de funções de módulo | uma extensão magnus sob os módulos originais | um driver Ruby gerado |
| Java | pacote de métodos `public static` | JNI tipado e um formato binário gerado, em classes wrapper com os métodos originais | um driver de reflexão gerado |
| PHP (pré-visualização) | pasta de funções e métodos `public static` | em breve: PHP FFI sobre uma ABI de C | hoje, aplicações completas: `app record`, `app replay` e a front door, para CLIs e servidores (`php -S`) |
| C# (pré-visualização) | pasta de métodos `public static` | em breve: P/Invoke sobre uma ABI de C | hoje, aplicações completas: `app record`, `app replay` e a front door, para CLIs e servidores ASP.NET (`dotnet <app>.dll`) |

Todas as linguagens exceto C têm também classes de lote (`--batch`): os itens atravessam a bridge uma vez e ficam em
Rust, e cada chamada corre sobre todos eles em todos os núcleos.

A linguagem é detetada a partir do repositório (`go.mod`, `pom.xml`, um `.csproj` ou `.sln`,
`pyproject.toml`, `Gemfile`, `composer.json`, `package.json`, ou então os ficheiros de código mais comuns);
`--lang` sobrepõe-se a essa deteção. Os tipos vêm do código-fonte: tipos TS, type hints de Python, assinaturas de C, Go, Java e C#,
tags YARD de Ruby, tipos de PHP e phpdoc; parâmetros sem tipo utilizados como números são inferidos
como números.

O PHP está em pré-visualização: o `analyze` planeia uma aplicação PHP (pacotes Composer e autoloading PSR-4, riscos
como a comparação não estrita e o overflow de inteiros para floats, pacotes Composer mapeados para crates),
e uma CLI ou um servidor PHP pode ser registado, reproduzido e colocado atrás de uma front door em Rust. Migrar as suas
unidades para Rust através da FFI do PHP é o próximo passo ([issue #2](https://github.com/marprojector/everything-rust/issues/2)).

O C# está em pré-visualização da mesma forma: o `analyze` lê projetos `.csproj` e os respetivos pacotes NuGet
(mapeados para crates), resolve as diretivas `using` para pastas e assinala o que o C# faz de forma diferente
(`decimal`, `dynamic`, as consultas diferidas do LINQ, `Task`, `lock`, reflexão). Uma CLI ou um servidor ASP.NET
em C# pode ser registado, reproduzido e colocado atrás de uma front door em Rust; a migração de unidades através de P/Invoke
vem a seguir.

## Estado por linguagem

| Linguagem | Estado | Última execução real |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | Verificado | json-server, Fuse.js, vercel/ms, `packages/core` do x18n |
| [Python](/docs/languages/python) | Verificado | `colorsys.py` do CPython 3.13: 2,400/2,400 |
| [Go](/docs/languages/go) | Verificado | `path` do Go 1.24: 2,642/2,642 |
| [Ruby](/docs/languages/ruby) | Verificado | `shellwords.rb` do Ruby 3.3: 1,617/1,617 |
| [Java](/docs/languages/java) | Verificado | JDK 21 por trás de uma fachada: 4,492/4,492 |
| [C](/docs/languages/c) | Verificado | glibc por trás de uma fachada: 1,859/1,859 depois de retirar um caso com comportamento indefinido |
| [PHP](/docs/languages/php) | Pré-visualização | apenas `analyze` e aplicações completas |
| [C#](/docs/languages/csharp) | Pré-visualização | apenas `analyze` e aplicações completas |

A tabela completa, com os comandos, está na página [Estado](/docs/project/status).
