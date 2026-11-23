// Per-keystroke times for typing a query, the TS pipeline against FuzzyScoreBatch (median of 21
// runs, results checked identical): the numbers in docs/assets/race.gif.
// Run after `everything-rust build .`: node --experimental-strip-types race.ts <x18n checkout>
import { createRequire } from 'node:module'
import { resolve } from 'node:path'
const require = createRequire(import.meta.url)
const x18n = resolve(process.argv[2] ?? '../../../x18n')
const { fuzzyScore } = await import(`${x18n}/packages/core/src/fuzzy.ts`)
const { FuzzyScoreBatch } = require('./index.cjs')
const words = ['app', 'settings', 'title', 'button', 'save', 'cancel', 'error', 'message', 'user', 'profile', 'nav', 'home', 'dashboard', 'form', 'label', 'placeholder', 'tooltip', 'dialog', 'confirm', 'delete']
let seed = 7
const r = (k: number) => (seed = (seed * 1103515245 + 12345) % 2147483648) % k
const keys = Array.from({ length: 50000 }, (_, i) => Array.from({ length: 3 + r(3) }, () => words[r(words.length)]).join('.') + i)
const ts = (q: string) => keys.map(key => ({ key, score: fuzzyScore(q, key) })).filter(m => m.score >= 0).sort((a, b) => b.score - a.score).map(m => m.key)
const batch = new FuzzyScoreBatch(keys)
const rust = (q: string) => { const o = batch.rank(q, 0); const out = new Array(o.length); for (let i = 0; i < o.length; i++) out[i] = keys[o[i]]; return out }
const query = 'user.profile'
const median = (f: () => void) => { const t: number[] = []; for (let i = 0; i < 21; i++) { const s = performance.now(); f(); t.push(performance.now() - s) } return t.sort((a, b) => a - b)[10] }
for (let w = 0; w < 3; w++) for (let i = 1; i <= query.length; i++) { ts(query.slice(0, i)); rust(query.slice(0, i)) }
const rows = []
for (let i = 1; i <= query.length; i++) {
  const q = query.slice(0, i)
  const a = ts(q), b = rust(q)
  if (a.join('\n') !== b.join('\n')) throw new Error('differs at ' + q)
  // Narrowing needs the previous query: time the Rust side as typed (previous prefix first).
  const tsMs = median(() => ts(q))
  const rustMs = median(() => { if (i > 1) batch.rank(query.slice(0, i - 1), 0); const s = performance.now(); rust(q); last = performance.now() - s })
  rows.push({ q, hits: a.length, ts: +tsMs.toFixed(2), rust: +medianLast(q, i).toFixed(3), top: a.slice(0, 5) })
}
var last = 0
function medianLast(q: string, i: number) { const t: number[] = []; for (let k = 0; k < 21; k++) { if (i > 1) batch.rank(query.slice(0, i - 1), 0); else batch.rank('zz', 0); const s = performance.now(); rust(q); t.push(performance.now() - s) } return t.sort((a, b) => a - b)[10] }
console.log(JSON.stringify(rows))
