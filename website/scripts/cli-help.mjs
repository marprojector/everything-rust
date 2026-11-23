import { spawnSync } from 'node:child_process'
import { existsSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { REPO_ROOT, WEBSITE_ROOT } from './md-include.mjs'

function findBinary() {
  const candidates = [
    process.argv[2],
    process.env.EVERYTHING_RUST_BIN,
    join(REPO_ROOT, 'target', 'release', 'everything-rust'),
    join(REPO_ROOT, 'target', 'debug', 'everything-rust'),
  ].filter(Boolean)
  return candidates.find((c) => existsSync(c)) ?? 'everything-rust'
}

function run(bin, args) {
  const r = spawnSync(bin, args, { encoding: 'utf8', timeout: 20000 })
  if (r.status !== 0) throw new Error(`${bin} ${args.join(' ')} failed: ${r.stderr || r.error}`)
  return r.stdout
}

function subcommands(help) {
  const section = help.split(/^Commands:\n/m)[1]?.split(/\n\n/)[0] ?? ''
  return section
    .split('\n')
    .map((l) => /^ {2}([a-z][a-z-]*)\s/.exec(l)?.[1])
    .filter((c) => c && c !== 'help')
}

const bin = findBinary()
const version = run(bin, ['--version']).trim().replace(/^everything-rust\s+/, '')
const commands = []
function walk(path) {
  const help = run(bin, [...path, '--help'])
  commands.push({ command: path.join(' '), help })
  for (const sub of subcommands(help)) walk([...path, sub])
}
walk([])
const out = join(WEBSITE_ROOT, 'content', 'cli-help.json')
writeFileSync(out, JSON.stringify({ version, commands }, null, 2) + '\n')
console.log(`cli-help: ${commands.length} commands from ${bin} (${version}) -> ${out}`)
