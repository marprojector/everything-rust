// Adapter for everything-rust: serves `createApp` on in-memory data and sends it a list of
// requests over HTTP, one after another; returns what each response held (status, the headers
// that do not change between runs, body) and the data afterwards. Ids made by POST are random,
// so each is replaced by `#1`, `#2`... in the order they were made.
import { request } from 'node:http'
import type { AddressInfo } from 'node:net'

import { Low, Memory } from 'lowdb'

import { createApp } from '../app.ts'
import type { Data } from '../service.ts'

export type Req = {
  method?: string
  path: string
  headers?: Record<string, string>
  body?: string
}

const SKIPPED = new Set(['date', 'connection', 'keep-alive', 'x-powered-by', 'etag', 'last-modified', 'transfer-encoding'])

function send(port: number, r: Req): Promise<unknown> {
  return new Promise((resolve) => {
    let req
    try {
      req = request(
        { host: '127.0.0.1', port, method: r.method ?? 'GET', path: r.path, headers: { connection: 'close', ...r.headers } },
        (res) => {
          const chunks: Buffer[] = []
          res.on('data', (c: Buffer) => chunks.push(c))
          res.on('end', () => {
            const headers: Record<string, string> = {}
            for (let i = 0; i < res.rawHeaders.length; i += 2) {
              const name = res.rawHeaders[i]!.toLowerCase()
              if (SKIPPED.has(name)) continue
              headers[name] = headers[name] === undefined ? res.rawHeaders[i + 1]! : `${headers[name]}, ${res.rawHeaders[i + 1]}`
            }
            resolve({ status: res.statusCode, headers, body: Buffer.concat(chunks).toString('utf8') })
          })
        },
      )
    } catch (e) {
      resolve({ threw: (e as Error).message })
      return
    }
    req.on('error', (e) => resolve({ failed: e.message }))
    if (r.body !== undefined) req.end(r.body)
    else req.end()
  })
}

export async function appRequests(data: Data, requests: Req[]): Promise<unknown> {
  const db = new Low<Data>(new Memory<Data>(), structuredClone(data))
  const app = createApp(db, { logger: false })
  const server = app.listen(0, undefined, '127.0.0.1')
  await new Promise((ok) => server.once('listening', ok))
  const { port } = server.address() as AddressInfo
  const responses: unknown[] = []
  for (const r of requests) responses.push(await send(port, r))
  await new Promise((ok) => server.close(ok))
  const made: string[] = []
  for (const res of responses as { status?: number; body?: string }[]) {
    if (res.status !== 201 || !res.body) continue
    try {
      const id = JSON.parse(res.body).id
      if (typeof id === 'string') made.push(id)
    } catch {
      // not JSON
    }
  }
  let text = JSON.stringify({ responses, data: db.data })
  made.forEach((id, i) => {
    text = text.split(JSON.stringify(id)).join(JSON.stringify(`#${i + 1}`))
    text = text.split(id).join(`#${i + 1}`)
  })
  return JSON.parse(text)
}
