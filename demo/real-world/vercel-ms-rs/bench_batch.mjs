// ParseBatch and FormatBatch against vercel/ms: same results first, then the time for 100,000
// values. Run after `everything-rust build .`: node bench_batch.mjs
import { createRequire } from 'node:module'
import { parse, format } from '../../src/index.ts'
const { ParseBatch, FormatBatch } = createRequire(import.meta.url)('./index.cjs')

const units = ['ms', 's', 'sec', 'm', 'min', 'h', 'hrs', 'd', 'days', 'w', 'weeks', 'mo', 'y', 'years', '']
const n = 100_000
let seed = 7
const rand = () => (seed = (seed * 1103515245 + 12345) % 2 ** 31) / 2 ** 31
const texts = Array.from({ length: n }, () => {
  const v = (rand() * 1000 - 100).toFixed(rand() < 0.5 ? 0 : 2)
  return `${v}${rand() < 0.5 ? ' ' : ''}${units[Math.floor(rand() * units.length)]}`
})
const numbers = Array.from({ length: n }, () => Math.round((rand() - 0.2) * 1e10))

const same = (a, b) => a.length === b.length && a.every((x, i) => Object.is(x, b[i]))
const best = (f, runs = 15) => {
  let t = Infinity
  for (let i = 0; i < runs; i++) {
    const s = performance.now()
    f()
    t = Math.min(t, performance.now() - s)
  }
  return t
}

const pb = new ParseBatch(texts)
if (!same(texts.map(parse), Array.from(pb.run()))) throw new Error('parse results differ')
const fb = new FormatBatch(numbers)
for (const o of [undefined, { long: true }]) {
  if (!same(numbers.map(x => format(x, o)), fb.run(o))) throw new Error('format results differ')
}

const row = (name, js, once, run) =>
  console.log(`${name.padEnd(22)} JS ${js.toFixed(2)} ms · batch ${run.toFixed(2)} ms (${(js / run).toFixed(1)}x) · with handover ${(once + run).toFixed(2)} ms (${(js / (once + run)).toFixed(1)}x)`)
row('parse x100,000', best(() => texts.map(parse)), best(() => new ParseBatch(texts)), best(() => pb.run()))
row('format x100,000', best(() => numbers.map(x => format(x))), best(() => new FormatBatch(numbers)), best(() => fb.run()))
row('format long x100,000', best(() => numbers.map(x => format(x, { long: true }))), best(() => new FormatBatch(numbers)), best(() => fb.run({ long: true })))
