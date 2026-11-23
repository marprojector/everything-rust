// Adapter for everything-rust: runs a list of Service calls on in-memory data and returns every
// result, as it was when the call returned, and the data afterwards. Ids made by `create` are random, so each is replaced by
// `#1`, `#2`... in the order they were made.
import { Low, Memory } from 'lowdb'

import { Service } from '../service.ts'
import type { Data } from '../service.ts'

export type Call = {
  op: string
  name: string
  id?: string
  body?: Record<string, unknown>
  where?: Record<string, unknown>
  sort?: string
  page?: number
  perPage?: number
  embed?: string | string[]
  dependent?: string | string[]
}

export async function serviceCalls(data: Data, calls: Call[]): Promise<unknown> {
  const db = new Low<Data>(new Memory<Data>(), structuredClone(data))
  const service = new Service(db)
  const results: unknown[] = []
  const made: string[] = []
  for (const c of calls) {
    try {
      let r: unknown
      switch (c.op) {
        case 'find':
          r = service.find(c.name, {
            where: (c.where ?? {}) as never,
            sort: c.sort,
            page: c.page,
            perPage: c.perPage,
            embed: c.embed,
          })
          break
        case 'findById':
          r = service.findById(c.name, c.id ?? '', { _embed: c.embed })
          break
        case 'create': {
          const item = await service.create(c.name, c.body)
          if (item) made.push(String(item['id']))
          r = item
          break
        }
        case 'update':
          r = await service.update(c.name, c.body)
          break
        case 'patch':
          r = await service.patch(c.name, c.body)
          break
        case 'updateById':
          r = await service.updateById(c.name, c.id ?? '', c.body)
          break
        case 'patchById':
          r = await service.patchById(c.name, c.id ?? '', c.body)
          break
        case 'destroyById':
          r = await service.destroyById(c.name, c.id ?? '', c.dependent)
          break
        default:
          r = { unknownOp: c.op }
      }
      // Snapshot now: later calls mutate the items a read returned, as the server's response
      // would already have been sent.
      results.push(
        r === undefined
          ? { undefined: true }
          : typeof r === 'function'
            ? null
            : JSON.parse(JSON.stringify(r)),
      )
    } catch (e) {
      results.push({ threw: (e as Error).message })
    }
  }
  let text = JSON.stringify({ results, data: db.data })
  made.forEach((id, i) => {
    text = text.split(JSON.stringify(id)).join(JSON.stringify(`#${i + 1}`))
  })
  return JSON.parse(text)
}
