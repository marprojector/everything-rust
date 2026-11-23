import { useEffect, useState } from 'react'
import { createPortal } from 'react-dom'
import Logo from '../Logo'
import ThemeToggle from './ThemeToggle'
import SearchDialog, { SearchButton } from './SearchDialog'
import { CloseIcon, GitHubIcon, MenuIcon } from './icons'
import type { NavGroup } from '../../lib/nav'
import type { Messages } from '../../lib/messages/en'
import { LOCALES, stripLocale, withLocale } from '../../lib/locales'
import { REPO_URL } from '../../lib/site'

function LanguageMenu({ currentPath, locale, label }: { currentPath: string; locale: string; label: string }) {
  const rest = stripLocale(currentPath)
  const current = LOCALES.find((l) => l.code === locale) ?? LOCALES[0]
  return (
    <details className="lang-menu relative">
      <summary
        className="inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-md px-2 text-sm text-fg/80 hover:text-fg"
        aria-label={label}
        title={label}
      >
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" aria-hidden>
          <circle cx="12" cy="12" r="9" />
          <path d="M3 12h18M12 3c2.5 2.6 3.8 5.6 3.8 9s-1.3 6.4-3.8 9c-2.5-2.6-3.8-5.6-3.8-9S9.5 5.6 12 3z" />
        </svg>
        <span className="hidden xl:inline">{current.name}</span>
      </summary>
      <ul className="thin-scrollbar absolute end-0 top-11 z-[70] max-h-[70vh] w-48 overflow-y-auto rounded-lg border border-line bg-bg p-1 shadow-xl">
        {LOCALES.map((l) => (
          <li key={l.code}>
            <a
              href={withLocale(rest, l.code)}
              lang={l.lang}
              hrefLang={l.lang}
              aria-current={l.code === locale ? 'true' : undefined}
              className={`block rounded-md px-3 py-1.5 text-sm ${l.code === locale ? 'bg-brand-soft font-medium text-brand-ink' : 'text-fg/80 hover:bg-bg-soft hover:text-fg'}`}
              onClick={() => {
                try {
                  localStorage.setItem('locale', l.code)
                } catch {}
              }}
            >
              {l.name}
            </a>
          </li>
        ))}
      </ul>
    </details>
  )
}

export default function Navbar({
  currentPath,
  locale = 'en',
  themeToggle = true,
  ui,
  nav,
}: {
  currentPath: string
  locale?: string
  themeToggle?: boolean
  ui: Messages['ui']
  nav: NavGroup[]
}) {
  const L = (p: string) => withLocale(p, locale)
  const LINKS = [
    { title: ui.docs, href: L('/docs/introduction/what-is-everything-rust'), match: L('/docs/') },
    { title: ui.languages, href: L('/docs/languages/overview'), match: L('/docs/languages/') },
    { title: ui.realWorld, href: L('/docs/real-world/overview'), match: L('/docs/real-world/') },
  ]
  const [searchOpen, setSearchOpen] = useState(false)
  const [menuOpen, setMenuOpen] = useState(false)

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault()
        setSearchOpen((o) => !o)
      } else if (e.key === '/' && !(e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement)) {
        e.preventDefault()
        setSearchOpen(true)
      }
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [])

  useEffect(() => {
    document.body.style.overflow = menuOpen ? 'hidden' : ''
  }, [menuOpen])

  const active = (match: string) =>
    match === LINKS[0].match
      ? currentPath.startsWith('/docs/') && !LINKS.slice(1).some((l) => currentPath.startsWith(l.match))
      : currentPath.startsWith(match)

  return (
    <div className="mx-auto flex h-16 max-w-[1440px] items-center gap-4 px-4 sm:px-6">
      <a href={L('/')} className="shrink-0 font-display text-[17px] font-bold" aria-label={ui.home}>
        <Logo size={30} />
      </a>
      <div className="ml-auto flex items-center gap-1 sm:gap-2">
        <nav className="mr-2 hidden items-center gap-1 md:flex" aria-label="Main">
          {LINKS.map((l) => (
            <a
              key={l.href}
              href={l.href}
              className={`rounded-md px-3 py-1.5 text-sm ${active(l.match) ? 'font-semibold text-fg' : 'text-muted hover:text-fg'}`}
              aria-current={active(l.match) ? 'page' : undefined}
            >
              {l.title}
            </a>
          ))}
        </nav>
        <div className="hidden sm:block">
          <SearchButton ui={ui} onOpen={() => setSearchOpen(true)} />
        </div>
        <div className="sm:hidden">
          <SearchButton ui={ui} compact onOpen={() => setSearchOpen(true)} />
        </div>
        <LanguageMenu currentPath={currentPath} locale={locale} label={ui.language} />
        <a
          href={REPO_URL}
          className="inline-flex h-9 w-9 items-center justify-center rounded-md text-fg/80 hover:text-fg"
          aria-label={ui.github}
        >
          <GitHubIcon className="h-[22px] w-[22px]" />
        </a>
        <button
          type="button"
          className="inline-flex h-9 w-9 items-center justify-center rounded-md text-fg/80 hover:text-fg lg:hidden"
          onClick={() => setMenuOpen(true)}
          aria-label={ui.openMenu}
          aria-expanded={menuOpen}
        >
          <MenuIcon />
        </button>
      </div>

      {menuOpen &&
        createPortal(
        <div data-theme={themeToggle ? undefined : "dark"} className="fixed inset-0 z-[65] lg:hidden" role="dialog" aria-modal="true" aria-label={ui.menu}>
          <div className="absolute inset-0 bg-black/50" onClick={() => setMenuOpen(false)} />
          <div className="absolute inset-y-0 right-0 flex w-[min(20rem,88vw)] flex-col border-l border-line bg-bg text-fg">
            <div className="flex h-16 items-center justify-between border-b border-line px-4">
              <span className="font-display font-bold"><Logo size={26} /></span>
              <button
                type="button"
                className="inline-flex h-9 w-9 items-center justify-center rounded-md text-muted hover:text-fg"
                onClick={() => setMenuOpen(false)}
                aria-label={ui.closeMenu}
              >
                <CloseIcon />
              </button>
            </div>
            <nav className="thin-scrollbar flex-1 overflow-y-auto px-3 py-4" aria-label={ui.docs}>
              <div className="mb-4 flex flex-col md:hidden">
                {LINKS.map((l) => (
                  <a key={l.href} href={l.href} className="rounded-md px-2 py-1.5 text-sm font-semibold">
                    {l.title}
                  </a>
                ))}
              </div>
              {nav.map((g) => (
                <div key={g.title} className="mb-4">
                  <div className="mb-1 px-2 text-xs font-semibold text-muted">{g.title}</div>
                  <ul>
                    {g.items.map((i) => (
                      <li key={i.path}>
                        <a
                          href={i.path}
                          className={`block rounded-md px-2 py-1.5 text-sm ${i.path === currentPath ? 'bg-brand-soft font-medium text-brand-ink' : 'text-fg/80 hover:bg-bg-soft'}`}
                        >
                          {i.title}
                        </a>
                      </li>
                    ))}
                  </ul>
                </div>
              ))}
            </nav>
            {themeToggle && (
              <div className="border-t border-line px-3 py-3">
                <ThemeToggle withLabel ui={ui} />
              </div>
            )}
          </div>
        </div>,
          document.body,
        )}

      <SearchDialog open={searchOpen} onClose={() => setSearchOpen(false)} ui={ui} locale={locale} />
    </div>
  )
}
