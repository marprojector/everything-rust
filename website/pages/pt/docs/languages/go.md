---
title: Go
description: Migre um pacote Go para Rust através de chamadas cgo tipadas e de um formato binário gerado, num pacote com a API original.
---

# Go

| | |
| --- | --- |
| Unidade | um pacote |
| Bridge | chamadas cgo tipadas e um formato binário gerado, num pacote com a API original |
| Registado e reproduzido por | um driver Go gerado, compilado como módulo próprio |
| Requisitos | Go 1.21+ |
| Estado | Verificado em `path` do Go 1.24: 2,642/2,642 (542 das próprias tabelas de testes do Go), o gate passa |

O `profile` funciona através de `go test`: `everything-rust profile <repo> -- go test -run X -bench . ./pkg`. Os lotes são `NewFnBatch(items)`; feche-os quando terminar.

## O que uma migração tem de acertar

- `path.Clean`: o `lazybuf` do Go lê o byte no índice de escrita depois de este recuar, não o último byte mantido (9 casos, da própria tabela de testes do Go).
- `strings.ToLower` converte cada rune isoladamente: `ΣΊΣΥΦΟΣ` passa a `σίσυφοσ`, enquanto o `to_lowercase` do Rust aplica a regra do sigma final. O Go também converte `İ` em `i`, enquanto o Rust dá `i̇`.
- O Go corta strings por bytes; um corte a meio de uma rune chega ao JSON como um U+FFFD por byte.

## Velocidade

O Go executa funções folha em poucos nanossegundos, e uma única chamada cgo custa cerca de 80 ns. Em `path`, o `bench` mediu a migração a 0.04x por chamada: migre unidades maiores. Em lote, o trabalho passa para Rust em todos os núcleos: `Slugify` sobre 50,000 strings demorou 49 ms em Go, e o lote correu **4x mais rápido** (a migração por chamada: 0.93x). As crates de lote alocam com mimalloc: com o alocador do sistema, o lote em Go era mais lento do que uma única thread.

Detalhes: [Seis linguagens](/docs/real-world/six-languages).
