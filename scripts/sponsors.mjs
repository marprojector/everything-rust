import { execFileSync } from 'node:child_process'
import { mkdirSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'

const OUT = join(dirname(new URL(import.meta.url).pathname), '..', 'docs', 'assets', 'sponsors')
const MAINTAINER = 'marprojector'

const COMPANIES = [{ login: 'coderabbitai', name: 'CodeRabbit', url: 'coderabbit.ai' }]

const KNOWN_ORDER = ['santoshyadavdev', 'hakansoyturk', 'aliyilmazco', 'ashley-hunter', 'fatihuzuntas']
const rank = (login) => {
  const i = KNOWN_ORDER.indexOf(login)
  return i === -1 ? KNOWN_ORDER.length : i
}

function individuals() {
  try {
    const query = `{ user(login: "${MAINTAINER}") { sponsors(first: 100) { nodes { ... on User { login name } ... on Organization { login name } } } } }`
    const out = execFileSync('gh', ['api', 'graphql', '-f', `query=${query}`], { encoding: 'utf8' })
    const companies = new Set(COMPANIES.map((c) => c.login))
    return JSON.parse(out)
      .data.user.sponsors.nodes.filter((n) => n.login && !companies.has(n.login))
      .map((n) => ({ login: n.login, name: n.name || n.login }))
      .sort((a, b) => rank(a.login) - rank(b.login))
  } catch (e) {
    console.error(`gh api failed (${e.message.split('\n')[0]}), keeping the list in this script`)
    return [
      { login: 'santoshyadavdev', name: 'Santosh Yadav' },
      { login: 'hakansoyturk', name: 'Hakan Soytürk' },
      { login: 'aliyilmazco', name: 'Ali Yilmaz' },
      { login: 'ashley-hunter', name: 'Ashley Hunter' },
      { login: 'fatihuzuntas', name: 'Fatih Uzuntaş' },
    ]
  }
}

async function avatar(login, size) {
  const res = await fetch(`https://github.com/${login}.png?size=${size}`)
  if (!res.ok) throw new Error(`avatar ${login}: ${res.status}`)
  const type = res.headers.get('content-type') ?? 'image/png'
  return `data:${type};base64,${Buffer.from(await res.arrayBuffer()).toString('base64')}`
}

const esc = (s) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;')

const THEMES = {
  dark: { text: '#f0f6fc', muted: '#9198a1', ring: '#e0763f', line: '#3d444d' },
  light: { text: '#1f2328', muted: '#59636e', ring: '#c4521d', line: '#d1d9e0' },
}

const FONT = `font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif"`

function companySvg(c, img, t) {
  const W = 800
  const H = 190
  const R = 52
  const cx = W / 2 - 130
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" viewBox="0 0 ${W} ${H}" ${FONT}>
  <text x="${W / 2}" y="26" text-anchor="middle" font-size="13" font-weight="600" letter-spacing="1.5" fill="${t.muted}">COMPANY SPONSOR</text>
  <defs><clipPath id="c"><circle cx="${cx}" cy="${H / 2 + 14}" r="${R}"/></clipPath></defs>
  <circle cx="${cx}" cy="${H / 2 + 14}" r="${R + 5}" fill="none" stroke="${t.ring}" stroke-width="2" opacity="0.6"/>
  <image href="${img}" x="${cx - R}" y="${H / 2 + 14 - R}" width="${R * 2}" height="${R * 2}" clip-path="url(#c)"/>
  <text x="${cx + R + 28}" y="${H / 2 + 22}" font-size="44" font-weight="700" fill="${t.text}">${esc(c.name)}</text>
  <text x="${cx + R + 30}" y="${H / 2 + 50}" font-size="16" fill="${t.muted}">${esc(c.url)}</text>
</svg>
`
}

function peopleSvg(people, imgs, t) {
  const W = 800
  const R = 36
  const cell = 128
  const perRow = 6
  const rows = Math.ceil(people.length / perRow)
  const H = 46 + rows * 150
  let body = `<text x="${W / 2}" y="26" text-anchor="middle" font-size="13" font-weight="600" letter-spacing="1.5" fill="${t.muted}">SPONSORS</text>\n`
  let defs = ''
  people.forEach((p, i) => {
    const row = Math.floor(i / perRow)
    const inRow = Math.min(perRow, people.length - row * perRow)
    const col = i - row * perRow
    const x = W / 2 - ((inRow - 1) * cell) / 2 + col * cell
    const y = 56 + row * 150 + R
    defs += `<clipPath id="a${i}"><circle cx="${x}" cy="${y}" r="${R}"/></clipPath>`
    body += `  <circle cx="${x}" cy="${y}" r="${R + 4}" fill="none" stroke="${t.ring}" stroke-width="2" opacity="0.55"/>
  <image href="${imgs[i]}" x="${x - R}" y="${y - R}" width="${R * 2}" height="${R * 2}" clip-path="url(#a${i})"/>
  <text x="${x}" y="${y + R + 28}" text-anchor="middle" font-size="15" font-weight="600" fill="${t.text}">${esc(p.name)}</text>
`
  })
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" viewBox="0 0 ${W} ${H}" ${FONT}>
  <defs>${defs}</defs>
${body}</svg>
`
}

mkdirSync(OUT, { recursive: true })
const people = individuals()
const companyImgs = await Promise.all(COMPANIES.map((c) => avatar(c.login, 240)))
const peopleImgs = await Promise.all(people.map((p) => avatar(p.login, 160)))
for (const [name, t] of Object.entries(THEMES)) {
  writeFileSync(join(OUT, `company-${name}.svg`), companySvg(COMPANIES[0], companyImgs[0], t))
  writeFileSync(join(OUT, `sponsors-${name}.svg`), peopleSvg(people, peopleImgs, t))
}
console.log(`sponsors: ${COMPANIES.length} company, ${people.length} individual -> ${OUT}`)
