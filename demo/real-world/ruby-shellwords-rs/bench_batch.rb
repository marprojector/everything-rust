# `ShellescapeBatch` against Ruby's `Shellwords.shellescape` over 50,000 values of mixed types: same
# results, and the time. Run after `everything-rust build .`: ruby bench_batch.rb
require "shellwords"
words = (0...50_000).map { |i| ["plain#{i}", "it's #{i}", "", "a b\nc", "日本#{i}", i, 1.5, nil, :sym][i % 9] }
want = words.map { |w| Shellwords.shellescape(w) }
t = Process.clock_gettime(Process::CLOCK_MONOTONIC); 5.times { words.map { |w| Shellwords.shellescape(w) } }; a = (Process.clock_gettime(Process::CLOCK_MONOTONIC) - t) * 200
require File.join(__dir__, "ruby/shellwords_rs")
b = Shellwords::ShellescapeBatch.new(words)
raise "differs" unless b.run == want
t = Process.clock_gettime(Process::CLOCK_MONOTONIC); 5.times { b.run }; c = (Process.clock_gettime(Process::CLOCK_MONOTONIC) - t) * 200
puts format("shellescape x50000 (mixed types): Ruby %.1f ms, batch %.1f ms: %.1fx", a, c, a / c)
