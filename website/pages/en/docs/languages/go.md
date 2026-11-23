---
title: Go
description: Port a Go package to Rust through typed cgo calls and a generated binary wire, in a package with the original API.
---

# Go

| | |
| --- | --- |
| Unit | a package |
| Bridge | typed cgo calls and a generated binary wire, in a package with the original API |
| Recorded and replayed by | a generated Go driver, built as its own module |
| Needs | Go 1.21+ |
| Status | Verified on `path` from Go 1.24: 2,642/2,642 (542 from Go's own test tables), gate passes |

`profile` works through `go test`: `everything-rust profile <repo> -- go test -run X -bench . ./pkg`. Batches are `NewFnBatch(items)`; close them when done.

## What a port has to get right

- `path.Clean`: Go's `lazybuf` reads the byte at the write index after it moves back, not the last kept byte (9 cases, from Go's own test table).
- `strings.ToLower` lowers each rune alone: `ΣΊΣΥΦΟΣ` becomes `σίσυφοσ`, where Rust's `to_lowercase` applies the final-sigma rule. Go also maps `İ` to `i`, where Rust gives `i̇`.
- Go slices strings by bytes; a cut through a rune reaches JSON as one U+FFFD per byte.

## Speed

Go runs leaf functions in a few nanoseconds, and one cgo call alone costs about 80 ns. On `path`, `bench` measured the port at 0.04x per call: port bigger units. As a batch, the work moves to Rust on all cores: `Slugify` over 50,000 strings took 49 ms in Go, and the batch ran **4x faster** (the per-call port: 0.93x). Batch crates allocate with mimalloc: with the system allocator the Go batch was slower than one thread.

Details: [Six languages](/docs/real-world/six-languages).
