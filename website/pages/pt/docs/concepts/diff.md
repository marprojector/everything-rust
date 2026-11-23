---
title: Diff e diferenças conhecidas
description: O diff reproduz cada caso registado contra o Rust. O known-diffs.jsonl regista as diferenças intencionais, e registos desatualizados fazem falhar o gate.
---

# Diff e diferenças conhecidas

```sh
everything-rust build <crate>
everything-rust diff <crate>                     # tem de dar N/N
everything-rust diff <crate> --json
```

O `diff` reproduz o `cases.jsonl` contra o módulo Rust compilado, chamado da mesma forma que os chamadores do original o chamam, e lista as divergências. Uma divergência é uma diferença de comportamento: corrija a migração, nunca o registo.

```
198/200 cases match
- fuzzyScore("b😀", "a😀b😀c"): expected {"ok":1997}, got {"ok":1998}
```

## Diferenças conhecidas

Por vezes a migração deve mesmo diferir: o utilizador quer que corrija um bug do original. Pergunte primeiro e depois adicione uma linha por caso a `<crate>/known-diffs.jsonl`:

**known-diffs.jsonl**
```json
{"fn": "...", "args": [...], "actual": ..., "reason": "..."}
```

Copie `fn`, `args` e `actual` do `diff --json`. Esse caso passa então apenas com exatamente esse resultado, e o `gate` falha assim que deixar de diferir, por isso remova a linha nessa altura.

## Registos desatualizados

O `record` guarda uma impressão digital dos ficheiros da unidade original em `recorded-from.json`. Quando o original muda mais tarde, o `diff` avisa e o `gate` falha até registar de novo, em vez de passar contra o que o original costumava fazer. Se o `gate` disser que o registo está desatualizado, registe de novo antes de continuar a migrar.

## Quando a migração estagna

O `diff` acompanha o progresso entre compilações. Quando indica que a migração estagnou (o seu `trend.stalled`: três alterações seguidas ao `port.rs` não fizeram corresponder nenhum caso novo), pare de editar. As divergências restantes podem precisar de uma regra que só o utilizador conhece, ou essas funções podem ficar no original por agora.

## Tolerância de floats

Se a crate foi registada com `--float-tolerance`, o `diff` utiliza-a; `--float-tolerance` no `diff` substitui-a numa execução. Consulte [Tolerância de floats](/docs/concepts/float-tolerance).
