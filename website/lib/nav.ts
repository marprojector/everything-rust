import { messages, withLocale } from './i18n'

export interface NavItem {
  title: string
  path: string
}

export interface NavGroup {
  key: string
  title: string
  items: NavItem[]
}

export const docsNav: NavGroup[] = [
  {
    key: 'introduction',
    title: 'Introduction',
    items: [
      { title: 'What is everything-rust', path: '/docs/introduction/what-is-everything-rust' },
      { title: 'Quick start', path: '/docs/introduction/quick-start' },
      { title: 'Install', path: '/docs/introduction/install' },
      { title: 'Set up your agent', path: '/docs/introduction/agents' },
      { title: 'Doctor', path: '/docs/introduction/doctor' },
    ],
  },
  {
    key: 'concepts',
    title: 'Concepts',
    items: [
      { title: 'The loop', path: '/docs/concepts/the-loop' },
      { title: 'Only the slow part', path: '/docs/concepts/go' },
      { title: 'Profile and analyze', path: '/docs/concepts/analyze' },
      { title: 'Bridge', path: '/docs/concepts/bridge' },
      { title: 'Record', path: '/docs/concepts/record' },
      { title: 'Port', path: '/docs/concepts/port' },
      { title: 'Diff and known diffs', path: '/docs/concepts/diff' },
      { title: 'Bench', path: '/docs/concepts/bench' },
      { title: 'Shadow', path: '/docs/concepts/shadow' },
      { title: 'Switch on and off', path: '/docs/concepts/switch' },
      { title: 'Gate', path: '/docs/concepts/gate' },
      { title: 'Coverage', path: '/docs/concepts/coverage' },
      { title: 'Batches and hot paths', path: '/docs/concepts/batches' },
      { title: 'Float tolerance', path: '/docs/concepts/float-tolerance' },
      { title: "What it can't catch", path: '/docs/concepts/limits' },
    ],
  },
  {
    key: 'languages',
    title: 'Languages',
    items: [
      { title: 'Overview', path: '/docs/languages/overview' },
      { title: 'TypeScript/JavaScript', path: '/docs/languages/typescript' },
      { title: 'Python', path: '/docs/languages/python' },
      { title: 'C', path: '/docs/languages/c' },
      { title: 'Go', path: '/docs/languages/go' },
      { title: 'Ruby', path: '/docs/languages/ruby' },
      { title: 'Java', path: '/docs/languages/java' },
      { title: 'PHP', path: '/docs/languages/php' },
      { title: 'C#', path: '/docs/languages/csharp' },
    ],
  },
  {
    key: 'wholeApps',
    title: 'Whole apps',
    items: [
      { title: 'Port a whole app', path: '/docs/whole-apps/overview' },
      { title: 'Record and replay', path: '/docs/whole-apps/record-replay' },
      { title: 'Front door and wire', path: '/docs/whole-apps/front-door' },
      { title: 'Lift', path: '/docs/whole-apps/lift' },
      { title: 'Servers', path: '/docs/whole-apps/servers' },
      { title: 'Libraries', path: '/docs/whole-apps/libraries' },
    ],
  },
  {
    key: 'reference',
    title: 'Reference',
    items: [
      { title: 'CLI', path: '/docs/reference/cli' },
      { title: 'MCP tools', path: '/docs/reference/mcp-tools' },
    ],
  },
  {
    key: 'realWorld',
    title: 'Real-world ports',
    items: [
      { title: 'Overview', path: '/docs/real-world/overview' },
      { title: 'json-server', path: '/docs/real-world/json-server' },
      { title: 'Fuse.js', path: '/docs/real-world/fuse-js' },
      { title: 'vercel/ms', path: '/docs/real-world/vercel-ms' },
      { title: 'x18n', path: '/docs/real-world/x18n' },
      { title: 'Six languages', path: '/docs/real-world/six-languages' },
      { title: 'Fixtures and static runs', path: '/docs/real-world/fixtures' },
    ],
  },
  {
    key: 'project',
    title: 'Project',
    items: [
      { title: 'Status', path: '/docs/project/status' },
      { title: 'FAQ', path: '/docs/project/faq' },
      { title: 'Contributing', path: '/docs/project/contributing' },
    ],
  },
]

export const allDocs: NavItem[] = docsNav.flatMap((g) => g.items)

export function groupOf(path: string): NavGroup | undefined {
  return docsNav.find((g) => g.items.some((i) => i.path === path))
}

export function pagerFor(path: string): { prev?: NavItem; next?: NavItem } {
  const i = allDocs.findIndex((d) => d.path === path)
  if (i === -1) return {}
  return { prev: allDocs[i - 1], next: allDocs[i + 1] }
}

export function publicPath(path: string): string {
  const p = path.replace(/\/+$/, '').replace(/^\/en(?=\/|$)/, '')
  return p === '' ? '/' : p
}

export function localizedNav(locale: string): NavGroup[] {
  const m = messages(locale).nav
  return docsNav.map((g) => ({
    key: g.key,
    title: (m.groups as Record<string, string>)[g.key] ?? g.title,
    items: g.items.map((i) => ({ title: m.items[i.path] ?? i.title, path: withLocale(i.path, locale) })),
  }))
}
