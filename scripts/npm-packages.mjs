import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const [version, binDirArg, outDirArg] = process.argv.slice(2)
if (!version || !binDirArg || !outDirArg) {
  console.error('usage: node scripts/npm-packages.mjs <version> <binaries dir> <out dir>')
  process.exit(2)
}
const binDir = path.resolve(binDirArg)
const outDir = path.resolve(outDirArg)
const root = path.join(path.dirname(fileURLToPath(import.meta.url)), '..')
const cargo = fs.readFileSync(path.join(root, 'Cargo.toml'), 'utf8')
const cargoVersion = cargo.match(/^version = "(.*)"/m)[1]
if (cargoVersion !== version) {
  console.error(`version ${version} does not match Cargo.toml ${cargoVersion}`)
  process.exit(1)
}

const platforms = ['darwin-arm64', 'darwin-x64', 'linux-arm64', 'linux-x64', 'win32-x64']

fs.rmSync(outDir, { recursive: true, force: true })
const pkgDir = path.join(outDir, 'everything-rust')
fs.cpSync(path.join(root, 'npm/everything-rust'), pkgDir, { recursive: true })
for (const key of platforms) {
  const exe = key.startsWith('win32') ? 'everything-rust.exe' : 'everything-rust'
  const from = path.join(binDir, `npm-${key}`, exe)
  if (!fs.existsSync(from)) {
    console.error(`missing binary for ${key}: ${from}`)
    process.exit(1)
  }
  const to = path.join(pkgDir, 'binaries', key, exe)
  fs.mkdirSync(path.dirname(to), { recursive: true })
  fs.copyFileSync(from, to)
  fs.chmodSync(to, 0o755)
}
const pkg = JSON.parse(fs.readFileSync(path.join(pkgDir, 'package.json'), 'utf8'))
pkg.version = version
fs.writeFileSync(path.join(pkgDir, 'package.json'), JSON.stringify(pkg, null, 2) + '\n')
fs.copyFileSync(path.join(root, 'README.md'), path.join(pkgDir, 'README.md'))
fs.copyFileSync(path.join(root, 'LICENSE'), path.join(pkgDir, 'LICENSE'))

console.log(pkgDir)
