import type { Metadata } from 'next';
import { notFound } from 'next/navigation';
import { DocsBody } from 'fumadocs-ui/layouts/docs/page';
import { getMDXComponents } from '@/components/mdx';
import { changelog } from '@/lib/source';
import Footer from '@/components/footer';
import Nav from '@/components/nav';

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
    <>
      <Nav />
      <main id="main-content" className="flex-1" tabIndex={-1}>
        <div className="mx-auto max-w-2xl px-6 py-10">
          <DocsBody>
            <Content components={getMDXComponents()} />
          </DocsBody>

          <div className="mt-8">
            <a
              href="https://github.com/hars-21/reqsh/releases"
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm font-medium text-foreground underline underline-offset-4 transition-colors hover:text-accent"
            >
              View all releases on GitHub
            </a>
          </div>
        </div>
      </main>
      <Footer />
    </>
  );
}
