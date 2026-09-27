import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const websiteDir = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const source = resolve(websiteDir, '../CHANGELOG.md');
const target = resolve(websiteDir, 'content/changelog.mdx');

const markdown = await readFile(source, 'utf8');
const frontmatter = `---
title: Changelog
description: Release history for reqsh.
---

`;

await mkdir(dirname(target), { recursive: true });
await writeFile(target, `${frontmatter}${markdown}`, 'utf8');

console.log('CHANGELOG.md -> website/content/changelog.mdx');
