// Loads the wasm-bindgen (nodejs target) output built by `everything-rust build`.
const native = require('./pkg/plugins_rs.js')
module.exports = native
