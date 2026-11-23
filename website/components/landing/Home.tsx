import { REPO_URL } from '../../lib/site'
import { messages, withLocale, fill, type Messages } from '../../lib/i18n'
import Rich from '../Rich'
import './landing.css'

function RecordViz() {
  const rows = [
    ['wordCount', '"a\\u00a0b"'],
    ['parse', '"1 \u017f"'],
    ['slugify', '"\u03a3\u038a\u03a3\u03a5\u03a6\u039f\u03a3"'],
    ['format', '-0'],
    ['round', '-2.5'],
  ]
  return (
    <div className="w-full font-mono text-[11px] leading-[22px] text-muted">
      {rows.map(([fn, args]) => (
        <div key={fn} className="truncate">
          {'{'}<span className="text-brand-ink">"fn"</span>:"{fn}",<span className="text-brand-ink">"args"</span>:[{args}]{'}'}
        </div>
      ))}
    </div>
  )
}

function PortViz({ m }: { m: Messages['home'] }) {
  return (
    <div className="flex w-full items-center justify-center gap-3 font-mono text-xs">
      <div className="grid grid-cols-2 gap-1.5">
        {['TS', 'Py', 'Go', 'Rb', 'Java', 'C'].map((l) => (
          <span key={l} className="rounded border border-line px-2 py-0.5 text-center text-muted">
            {l}
          </span>
        ))}
      </div>
      <svg width="48" height="20" viewBox="0 0 48 20" aria-hidden className="text-brand">
        <path d="M0 10 H42 M34 3 L43 10 L34 17" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      </svg>
      <div className="rounded-md border border-brand/50 bg-brand-soft px-3 py-2 text-center text-brand-ink">
        port.rs
        <div className="mt-0.5 text-[10px] text-muted">{m.vizSameApi}</div>
      </div>
    </div>
  )
}

function CheckViz({ m }: { m: Messages['home'] }) {
  return (
    <div className="text-center">
      <div className="font-display text-4xl font-semibold tracking-tight">
        3,414<span className="text-muted">/</span>3,414
      </div>
      <div className="mt-2 text-xs text-muted">{m.vizCheck}</div>
      <div className="mt-3 inline-flex items-center gap-1.5 rounded-full bg-good-soft px-2.5 py-0.5 text-xs font-medium text-good">{m.vizGate}</div>
    </div>
  )
}

function LiftViz({ m }: { m: Messages['home'] }) {
  const steps = [99.8, 22.6, 0].map((v, i) => ({ label: m.vizLiftSteps[i], v }))
  return (
    <div className="w-full space-y-2.5">
      <div className="text-[11px] text-muted">{m.vizLift}</div>
      {steps.map((x) => (
        <div key={x.label}>
          <div className="mb-1 flex justify-between text-[11px] text-muted">
            <span>{x.label}</span>
            <span className="tabular-nums">{x.v}%</span>
          </div>
          <div className="h-1.5 rounded-full bg-line">
            <div className="h-1.5 rounded-full bg-brand" style={{ width: `${Math.max(x.v, 1.5)}%`, opacity: x.v ? 1 : 0.4 }} />
          </div>
        </div>
      ))}
    </div>
  )
}

const STEP_CMDS = ['record', 'bridge', 'diff, gate', 'app lift']

const LANGS = [
  { key: 'typescript', name: 'TypeScript/JavaScript' },
  { key: 'python', name: 'Python' },
  { key: 'c', name: 'C' },
  { key: 'go', name: 'Go' },
  { key: 'ruby', name: 'Ruby' },
  { key: 'java', name: 'Java' },
  { key: 'php', name: 'PHP', wholeApp: true },
  { key: 'csharp', name: 'C#', wholeApp: true },
] as const

const SPONSORS = [
  { login: 'aliyilmazco', name: 'Ali Yilmaz' },
  { login: 'ashley-hunter', name: 'Ashley Hunter' },
  { login: 'fatihuzuntas', name: 'Fatih Uzuntaş' },
]

const BENCH = [
  ['GET /posts', '11.1k / 10.8k', '27.3k / 23.9k', '19.7k / 20.4k', '115.0k / 110.7k'],
  ['GET /posts/1', '11.0k / 11.3k', '27.7k / 25.4k', '21.9k / 22.6k', '119.9k / 99.0k'],
  ['?_embed=comments', '8.7k / 6.6k', '22.5k / 21.8k', '13.8k / 11.0k', '98.3k / 100.6k'],
]

export default function Home({ hero, install, locale = 'en' }: { hero: React.ReactNode; install: React.ReactNode; locale?: string }) {
  const m = messages(locale).home
  const L = (p: string) => withLocale(p, locale)
  const vizzes = [<RecordViz key="r" />, <PortViz key="p" m={m} />, <CheckViz key="c" m={m} />, <LiftViz key="l" m={m} />]
  return (
    <div className="home">
      {hero}
      <div className="wrap">
        <div className="stats">
          {m.stats.map((x) => (
            <div key={x.k} className="stat">
              <div className="v">
                {x.v}
                {x.unit && <small>{x.unit}</small>}
              </div>
              <div className="k">{x.k}</div>
            </div>
          ))}
        </div>
      </div>

      <section className="section" aria-labelledby="install-title">
        <div className="wrap">
          <div className="section-head">
            <h2 id="install-title" className="section-title">
              {m.installTitle}
            </h2>
            <p className="section-desc">{m.installDesc}</p>
          </div>
          <div className="install">{install}</div>
          <p className="note">
            <Rich text={fill(m.installNote, { releases: `${REPO_URL}/releases` })} />
          </p>
        </div>
      </section>

      <section className="section" aria-labelledby="loop-title">
        <div className="wrap">
          <div className="section-head">
            <h2 id="loop-title" className="section-title">
              {m.loopTitle}
            </h2>
            <p className="section-desc">{m.loopDesc}</p>
          </div>
          <ol className="steps">
            {m.steps.map((s, i) => (
              <li key={i} className="step">
                <div className="step-viz" aria-hidden>
                  {vizzes[i]}
                </div>
                <span className="step-n" aria-hidden>
                  {i + 1}
                </span>
                <h3>{s.title}</h3>
                <code className="step-cmd">{STEP_CMDS[i]}</code>
                <p>
                  <Rich text={s.text} />
                </p>
              </li>
            ))}
          </ol>
        </div>
      </section>

      <section className="section" aria-labelledby="demo-title">
        <div className="wrap">
          <div className="section-head">
            <h2 id="demo-title" className="section-title">
              {m.demoTitle}
            </h2>
            <p className="section-desc">
              <Rich text={m.demoDesc} />
            </p>
          </div>
          <div className="panel">
            <div className="titlebar">
              <span>~/x18n</span>
            </div>
            <div className="split">
              <div>
                <pre>
                  <span className="c">{m.demoComments.profile}</span>
                  {'\n'}
                  <span className="p">$</span> everything-rust profile . -- node bench.mjs{'\n'}
                  <span className="p">$</span> everything-rust bridge . src/fuzzy.ts --batch fuzzyScore:text{'\n'}
                  <span className="c">{m.demoComments.write}</span>
                  {'\n'}
                  <span className="p">$</span> everything-rust record rust/fuzzy-rs{'\n'}
                  <span className="p">$</span> everything-rust build rust/fuzzy-rs{'\n'}
                  <span className="p">$</span> everything-rust diff rust/fuzzy-rs{'\n'}
                  <span className="p">$</span> everything-rust bench rust/fuzzy-rs{'\n'}
                  <span className="p">$</span> everything-rust gate rust/fuzzy-rs
                </pre>
              </div>
              <div>
                <pre>
                  <span className="c">{m.demoComments.first}</span>
                  {'\n'}
                  {m.demoComments.matchLine}
                  {'\n'}
                  <span className="bad">- fuzzyScore("b😀", "a😀b😀c"): expected {'{"ok":1997}'}, got {'{"ok":1998}'}</span>
                  {'\n\n'}
                  <span className="c">{m.demoComments.why}</span>
                  {'\n\n'}
                  <span className="ok">{m.demoComments.fixed}</span>
                </pre>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section className="section" aria-labelledby="ports-title">
        <div className="wrap">
          <div className="section-head">
            <h2 id="ports-title" className="section-title">
              {m.portsTitle}
            </h2>
            <p className="section-desc">{m.portsDesc}</p>
          </div>

          <div className="ports">
            {m.ports.map((p) => (
              <a key={p.href} href={L(p.href)} className="port-card">
                <div className="port-kind">{p.kind}</div>
                <div className="big">{p.big}</div>
                <p>
                  <Rich text={p.text} />
                </p>
                <dl>
                  {p.rows.map((r) => (
                    <Row key={r.k} k={r.k} v={r.v} slow={'slow' in r && !!r.slow} />
                  ))}
                </dl>
              </a>
            ))}
          </div>

          <div className="panel bench-panel">
            <div className="titlebar">
              <span>{m.benchTitle}</span>
            </div>
            <div className="overflow-x-auto">
              <table className="bench">
                <thead>
                  <tr>
                    {m.benchHead.map((h) => (
                      <th key={h}>{h}</th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {BENCH.map((r) => (
                    <tr key={r[0]}>
                      <td>{r[0]}</td>
                      <td>{r[1]}</td>
                      <td className="rust">{r[2]}</td>
                      <td>{r[3]}</td>
                      <td className="rust">{r[4]}</td>
                    </tr>
                  ))}
                  <tr>
                    <td>{m.benchPostRow}</td>
                    <td></td>
                    <td></td>
                    <td>2,973 / 2,467</td>
                    <td className="rust">5,821 / 5,976</td>
                  </tr>
                </tbody>
              </table>
            </div>
            <p className="panel-note">
              <Rich text={m.benchNote} locale={locale} />
            </p>
          </div>
        </div>
      </section>

      <section className="section" aria-labelledby="langs-title">
        <div className="wrap">
          <div className="section-head">
            <h2 id="langs-title" className="section-title">
              {m.langsTitle}
            </h2>
            <p className="section-desc">{m.langsDesc}</p>
          </div>
          <ul className="langs">
            {LANGS.map((l) => (
              <li key={l.key}>
                <a href={L(`/docs/languages/${l.key}`)} className="lang">
                  <span className="name">{l.name}</span>
                  <span className={`badge${'wholeApp' in l ? ' preview' : ''}`}>{'wholeApp' in l ? m.wholeApps : m.verified}</span>
                  <span className="meta">{m.langMeta[l.key]}</span>
                </a>
              </li>
            ))}
          </ul>
        </div>
      </section>

      <section className="section last" aria-labelledby="sponsors-title">
        <div className="wrap">
          <div className="section-head">
            <h2 id="sponsors-title" className="section-title">
              {m.sponsorsTitle}
            </h2>
            <p className="section-desc">{m.sponsorsDesc}</p>
          </div>
          <div className="sponsors">
            <a href="https://coderabbit.ai" className="sponsor-company">
              <img src="https://github.com/coderabbitai.png?size=120" width={48} height={48} alt="" loading="lazy" />
              <span>
                <span className="who">CodeRabbit</span>
                <span className="role">{m.companySponsor}</span>
              </span>
            </a>
            {SPONSORS.map((p) => (
              <a key={p.login} href={`https://github.com/${p.login}`} className="sponsor-person">
                <img src={`https://github.com/${p.login}.png?size=100`} width={40} height={40} alt="" loading="lazy" />
                {p.name}
              </a>
            ))}
            <a className="btn btn-quiet" href="https://github.com/sponsors/marprojector">
              {m.becomeSponsor}
            </a>
          </div>
        </div>
      </section>
    </div>
  )
}

function Row({ k, v, slow }: { k: string; v: string; slow: boolean }) {
  return (
    <>
      <dt className={slow ? 'slow' : undefined}>{k}</dt>
      <dd className={slow ? 'slow' : undefined}>{v}</dd>
    </>
  )
}
