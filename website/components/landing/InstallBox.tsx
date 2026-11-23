import type { Messages } from '../../lib/messages/en'
import { useState } from 'react'
import { CheckIcon, CopyIcon } from '../site/icons'


export default function InstallBox({ ui, agent }: { ui: Messages['ui']; agent: [string, string] }) {
  const OPTIONS = [
    { key: 'cargo', label: 'cargo', lines: ['cargo install everything-rust', 'everything-rust setup'] },
    { key: 'npm', label: 'npm', lines: ['npm install -g everything-rust', 'everything-rust setup'] },
    { key: 'agent', label: 'agent', lines: agent },
  ]
  const [tab, setTab] = useState('cargo')
  const [copied, setCopied] = useState(false)
  const current = OPTIONS.find((o) => o.key === tab) ?? OPTIONS[0]
  const copy = () => {
    navigator.clipboard?.writeText(current.lines.join('\n')).then(
      () => {
        setCopied(true)
        setTimeout(() => setCopied(false), 1800)
      },
      () => {},
    )
  }
  return (
    <div className="panel">
      <div className="titlebar" role="tablist" aria-label={ui.installWith}>
        {OPTIONS.map((o) => (
          <button
            key={o.key}
            role="tab"
            aria-selected={tab === o.key}
            onClick={() => setTab(o.key)}
            className={`whitespace-nowrap rounded-md px-2.5 py-1 font-mono text-xs ${tab === o.key ? 'bg-white/10 text-[var(--screen-fg)]' : 'text-[#8c959c] hover:text-[var(--screen-fg)]'}`}
          >
            {o.label}
          </button>
        ))}
        <button
          type="button"
          onClick={copy}
          className="ml-auto inline-flex items-center gap-1.5 rounded-md px-2 py-1 text-xs text-[#8c959c] hover:text-[var(--screen-fg)]"
          aria-label={ui.copyInstall}
        >
          {copied ? <CheckIcon className="text-[#63c3a7]" /> : <CopyIcon />}
          {copied ? ui.copied : ui.copy}
        </button>
      </div>
      <div className="py-2">
        {current.lines.map((l) => (
          <div key={l} className="copy-line">
            <span className="prompt">{tab === 'agent' ? '>' : '$'}</span>
            <code>{l}</code>
          </div>
        ))}
      </div>
    </div>
  )
}
