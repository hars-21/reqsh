import type { MetadataRoute } from 'next';
import { getAllDocs } from '@/lib/docs';

export default function sitemap(): MetadataRoute.Sitemap {
  const baseUrl = 'https://reqsh.dev';

  const allDocs = getAllDocs();

  const docPages = allDocs.map((doc) => ({
    url: `${baseUrl}/docs/${doc.slug}`,
    lastModified: new Date(),
    changeFrequency: 'monthly' as const,
    priority: doc.slug === 'introduction' ? 0.9 : 0.8,
  }));

  return [
    { url: baseUrl, lastModified: new Date(), changeFrequency: 'monthly', priority: 1 },
    ...docPages,
    {
      url: `${baseUrl}/changelog`,
      lastModified: new Date(),
      changeFrequency: 'weekly',
      priority: 0.7,
    },
    {
      url: `${baseUrl}/roadmap`,
      lastModified: new Date(),
      changeFrequency: 'monthly',
      priority: 0.7,
    },
  ];
}
