---
title: Configurar o seu agente
description: O comando everything-rust setup regista o servidor MCP e a skill do fluxo de trabalho no Claude Code, Codex, Cursor e Gemini CLI.
---

# Configurar o seu agente

```sh
everything-rust setup
```

O `setup` adiciona o servidor MCP a cada agente que encontrar e instala a skill do fluxo de trabalho no local onde o agente lê as skills. Mantém as suas outras definições, pode ser executado novamente sem risco, e `--remove` anula as alterações.

## Compatibilidade

| Agente | O que `everything-rust setup` adiciona |
| --- | --- |
| Claude Code | servidor MCP em `~/.claude.json`, skill em `~/.claude/skills/` |
| Codex | servidor MCP em `~/.codex/config.toml`, skill em `~/.codex/skills/` |
| Cursor | servidor MCP em `~/.cursor/mcp.json` |
| Gemini CLI | servidor MCP em `~/.gemini/settings.json` |
| Qualquer outro cliente MCP | adicione `everything-rust mcp` manualmente como servidor stdio (ver abaixo) |

Antes de alterar um ficheiro existente, o `setup` guarda uma cópia ao lado dele (`<file>.everything-rust.bak`). O `setup --remove` remove apenas as entradas do everything-rust; as cópias guardadas ficam até as apagar.

## O servidor MCP

O `everything-rust mcp` disponibiliza cada etapa como uma ferramenta MCP via stdio: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate` e, para uma aplicação completa, `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Cada ferramenta, os seus parâmetros e aquilo em que pode mexer: [Ferramentas MCP](/docs/reference/mcp-tools). O servidor oferece também dois prompts, `port_unit` e `port_whole_app`, construídos a partir da skill.

Para adicionar o servidor MCP manualmente a um único projeto:

**.mcp.json**
```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

## A skill

A skill do fluxo de trabalho ([`skills/everything-rust/SKILL.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/SKILL.md)) indica ao agente como executar o ciclo: fazer o profiling, planear e parar para aprovação, migrar uma unidade, prová-la, entregá-la atrás de um shadow e só a dar como concluída quando o `gate` passar. Para uma aplicação completa, segue o [`whole-app.md`](https://github.com/marprojector/everything-rust/blob/main/skills/everything-rust/whole-app.md); consulte [Migrar uma aplicação completa](/docs/whole-apps/overview).

## Plugin do Claude Code

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

O plugin traz o servidor MCP e a skill; a CLI tem de estar no seu `PATH`.
