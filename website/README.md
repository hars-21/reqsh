# reqsh website

This directory contains the [reqsh](https://reqsh.dev) landing page and documentation site. The
Rust CLI lives in the repository root; website changes should stay within this directory unless
they also require a product or documentation update.

## Stack

- Next.js 16 and React 19
- TypeScript
- Tailwind CSS 4
- Fumadocs for documentation and search

## Project structure

```text
src/
  app/          Routes, layouts, metadata, and global styles
  components/   Shared React and MDX components
  lib/          Fumadocs source and layout configuration
content/docs/   Documentation pages
public/         Static assets, including landing-page media
scripts/        Build-time utilities
```

## Local development

From this directory:

```sh
pnpm install
pnpm dev
```

The local site is available at [http://localhost:3000](http://localhost:3000).

## Commands

```sh
pnpm lint              # Run ESLint
pnpm build             # Create a production build
pnpm start             # Serve a completed production build
pnpm sync:changelog    # Regenerate content/changelog.mdx
pnpm format            # Format the repository with Prettier
```

`pnpm dev` and `pnpm build` automatically regenerate the changelog. Do not edit
`content/changelog.mdx` directly; update the repository-root `CHANGELOG.md` instead.

## Contributing

1. Create a focused branch and make your change.
2. Put documentation updates in `content/docs/`, route-level UI in `src/app/`, and reusable UI in
   `src/components/`.
3. Test affected pages in light and dark themes, including a narrow viewport for visual changes.
4. Run `pnpm lint` and `pnpm build` before opening a pull request.
5. In the pull request, describe the user-visible change and include screenshots or a recording for
   UI work.
