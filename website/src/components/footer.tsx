import Link from 'next/link';
import Image from 'next/image';

const resources = [
  { label: 'Documentation', href: '/docs' },
  { label: 'Installation', href: '/docs/install' },
  { label: 'Roadmap', href: '/roadmap' },
  { label: 'Changelog', href: '/changelog' },
];

const community = [
  { label: 'GitHub', href: 'https://github.com/hars-21/reqsh' },
  { label: 'Issues', href: 'https://github.com/hars-21/reqsh/issues' },
  { label: 'Releases', href: 'https://github.com/hars-21/reqsh/releases' },
  { label: 'License', href: 'https://github.com/hars-21/reqsh/blob/main/LICENSE' },
];

export default function Footer() {
  return (
    <footer className="mt-20 md:mt-32 border-t border-border" role="contentinfo">
      <div className="mx-auto max-w-5xl px-4 sm:px-6 py-12 md:py-20">
        <div className="flex flex-col justify-between gap-10 md:flex-row md:gap-16">
          <div className="max-w-xs">
            <span className="flex items-center gap-2.5 font-mono text-lg font-semibold tracking-tight text-foreground">
              <Image
                src="/logo.svg"
                alt="reqsh logo"
                width={24}
                height={24}
                className="rounded-md"
                priority
              />
              reqsh
            </span>
            <p className="mt-4 text-sm leading-relaxed text-foreground">
              An interactive, persistent shell for HTTP requests. Built with Rust for speed and
              simplicity.
            </p>
          </div>

          <div className="flex flex-wrap gap-8 sm:gap-12 md:gap-20">
            <nav aria-label="Resources">
              <h3 className="mb-5 text-sm font-semibold text-foreground">Resources</h3>
              <ul className="space-y-3.5">
                {resources.map((item) => (
                  <li key={item.href}>
                    <Link
                      href={item.href}
                      className="text-sm text-muted-foreground transition-snappy hover:text-foreground"
                    >
                      {item.label}
                    </Link>
                  </li>
                ))}
              </ul>
            </nav>
            <nav aria-label="Community">
              <h3 className="mb-5 text-sm font-semibold text-foreground">Community</h3>
              <ul className="space-y-3.5">
                {community.map((item) => (
                  <li key={item.href}>
                    <a
                      href={item.href}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-sm text-muted-foreground transition-snappy hover:text-foreground"
                    >
                      {item.label}
                    </a>
                  </li>
                ))}
              </ul>
            </nav>
          </div>
        </div>

        <div className="mt-16 flex flex-col items-center justify-between gap-3 border-t border-border pt-8 text-sm text-foreground md:flex-row">
          <p>© {new Date().getFullYear()} reqsh. Open source (MIT).</p>
          <p className="font-mono text-xs">macOS · Linux · Windows</p>
        </div>
      </div>
    </footer>
  );
}
