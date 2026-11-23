---
title: Contribuir
description: Configuração, onde colocar os testes, como adicionar um mapeamento de crate ou uma linguagem e como partilhar uma migração real.
---

# Contribuir

Obrigado pela ajuda. Relatórios de bugs, correções, suporte para novas linguagens e relatos de migrações reais são todos
bem-vindos, e não precisa de pedir autorização antes de abrir um pull request para corrigir um bug.

## Antes de começar

- **Um bug:** abra um [issue](https://github.com/marprojector/everything-rust/issues/new/choose) com
  o comando que executou, a respetiva saída, a sua plataforma e a versão do everything-rust
  (`everything-rust --version`). Um pequeno projeto que reproduza o problema é o que mais ajuda.
- **Uma alteração maior** (uma nova linguagem, um novo comando, uma mudança ao que é uma unidade ou uma gravação):
  abra primeiro um issue para podermos acordar a forma antes de a escrever. As razões por trás de escolhas
  anteriores estão em [DECISIONS.md](https://github.com/marprojector/everything-rust/blob/main/DECISIONS.md).
- **Um problema de segurança:** não abra um issue público; consulte [SECURITY.md](https://github.com/marprojector/everything-rust/blob/main/SECURITY.md).

## Configuração

Precisa de Rust (stable, 1.97 ou mais recente). Os testes também executam a linguagem de que cada parte trata, e
ignoram uma parte cujo runtime esteja em falta, por isso instale os que vai alterar:

| Para | Instalar |
| --- | --- |
| TypeScript/JavaScript | Node 24 (22.6+ funciona, exceto para `profile`) |
| Python | Python 3.9+ |
| C | um compilador de C (`cc`) |
| Go | Go 1.22+ |
| Ruby | Ruby 3.x |
| Java | um JDK (17+) |
| PHP | PHP 8 |
| C# | o .NET SDK |
| Bridges wasm | `rustup target add wasm32-unknown-unknown` e o `wasm-bindgen-cli` correspondente |

```sh
git clone https://github.com/marprojector/everything-rust
cd everything-rust
git config core.hooksPath .githooks
cargo test
```

`cargo run -- doctor` lista quais destes runtimes tem, as respetivas versões e o comando
que instala ou atualiza cada um. Para fazer com que um runtime em falta faça falhar os testes em vez de os ignorar
(a CI faz isto), indique-o em `EVERYTHING_RUST_REQUIRE`, por exemplo
`EVERYTHING_RUST_REQUIRE=node,python3,go,ruby,java cargo test`.

O hook executa `scripts/check-secrets.sh`, que recusa chaves, tokens e caminhos de pastas pessoais
no que fizer commit. A CI executa-o em todos os ficheiros.

## Fazer a alteração

- **Cada correção vem acompanhada de um teste que falha sem ela.** Execute o teste uma vez contra o código antigo para
  ter a certeza de que falha.
- **Onde colocar os testes:**
  - `tests/analyze.rs`: deteção de linguagem, unidades, riscos, dependências, `--whole-app`.
  - `tests/loop.rs`: bridge, record, diff e shadow por linguagem, com substitutos em vez de builds.
  - `tests/native.rs`: bridges compiladas de verdade, depois carregadas e reproduzidas.
  - `tests/app.rs` e `tests/http.rs`: record, replay, porta de entrada e lift de aplicações completas, para CLIs e
    servidores.
  - `tests/library.rs`: migrações de bibliotecas completas.
  - Os testes unitários ficam junto ao código.
- **As fixtures** são pequenas aplicações em `tests/fixtures/`. As fixtures de CLI e HTTP implementam os mesmos
  comandos e rotas em todas as linguagens (`tcli upper|len|div|words|copy|now`, `/hello`, `/upper`,
  `/div/{a}/{b}`, `/now`), por isso mantenha uma nova com comportamento idêntico às restantes.
- **O código gerado** (as bridges, drivers e portas de entrada que o everything-rust escreve) também tem de passar
  `clippy -D warnings`, já que o `gate` o executa nas crates dos utilizadores.
- **Estilo:** siga o código à sua volta. Reserve os comentários para o que o código não consegue dizer por si só, e
  coloque o raciocínio por trás de uma alteração no pull request.

## Adicionar um mapeamento de crate

O `analyze` indica a crate Rust para cada biblioteca que o código importa, a partir de `src/crates.toml`. Para adicionar
uma, acrescente uma entrada na respetiva linguagem: `names` (nomes exatos de pacotes) ou `prefixes` (o início de
um nome, para Java e C#), `crates`, `fit` (`good`, `partial` ou `none`) e uma `note` quando a adequação
precisar de explicação. A primeira entrada correspondente prevalece, por isso coloque as específicas antes dos prefixos amplos. Uma
biblioteca que não esteja na tabela é indicada como `unknown`; nunca adivinhe uma crate.

## Adicionar uma linguagem

PHP e C# foram adicionados como pré-visualizações em dois commits que vale a pena ler primeiro:
[e8ac243](https://github.com/marprojector/everything-rust/commit/e8ac243) (PHP) e
[caea59a](https://github.com/marprojector/everything-rust/commit/caea59a) (C#). Uma nova linguagem começa
da mesma forma:

1. Um plugin em `src/lang/<name>.rs`: descoberta, análise (imports e riscos), resolução e exports,
   mais as crates das suas bibliotecas como entradas `[[<name>]]` em `src/crates.toml`.
2. Deteção e encaminhamento em `src/lang/mod.rs`.
3. Suporte para aplicações completas em `src/whole_app.rs`: pontos de entrada, servidores, dependências declaradas.
4. As fixtures de CLI e HTTP, uma fixture de biblioteca e testes em `tests/analyze.rs`, `tests/app.rs`
   e `tests/http.rs`.
5. O `bridge` recusa as unidades da linguagem até existir uma bridge, com uma mensagem clara.

## Traduzir o README

As traduções são bem-vindas e mantidas pela comunidade. Uma nova tradução:

1. Fica em `docs/README_<CODE>.md` (por exemplo `docs/README_PT.md`); o `README.md` da raiz continua em
   inglês e nunca é renomeado nem substituído.
2. Utiliza caminhos que funcionam a partir de `docs/`: `assets/logo.svg`, `../LICENSE`, `../README.md`.
3. Começa com a mesma linha de idiomas das outras traduções, com o seu próprio idioma em texto
   simples, seguida da nota de que é mantida pela comunidade e pode estar desatualizada em relação ao
   README em inglês (copie-a de `docs/README_DE.md` e traduza-a).
4. Vai para o **fim** da linha de idiomas, no `README.md` e em todas as traduções. A ordem dos
   idiomas já listados não muda.

## Site de documentação

O site de documentação fica em `website/` (Void, `@void/md`, React). Muitas das suas páginas incluem
secções de `README.md`, `CONTRIBUTING.md` e `docs/*.md` no momento da build, por isso edite esses ficheiros e o
site acompanha. Execute-o com `cd website && npm ci --ignore-scripts && npm run dev` e faça a build com
`npm run build`. A referência da CLI vem de um snapshot do `--help` em
`website/content/cli-help.json`; depois de alterar a CLI, atualize-o com `cargo build --release` e
`npm run cli-help` em `website/`.

## Abrir o pull request

Antes de fazer push:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

- Diga o que estava errado, como reproduzir e como a alteração o corrige.
- Ligue o issue (`Fixes #123`).
- Indique o que não conseguiu testar (outro sistema operativo, um runtime que não tem).
- Limite-se a uma alteração: duas correções são mais fáceis de rever como dois pull requests.

A CI é executada em Ubuntu e macOS com todos os runtimes instalados. Para quem contribui pela primeira vez, o GitHub
espera que um maintainer aprove a execução da CI, por isso conte com um pequeno atraso antes de as verificações começarem.

Os pull requests são integrados mantendo os seus commits, por isso o seu histórico continua a ser seu. Se uma alteração precisar de
pequenos ajustes, um maintainer pode acrescentar um commit por cima em vez de pedir outra ronda.

## Partilhar uma migração real

As migrações de projetos reais são a melhor prova do que funciona e do que falha. Se utilizou o
everything-rust em algo real, um issue ou um pull request que o acrescente a
[docs/real-world.md](/docs/real-world/overview) é muito bem-vindo. Inclua o projeto, os comandos que
executou, o número de casos e tudo o que o tenha surpreendido.

## Código de conduta

Espera-se que todos os participantes sigam o [código de conduta](https://github.com/marprojector/everything-rust/blob/main/CODE_OF_CONDUCT.md).
