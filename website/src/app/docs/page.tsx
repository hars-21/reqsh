import type { Metadata } from 'next';
import { getDocBySlug } from '@/lib/docs';
import MarkdownRenderer from '@/components/markdown-renderer';

export const dynamic = 'force-dynamic';

const siteUrl = 'https://reqsh.dev';

export const metadata: Metadata = {
  title: 'Docs',
  description:
    'Getting started with reqsh. Learn how to install, configure and send HTTP requests from your terminal.',
  alternates: {
    canonical: `${siteUrl}/docs`,
  },
  openGraph: {
    title: 'Documentation | reqsh',
    description:
      'Getting started with reqsh. Learn how to install, configure and send HTTP requests from your terminal.',
    type: 'article',
    url: `${siteUrl}/docs`,
    siteName: 'reqsh',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Documentation | reqsh',
    description:
      'Getting started with reqsh. Learn how to install, configure and send HTTP requests from your terminal.',
  },
};

export default async function DocsPage() {
  const doc = getDocBySlug('introduction');
  if (!doc) return <div>Documentation not found</div>;

  return <MarkdownRenderer content={doc.content} />;
}
