// Renders the README GIFs in docs/assets/gif/*.html to docs/assets/*.gif: each page exposes
// `frameCount` and `render(i)`; frames are screenshotted and joined with ffmpeg.
// Run from the repository root: node scripts/gifs.mjs [name...]
import { createRequire } from 'node:module'
import { execFileSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'

const require = createRequire(import.meta.url)
const { chromium } = require('playwright')
const root = path.join(path.dirname(fileURLToPath(import.meta.url)), '..')
const dir = path.join(root, 'docs', 'assets', 'gif')
const names = process.argv.slice(2).length ? process.argv.slice(2) : fs.readdirSync(dir).filter(f => f.endsWith('.html')).map(f => f.slice(0, -5))
const fps = 15

const browser = await chromium.launch()
for (const name of names) {
  const page = await browser.newPage({ viewport: { width: 1200, height: 800 } })
  const errors = []
  page.on('pageerror', e => errors.push(e.message))
  await page.goto('file://' + path.join(dir, name + '.html'))
  await page.evaluate(() => document.fonts.ready)
  const count = await page.evaluate(() => window.frameCount)
  if (errors.length || !Number.isInteger(count) || count < 1) {
    throw new Error(`${name}.html did not load: ${errors.join('; ') || 'no frameCount'}`)
  }
  const frames = fs.mkdtempSync(path.join(os.tmpdir(), 'everything-rust-gif-'))
  const target = page.locator('body > *:first-child')
  for (let i = 0; i < count; i++) {
    await page.evaluate(i => window.render(i), i)
    await target.screenshot({ path: path.join(frames, `f${String(i).padStart(4, '0')}.png`) })
  }
  const out = path.join(root, 'docs', 'assets', name + '.gif')
  const input = ['-y', '-framerate', String(fps), '-i', path.join(frames, 'f%04d.png')]
  execFileSync('ffmpeg', [...input, '-vf', 'palettegen=max_colors=96:stats_mode=diff', path.join(frames, 'palette.png')], { stdio: 'ignore' })
  execFileSync('ffmpeg', [...input, '-i', path.join(frames, 'palette.png'), '-lavfi', 'paletteuse=dither=none:diff_mode=rectangle', '-loop', '0', out], { stdio: 'ignore' })
  fs.rmSync(frames, { recursive: true })
  console.log(`wrote docs/assets/${name}.gif (${count} frames, ${(fs.statSync(out).size / 1024).toFixed(0)} KB)`)
  await page.close()
}
await browser.close()
