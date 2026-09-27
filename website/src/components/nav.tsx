import Image from 'next/image';
import Link from 'next/link';
import ThemeToggle from '@/components/theme-toggle';

export default function Nav() {
  return (
    <header className="sticky top-0 z-50 border-b border-border/70 bg-background/90 backdrop-blur-md">
      <nav
        className="mx-auto flex h-14 max-w-6xl items-center justify-between px-5 sm:px-8"
        aria-label="Main navigation"
      >
        <Link href="/" className="flex items-center gap-2.5 font-semibold tracking-tight">
          <Image src="/logo.svg" alt="" width={22} height={22} priority />
          reqsh
        </Link>

        <div className="flex items-center gap-1 sm:gap-2">
          <Link
            href="/docs"
            className="rounded-md px-3 py-2 text-sm text-muted-foreground transition-colors hover:text-foreground"
          >
            Docs
          </Link>
          <a
            href="https://github.com/hars-21/reqsh"
            target="_blank"
            rel="noreferrer"
            className="rounded-md px-3 py-2 text-sm text-muted-foreground transition-colors hover:text-foreground"
          >
            GitHub
          </a>
          <ThemeToggle />
          <Link
            href="/docs/install"
            className="ml-1 hidden rounded-md bg-foreground px-3.5 py-2 text-xs font-medium text-background transition-opacity hover:opacity-85 sm:inline-flex"
          >
            Install
          </Link>
        </div>
      </nav>
    </header>
  );
}
