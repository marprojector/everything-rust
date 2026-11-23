---
title: 浮點容差
description: 數值程式碼在移植後很少能得到逐位元相同的浮點數。--float-tolerance 讓浮點數在較大值的某個比例內即視為一致。
---

# 浮點容差

```sh
everything-rust record <crate> --float-tolerance 1e-9
everything-rust app record . --float-tolerance 1e-9 -- python3 cli.py
```

數值程式碼（物理、統計、圖學）在移植後很少能得到逐位元相同的浮點數：運算順序、融合乘加（fused multiply-add）與數學函式庫在不同編譯器之間都有差異。當你接受這一點時，請以 `--float-tolerance` 加上你的數值來錄製。之後浮點數只要在較大值的該比例內即視為一致（小於 1 時為絕對值），`diff` 與 `gate` 都會使用它。設為 `0` 則移除容差。

仍然必須完全一致的部分：

- 整數、文字與結構；
- 同一個值的不同寫法（`3` 與 `3.0`）；
- 格式化。數字轉文字的格式在各語言之間有差異（`1e21`、`-0`、位數）；陷阱案例包含這些值，而 `--float-tolerance` 只原諒捨入誤差，從不原諒格式差異。

絕對不要為了讓不一致消失而放寬容差：超出容差的差異就是移植版本的錯誤。
