import { mkdirSync, readFileSync, writeFileSync } from 'node:fs'
import { basename, dirname, join } from 'node:path'
import { createRequire } from 'node:module'
import satori from 'satori'
import { Resvg, initWasm } from '@resvg/resvg-wasm'
import { REPO_ROOT, WEBSITE_ROOT } from './md-include.mjs'
import { docPages } from './pages.mjs'

const require = createRequire(import.meta.url)
const LOGO_FILE = join(REPO_ROOT, 'docs', 'assets', 'icon.svg')
const OG_LOGO = join(REPO_ROOT, 'website', 'public', 'icon-192.png')
function isClientDir(ctx, options) {
  if (!options.dir) return false
  const env = ctx.environment?.name
  return env ? env === 'client' : basename(options.dir) === 'client'
}

export function faviconPlugin() {
  const serve = (server) => {
    server.middlewares.use((req, res, next) => {
      if (req.url?.split('?')[0] !== '/favicon.svg') return next()
      res.setHeader('Content-Type', 'image/svg+xml')
      res.end(readFileSync(LOGO_FILE))
    })
  }
  return {
    name: 'everything-rust-favicon',
    configureServer: serve,
    configurePreviewServer: serve,
    writeBundle(options) {
      if (!isClientDir(this, options)) return
      writeFileSync(join(options.dir, 'favicon.svg'), readFileSync(LOGO_FILE))
    },
  }
}

const LOCALE_CODES = ['en', 'tr', 'zh', 'zh-tw', 'de', 'fr', 'ru', 'uk', 'ja', 'ko', 'az', 'uz', 'kk', 'ky', 'tk', 'tt', 'ug', 'pt']

function searchEntries(locale = 'en') {
  const prefix = locale === 'en' ? '/docs/' : `/${locale}/docs/`
  return docPages(locale)
    .filter((p) => p.route.startsWith(prefix))
    .map(({ route, title, description, headings, body }) => ({
      href: route,
      title,
      description,
      headings: headings.map(({ text, slug }) => ({ text, slug })),
      body,
    }))
}

export function searchIndexPlugin() {
  return {
    name: 'everything-rust-search-index',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        const m = /^\/search-index\.([a-z-]+)\.json$/.exec(req.url?.split('?')[0] ?? '')
        if (!m || !LOCALE_CODES.includes(m[1])) return next()
        res.setHeader('Content-Type', 'application/json')
        res.end(JSON.stringify(searchEntries(m[1])))
      })
    },
    writeBundle(options) {
      if (!isClientDir(this, options)) return
      let total = 0
      for (const loc of LOCALE_CODES) {
        const entries = searchEntries(loc)
        total += entries.length
        writeFileSync(join(options.dir, `search-index.${loc}.json`), JSON.stringify(entries))
      }
      console.log(`search-index: ${total} pages`)
    },
  }
}

let wasmReady
function fonts() {
  const file = (family, w) =>
    readFileSync(require.resolve(`@fontsource/${family}/files/${family}-latin-${w}-normal.woff`))
  return [
    { name: 'Plex', data: file('ibm-plex-sans', 500), weight: 500, style: 'normal' },
    { name: 'PlexCond', data: file('ibm-plex-sans-condensed', 600), weight: 600, style: 'normal' },
  ]
}

function h(type, style, ...kids) {
  const children = kids.filter((c) => c != null)
  const props = { style: { display: 'flex', ...style } }
  if (type === 'img') return { type, props: { ...style, style: { display: 'flex' } } }
  return { type, props: { ...props, children: children.length === 1 ? children[0] : children } }
}

export async function renderOg({ title, eyebrow }) {
  wasmReady ??= initWasm(readFileSync(require.resolve('@resvg/resvg-wasm/index_bg.wasm')))
  await wasmReady
  const logo = `data:image/png;base64,${readFileSync(OG_LOGO).toString('base64')}`
  const tree = h(
    'div',
    {
      width: 1040,
      height: 474,
      flexDirection: 'column',
      justifyContent: 'space-between',
      backgroundColor: '#f2f3ef',
      color: '#171b1e',
      padding: '72px 80px',
      fontFamily: 'Plex',
      borderLeft: '16px solid #a33a0f',
    },
    h(
      'div',
      { alignItems: 'center', gap: '24px' },
      h('img', { src: logo, width: 88, height: 88 }),
      h('div', { fontSize: 36, fontWeight: 600, fontFamily: 'PlexCond' }, 'everything-rust'),
    ),
    h(
      'div',
      { flexDirection: 'column', gap: '20px' },
      eyebrow ? h('div', { fontSize: 30, fontWeight: 500, color: '#a33a0f' }, eyebrow) : null,
      h('div', { fontFamily: 'PlexCond', fontSize: title.length > 40 ? 70 : 88, fontWeight: 600, lineHeight: 1.05, letterSpacing: '-0.01em' }, title),
    ),
    h(
      'div',
      { fontSize: 28, fontWeight: 500, color: '#596168' },
      'Rewrite your app in Rust, checked against the original at every step.',
    ),
  )
  const svg = await satori(tree, { width: 1200, height: 630, fonts: fonts() })
  return new Resvg(svg).render().asPng()
}

export function ogImagePlugin(groupOf) {
  return {
    name: 'everything-rust-og-images',
    apply: 'build',
    async writeBundle(options) {
      if (!isClientDir(this, options)) return
      const pages = [{ route: '/', title: 'Rewrite your whole app in Rust', eyebrow: '' }]
      for (const p of docPages()) {
        if (p.route.startsWith('/docs/')) pages.push({ route: p.route, title: p.title, eyebrow: groupOf(p.route) })
      }
      for (const p of pages) {
        const out = join(options.dir, 'og', p.route === '/' ? 'index.png' : `${p.route.slice(1)}.png`)
        mkdirSync(dirname(out), { recursive: true })
        writeFileSync(out, await renderOg(p))
      }
      console.log(`og-images: ${pages.length} images`)
    },
  }
}

export function notFoundPlugin() {
  return {
    name: 'everything-rust-404',
    apply: 'build',
    writeBundle(options) {
      if (!isClientDir(this, options)) return
      writeFileSync(
        join(options.dir, '404.html'),
        readFileSync(join(WEBSITE_ROOT, 'content', '404.html'), 'utf8'),
      )
    },
  }
}
