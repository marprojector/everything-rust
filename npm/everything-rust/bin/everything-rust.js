#!/usr/bin/env node
'use strict'
const fs = require('node:fs')
const path = require('node:path')
const { spawnSync } = require('node:child_process')

const key = `${process.platform}-${process.arch}`
const exe = process.platform === 'win32' ? 'everything-rust.exe' : 'everything-rust'
const bin = path.join(__dirname, '..', 'binaries', key, exe)
if (!fs.existsSync(bin)) {
  console.error(
    `everything-rust: no prebuilt binary for ${key}.\n` +
      'Install it with Cargo instead: cargo install everything-rust',
  )
  process.exit(1)
}

const result = spawnSync(bin, process.argv.slice(2), { stdio: 'inherit' })
if (result.error) {
  console.error(`everything-rust: ${result.error.message}`)
  process.exit(1)
}
if (result.signal) {
  process.kill(process.pid, result.signal)
}
process.exit(result.status ?? 1)
