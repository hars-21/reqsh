import { readFileSync } from 'fs';
import { join } from 'path';

export const dynamic = 'force-dynamic';

const siteUrl = 'https://reqsh.dev';

function escapeXml(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&apos;');
}

function parseChangelog(
  markdown: string
): Array<{ version: string; date: string; content: string }> {
  const releases: Array<{ version: string; date: string; content: string }> = [];
  const lines = markdown.split('\n');
  let current: { version: string; date: string; content: string } | null = null;

  const heading = /^##\s+.*?\[v?(\d+\.\d+\.\d+)\].*?[-—]?\s*(\S*)/i;

  for (const line of lines) {
    const headingMatch = line.match(heading);
    if (headingMatch) {
      if (current) releases.push(current);
      current = {
        version: headingMatch[1],
        date: headingMatch[2]?.trim() || '',
        content: '',
      };
    } else if (current) {
      current.content += line + '\n';
    }
  }
  if (current) releases.push(current);

  return releases.slice(0, 20);
}

export async function GET() {
  const filePath = join(process.cwd(), 'content', 'docs', 'changelog.md');
  let markdown: string;
  try {
    markdown = readFileSync(filePath, 'utf-8');
  } catch {
    return new Response('Changelog not found', { status: 404 });
  }

  const releases = parseChangelog(markdown);

  const items = releases
    .map(
      (r) => `
    <item>
      <title>v${escapeXml(r.version)}</title>
      <link>${siteUrl}/changelog</link>
      <guid isPermaLink="false">reqsh-v${escapeXml(r.version)}</guid>
      <pubDate>${r.date ? new Date(r.date).toUTCString() : new Date().toUTCString()}</pubDate>
      <description>${escapeXml(r.content.trim())}</description>
    </item>`
    )
    .join('');

  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>reqsh Changelog</title>
    <link>${siteUrl}/changelog</link>
    <description>Release history and changelog for reqsh, the interactive HTTP REPL built in Rust.</description>
    <language>en-us</language>
    <atom:link href="${siteUrl}/changelog.xml" rel="self" type="application/rss+xml" />
    <lastBuildDate>${new Date().toUTCString()}</lastBuildDate>
    ${items}
  </channel>
</rss>`;

  return new Response(xml, {
    headers: {
      'Content-Type': 'application/rss+xml; charset=utf-8',
      'Cache-Control': 'public, max-age=3600, stale-while-revalidate=86400',
    },
  });
}
