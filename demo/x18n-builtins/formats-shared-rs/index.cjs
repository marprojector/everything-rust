// Loads the wasm-bindgen (nodejs target) output built by `everything-rust build`.
const native = require('./pkg/shared_rs.js')

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
  indentUnit: [[0, 1, 0], 'raw', false],
  leafText: [[1], 'json', false],
  splice: [[0, 1, 0], 'json', false],
  splitLines: [[0], 'json', false],
  withBlankLines: [[0, 1], 'json', false],
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

// `indentUnit`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.indentUnit
const call = native.indentUnitWire
const encode = (a0, a1, a2) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (!wDyn(a1)) return false
if (typeof a2 !== 'string') return false
wStr(a2)
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.indentUnit = function indentUnit (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `leafText`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.leafText
const call = native.leafTextWire
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
module.exports.leafText = function leafText (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `splice`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.splice
const call = native.spliceWire
const encode = (a0, a1, a2) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "start": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "end": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
if (typeof a2 !== 'string') return false
wStr(a2)
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.splice = function splice (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `splitLines`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.splitLines
const call = native.splitLinesWire
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
case 2: {
let v1
v1 = rF64()
o1["next"] = v1
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
module.exports.splitLines = function splitLines (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `withBlankLines`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.withBlankLines
const call = native.withBlankLinesWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "start": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "end": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
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
v0 = rF64()
o0["start"] = v0
break
}
case 1: {
let v0
v0 = rF64()
o0["end"] = v0
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
module.exports.withBlankLines = function withBlankLines (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}
