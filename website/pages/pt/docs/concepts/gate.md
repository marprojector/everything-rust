---
title: Gate
description: O gate é a definição de concluído para uma migração. Todas as verificações que executa e como corrigir uma que falhe.
---

# Gate

```sh
everything-rust gate <crate>
```

Concluído significa que o `gate` passa. Verifica:

- `cargo fmt`, clippy com `-D warnings` e os testes do cargo;
- `unsafe` apenas com comentários `// SAFETY:`, e nada `unimplemented!`;
- um caso de falha registado para cada sítio onde o original falha de propósito (`throw`, `raise`, um resultado `error`). Adicione um com `record --inputs` quando o gate o indicar;
- nenhum `unwrap`, `expect` ou `panic!` numa função cujo original devolve um erro. Devolva `Err`; para um estado que o original também trata como impossível, um comentário `// INFALLIBLE: <why>`;
- a migração com no máximo 3x as linhas do original (mais 50 linhas para imports e assinaturas);
- o módulo compila e todos os exports estão presentes;
- todos os casos registados correspondem, [diferenças conhecidas](/docs/concepts/diff#known-diffs) incluídas, e o registo não está [desatualizado](/docs/concepts/diff#stale-recordings);
- para TS/JS, uma linha de [cobertura](/docs/concepts/coverage): quanto do original os casos executaram.

Cada verificação falhada traz um `fix`: execute-o primeiro.

## Depois, reportar e esperar

Quando o gate passa, a skill do fluxo de trabalho faz com que o agente reporte o que mudou (ficheiros, funções, bridge, lotes), o output do gate, as contagens do diff e os números do bench, as diferenças de comportamento ou riscos que restam e o que falta no plano, e espere pela autorização antes da unidade seguinte.
