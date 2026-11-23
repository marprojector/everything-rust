import Logo from '../Logo'
import { REPO_URL } from '../../lib/site'
import { messages, withLocale } from '../../lib/i18n'

const AUTHOR_LINKS = [
  {
    label: 'GitHub',
    href: 'https://github.com/marprojector',
    icon: 'M12 .5a11.5 11.5 0 0 0-3.64 22.41c.58.1.79-.25.79-.56v-2c-3.2.7-3.87-1.37-3.87-1.37-.53-1.33-1.28-1.69-1.28-1.69-1.05-.71.08-.7.08-.7 1.16.08 1.77 1.19 1.77 1.19 1.03 1.77 2.71 1.26 3.37.96.1-.75.4-1.26.73-1.55-2.56-.29-5.25-1.28-5.25-5.69 0-1.26.45-2.29 1.19-3.1-.12-.29-.52-1.46.11-3.05 0 0 .97-.31 3.17 1.18a11 11 0 0 1 5.77 0c2.2-1.49 3.17-1.18 3.17-1.18.63 1.59.23 2.76.11 3.05.74.81 1.19 1.84 1.19 3.1 0 4.42-2.7 5.39-5.27 5.68.41.36.78 1.06.78 2.14v3.17c0 .31.21.67.8.56A11.5 11.5 0 0 0 12 .5z',
  },
  {
    label: 'X',
    href: 'https://x.com/marprojector_ng',
    icon: 'M18.24 2.25h3.31l-7.23 8.26 8.5 11.24h-6.66l-5.21-6.82-5.97 6.82H1.67l7.73-8.84L1.25 2.25H8.08l4.71 6.23 5.45-6.23zm-1.16 17.52h1.83L7.08 4.13H5.12l11.96 15.64z',
  },
  {
    label: 'LinkedIn',
    href: 'https://www.linkedin.com/in/marprojector',
    icon: 'M20.45 20.45h-3.55v-5.57c0-1.33-.03-3.04-1.85-3.04-1.86 0-2.14 1.45-2.14 2.94v5.67H9.35V9h3.41v1.56h.05c.48-.9 1.64-1.85 3.37-1.85 3.6 0 4.27 2.37 4.27 5.46v6.28zM5.34 7.43a2.06 2.06 0 1 1 0-4.12 2.06 2.06 0 0 1 0 4.12zM7.12 20.45H3.56V9h3.56v11.45zM22.22 0H1.77C.79 0 0 .77 0 1.73v20.54C0 23.23.79 24 1.77 24h20.45c.98 0 1.78-.77 1.78-1.73V1.73C24 .77 23.2 0 22.22 0z',
  },
  {
    label: 'Bluesky',
    href: 'https://bsky.app/profile/marprojector.dev',
    icon: 'M5.2 3.2C7.95 5.27 10.9 9.46 12 11.73c1.1-2.27 4.05-6.46 6.8-8.53 1.98-1.49 5.2-2.65 5.2 1.03 0 .73-.42 6.17-.67 7.05-.85 3.06-3.97 3.84-6.74 3.37 4.84.82 6.07 3.55 3.41 6.28-5.05 5.18-7.26-1.3-7.83-2.96-.1-.3-.15-.45-.17-.33-.02-.12-.07.03-.17.33-.57 1.66-2.78 8.14-7.83 2.96-2.66-2.73-1.43-5.46 3.41-6.28-2.77.47-5.89-.31-6.74-3.37C.42 10.4 0 4.96 0 4.23 0 .55 3.22 1.71 5.2 3.2z',
  },
]

export default function Footer({ locale = 'en' }: { locale?: string }) {
  const ui = messages(locale).ui
  const L = (p: string) => withLocale(p, locale)
  return (
    <footer className="border-t border-line bg-bg text-fg">
      <div
        className="mx-auto flex max-w-[1440px] flex-col gap-8 px-5 py-12 text-sm text-muted sm:px-8 md:flex-row md:items-start md:justify-between md:py-16"
      >
        <div className="max-w-md space-y-3">
          <span className="text-base"><Logo size={28} /></span>
          <p className="leading-relaxed">{ui.legal}</p>
          <div className="flex flex-wrap items-center gap-x-4 gap-y-2 pt-2">
            <a href="https://marprojector.dev" className="inline-flex items-center gap-2 text-fg hover:text-brand-ink">
              <img src="https://github.com/marprojector.png?size=64" width={24} height={24} alt="" className="rounded-full" loading="lazy" />
              <span>
                {ui.madeBy} <span className="font-medium">M. Ammar R. Arief</span>
              </span>
            </a>
            <span className="flex items-center gap-3">
              {AUTHOR_LINKS.map((l) => (
                <a key={l.label} href={l.href} aria-label={l.label} title={l.label} className="hover:text-fg">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" aria-hidden>
                    <path d={l.icon} />
                  </svg>
                </a>
              ))}
            </span>
          </div>
        </div>
        <div className="grid grid-cols-2 gap-x-12 gap-y-2">
          <a className="hover:text-fg" href={L('/docs/introduction/quick-start')}>{ui.quickStart}</a>
          <a className="hover:text-fg" href={REPO_URL}>GitHub</a>
          <a className="hover:text-fg" href={L('/docs/reference/cli')}>{ui.cliReference}</a>
          <a className="hover:text-fg" href={`${REPO_URL}/issues`}>{ui.issues}</a>
          <a className="hover:text-fg" href={L('/docs/project/status')}>{ui.status}</a>
          <a className="hover:text-fg" href="https://github.com/sponsors/marprojector">{ui.sponsor}</a>
        </div>
      </div>
    </footer>
  )
}
