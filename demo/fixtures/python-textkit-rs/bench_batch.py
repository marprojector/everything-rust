# `SlugifyBatch` and `PageCountBatch` against the Python originals: same results, same
# exceptions, and the time for 20,000 slugs. Run after `everything-rust build .`: python3 bench_batch.py
import sys, time
import os
here = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, here + "/python"); sys.path.insert(0, os.path.join(here, "../../../tests/fixtures/python/src"))
from textkit_rs import SlugifyBatch, PageCountBatch
from textkit.words import slugify, page_count
texts = [f"Hello, Wörld {i} -- Ünïcode Title_{i % 7}!" for i in range(20000)]
b = SlugifyBatch(texts)
assert b.run("-", None) == [slugify(t) for t in texts]
assert b.run("_", 12) == [slugify(t, "_", max_length=12) for t in texts]
p = PageCountBatch(list(range(0, 20000)))
assert list(p.run(7)) == [page_count(t, 7) for t in range(20000)], "page"
order = list(p.rank(3, 2000))
want = sorted([i for i in range(20000) if page_count(i, 3) >= 2000], key=lambda i: -page_count(i, 3))
assert order == want, (order[:5], want[:5])
try:
    p.run(0); print("no error?")
except Exception as e: print("error as in python:", type(e).__name__, e)
try:
    page_count(1, 0)
except Exception as e: print("python:", type(e).__name__, e)
t=time.perf_counter(); [slugify(x) for x in texts]; a=time.perf_counter()-t
t=time.perf_counter(); b.run("-", None); c=time.perf_counter()-t
print(f"slugify x20000: python {a*1e3:.1f} ms, batch {c*1e3:.1f} ms: {a/c:.1f}x")
