// Batches against the original Go functions: same results, same errors, and time.
// Run after `everything-rust build ..`: go run .
package main

import (
	"fmt"
	"reflect"
	"sort"
	"time"

	orig "example.com/strkit/strkit"
	rs "everything-rust.local/strkit_rs"
)

func main() {
	texts := make([]string, 50000)
	ports := make([]string, 50000)
	for i := range texts {
		texts[i] = fmt.Sprintf("Hello, Wörld %d -- Ünïcode Title_%d! ΣΊΣΥΦΟΣ  and   more words here", i, i%7)
		ports[i] = fmt.Sprint(i % 70000)
	}
	want := make([]string, len(texts))
	for i, t := range texts {
		want[i] = orig.Slugify(t, 100)
	}
	b := rs.NewSlugifyBatch(texts)
	defer b.Close()
	if got := b.Run(100); !reflect.DeepEqual(got, want) {
		for i := range got {
			if got[i] != want[i] {
				fmt.Printf("%d %q: rust %q go %q per-call %q\n", i, texts[i], got[i], want[i], rs.Slugify(texts[i], 100))
				break
			}
		}
		panic("slugify differs")
	}
	p := rs.NewParsePortBatch(ports)
	if _, err := p.Run(); err == nil || err.Error() != "out of range" {
		panic(fmt.Sprint("expected the first error: ", err))
	}
	q := rs.NewParsePortBatch(ports[1:60000%len(ports)])
	got, err := q.Run()
	for i, s := range ports[1 : 60000%len(ports)] {
		w, _ := orig.ParsePort(s)
		if err != nil || got[i] != w {
			panic("parse differs")
		}
	}
	order, _ := q.Rank(5000)
	var expect []int
	for i, v := range got {
		if v >= 5000 {
			expect = append(expect, i)
		}
	}
	sort.SliceStable(expect, func(a, c int) bool { return got[expect[a]] > got[expect[c]] })
	if len(expect) == 0 || !reflect.DeepEqual(order, expect) {
		fmt.Println(len(order), len(expect), order[:5], expect[:5], len(got))
		panic("rank differs")
	}
	fmt.Println("same results and errors")
	timeit := func(f func()) time.Duration {
		best := time.Duration(1 << 62)
		for r := 0; r < 15; r++ {
			start := time.Now()
			f()
			if d := time.Since(start); d < best {
				best = d
			}
		}
		return best
	}
	g := timeit(func() {
		for i, t := range texts {
			want[i] = orig.Slugify(t, 100)
		}
	})
	r := timeit(func() { b.Run(100) })
	per := timeit(func() {
		for i, t := range texts {
			want[i] = rs.Slugify(t, 100)
		}
	})
	fmt.Printf("Slugify x50000: Go %v, Rust per call %v (%.2fx), Rust batch %v: %.1fx\n", g, per, float64(g)/float64(per), r, float64(g)/float64(r))
}
