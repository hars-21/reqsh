import Image from 'next/image';
import Link from 'next/link';

export default function Footer() {
  return (
    <footer>
      <div className="mx-auto flex max-w-(--fd-layout-width) flex-col gap-5 px-5 py-7 text-sm text-fd-muted-foreground sm:flex-row sm:items-center sm:justify-between sm:border-x sm:border-fd-border sm:px-10 lg:px-14 xl:px-20">
        <div className="flex items-center gap-3">
          <Image src="/logo.svg" alt="" width={22} height={22} />
          <span className="font-semibold tracking-tight text-fd-foreground">reqsh</span>
          <span aria-hidden="true">·</span>
          <span>Open source under the MIT License</span>
        </div>
        <nav className="flex items-center gap-5" aria-label="Footer navigation">
          <Link href="/docs" className="transition-colors hover:text-fd-foreground">
            Documentation
          </Link>
          <Link href="/changelog" className="transition-colors hover:text-fd-foreground">
            Changelog
          </Link>
          <a
            href="https://github.com/hars-21/reqsh"
            className="transition-colors hover:text-fd-foreground"
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
