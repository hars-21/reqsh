import type { Metadata } from 'next';
import { Inter, Geist_Mono } from 'next/font/google';
import { SpeedInsights } from '@vercel/speed-insights/next';
import { Analytics } from '@vercel/analytics/next';
import { ThemeProvider } from 'next-themes';
import Nav from '@/components/nav';
import Footer from '@/components/footer';
import './globals.css';

const inter = Inter({
  variable: '--font-inter',
  subsets: ['latin'],
  display: 'swap',
});

const geistMono = Geist_Mono({
  variable: '--font-geist-mono',
  subsets: ['latin'],
  display: 'swap',
});

const siteUrl = 'https://reqsh.dev';

export const metadata: Metadata = {
  metadataBase: new URL(siteUrl),
  title: {
    default: 'reqsh | Interactive HTTP REPL',
    template: '%s | reqsh',
  },
  description:
    'An interactive REPL shell for HTTP requests. Set base URLs, manage headers, use variables, save and run requests, re-run from history. Supports GET, POST, PUT, PATCH, DELETE. Built in Rust. Open source.',
  keywords: [
    'reqsh',
    'HTTP shell',
    'terminal',
    'REPL',
    'Rust',
    'CLI',
    'HTTP client',
    'variables',
    'save requests',
    'PATCH',
    'API client',
  ],
  authors: [{ name: 'hars-21', url: 'https://github.com/hars-21' }],
  creator: 'hars-21',
  openGraph: {
    title: 'reqsh | Interactive HTTP REPL',
    description:
      'An interactive REPL shell for HTTP requests. Set base URLs, manage headers, use variables, save and run requests and re-run from history. Supports GET, POST, PUT, PATCH, DELETE.',
    type: 'website',
    locale: 'en_US',
    siteName: 'reqsh',
    url: siteUrl,
  },
  twitter: {
    card: 'summary_large_image',
    title: 'reqsh | Interactive HTTP REPL',
    description: 'An interactive REPL shell for HTTP requests. Built in Rust. Open source.',
  },
  robots: {
    index: true,
    follow: true,
  },
  alternates: {
    canonical: siteUrl,
    types: {
      'application/rss+xml': `${siteUrl}/changelog.xml`,
    },
  },

  icons: {
    icon: [
      {
        url: '/icon-16.png',
        sizes: '16x16',
        type: 'image/png',
      },
      {
        url: '/icon-32.png',
        sizes: '32x32',
        type: 'image/png',
      },
      {
        url: '/icon-192.png',
        sizes: '192x192',
        type: 'image/png',
      },
      {
        url: '/icon-512.png',
        sizes: '512x512',
        type: 'image/png',
      },
    ],
    apple: [
      {
        url: '/apple-touch-icon.png',
        sizes: '180x180',
      },
    ],
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const jsonLd = [
    {
      '@context': 'https://schema.org',
      '@type': 'SoftwareApplication',
      name: 'reqsh',
      applicationCategory: 'DeveloperApplication',
      operatingSystem: 'macOS, Linux, Windows',
      description:
        'An interactive REPL shell for HTTP requests. Set base URLs, manage headers, use variables, save and run requests and re-run from history. Supports GET, POST, PUT, PATCH, DELETE.',
      url: siteUrl,
      downloadUrl: `${siteUrl}/docs/install`,
      installationUrl: `${siteUrl}/docs/install`,
      author: {
        '@type': 'Person',
        name: 'hars-21',
        url: 'https://github.com/hars-21',
      },
      offers: {
        '@type': 'Offer',
        price: '0',
        priceCurrency: 'USD',
      },
      screenshot: `${siteUrl}/opengraph-image`,
      softwareVersion: '0.2.0',
      applicationSuite: 'Terminal',
      featureList: [
        'Interactive REPL for HTTP requests',
        'Persistent base URLs and headers',
        'Variable interpolation',
        'Request history and rerun',
        'Case-insensitive HTTP methods',
        'Pretty-printed JSON output',
        'Built-in request timing',
      ],
      repo: 'https://github.com/hars-21/reqsh',
    },
    {
      '@context': 'https://schema.org',
      '@type': 'WebSite',
      name: 'reqsh',
      url: siteUrl,
      description: 'An interactive REPL shell for HTTP requests. Built in Rust. Open source.',
      publisher: {
        '@type': 'Organization',
        name: 'reqsh',
        url: siteUrl,
      },
    },
    {
      '@context': 'https://schema.org',
      '@type': 'BreadcrumbList',
      itemListElement: [
        {
          '@type': 'ListItem',
          position: 1,
          name: 'Home',
          item: siteUrl,
        },
        {
          '@type': 'ListItem',
          position: 2,
          name: 'Documentation',
          item: `${siteUrl}/docs`,
        },
      ],
    },
  ];

  return (
    <html
      lang="en"
      className={`${inter.variable} ${geistMono.variable} bg-background`}
      data-scroll-behavior="smooth"
      suppressHydrationWarning
    >
      <head>
        <link rel="preconnect" href="https://raw.githubusercontent.com" />
        <link rel="preconnect" href="https://api.github.com" />
        <link rel="dns-prefetch" href="https://raw.githubusercontent.com" />
        <link rel="dns-prefetch" href="https://api.github.com" />
        <link rel="alternate" type="text/plain" href="/llms.txt" title="LLM-friendly docs" />
      </head>
      <body className="min-h-screen flex flex-col antialiased">
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
        />
        <ThemeProvider
          attribute="class"
          defaultTheme="dark"
          enableSystem={false}
          disableTransitionOnChange
        >
          <a
            href="#main-content"
            className="sr-only focus:not-sr-only focus:fixed focus:top-4 focus:left-4 focus:z-100 focus:rounded-lg focus:bg-accent focus:px-4 focus:py-2 focus:text-sm focus:font-semibold focus:text-accent-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background"
          >
            Skip to content
          </a>
          <Nav />
          <main id="main-content" className="flex-1" tabIndex={-1}>
            {children}
          </main>
          <Footer />
        </ThemeProvider>
        <Analytics />
        <SpeedInsights />
      </body>
    </html>
  );
}
