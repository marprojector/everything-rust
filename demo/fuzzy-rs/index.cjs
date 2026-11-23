// Loads the wasm-bindgen (nodejs target) output built by `everything-rust build`.
const native = require('./pkg/fuzzy_rs.js')
module.exports = native
