import type { Metadata } from 'next';
import { readFileSync } from 'fs';
import { join } from 'path';
import MarkdownRenderer from '@/components/markdown-renderer';

export const dynamic = 'force-dynamic';

const siteUrl = 'https://reqsh.dev';

export const metadata: Metadata = {
  title: 'Roadmap',
  description: 'Upcoming features and plans for reqsh.',
  alternates: {
    canonical: `${siteUrl}/roadmap`,
  },
  openGraph: {
    title: 'Roadmap | reqsh',
    description: 'Upcoming features and plans for reqsh.',
    type: 'article',
    url: `${siteUrl}/roadmap`,
    siteName: 'reqsh',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Roadmap | reqsh',
    description: 'Upcoming features and plans for reqsh.',
  },
};

function stripCheckboxes(markdown: string): string {
  return markdown.replace(/^(\s*[-*])\s+\[[ x]\]\s*/gm, '$1 ');
}

export default async function RoadmapPage() {
  const filePath = join(process.cwd(), 'content', 'docs', 'roadmap.md');
  const raw = readFileSync(filePath, 'utf-8');
  const content = stripCheckboxes(raw);

  return (
    <div className="mx-auto max-w-2xl px-6 py-10">
      <MarkdownRenderer content={content} />
    </div>
  );
}
