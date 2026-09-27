<p align="center">
  <a href="https://reqsh.dev">
    <img src="public/readme-banner.png" alt="reqsh.dev">
  </a>
</p>

<p align="center">
  <strong>The interactive shell for HTTP requests.</strong>
</p>

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-red.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)

</div>

## About

This directory contains **[reqsh.dev](https://reqsh.dev)**, the landing page and documentation site for reqsh. It lives inside the main reqsh repository so the website and product documentation change together.

## Tech Stack

- [Next.js](https://nextjs.org) 16 (App Router)
- [Fumadocs](https://fumadocs.dev) for documentation
- [Tailwind CSS](https://tailwindcss.com) v4
- [TypeScript](https://www.typescriptlang.org)

## Getting Started

```sh
pnpm install
pnpm dev
```

The site compiles documentation from `content/docs/`. The changelog is copied from the repository
root before development and production builds:

```sh
pnpm sync:changelog
pnpm build
```

## Project Structure

```
src/
  app/
    page.tsx            # Landing page
    changelog/          # Generated changelog page
    docs/               # Fumadocs routes and layout
    install.sh/         # Install script proxy
  components/           # Shared site and MDX components
  lib/source.ts         # Fumadocs sources for repository content
content/
  docs/                 # Product documentation
  changelog.mdx         # Generated from ../CHANGELOG.md
scripts/
  sync-changelog.mjs    # Generates the changelog content
../src/                 # Rust CLI source
```

## License

[MIT](https://github.com/hars-21/reqsh/blob/main/LICENSE)
