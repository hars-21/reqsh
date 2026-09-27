import Link from 'next/link';

export default function Footer() {
  return (
    <footer className="mt-20 border-t border-border">
      <div className="mx-auto flex max-w-6xl flex-col gap-5 px-5 py-8 text-sm text-muted-foreground sm:flex-row sm:items-center sm:justify-between sm:px-8">
        <p>reqsh is open source under the MIT License.</p>
        <nav className="flex items-center gap-5" aria-label="Footer navigation">
          <Link href="/docs" className="transition-colors hover:text-foreground">
            Docs
          </Link>
          <Link href="/changelog" className="transition-colors hover:text-foreground">
            Changelog
          </Link>
          <a
            href="https://github.com/hars-21/reqsh"
            className="transition-colors hover:text-foreground"
            target="_blank"
            rel="noreferrer"
          >
            GitHub
          </a>
        </nav>
      </div>
    </footer>
  );
}
