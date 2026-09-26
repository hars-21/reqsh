import { readFileSync, readdirSync, statSync } from 'fs';
import { join } from 'path';

const DOCS_DIR = join(process.cwd(), 'content', 'docs');

export interface DocNode {
  slug: string;
  title: string;
  description?: string;
  order: number;
  children: DocNode[];
}

export interface DocPage {
  slug: string;
  title: string;
  description?: string;
  order: number;
  content: string;
}

interface Frontmatter {
  title: string;
  description?: string;
  order: number;
}

function parseFrontmatter(raw: string): Frontmatter | null {
  const match = raw.match(/^---\s*\n([\s\S]*?)\n---\s*\n/);
  if (!match) return null;

  const meta: Record<string, string> = {};
  for (const line of match[1].split('\n')) {
    const kv = line.match(/^(\w+):\s*(.*)$/);
    if (kv) meta[kv[1]] = kv[2].trim();
  }

  if (!meta.title) return null;

  return {
    title: meta.title,
    description: meta.description,
    order: meta.order ? parseInt(meta.order, 10) : 999,
  };
}

function readDocFile(
  filePath: string
): { slug: string; content: string; meta: Frontmatter } | null {
  try {
    const raw = readFileSync(filePath, 'utf-8');
    const slug = filePath.split('/').pop()!.replace(/\.md$/, '');

    if (slug === 'changelog' || slug === 'roadmap') return null;

    const meta = parseFrontmatter(raw);
    if (!meta) return null;

    const body = raw.replace(/^---\s*\n[\s\S]*?\n---\s*\n/, '');
    return { slug, content: body, meta };
  } catch {
    return null;
  }
}

function scanDir(dir: string, prefix: string = ''): DocNode[] {
  const entries: DocNode[] = [];

  try {
    for (const name of readdirSync(dir)) {
      const fullPath = join(dir, name);
      const stat = statSync(fullPath);

      if (stat.isDirectory()) {
        const children = scanDir(fullPath, prefix);
        if (children.length === 0) continue;
        entries.push({
          slug: prefix ? `${prefix}/${name}` : name,
          title: name,
          order: 999,
          children,
        });
      } else if (name.endsWith('.md')) {
        const doc = readDocFile(fullPath);
        if (!doc) continue;

        const slug = prefix ? `${prefix}/${doc.slug}` : doc.slug;
        entries.push({
          slug,
          title: doc.meta.title,
          description: doc.meta.description,
          order: doc.meta.order,
          children: [],
        });
      }
    }
  } catch {
    return [];
  }

  entries.sort((a, b) => a.order - b.order);
  return entries;
}

function flattenTree(nodes: DocNode[]): DocNode[] {
  const result: DocNode[] = [];
  for (const node of nodes) {
    result.push(node);
    result.push(...flattenTree(node.children));
  }
  return result;
}

export function getSidebarTree(): DocNode[] {
  return scanDir(DOCS_DIR);
}

export function getAllDocs(): DocNode[] {
  return flattenTree(getSidebarTree());
}

export function getDocBySlug(slug: string): DocPage | null {
  const parts = slug.split('/');
  let dir = DOCS_DIR;
  for (let i = 0; i < parts.length - 1; i++) {
    dir = join(dir, parts[i]);
  }
  const filePath = join(dir, `${parts[parts.length - 1]}.md`);

  const doc = readDocFile(filePath);
  if (!doc) return null;

  return {
    slug: doc.slug,
    title: doc.meta.title,
    description: doc.meta.description,
    order: doc.meta.order,
    content: doc.content,
  };
}
