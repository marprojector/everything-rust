import { defineMiddleware } from 'void'
import { DESCRIPTION } from '../lib/site.ts'
import { publicPath } from '../lib/nav.ts'
import { LOCALES, localeInfo, localeOf, stripLocale, withLocale } from '../lib/locales.ts'

declare module 'void' {
  interface CloudContextVariables {
    shared: { path: string }
  }
}

const THEME_BOOTSTRAP = `(function(){
var mq=matchMedia("(prefers-color-scheme: dark)");
function read(){try{return localStorage.getItem("theme")}catch(_){return null}}
function apply(d){document.documentElement.setAttribute("data-theme",d?"dark":"light")}
function resolve(t){return t==="light"?false:t==="system"?mq.matches:true}
apply(resolve(read()));
mq.addEventListener("change",function(){if(read()==="system")apply(mq.matches)});
})();`

const LOCALE_LINKS = (rest: string) =>
  LOCALES.map((l) => `<link rel="alternate" hreflang="${l.lang}" href="${withLocale(rest, l.code)}">`).join('')

const escapeAttr = (s: string) =>
  s.replace(/&/g, '&amp;').replace(/"/g, '&quot;').replace(/</g, '&lt;').replace(/>/g, '&gt;')

const decode = (s: string) =>
  s
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'")
    .replace(/&amp;/g, '&')

function ogImageFor(path: string): string {
  const p = stripLocale(path)
  return p.startsWith('/docs/') ? `/og${p}.png` : '/og-home.png'
}

export default defineMiddleware(async (c, next) => {
  const raw = c.isRewritten() ? (c.originalUrl()?.pathname ?? c.req.path) : c.req.path
  const path = publicPath(raw)
  c.set('shared', { path })
  c.set('headDefaults', {
    script: [{ innerHTML: THEME_BOOTSTRAP }],
    meta: [
      { property: 'og:image', content: ogImageFor(path) },
      { name: 'twitter:image', content: ogImageFor(path) },
    ],
  })
  await next()

  if (c.req.method !== 'GET' && c.req.method !== 'HEAD') return
  const res = c.res
  if (!(res.headers.get('content-type') ?? '').includes('text/html')) return
  const html = await res.text()
  const headEnd = html.indexOf('</head>')
  const rebuild = (body: string) => {
    const headers = new Headers(res.headers)
    headers.delete('content-length')
    c.res = new Response(body, { status: res.status, statusText: res.statusText, headers })
  }
  const info0 = localeInfo(localeOf(path))
  if (headEnd === -1 || html.includes('property="og:title"'))
    return rebuild(html.replace(/<html lang="en"/, `<html lang="${info0.lang}"${'rtl' in info0 && info0.rtl ? ' dir="rtl"' : ''}`))
  const head = html.slice(0, headEnd)
  const title =
    decode(head.match(/<title>([\s\S]*?)<\/title>/i)?.[1] ?? '').replace(/\s+·\s+everything-rust\s*$/, '') ||
    'everything-rust'
  const descMatch = head.match(/<meta[^>]*name="description"[^>]*content="([^"]*)"/i)
  const description = descMatch ? decode(descMatch[1]) : DESCRIPTION
  const tags = [
    descMatch ? '' : `<meta name="description" content="${escapeAttr(description)}">`,
    `<meta property="og:title" content="${escapeAttr(title)}">`,
    `<meta property="og:description" content="${escapeAttr(description)}">`,
    `<meta name="twitter:title" content="${escapeAttr(title)}">`,
    `<meta name="twitter:description" content="${escapeAttr(description)}">`,
  ].join('')
  const info = localeInfo(localeOf(path))
  const htmlOpen = `<html lang="${info.lang}"${'rtl' in info && info.rtl ? ' dir="rtl"' : ''}`
  const alternates = LOCALE_LINKS(stripLocale(path))
  rebuild((html.slice(0, headEnd) + tags + alternates + html.slice(headEnd)).replace(/<html lang="en"/, htmlOpen))
})
