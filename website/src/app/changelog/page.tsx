import type { Metadata } from 'next';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import { ArrowRight, ArrowUpRight } from 'lucide-react';
import { HomeLayout } from 'fumadocs-ui/layouts/home';
import { DocsBody } from 'fumadocs-ui/layouts/docs/page';
import { getMDXComponents } from '@/components/mdx';
import { changelog } from '@/lib/source';
import Footer from '@/components/footer';
import { baseOptions } from '@/lib/layout.shared';

const siteUrl = 'https://reqsh.dev';

export const metadata: Metadata = {
  title: "What's New",
  description: 'Changelog and release history for reqsh.',
  alternates: {
    canonical: `${siteUrl}/changelog`,
  },
  openGraph: {
    title: "What's New | reqsh",
    description: 'Changelog and release history for reqsh.',
    type: 'article',
    url: `${siteUrl}/changelog`,
    siteName: 'reqsh',
  },
  twitter: {
    card: 'summary_large_image',
    title: "What's New | reqsh",
    description: 'Changelog and release history for reqsh.',
  },
};

export default async function ChangelogPage() {
  const page = changelog.get('changelog.mdx');
  if (!page) notFound();

  const Content = page.body;

  return (
    <HomeLayout
      {...baseOptions()}
      id="main-content"
      className="bg-fd-background text-fd-foreground"
      tabIndex={-1}
    >
      <section className="border-b border-fd-border" aria-labelledby="changelog-heading">
        <div className="mx-auto grid w-full max-w-(--fd-layout-width) sm:border-x sm:border-fd-border lg:min-h-97.5 lg:grid-cols-[0.88fr_1.12fr]">
          <div className="relative flex items-end px-5 py-16 sm:px-10 sm:py-20 lg:border-r lg:border-fd-border lg:px-14 lg:py-24 xl:px-20">
            <span
              className="absolute top-0 left-5 h-1 w-20 bg-brand sm:left-10 lg:left-14 xl:left-20"
              aria-hidden="true"
            />
            <h1
              id="changelog-heading"
              className="text-[clamp(3.5rem,8vw,8rem)] leading-[0.9] font-semibold tracking-[-0.07em] text-balance"
            >
              Changelog
            </h1>
          </div>

          <div className="flex flex-col justify-end border-t border-fd-border bg-fd-muted/45 px-5 py-12 sm:px-10 sm:py-16 lg:border-t-0 lg:px-14 lg:py-24 xl:px-20">
            <p className="max-w-xl text-lg leading-8 text-fd-muted-foreground sm:text-xl sm:leading-9">
              Every reqsh release, from new workflows to the smallest fixes. Newest updates appear
              first.
            </p>
            <a
              href="https://github.com/hars-21/reqsh/releases"
              target="_blank"
              rel="noopener noreferrer"
              className="mt-7 inline-flex w-fit items-center gap-2 border-b border-fd-foreground/35 pb-1 text-sm font-medium transition-colors hover:border-brand hover:text-brand"
            >
              View releases on GitHub
              <ArrowUpRight size={15} aria-hidden="true" />
            </a>
          </div>
        </div>
      </section>

      <section className="border-b border-fd-border" aria-label="Release history">
        <div className="mx-auto grid w-full max-w-(--fd-layout-width) sm:border-x sm:border-fd-border lg:grid-cols-[0.36fr_1fr]">
          <aside className="border-b border-fd-border px-5 py-10 sm:px-10 lg:border-r lg:border-b-0 lg:px-14 lg:py-16 xl:px-20">
            <div className="lg:sticky lg:top-24">
              <p className="text-lg font-semibold tracking-[-0.02em]">Release archive</p>
              <p className="mt-3 max-w-xs leading-7 text-fd-muted-foreground">
                Features, fixes, and contributor notes, ordered from newest to oldest.
              </p>

              <div className="mt-8 border-y border-fd-border py-5">
                <p className="font-mono text-xs text-fd-muted-foreground">Install the latest</p>
                <Link
                  href="/docs/install"
                  className="mt-3 inline-flex items-center gap-2 text-sm font-semibold text-brand transition-colors hover:text-brand-strong"
                >
                  Installation guide
                  <ArrowRight size={15} aria-hidden="true" />
                </Link>
              </div>
            </div>
          </aside>

          <article className="min-w-0 px-5 py-12 sm:px-10 sm:py-16 lg:px-14 lg:py-20 xl:px-20">
            <DocsBody className="changelog-content">
              <Content
                components={getMDXComponents({
                  h2: (props) => <h2 {...props} />,
                })}
              />
            </DocsBody>

            <div className="mt-16 border-t border-fd-border pt-8">
              <p className="max-w-lg leading-7 text-fd-muted-foreground">
                Looking for release assets, checksums, or older tags?
              </p>
              <a
                href="https://github.com/hars-21/reqsh/releases"
                target="_blank"
                rel="noopener noreferrer"
                className="mt-4 inline-flex items-center gap-2 text-sm font-semibold text-brand transition-colors hover:text-brand-strong"
              >
                Browse every GitHub release
                <ArrowUpRight size={15} aria-hidden="true" />
              </a>
            </div>
          </article>
        </div>
      </section>

      <Footer />
    </HomeLayout>
  );
}
