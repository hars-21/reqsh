import type { Metadata } from 'next';
import Link from 'next/link';
import { ArrowLeft } from 'lucide-react';
import Image from 'next/image';

export const metadata: Metadata = {
  title: 'Not Found',
  description: 'The page you are looking for does not exist.',
  robots: {
    index: false,
    follow: false,
  },
};

export default function NotFound() {
  return (
    <div className="mx-auto flex flex-col items-center justify-center py-24 px-6 text-center">
      <div className="flex items-center justify-center gap-4 mt-12">
        <Image src="/logo.svg" alt="reqsh logo" width={84} height={84} />

        <h1 className="text-7xl font-bold text-foreground" aria-label="404 Error">
          404
        </h1>
      </div>
      <h2 className="mt-12 text-xl font-semibold text-foreground/90">Page not found</h2>

      <p className="mt-4 max-w-lg text-lg leading-relaxed text-muted-foreground">
        Looks like this page took a wrong turn. The URL you entered doesn&apos;t exist or may have
        moved somewhere else.
      </p>

      <Link
        href="/"
        className="group mt-10 flex items-center justify-center gap-2 rounded-full bg-accent px-8 py-3.5 text-sm font-bold text-accent-foreground transition-snappy hover:brightness-110 hover:shadow-[0_0_30px_-5px_color-mix(in_oklch,var(--accent)_45%,transparent)] focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
      >
        <ArrowLeft size={16} className="transition-snappy group-hover:-translate-x-1" />
        Back to Home
      </Link>
    </div>
  );
}
