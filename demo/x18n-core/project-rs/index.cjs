// Loads the wasm-bindgen (nodejs target) output built by `everything-rust build`.
const native = require('./pkg/project_rs.js')

// Structured values cross as JSON text. JSON has no `undefined`, NaN or -0, and Rust strings no
// lone surrogates: those travel as `{"$undefined": true}`, `{"$num": "NaN"}` and
// `{"$utf16": [...]}` (see `js.rs`), so `{ a: undefined }`, `null` and half an emoji survive.
const replacer = (_, v) => v === undefined ? { $undefined: true }
  : typeof v === 'number' ? (Number.isFinite(v) && !Object.is(v, -0) ? v : { $num: Object.is(v, -0) ? '-0' : String(v) })
  : typeof v === 'string' && !v.isWellFormed() ? { $utf16: Array.from({ length: v.length }, (_, i) => v.charCodeAt(i)) }
  : v
const encode = v => JSON.stringify(v, replacer)
const single = (v, key) => Object.keys(v).length === 1 && key in v
const revive = v => Array.isArray(v) ? v.map(revive)
  : v !== null && typeof v === 'object'
    ? (single(v, '$undefined') ? undefined
      : single(v, '$utf16') ? String.fromCharCode(...v.$utf16)
      : single(v, '$num') ? Number(v.$num)
      : Object.fromEntries(Object.entries(v).map(([k, x]) => [k, revive(x)])))
    : v
// Most results carry no marker: skip the walk unless the text has one.
const decode = text => {
  const value = JSON.parse(text)
  return text.includes('"$') ? revive(value) : value
}
const fromUnits = (units) => {
  let out = ''
  for (let i = 0; i < units.length; i += 8192)
    out += String.fromCharCode.apply(null, Array.prototype.slice.call(units, i, i + 8192))
  return out
}

module.exports = { ...native }
const specs = {
  isOutdated: [[1, 1], 'raw', false],
  valueText: [[1], 'json', false],
  sourceHash: [[1], 'raw', false],
  findReview: [[1, 0], 'json', false],
  sortReviews: [[1], 'json', false],
  prefixMatches: [[1, 0], 'raw', false],
  suppressedRanges: [[0], 'json', false],
  categoriesOf: [[1, 1], 'json', false],
  renderPrompt: [[1], 'json', false],
}
const classes = {

}
const convert = result => result === 'json' ? decode : result === 'units' ? fromUnits : x => x
// An omitted structured argument is `undefined`, not a missing value.
const prepare = (a, structured) => Array.from({ length: Math.max(a.length, structured.length) }, (_, i) => structured[i] ? encode(a[i]) : a[i])
// Functions: [structured arguments, result kind, async].
for (const [name, [structured, result, isAsync]] of Object.entries(specs)) {
  const call = native[name]
  const out = convert(result)
  module.exports[name] = isAsync
    ? async (...a) => out(await call(...prepare(a, structured)))
    : (...a) => out(call(...prepare(a, structured)))
}
// Classes: a JS class around the native one, converting arguments and results.
const INNER = Symbol('native')
for (const [name, [ctor, methods]] of Object.entries(classes)) {
  const Native = native[name]
  const Wrapped = { [name]: class { constructor(...a) { this[INNER] = new Native(...prepare(a, ctor)) } } }[name]
  for (const [method, [structured, result]] of Object.entries(methods)) {
    const out = convert(result)
    Wrapped.prototype[method] = function (...a) { return out(this[INNER][method](...prepare(a, structured))) }
  }
  module.exports[name] = Wrapped
}

// The binary wire (see everything-rust's `wire.rs`): arguments are written into `wIn` and copied into
// wasm memory in one step; results are read where the Rust side left them.
let wIn = new Uint8Array(1 << 16), iv = new DataView(wIn.buffer), at = 0, p = 0
const mem = native.everythingRustWireMemory()
let wOut = new Uint8Array(mem.buffer), ov = new DataView(mem.buffer)
const regrow = () => {}
// Growing wasm memory replaces its buffer: views are made again when that happened.
const views = () => {
  if (wOut.buffer !== mem.buffer) {
    wOut = new Uint8Array(mem.buffer)
    ov = new DataView(mem.buffer)
  }
}
const wireCall = fn => {
  const ptr = native.everythingRustWireIn(at)
  views()
  wOut.set(wIn.subarray(0, at), ptr)
  if (fn(at) === 0xFFFFFFFF) return false
  views()
  p = native.everythingRustWireOut()
  return true
}

const need = n => {
  if (at + n <= wIn.length) return
  const grown = new Uint8Array(Math.max(wIn.length * 2, at + n))
  grown.set(wIn)
  wIn = grown
  iv = new DataView(wIn.buffer)
  regrow()
}
const wU8 = v => { need(1); iv.setUint8(at++, v) }
const wU32 = v => { need(4); iv.setUint32(at, v, true); at += 4 }
const wF64 = v => { need(8); iv.setFloat64(at, v, true); at += 8 }
const wUnits = s => {
  need(4 + 2 * s.length)
  iv.setUint32(at, s.length, true)
  at += 4
  for (let i = 0; i < s.length; i++) { iv.setUint16(at, s.charCodeAt(i), true); at += 2 }
}
const wStr = s => { wU8(4); wUnits(s) }
const wDyn = v => {
  switch (typeof v) {
    case 'undefined': wU8(0); return true
    case 'boolean': wU8(v ? 3 : 2); return true
    case 'number': wU8(4); wF64(v); return true
    case 'string': wU8(5); wUnits(v); return true
    case 'object':
      if (v === null) { wU8(1); return true }
      if (Array.isArray(v)) {
        wU8(6); wU32(v.length)
        for (let i = 0; i < v.length; i++) if (!wDyn(v[i])) return false
        return true
      }
      if (Object.getPrototypeOf(v) !== Object.prototype && Object.getPrototypeOf(v) !== null) return false
      {
        const keys = Object.keys(v)
        wU8(7); wU32(keys.length)
        for (const k of keys) { wUnits(k); if (!wDyn(v[k])) return false }
        return true
      }
  }
  return false
}
const rU8 = () => ov.getUint8(p++)
const rU32 = () => { const v = ov.getUint32(p, true); p += 4; return v }
const rF64 = () => { const v = ov.getFloat64(p, true); p += 8; return v }
// ignoreBOM: a leading U+FEFF is part of the string, not a byte-order mark.
const utf16 = new TextDecoder('utf-16le', { ignoreBOM: true })
const rUnits = (wellFormed) => {
  const n = rU32()
  let s
  if (n > 24 && wellFormed) s = utf16.decode(wOut.subarray(p, p + 2 * n))
  else {
    s = ''
    for (let i = 0; i < n; i++) s += String.fromCharCode(ov.getUint16(p + 2 * i, true))
  }
  p += 2 * n
  return s
}
// Tag 1: a string from Rust (well-formed); 4: raw code units, lone surrogates included.
const rStr = () => rUnits(rU8() === 1)
const rDyn = () => {
  switch (rU8()) {
    case 0: return undefined
    case 1: return null
    case 2: return false
    case 3: return true
    case 4: return rF64()
    case 5: return rUnits(false)
    case 6: { const n = rU32(); const a = new Array(n); for (let i = 0; i < n; i++) a[i] = rDyn(); return a }
    case 7: { const n = rU32(); const o = {}; for (let i = 0; i < n; i++) { const k = rUnits(false); o[k] = rDyn() } return o }
  }
}

// `isOutdated`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isOutdated
const call = native.isOutdatedWire
const encode = (a0, a1) => {
at = 0
if (a0 === undefined) {
wU8(0)
} else if (a0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "updatedAt": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "revision": {
wU8(1)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "author": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "value": {
wU8(3)
const f0 = a0[k0]
if (!wDyn(f0)) return false
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 1) return false
iv.setUint8(at0, n0)
}
}
if (a1 === undefined) {
wU8(0)
} else if (a1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "updatedAt": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "revision": {
wU8(1)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "author": {
wU8(2)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "value": {
wU8(3)
const f0 = a1[k0]
if (!wDyn(f0)) return false
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 1) return false
iv.setUint8(at0, n0)
}
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isOutdated = function isOutdated (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `valueText`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.valueText
const call = native.valueTextWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.valueText = function valueText (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `sourceHash`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.sourceHash
const call = native.sourceHashWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.sourceHash = function sourceHash (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `findReview`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.findReview
const call = native.findReviewWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "threads": {
wU8(0)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const k1 = Object.keys(f0)
wU8(1)
wU32(k1.length)
for (const key1 of k1) {
wStr(key1)
const v1 = f0[key1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const k2 = Object.keys(v1)
wU8(1)
wU32(k2.length)
for (const key2 of k2) {
wStr(key2)
const v2 = v1[key2]
if (!Array.isArray(v2)) return false
wU8(1)
wU32(v2.length)
for (let i3 = 0; i3 < v2.length; i3++) {
const v3 = v2[i3]
if (v3 === null || typeof v3 !== 'object' || Array.isArray(v3)) return false
{
const at4 = at
wU8(0)
let n4 = 0, r4 = 0
for (const k4 in v3) {
switch (k4) {
case "id": {
wU8(0)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "type": {
wU8(1)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "author": {
wU8(2)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "time": {
wU8(3)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "text": {
wU8(4)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "suggestion": {
wU8(5)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "value": {
wU8(6)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "resolved": {
wU8(7)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
default: return false
}
if (++n4 > 255) return false
}
if (r4 !== 0) return false
iv.setUint8(at4, n4)
}
}
}
}
}
}
break
}
case "sources": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const k1 = Object.keys(f0)
wU8(1)
wU32(k1.length)
for (const key1 of k1) {
wStr(key1)
const v1 = f0[key1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const k2 = Object.keys(v1)
wU8(1)
wU32(k2.length)
for (const key2 of k2) {
wStr(key2)
const v2 = v1[key2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
}
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["key"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["locale"] = v1
break
}
case 2: {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["id"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["type"] = v2
break
}
case 2: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rDyn()
o4["name"] = v4
break
}
case 1: {
let v4
v4 = rDyn()
o4["email"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["author"] = v2
break
}
case 3: {
let v2
v2 = rStr()
o2["time"] = v2
break
}
case 4: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["text"] = v2
break
}
case 5: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["suggestion"] = v2
break
}
case 6: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["value"] = v2
break
}
case 7: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["resolved"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
o1["comment"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.findReview = function findReview (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `sortReviews`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.sortReviews
const call = native.sortReviewsWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "threads": {
wU8(0)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const k1 = Object.keys(f0)
wU8(1)
wU32(k1.length)
for (const key1 of k1) {
wStr(key1)
const v1 = f0[key1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const k2 = Object.keys(v1)
wU8(1)
wU32(k2.length)
for (const key2 of k2) {
wStr(key2)
const v2 = v1[key2]
if (!Array.isArray(v2)) return false
wU8(1)
wU32(v2.length)
for (let i3 = 0; i3 < v2.length; i3++) {
const v3 = v2[i3]
if (v3 === null || typeof v3 !== 'object' || Array.isArray(v3)) return false
{
const at4 = at
wU8(0)
let n4 = 0, r4 = 0
for (const k4 in v3) {
switch (k4) {
case "id": {
wU8(0)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "type": {
wU8(1)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "author": {
wU8(2)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "time": {
wU8(3)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "text": {
wU8(4)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "suggestion": {
wU8(5)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "value": {
wU8(6)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
case "resolved": {
wU8(7)
const f4 = v3[k4]
if (!wDyn(f4)) return false
break
}
default: return false
}
if (++n4 > 255) return false
}
if (r4 !== 0) return false
iv.setUint8(at4, n4)
}
}
}
}
}
}
break
}
case "sources": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const k1 = Object.keys(f0)
wU8(1)
wU32(k1.length)
for (const key1 of k1) {
wStr(key1)
const v1 = f0[key1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const k2 = Object.keys(v1)
wU8(1)
wU32(k2.length)
for (const key2 of k2) {
wStr(key2)
const v2 = v1[key2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
}
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const o1 = {}
for (let i1 = 0; i1 < n1; i1++) {
const k1 = rStr()
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const o2 = {}
for (let i2 = 0; i2 < n2; i2++) {
const k2 = rStr()
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rDyn()
o4["id"] = v4
break
}
case 1: {
let v4
v4 = rDyn()
o4["type"] = v4
break
}
case 2: {
let v4
v4 = rDyn()
o4["author"] = v4
break
}
case 3: {
let v4
v4 = rDyn()
o4["time"] = v4
break
}
case 4: {
let v4
v4 = rDyn()
o4["text"] = v4
break
}
case 5: {
let v4
v4 = rDyn()
o4["suggestion"] = v4
break
}
case 6: {
let v4
v4 = rDyn()
o4["value"] = v4
break
}
case 7: {
let v4
v4 = rDyn()
o4["resolved"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v3 = o4
}
a3[i3] = v3
}
v2 = a3
}
o2[k2] = v2
}
v1 = o2
}
o1[k1] = v1
}
v0 = o1
}
o0["threads"] = v0
break
}
case 1: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const o1 = {}
for (let i1 = 0; i1 < n1; i1++) {
const k1 = rStr()
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const o2 = {}
for (let i2 = 0; i2 < n2; i2++) {
const k2 = rStr()
let v2
v2 = rStr()
o2[k2] = v2
}
v1 = o2
}
o1[k1] = v1
}
v0 = o1
}
o0["sources"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.sortReviews = function sortReviews (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `prefixMatches`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.prefixMatches
const call = native.prefixMatchesWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "key": {
wU8(0)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "raw": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "path": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "range": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "start": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "end": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
break
}
case "expression": {
wU8(4)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "start": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "end": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
break
}
case "scanner": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "kind": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "dynamic": {
wU8(7)
const f0 = a0[k0]
if (!wDyn(f0)) return false
break
}
case "locale": {
wU8(8)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "scopeNamespace": {
wU8(9)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "scopePrefix": {
wU8(10)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 5) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.prefixMatches = function prefixMatches (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `suppressedRanges`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.suppressedRanges
const call = native.suppressedRangesWire
const encode = (a0) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rF64()
o1["start"] = v1
break
}
case 1: {
let v1
v1 = rF64()
o1["end"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.suppressedRanges = function suppressedRanges (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `categoriesOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.categoriesOf
const call = native.categoriesOfWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "code": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "language": {
wU8(1)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "region": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "displayName": {
wU8(3)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "flag": {
wU8(4)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "pluralCategories": {
wU8(5)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (!wDyn(v1)) return false
}
break
}
case "ordinalCategories": {
wU8(6)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (!wDyn(v1)) return false
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 3) return false
iv.setUint8(at0, n0)
}
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
v0 = rDyn()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.categoriesOf = function categoriesOf (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `renderPrompt`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.renderPrompt
const call = native.renderPromptWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "kind": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "project": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "from": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "to": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "targetName": {
wU8(4)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "total": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "remaining": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "nextCursor": {
wU8(7)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "rules": {
wU8(8)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
case "notes": {
wU8(9)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
case "glossary": {
wU8(10)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "term": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "translation": {
wU8(1)
const f2 = v1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "note": {
wU8(2)
const f2 = v1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "doNotTranslate": {
wU8(3)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
case "examples": {
wU8(11)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "key": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "source": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "target": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "items": {
wU8(12)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "key": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "source": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "placeholders": {
wU8(2)
r2++
const f2 = v1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (typeof v3 !== 'string') return false
wStr(v3)
}
break
}
case "existing": {
wU8(3)
const f2 = v1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "previousSource": {
wU8(4)
const f2 = v1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "context": {
wU8(5)
r2++
const f2 = v1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "instructions": {
wU8(13)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 12) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.renderPrompt = function renderPrompt (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}
