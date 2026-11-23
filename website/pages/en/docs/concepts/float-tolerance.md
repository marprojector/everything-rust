---
title: Float tolerance
description: Numeric code rarely gives bit-identical floats once ported. --float-tolerance lets floats match within a share of the larger value.
---

# Float tolerance

```sh
everything-rust record <crate> --float-tolerance 1e-9
everything-rust app record . --float-tolerance 1e-9 -- python3 cli.py
```

Numeric code (physics, statistics, graphics) rarely gives bit-identical floats once ported: operation order, fused multiply-add and the math library differ between compilers. When you accept that, record with `--float-tolerance` and your number. Floats then match within that share of the larger value (absolute below 1), and `diff` and `gate` use it. `0` removes it.

What still has to match exactly:

- integers, text and structure;
- the same value written differently (`3` and `3.0`);
- formatting. Number-to-text formatting differs between languages (`1e21`, `-0`, digit counts); the traps include those values, and `--float-tolerance` forgives rounding only, never formatting.

Never widen a tolerance to make a mismatch go away: a difference beyond it is a bug in the port.
