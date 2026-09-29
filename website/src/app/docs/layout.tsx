import type { ReactNode } from 'react';
import { source } from '@/lib/source';
import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import { baseOptions } from '@/lib/layout.shared';

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <main id="main-content" className="min-h-screen" tabIndex={-1}>
      <DocsLayout
        tree={source.getPageTree()}
        {...baseOptions({ navOnlyLinks: true })}
        sidebar={{
          banner: (
            <p className="px-2 pt-2 text-sm font-semibold tracking-[-0.01em] text-fd-foreground">
              Documentation
            </p>
          ),
        }}
      >
        {children}
      </DocsLayout>
    </main>
  );
}
