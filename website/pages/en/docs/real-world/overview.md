---
title: Real-world ports
description: What happened when everything-rust met real code, with the slow results next to the fast ones, and what changed in the tool because of it.
source: docs/real-world.md
---

# Real-world ports

What happened when everything-rust met real code, and what changed in the tool because of it. Every number below comes from a benchmark that first checks the results are identical, and the slow cases are listed next to the fast ones.

| Port | What it is | Result |
| --- | --- | --- |
| [json-server](/docs/real-world/json-server) | a whole Node server, ported end to end with `app record`, `front-door`, `wire` and `lift` | a pure Rust binary that passes 514/514 recorded HTTP cases; six units pass `gate` |
| [Fuse.js](/docs/real-world/fuse-js) | fuzzy search, the path a search spends its time on | 3,414/3,414 cases match; 32x faster on 50,000 strings (5.3x on one core) |
| [vercel/ms](/docs/real-world/vercel-ms) | a tiny duration parser, end to end | 975/975 cases match; 0.56x per call, 18x as a `parse` batch, 0.8x as a `format` batch |
| [x18n](/docs/real-world/x18n) | the case-study repository | every gate passes on `packages/core`, slower than TS through a bridge; its search 18-22x as a batch |
| [Six languages](/docs/real-world/six-languages) | Go's `path`, Ruby's `shellwords`, Python's `colorsys`, JDK and glibc behavior behind small facades | every port passes its gate, 13,010 recorded cases in all |
| [Fixtures and static runs](/docs/real-world/fixtures) | fixtures written to stress the tool, and libraries analyzed but not run | |

Ported something real? A write-up is welcome: see [Contributing](/docs/project/contributing#share-a-real-port).
