# `SlugifyBatch`, `PageCountBatch` and `WordCountBatch` against the Ruby originals: same results,
# same errors, and the time for 20,000 slugs. Run after `everything-rust build .`: ruby bench_batch.rb
crate = __dir__
fixture = File.expand_path("../../../tests/fixtures/ruby", __dir__)
require File.join(fixture, "lib/textkit")
texts = (0...20_000).map { |i| "Hello, Wörld #{i} -- Ünïcode Title_#{i % 7}! ΣΊΣΥΦΟΣ" }
want = texts.map { |t| Textkit::Words.slugify(t) }
want2 = texts.map { |t| Textkit::Words.slugify(t, "_", max: 12) }
totals = (0...20_000).to_a
want_pages = totals.map { |t| Textkit::Nums.page_count(t, 7) }
want_counts = texts.first(500).map { |t| Textkit::Words.word_count(t) }
t = Process.clock_gettime(Process::CLOCK_MONOTONIC); 5.times { texts.map { |x| Textkit::Words.slugify(x) } }; ruby_ms = (Process.clock_gettime(Process::CLOCK_MONOTONIC) - t) * 200
require File.join(crate, "ruby/textkit_rs")
b = Textkit::Words::SlugifyBatch.new(texts)
raise "slug" unless b.run == want
raise "slug2" unless b.run("_", max: 12) == want2
p = Textkit::Nums::PageCountBatch.new(totals)
raise "pages" unless p.run(7) == want_pages
order = p.rank(3, min: 2000)
expect = totals.select { |x| x.fdiv(1) >= 0 && (Textkit::Nums.page_count(x, 3) rescue -1) >= 2000 }
raise "rank" unless order.map { |i| totals[i] } == expect.sort_by.with_index { |x, i| [-Textkit::Nums.page_count(x, 3), i] }
raise "counts" unless Textkit::Words::WordCountBatch.new(texts.first(500)).run == want_counts
begin; p.run(0); puts "no error?"; rescue => e; puts "batch raises #{e.class}: #{e.message}"; end
t = Process.clock_gettime(Process::CLOCK_MONOTONIC); 5.times { b.run }; rust_ms = (Process.clock_gettime(Process::CLOCK_MONOTONIC) - t) * 200
puts format("slugify x20000: Ruby %.1f ms, batch %.1f ms: %.1fx", ruby_ms, rust_ms, ruby_ms / rust_ms)
