import { existsSync, readFileSync, statSync } from 'node:fs'
import { join, posix, resolve } from 'node:path'

export const WEBSITE_ROOT = resolve(import.meta.dirname, '..')
export const REPO_ROOT = resolve(WEBSITE_ROOT, '..')
const REPO_URL = 'https://github.com/marprojector/everything-rust'
const INCLUDE_RE = /^<!--\s*include:\s*(\S+)\s*-->\s*$/gm

const LINK_MAP = {
  'README.md': '/',
  'README.md#quick-start': '/docs/introduction/quick-start',
  'docs/install.md#install': '/docs/introduction/install',
  'README.md#works-with': '/docs/introduction/agents',
  'README.md#languages': '/docs/languages/overview',
  'docs/how-it-works.md#how-it-works': '/docs/concepts/the-loop',
  'docs/how-it-works.md#the-loop': '/docs/concepts/the-loop',
  'docs/whole-apps.md#port-a-whole-app': '/docs/whole-apps/overview',
  'docs/whole-apps.md#servers': '/docs/whole-apps/servers',
  'docs/whole-apps.md#libraries': '/docs/whole-apps/libraries',
  'docs/how-it-works.md#hot-paths-keep-the-data-in-rust': '/docs/concepts/batches',
  'docs/how-it-works.md#switch-a-port-on-and-off': '/docs/concepts/switch',
  'docs/how-it-works.md#one-build-cache': '/docs/concepts/switch',
  'docs/how-it-works.md#what-it-cant-catch': '/docs/concepts/limits',
  'docs/how-it-works.md#what-bridge-decides': '/docs/concepts/bridge',
  'docs/how-it-works.md#what-analyze-looks-at': '/docs/concepts/analyze',
  'docs/faq.md#faq': '/docs/project/faq',
  'docs/how-it-works.md': '/docs/concepts/the-loop',
  'docs/whole-apps.md': '/docs/whole-apps/overview',
  'docs/install.md': '/docs/introduction/install',
  'docs/faq.md': '/docs/project/faq',
  'docs/examples.md': '/docs/concepts/port',
  'CONTRIBUTING.md': '/docs/project/contributing',
  'docs/STATUS.md': '/docs/project/status',
  'docs/tools.md': '/docs/reference/mcp-tools',
  'docs/real-world.md': '/docs/real-world/overview',
  'docs/real-world.md#x18n-the-case-study-repository': '/docs/real-world/x18n',
  'docs/real-world.md#vercelms': '/docs/real-world/vercel-ms',
  'docs/real-world.md#fusejs': '/docs/real-world/fuse-js',
  'docs/real-world.md#json-server': '/docs/real-world/json-server',
  'docs/real-world.md#fixtures-written-to-stress-the-tool': '/docs/real-world/fixtures',
  'docs/real-world.md#external-libraries-static-only': '/docs/real-world/fixtures',
  'docs/real-world-languages.md': '/docs/real-world/six-languages',
}

export function githubSlug(text) {
  return text
    .trim()
    .toLowerCase()
    .replace(/<[^>]+>/g, '')
    .replace(/[^\p{L}\p{N}\s_-]/gu, '')
    .replace(/\s/g, '-')
}

function headingOf(line) {
  const m = /^(#{1,6})\s+(.+?)\s*#*\s*$/.exec(line)
  return m ? { level: m[1].length, text: m[2] } : null
}

function scanLines(markdown) {
  const out = []
  let fence = null
  for (const line of markdown.split('\n')) {
    const f = /^\s{0,3}(`{3,}|~{3,})/.exec(line)
    if (f) {
      if (!fence) fence = f[1]
      else if (line.trim().startsWith(fence)) fence = null
      out.push({ line, heading: null })
      continue
    }
    out.push({ line, heading: fence ? null : headingOf(line) })
  }
  return out
}

function extract(markdown, anchor) {
  const lines = scanLines(markdown)
  if (!anchor) {
    const h1 = lines.findIndex((l) => l.heading?.level === 1)
    return h1 === -1 ? lines : lines.slice(h1 + 1)
  }
  const start = lines.findIndex((l) => l.heading && githubSlug(l.heading.text) === anchor)
  if (start === -1) throw new Error(`md-include: no heading #${anchor}`)
  const level = lines[start].heading.level
  let end = lines.findIndex((l, i) => i > start && l.heading && l.heading.level <= level)
  if (end === -1) end = lines.length
  return lines.slice(start + 1, end)
}

function shiftHeadings(lines) {
  const levels = lines.filter((l) => l.heading).map((l) => l.heading.level)
  if (!levels.length) return lines.map((l) => l.line)
  const delta = 2 - Math.min(...levels)
  return lines.map((l) =>
    l.heading && delta !== 0
      ? '#'.repeat(Math.min(6, l.heading.level + delta)) + ' ' + l.heading.text
      : l.line,
  )
}

export function rewriteTarget(target, sourceFile, image = false) {
  if (/^[a-z][a-z0-9+.-]*:/i.test(target) || target.startsWith('//') || target.startsWith('/')) {
    return target
  }
  const [p, hash] = target.split('#')
  let repoPath = p ? posix.normalize(posix.join(posix.dirname(sourceFile), p)) : sourceFile
  repoPath = repoPath.replace(/^\.\//, '').replace(/\/$/, '')
  if (repoPath.startsWith('..')) return target
  if (!image) {
    const key = hash ? `${repoPath}#${hash}` : repoPath
    if (LINK_MAP[key]) return LINK_MAP[key]
    if (!hash && LINK_MAP[repoPath]) return LINK_MAP[repoPath]
    if (hash && sourceFile === repoPath && !p) return `#${hash}`
  }
  if (image) return `https://raw.githubusercontent.com/marprojector/everything-rust/main/${repoPath}`
  const abs = join(REPO_ROOT, repoPath)
  const kind = existsSync(abs) && statSync(abs).isDirectory() ? 'tree' : 'blob'
  return `${REPO_URL}/${kind}/main/${repoPath}${hash ? '#' + hash : ''}`
}

function rewriteLinks(markdown, sourceFile) {
  return markdown
    .replace(/(!?)\[([^\]]*)\]\(([^)\s]+)((?:\s+"[^"]*")?)\)/g, (_, bang, text, target, title) =>
      `${bang}[${text}](${rewriteTarget(target, sourceFile, bang === '!')}${title})`,
    )
    .replace(/\b(href|src)="([^"]+)"/g, (_, attr, target) =>
      `${attr}="${rewriteTarget(target, sourceFile, attr === 'src')}"`,
    )
}

function readCliHelp() {
  return JSON.parse(readFileSync(join(WEBSITE_ROOT, 'content', 'cli-help.json'), 'utf8'))
}

export function renderCliHelp() {
  const { version, commands } = readCliHelp()
  const parts = [
    `Generated from \`everything-rust --help\` and each subcommand's \`--help\` (everything-rust ${version}).`,
  ]
  for (const { command, help } of commands) {
    const title = command ? `everything-rust ${command}` : 'everything-rust'
    parts.push(`## \`${title}\``, '```text\n' + help.trimEnd() + '\n```')
  }
  return parts.join('\n\n')
}

export function includeFile(spec) {
  if (spec === 'cli-help') return renderCliHelp()
  const [file, anchor] = spec.split('#')
  const source = readFileSync(join(REPO_ROOT, file), 'utf8').replace(/\r\n/g, '\n')
  const body = shiftHeadings(extract(source, anchor)).join('\n').trim()
  return rewriteLinks(body, file)
}

export function includedFiles(markdown) {
  return [...markdown.matchAll(INCLUDE_RE)]
    .map((m) => m[1])
    .filter((s) => s !== 'cli-help')
    .map((s) => join(REPO_ROOT, s.split('#')[0]))
}

export function expandIncludes(markdown) {
  return markdown.replace(INCLUDE_RE, (_, spec) => includeFile(spec))
}

const escapeHtml = (s) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;')

export function transformTabs(markdown) {
  let n = 0
  return markdown.replace(/^::: tabs\n([\s\S]*?)\n:::$/gm, (_, inner) => {
    const id = `tabs-${++n}`
    const panes = inner.split(/^== (.+)$/m).slice(1)
    const names = panes.filter((_, i) => i % 2 === 0).map((t) => t.trim())
    const bodies = panes.filter((_, i) => i % 2 === 1)
    const controls = names
      .map((name, i) => {
        const key = `${id}-${i}`
        return `<input type="radio" class="tabs-input" name="${id}" id="${key}"${i === 0 ? ' checked' : ''}><label for="${key}">${escapeHtml(name)}</label>`
      })
      .join('')
    const panels = bodies.map((b) => `<div class="tabs-panel">\n\n${b.trim()}\n\n</div>`).join('\n')
    return `<div class="tabs">${controls}\n${panels}\n</div>`
  })
}

export function markCodeFilenames(markdown) {
  return markdown.replace(
    /^\*\*([^*\n]+)\*\*\n(?=(`{3,}|~{3,}))/gm,
    (_, name) => `<div class="code-filename">${escapeHtml(name.trim())}</div>\n\n`,
  )
}

export function mdIncludePlugin() {
  return {
    name: 'everything-rust-md-include',
    enforce: 'pre',
    transform(code, id) {
      if (!id.endsWith('.md')) return
      INCLUDE_RE.lastIndex = 0
      for (const f of includedFiles(code)) this.addWatchFile(f)
      const loc = /\/pages\/([a-z]{2}(?:-[a-z]{2})?)\//.exec(id.replace(/\\/g, '/'))?.[1]
      let out = markCodeFilenames(transformTabs(expandIncludes(code)))
      if (loc && loc !== 'en') {
        out = out
          .replace(/\]\(\/docs\//g, `](/${loc}/docs/`)
          .replace(/href="\/docs\//g, `href="/${loc}/docs/`)
      }
      return out === code ? undefined : { code: out, map: null }
    },
  }
}
