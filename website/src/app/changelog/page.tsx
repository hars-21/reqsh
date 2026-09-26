import type { Metadata } from 'next';
import { readFileSync } from 'fs';
import { join } from 'path';
import MarkdownRenderer from '@/components/markdown-renderer';

export const dynamic = 'force-dynamic';

const siteUrl = 'https://reqsh.dev';

export const metadata: Metadata = {
  title: "What's New",
  description: 'Changelog and release history for reqsh.',
  alternates: {
    canonical: `${siteUrl}/changelog`,
    types: {
      'application/rss+xml': `${siteUrl}/changelog.xml`,
    },
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
  const filePath = join(process.cwd(), 'content', 'docs', 'changelog.md');
  const content = readFileSync(filePath, 'utf-8');

  return (
    <div className="mx-auto max-w-2xl px-6 py-10">
      <MarkdownRenderer content={content} />

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
  );
}
