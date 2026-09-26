import type { Metadata } from 'next';
import { notFound } from 'next/navigation';
import { getDocBySlug, getAllDocs } from '@/lib/docs';
import MarkdownRenderer from '@/components/markdown-renderer';

export const dynamic = 'force-dynamic';

const siteUrl = 'https://reqsh.dev';

interface DocPageProps {
  params: Promise<{ slug: string[] }>;
}

export async function generateStaticParams() {
  const all = getAllDocs();
  return all.map((doc) => ({
    slug: doc.slug.split('/'),
  }));
}

export async function generateMetadata({ params }: DocPageProps): Promise<Metadata> {
  const { slug: slugs } = await params;
  const slug = slugs.join('/');

  const doc = getDocBySlug(slug);
  if (!doc) return { title: 'Not Found' };

  const pageUrl = `${siteUrl}/docs/${slug}`;

  return {
    title: doc.title,
    description:
      doc.description ??
      `Documentation for reqsh. ${doc.title}. Learn how to use reqsh, the interactive HTTP REPL built in Rust.`,
    alternates: { canonical: pageUrl },
    openGraph: {
      title: `${doc.title} | reqsh`,
      description: doc.description ?? `Documentation for reqsh. ${doc.title}.`,
      type: 'article',
      url: pageUrl,
      siteName: 'reqsh',
    },
    twitter: {
      card: 'summary_large_image',
      title: `${doc.title} | reqsh`,
      description: doc.description ?? `Documentation for reqsh. ${doc.title}.`,
    },
  };
}

export default async function DocPage({ params }: DocPageProps) {
  const { slug: slugs } = await params;
  const slug = slugs.join('/');

  const doc = getDocBySlug(slug);
  if (!doc) notFound();

  const allDocs = getAllDocs();
  const docIndex = allDocs.findIndex((d) => d.slug === slug);

  const jsonLd = {
    '@context': 'https://schema.org',
    '@type': 'TechArticle',
    headline: doc.title,
    description: doc.description ?? `Documentation for reqsh. ${doc.title}.`,
    author: {
      '@type': 'Person',
      name: 'hars-21',
      url: 'https://github.com/hars-21',
    },
    publisher: {
      '@type': 'Organization',
      name: 'reqsh',
      url: siteUrl,
    },
    url: `${siteUrl}/docs/${slug}`,
    mainEntityOfPage: {
      '@type': 'WebPage',
      '@id': `${siteUrl}/docs/${slug}`,
    },
    position: docIndex >= 0 ? docIndex + 1 : 1,
  };

  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
      />
      <MarkdownRenderer content={doc.content} />
    </>
  );
}
