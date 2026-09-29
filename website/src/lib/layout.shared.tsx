import Image from 'next/image';
import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';

export function baseOptions({
  navOnlyLinks = false,
}: { navOnlyLinks?: boolean } = {}): BaseLayoutProps {
  return {
    nav: {
      title: (
        <span className="inline-flex items-center gap-2.5 font-semibold tracking-tight">
          <Image src="/logo.svg" alt="" width={22} height={22} />
          reqsh
        </span>
      ),
      url: '/',
    },
    githubUrl: 'https://github.com/hars-21/reqsh',
    links: [
      {
        text: 'Documentation',
        url: '/docs',
        on: navOnlyLinks ? 'nav' : 'all',
      },
      {
        text: 'Changelog',
        url: '/changelog',
        on: navOnlyLinks ? 'nav' : 'all',
      },
    ],
    themeSwitch: {
      mode: 'light-dark',
    },
  };
}
