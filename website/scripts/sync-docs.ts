import { writeFileSync, mkdirSync } from 'fs';
import { join, dirname } from 'path';

const GITHUB_REPO = 'hars-21/reqsh';
const GITHUB_BRANCH = 'main';
const CONTENT_DIR = join(import.meta.dirname, '..', 'content', 'docs');
const RAW = 'https://raw.githubusercontent.com';

const DOCS_TO_SYNC = [
  { remote: 'docs/introduction.md', slug: 'introduction' },
  { remote: 'docs/install.md', slug: 'install' },
  { remote: 'docs/usage.md', slug: 'usage' },
  { remote: 'docs/commands.md', slug: 'commands' },
  { remote: 'docs/variables.md', slug: 'variables' },
  { remote: 'docs/migration.md', slug: 'migration' },
  { remote: 'CHANGELOG.md', slug: 'changelog' },
  { remote: 'ROADMAP.md', slug: 'roadmap' },
];

async function sync() {
  mkdirSync(CONTENT_DIR, { recursive: true });

  let count = 0;
  for (const { remote, slug } of DOCS_TO_SYNC) {
    const url = `${RAW}/${GITHUB_REPO}/${GITHUB_BRANCH}/${remote}`;
    const destPath = join(CONTENT_DIR, `${slug}.md`);

    const res = await globalThis.fetch(url);
    if (!res.ok) {
      console.warn(`  \u26A0\uFE0F  ${remote} not found (${res.status}), skipping`);
      continue;
    }

    const content = await res.text();
    mkdirSync(dirname(destPath), { recursive: true });
    writeFileSync(destPath, content, 'utf-8');
    count++;
    console.log(`  ${remote} -> content/docs/${slug}.md`);
  }

  console.log(`\n\u2713 Synced ${count} docs`);
}

sync().catch((err) => {
  console.error('Sync failed:', err);
  process.exit(1);
});
