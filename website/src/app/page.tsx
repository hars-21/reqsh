import Link from 'next/link';
import { ArrowRight, ArrowUpRight } from 'lucide-react';
import { HomeLayout } from 'fumadocs-ui/layouts/home';
import Footer from '@/components/footer';
import InstallCommand from '@/components/install-command';
import { baseOptions } from '@/lib/layout.shared';
import Image from 'next/image';

const sessionFeatures = [
  {
    title: 'Variables',
    description: 'Keep base URLs, tokens, and values available throughout the session.',
  },
  {
    title: 'Saved requests',
    description: 'Give useful requests a name and run them again without rebuilding them.',
  },
  {
    title: 'Persistent context',
    description: 'Reopen reqsh and continue with your headers, variables, and requests intact.',
  },
];

export default function Home() {
  return (
    <HomeLayout
      {...baseOptions()}
      id="main-content"
      className="bg-fd-background text-fd-foreground"
      tabIndex={-1}
    >
      <section className="border-b border-fd-border" aria-labelledby="hero-heading">
        <div className="mx-auto grid w-full max-w-(--fd-layout-width) sm:border-x sm:border-fd-border lg:min-h-160 lg:grid-cols-[0.88fr_1.12fr]">
          <div className="relative flex flex-col justify-center px-5 py-16 sm:px-10 sm:py-20 lg:border-r lg:border-fd-border lg:px-14 lg:py-24 xl:px-20">
            <span
              className="absolute top-0 left-5 h-1 w-20 bg-brand sm:left-10 lg:left-14 xl:left-20"
              aria-hidden="true"
            />
            <h1
              id="hero-heading"
              className="max-w-2xl text-[clamp(2.75rem,4.5vw,4.75rem)] leading-[0.96] font-semibold tracking-[-0.055em] text-balance"
            >
              HTTP requests, without starting over.
            </h1>
            <p className="mt-7 max-w-xl text-base leading-7 text-fd-muted-foreground sm:text-lg sm:leading-8">
              Build, send, save, and replay requests from an interactive shell. Set the URL,
              headers, and variables once, then keep working at the prompt.
            </p>
            <div className="mt-9 flex flex-wrap items-center gap-4">
              <Link
                href="/docs/install"
                className="inline-flex h-11 items-center gap-2 rounded-md bg-brand px-5 text-sm font-semibold text-brand-foreground transition-colors hover:bg-brand-strong"
              >
                Install reqsh
                <ArrowRight size={16} aria-hidden="true" />
              </Link>
              <Link
                href="/docs"
                className="inline-flex h-11 items-center border-b border-fd-foreground/35 text-sm font-medium transition-colors hover:border-brand hover:text-brand"
              >
                Read the docs
              </Link>
            </div>
          </div>

          <div className="flex min-w-0 items-center bg-fd-muted/45 px-5 py-10 sm:px-10 sm:py-14 lg:px-14">
            <figure className="w-full overflow-hidden rounded-lg border border-terminal-border bg-terminal shadow-[0_24px_70px_-38px_rgba(16,10,12,0.72)]">
              <video
                src="/landing/live-terminal.mp4"
                aria-label="A reqsh terminal session sending an HTTP request and receiving a response"
                width={960}
                height={660}
                controls
                muted
                playsInline
                autoPlay
                loop
                preload="metadata"
                className="aspect-16/11 w-full object-cover"
              />
            </figure>
          </div>
        </div>
      </section>

      <section className="border-b border-fd-border" aria-label="Install reqsh">
        <div className="mx-auto w-full max-w-(--fd-layout-width) sm:border-x sm:border-fd-border">
          <InstallCommand location="home_install_bar" />
        </div>
      </section>

      <section className="border-b border-fd-border" aria-labelledby="context-heading">
        <div className="mx-auto w-full max-w-(--fd-layout-width) sm:border-x sm:border-fd-border">
          <div className="px-5 pt-16 pb-10 sm:px-10 sm:pt-20 lg:px-14 lg:pt-24 xl:px-20">
            <h2
              id="context-heading"
              className="max-w-3xl text-3xl leading-tight font-semibold tracking-[-0.045em] text-balance sm:text-5xl"
            >
              One prompt. Your whole request context.
            </h2>
          </div>

          <div className="grid border-t border-fd-border md:grid-cols-3">
            {sessionFeatures.map((feature) => (
              <article
                key={feature.title}
                className="border-b border-fd-border px-5 py-8 last:border-b-0 sm:px-10 md:border-r md:border-b-0 md:last:border-r-0 lg:px-12 lg:py-11"
              >
                <h3 className="text-lg font-semibold tracking-[-0.02em]">{feature.title}</h3>
                <p className="mt-3 max-w-sm leading-7 text-fd-muted-foreground">
                  {feature.description}
                </p>
              </article>
            ))}
          </div>
        </div>
      </section>

      <section className="border-b border-fd-border" aria-labelledby="readable-heading">
        <div className="mx-auto w-full max-w-(--fd-layout-width) sm:border-x sm:border-fd-border">
          <div className="grid lg:grid-cols-[0.62fr_1.38fr]">
            <div className="px-5 py-16 sm:px-10 sm:py-20 lg:border-r lg:border-fd-border lg:px-14 lg:py-24 xl:px-20">
              <h2
                id="readable-heading"
                className="text-3xl leading-tight font-semibold tracking-[-0.04em] text-balance sm:text-4xl"
              >
                Write requests like you read them.
              </h2>
              <p className="mt-5 max-w-md leading-7 text-fd-muted-foreground">
                Put the method, path, headers, and body on separate lines. End with{' '}
                <code className="rounded-sm border border-fd-border bg-fd-muted px-1.5 py-0.5 font-mono text-sm text-fd-foreground">
                  ###
                </code>{' '}
                and read the formatted response beside it.
              </p>
            </div>

            <div className="grid min-w-0 border-t border-fd-border bg-fd-muted/35 sm:grid-cols-2 lg:border-t-0">
              <figure className="flex min-w-0 flex-col border-b border-fd-border sm:border-r sm:border-b-0">
                <figcaption className="border-b border-fd-border px-5 py-3 text-sm font-medium">
                  Request
                </figcaption>
                <div className="relative aspect-4/3 w-full overflow-hidden">
                  <Image
                    src="/landing/request.png"
                    alt="A multiline POST request written in reqsh"
                    fill
                    sizes="(min-width: 1024px) 34vw, (min-width: 640px) 50vw, 100vw"
                    className="object-cover"
                  />
                </div>
              </figure>
              <figure className="flex min-w-0 flex-col">
                <figcaption className="flex items-center justify-between border-b border-fd-border px-5 py-3 text-sm font-medium">
                  <span>Response</span>
                  <span className="font-mono text-xs font-normal text-emerald-700 dark:text-emerald-400">
                    201 Created
                  </span>
                </figcaption>
                <div className="relative aspect-4/3 w-full overflow-hidden">
                  <Image
                    src="/landing/response.png"
                    alt="A formatted successful HTTP response displayed in reqsh"
                    fill
                    sizes="(min-width: 1024px) 34vw, (min-width: 640px) 50vw, 100vw"
                    className="object-cover"
                  />
                </div>
              </figure>
            </div>
          </div>
        </div>
      </section>

      <section className="border-b border-fd-border" aria-labelledby="repeat-heading">
        <div className="mx-auto grid w-full max-w-(--fd-layout-width) sm:border-x sm:border-fd-border lg:grid-cols-[1.18fr_0.82fr]">
          <div className="min-w-0 bg-fd-muted/35 p-5 sm:p-10 lg:border-r lg:border-fd-border lg:p-14">
            <figure className="overflow-hidden rounded-lg border border-terminal-border bg-terminal">
              <video
                src="/landing/history.mp4"
                aria-label="Using reqsh history to find and rerun an earlier request"
                width={960}
                height={620}
                controls
                muted
                autoPlay
                loop
                playsInline
                preload="metadata"
                className="aspect-48/31 w-full object-cover"
              />
            </figure>
          </div>

          <div className="flex flex-col justify-center px-5 py-16 sm:px-10 sm:py-20 lg:px-14 lg:py-24 xl:px-20">
            <h2
              id="repeat-heading"
              className="text-3xl leading-tight font-semibold tracking-[-0.045em] text-balance sm:text-5xl"
            >
              Repeat, don’t rebuild.
            </h2>
            <p className="mt-5 max-w-md leading-7 text-fd-muted-foreground">
              Find any earlier command, rerun it by ID, or save a useful request under a memorable
              name.
            </p>
            <dl className="mt-8 border-y border-fd-border font-mono text-sm">
              <div className="grid gap-1 border-b border-fd-border py-4 sm:grid-cols-[1fr_auto] sm:items-center sm:gap-6">
                <dt className="text-fd-foreground">history rerun 14</dt>
                <dd className="font-sans text-sm text-fd-muted-foreground">Recall any command</dd>
              </div>
              <div className="grid gap-1 py-4 sm:grid-cols-[1fr_auto] sm:items-center sm:gap-6">
                <dt className="text-fd-foreground">req save get-posts</dt>
                <dd className="font-sans text-sm text-fd-muted-foreground">Name useful requests</dd>
              </div>
            </dl>
          </div>
        </div>
      </section>

      <section className="border-b border-fd-border" aria-labelledby="cta-heading">
        <div className="mx-auto flex w-full max-w-(--fd-layout-width) flex-col items-start gap-8 px-5 py-16 sm:border-x sm:border-fd-border sm:px-10 sm:py-20 md:flex-row md:items-end md:justify-between lg:px-14 lg:py-24 xl:px-20">
          <div>
            <h2
              id="cta-heading"
              className="max-w-3xl text-3xl leading-tight font-semibold tracking-[-0.045em] text-balance sm:text-5xl"
            >
              Your next request starts at the prompt.
            </h2>
            <p className="mt-4 max-w-xl leading-7 text-fd-muted-foreground">
              Install reqsh, open a shell, and send the first request in a few commands.
            </p>
          </div>
          <div className="flex shrink-0 flex-wrap items-center gap-4">
            <Link
              href="/docs/install"
              className="inline-flex h-11 items-center gap-2 rounded-md bg-brand px-5 text-sm font-semibold text-brand-foreground transition-colors hover:bg-brand-strong"
            >
              Install reqsh
              <ArrowRight size={16} aria-hidden="true" />
            </Link>
            <a
              href="https://github.com/hars-21/reqsh"
              target="_blank"
              rel="noreferrer"
              className="inline-flex h-11 items-center gap-2 rounded-md border border-fd-border bg-fd-background px-5 text-sm font-medium transition-colors hover:border-fd-foreground/40 hover:bg-fd-muted"
            >
              View on GitHub
              <ArrowUpRight size={15} aria-hidden="true" />
            </a>
          </div>
        </div>
      </section>

      <Footer />
    </HomeLayout>
  );
}
