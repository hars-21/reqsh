'use client';

import Link from 'next/link';
import Image from 'next/image';
import posthog from 'posthog-js';
import { Star, Download, Menu } from 'lucide-react';
import { useState, useEffect } from 'react';
import ThemeToggle from '@/components/theme-toggle';
import { Sheet, SheetTrigger, SheetContent, SheetHeader, SheetTitle } from '@/components/ui/sheet';

export default function Nav() {
  const [scrolled, setScrolled] = useState(false);
  const [stars, setStars] = useState<number | null>(null);
  const [mobileOpen, setMobileOpen] = useState(false);

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 0);
    window.addEventListener('scroll', onScroll);
    return () => window.removeEventListener('scroll', onScroll);
  }, []);

  useEffect(() => {
    fetch('https://api.github.com/repos/hars-21/reqsh')
      .then((res) => res.json())
      .then((data) => {
        if (data.stargazers_count) setStars(data.stargazers_count);
      })
      .catch(() => {});
  }, []);

  return (
    <header
      className={`sticky top-0 z-50 w-full bg-background/95 backdrop-blur-md transition-colors ${scrolled ? 'border-b border-border/60' : ''}`}
    >
      <nav
        className="mx-auto flex h-14 max-w-7xl items-center justify-between px-4 sm:px-6"
        aria-label="Main navigation"
      >
        <div className="flex items-center gap-1">
          <Link href="/" className="group flex items-center gap-2.5 px-2.5 py-1.5">
            <Image
              src="/logo.svg"
              alt="reqsh logo"
              width={24}
              height={24}
              className="rounded-md"
              loading="eager"
            />
            <span className="text-lg font-bold text-foreground">reqsh</span>
          </Link>

          <div className="mx-1.5 hidden h-4 w-px bg-border md:block" />
          <Link
            href="/docs"
            className="hidden rounded-md px-2.5 py-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground md:block"
          >
            Docs
          </Link>
          <Link
            href="/roadmap"
            className="hidden rounded-md px-2.5 py-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground md:block"
          >
            Roadmap
          </Link>
          <Link
            href="/changelog"
            className="hidden rounded-md px-2.5 py-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground md:block"
          >
            Changelog
          </Link>
        </div>

        <div className="hidden items-center gap-0.5 md:flex">
          <a
            href="https://github.com/hars-21/reqsh"
            target="_blank"
            rel="noopener noreferrer"
            onClick={() => posthog.capture('github_link_clicked', { location: 'nav' })}
            className="group flex items-center gap-1.5 rounded-md px-2.5 py-1.5 text-sm font-medium text-muted-foreground transition-colors hover:text-foreground"
          >
            <Star size={14} className="transition-colors group-hover:text-accent" />
            <span className="font-mono text-xs tabular-nums">{stars !== null ? stars : '—'}</span>
          </a>

          <div className="mx-1.5 h-4 w-px bg-border" />

          <ThemeToggle />

          <div className="mx-1.5 h-4 w-px bg-border" />

          <Link
            href="/docs/install"
            className="mx-2.5 inline-flex items-center gap-1.5 rounded-md bg-foreground px-3 py-1.5 text-xs font-medium text-background transition-colors hover:bg-foreground/90"
          >
            <Download size={13} />
            Install
          </Link>
        </div>

        <div className="flex items-center gap-1.5 md:hidden">
          <ThemeToggle />
          <Sheet open={mobileOpen} onOpenChange={setMobileOpen}>
            <SheetTrigger
              render={
                <button
                  className="flex size-9 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
                  aria-label="Open menu"
                >
                  <Menu size={20} />
                </button>
              }
            ></SheetTrigger>
            <SheetContent side="right" className="flex flex-col justify-between w-4/5 max-w-sm p-6">
              <div className="flex flex-col gap-6">
                <SheetHeader className="p-0 text-left">
                  <SheetTitle
                    render={
                      <Link
                        href="/"
                        onClick={() => setMobileOpen(false)}
                        className="flex items-center gap-2.5 text-lg font-bold text-foreground"
                      >
                        <Image
                          src="/logo.svg"
                          alt="reqsh logo"
                          width={24}
                          height={24}
                          className="rounded-md"
                        />
                        <span>reqsh</span>
                      </Link>
                    }
                  />
                </SheetHeader>

                <nav className="flex flex-col gap-1 font-medium" aria-label="Mobile navigation">
                  <Link
                    href="/docs"
                    onClick={() => setMobileOpen(false)}
                    className="rounded-lg px-3 py-2.5 text-base text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
                  >
                    Docs
                  </Link>
                  <Link
                    href="/roadmap"
                    onClick={() => setMobileOpen(false)}
                    className="rounded-lg px-3 py-2.5 text-base text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
                  >
                    Roadmap
                  </Link>
                  <Link
                    href="/changelog"
                    onClick={() => setMobileOpen(false)}
                    className="rounded-lg px-3 py-2.5 text-base text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
                  >
                    Changelog
                  </Link>
                </nav>
              </div>

              <div className="flex flex-col gap-4 border-t border-border/60 pt-6">
                <a
                  href="https://github.com/hars-21/reqsh"
                  target="_blank"
                  rel="noopener noreferrer"
                  onClick={() => posthog.capture('github_link_clicked', { location: 'mobile_nav' })}
                  className="flex items-center justify-between rounded-lg border border-border px-3.5 py-2.5 text-sm font-medium text-foreground transition-colors hover:bg-muted"
                >
                  <span className="flex items-center gap-2">
                    <Star size={16} className="text-accent" />
                    <span>Star on GitHub</span>
                  </span>
                  <span className="font-mono text-xs tabular-nums text-muted-foreground">
                    {stars !== null ? stars : '—'}
                  </span>
                </a>

                <Link
                  href="/docs/install"
                  onClick={() => setMobileOpen(false)}
                  className="flex w-full items-center justify-center gap-2 rounded-lg bg-foreground py-3 text-sm font-semibold text-background transition-colors hover:bg-foreground/90"
                >
                  <Download size={16} />
                  Install reqsh
                </Link>
              </div>
            </SheetContent>
          </Sheet>
        </div>
      </nav>
    </header>
  );
}
