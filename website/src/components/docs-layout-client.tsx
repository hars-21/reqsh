'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { ArrowLeft, ArrowRight, Menu, X } from 'lucide-react';
import { useState, useEffect, useCallback, startTransition } from 'react';
import { cn } from '@/lib/utils';
import type { DocNode } from '@/lib/docs';

interface SidebarItem {
  title: string;
  href: string;
  depth: number;
}

function getPrevNext(
  slug: string,
  allDocs: DocNode[]
): { prev: DocNode | null; next: DocNode | null } {
  const idx = allDocs.findIndex((d) => d.slug === slug);
  if (idx === -1) return { prev: null, next: null };
  return {
    prev: idx > 0 ? allDocs[idx - 1] : null,
    next: idx < allDocs.length - 1 ? allDocs[idx + 1] : null,
  };
}

export default function DocsLayoutClient({
  children,
  sidebarItems,
  allDocs,
}: {
  children: React.ReactNode;
  sidebarItems: SidebarItem[];
  allDocs: DocNode[];
}) {
  const pathname = usePathname();
  const [mobileOpen, setMobileOpen] = useState(false);

  const currentSlug = pathname.replace(/^\/docs\//, '').replace(/^\/docs$/, 'introduction');
  const { prev, next } = getPrevNext(currentSlug, allDocs);

  const closeMobile = useCallback(() => {
    startTransition(() => {
      setMobileOpen(false);
    });
  }, []);

  useEffect(() => {
    closeMobile();
  }, [pathname, closeMobile]);

  useEffect(() => {
    if (mobileOpen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
    return () => {
      document.body.style.overflow = '';
    };
  }, [mobileOpen]);

  return (
    <div className="mx-auto max-w-7xl px-4 sm:px-6">
      <div className="flex min-h-[calc(100svh-3.5rem)] gap-8 lg:gap-12">
        <button
          onClick={() => setMobileOpen(!mobileOpen)}
          className="fixed bottom-5 right-5 z-60 flex size-12 items-center justify-center rounded-full border border-border bg-background shadow-xl lg:hidden text-foreground hover:bg-muted"
          aria-label={mobileOpen ? 'Close navigation' : 'Open navigation'}
        >
          {mobileOpen ? <X size={20} /> : <Menu size={20} />}
        </button>

        {mobileOpen && (
          <div
            className="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm lg:hidden"
            onClick={closeMobile}
            aria-hidden="true"
          />
        )}

        <aside
          className={cn(
            'fixed top-14 bottom-0 z-50 flex w-64 flex-col border-r border-border/60 bg-background transition-transform duration-200 ease-out lg:sticky lg:translate-x-0 lg:shrink-0',
            mobileOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'
          )}
          style={{ height: 'calc(100svh - 3.5rem)' }}
          aria-label="Documentation navigation"
        >
          <nav className="flex flex-1 flex-col gap-0.5 overflow-y-auto px-3 py-6" role="navigation">
            {sidebarItems.map((item) => {
              const isActive = pathname === item.href;
              return (
                <Link
                  key={item.href}
                  href={item.href}
                  onClick={closeMobile}
                  className={cn(
                    'rounded-md px-2.5 py-1.5 text-sm transition-colors',
                    isActive
                      ? 'font-medium text-accent'
                      : 'text-muted-foreground hover:text-foreground',
                    item.depth > 0 && 'pl-6'
                  )}
                  aria-current={isActive ? 'page' : undefined}
                >
                  {item.title}
                </Link>
              );
            })}
          </nav>
        </aside>

        <main className="flex min-w-0 flex-1 flex-col py-8 sm:py-10">
          <article className="w-full max-w-2xl overflow-hidden">{children}</article>

          <div className="mt-16 border-t border-border pt-6">
            <div className="flex flex-col sm:flex-row items-stretch sm:items-center gap-3.5 sm:gap-4">
              {prev ? (
                <Link
                  href={`/docs/${prev.slug}`}
                  className="group flex-1 flex items-center gap-3 rounded-lg border border-border/60 px-4 py-3 transition-colors hover:border-border hover:bg-muted/30"
                >
                  <ArrowLeft
                    size={14}
                    className="text-muted-foreground shrink-0 transition-colors group-hover:text-foreground"
                  />
                  <div className="flex flex-col min-w-0">
                    <span className="text-[11px] text-muted-foreground">Previous</span>
                    <span className="text-sm font-medium text-foreground truncate">
                      {prev.title}
                    </span>
                  </div>
                </Link>
              ) : (
                <div className="hidden sm:block sm:flex-1" />
              )}

              {next ? (
                <Link
                  href={`/docs/${next.slug}`}
                  className="group flex-1 flex items-center justify-end gap-3 rounded-lg border border-border/60 px-4 py-3 transition-colors hover:border-border hover:bg-muted/30"
                >
                  <div className="flex flex-col text-right min-w-0">
                    <span className="text-[11px] text-muted-foreground">Next</span>
                    <span className="text-sm font-medium text-foreground truncate">
                      {next.title}
                    </span>
                  </div>
                  <ArrowRight
                    size={14}
                    className="text-muted-foreground shrink-0 transition-colors group-hover:text-foreground"
                  />
                </Link>
              ) : (
                <div className="hidden sm:block sm:flex-1" />
              )}
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}
