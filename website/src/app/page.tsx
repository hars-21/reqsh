'use client';

import Link from 'next/link';
import { useState } from 'react';
import posthog from 'posthog-js';
import { ArrowRight, Copy, Check } from 'lucide-react';
import Reveal from '@/components/reveal';
import { Cmd, Code, FeatureRow, Fg, Line, Method, Muted, TerminalCard } from '@/components/feature';

export default function Home() {
  const [curlCopied, setCurlCopied] = useState(false);

  const handleCopyCurl = (location: 'hero' | 'cta') => {
    const cmd = 'curl -fsSL https://reqsh.dev/install.sh | sh';
    navigator.clipboard.writeText(cmd);
    setCurlCopied(true);
    setTimeout(() => setCurlCopied(false), 2000);
    posthog.capture('install_command_copied', { location });
  };

  return (
    <div className="relative">
      <div className="hero-glow pointer-events-none absolute inset-x-0 top-0 -z-10 h-184" />

      <div className="mx-auto max-w-6xl px-4 sm:px-6">
        <section
          className="flex flex-col items-center pt-16 pb-4 text-center sm:pt-20 md:pt-28 mb-16"
          aria-labelledby="hero-heading"
        >
          <Reveal delay={0.08}>
            <h1
              id="hero-heading"
              className="mt-6 max-w-4xl text-4xl leading-[1.1] font-bold tracking-tighter text-balance text-foreground sm:text-6xl md:text-7xl lg:text-[5.25rem]"
            >
              The interactive shell for <span className="text-accent">HTTP</span> requests.
            </h1>
          </Reveal>

          <Reveal delay={0.16}>
            <p className="mt-6 max-w-2xl text-base leading-relaxed text-pretty text-muted-foreground sm:text-lg md:mt-8 md:text-xl">
              Set a base URL once. Add headers once. Use variables, save requests, and re-run them
              from history. Pure terminal efficiency, built in Rust.
            </p>
          </Reveal>

          <Reveal delay={0.24}>
            <div className="mt-9 flex flex-col sm:flex-row items-center justify-center gap-3.5 w-full sm:w-auto px-2 sm:px-0">
              <Link
                href="/docs/install"
                className="group flex w-full sm:w-auto items-center justify-center gap-2 rounded-full bg-accent px-7 py-3.5 text-sm font-semibold text-accent-foreground transition-snappy hover:brightness-110 hover:shadow-[0_0_32px_-6px_color-mix(in_oklch,var(--accent)_60%,transparent)]"
              >
                Install reqsh
                <ArrowRight
                  size={15}
                  className="stroke-current transition-snappy group-hover:translate-x-1"
                />
              </Link>
              <Link
                href="/docs"
                className="flex w-full sm:w-auto items-center justify-center rounded-full border border-border bg-card px-7 py-3.5 text-sm font-semibold text-foreground transition-snappy hover:bg-muted"
              >
                Documentation
              </Link>
            </div>
          </Reveal>

          <Reveal delay={0.32}>
            <div className="mx-auto my-7 flex w-full max-w-md sm:max-w-lg items-center justify-between gap-2 rounded-xl border border-terminal-border bg-terminal px-3 py-2 font-mono text-[11px] sm:text-xs md:text-sm shadow-sm sm:px-4 sm:py-2.5">
              <div className="flex min-w-0 flex-1 items-center gap-2 overflow-x-auto scrollbar-none py-0.5">
                <span className="text-terminal-accent shrink-0 select-none text-xs sm:text-sm">
                  $
                </span>
                <span className="whitespace-nowrap text-terminal-foreground">
                  curl -fsSL https://reqsh.dev/install.sh | sh
                </span>
              </div>
              <button
                onClick={() => handleCopyCurl('hero')}
                className="shrink-0 rounded-md p-1 text-terminal-muted hover:bg-white/10 hover:text-terminal-foreground transition-colors"
                aria-label="Copy curl command"
              >
                {curlCopied ? (
                  <Check size={14} className="text-terminal-success" />
                ) : (
                  <Copy size={14} />
                )}
              </button>
            </div>
          </Reveal>

          <Reveal delay={0.4}>
            <div className="mx-auto w-full max-w-4xl overflow-hidden rounded-2xl border border-border/80 bg-card shadow-2xl">
              <video className="w-full h-auto block rounded-2xl" autoPlay loop muted playsInline>
                <source src="/demo.mp4" type="video/mp4" />
              </video>
            </div>
          </Reveal>
        </section>

        <section className="pb-8" aria-labelledby="features-heading">
          <Reveal>
            <h2
              id="features-heading"
              className="mx-auto mt-6 max-w-2xl text-center text-2xl font-bold tracking-tight text-balance text-foreground sm:text-3xl md:text-5xl"
            >
              Everything a terminal-first workflow needs.
            </h2>
          </Reveal>

          <FeatureRow
            title="A persistent REPL."
            description="Stop typing the same host and authentication headers over and over. Set your base URL and headers once per session, every subsequent request uses them automatically."
          >
            <TerminalCard>
              <Line prompt>
                <Cmd>base</Cmd> <Muted>https://api.stripe.com</Muted>
              </Line>
              <Line prompt>
                <Cmd>header set</Cmd> <Muted>Authorization Bearer sk_test</Muted>
              </Line>
              <div className="mt-4" />
              <Line prompt>
                <Method>GET</Method> <Fg>/v1/customers</Fg>
              </Line>
              <Line cont>
                <Fg>###</Fg>
              </Line>
            </TerminalCard>
          </FeatureRow>

          <FeatureRow
            reverse
            title="Time-travel with history."
            description={
              <>
                Every command is saved. Type <Code>history</Code> to see everything you&apos;ve done
                in the current session. Made a typo or need to repeat a request? Just use{' '}
                <Code>rerun &lt;id&gt;</Code> to execute it instantly.
              </>
            }
          >
            <TerminalCard>
              <Line prompt>
                <Cmd>history</Cmd>
              </Line>
              <div className="mt-2 ml-4 space-y-0.5">
                <div className="flex gap-4">
                  <Muted>1:</Muted>
                  <Muted>base https://api.stripe.com</Muted>
                </div>
                <div className="flex gap-4">
                  <Muted>2:</Muted>
                  <Muted>header set Authorization Bearer sk_test</Muted>
                </div>
                <div className="flex gap-4">
                  <Muted>3:</Muted>
                  <Muted>GET /v1/customers</Muted>
                </div>
              </div>
              <div className="mt-4" />
              <Line prompt>
                <Cmd>history rerun</Cmd> <Muted>3</Muted>
              </Line>
            </TerminalCard>
          </FeatureRow>

          <FeatureRow
            title="Variables. Save. Run."
            description={
              <>
                Use{' '}
                <Code>
                  {'{{'}name{'}}'}
                </Code>{' '}
                syntax to interpolate values into paths, headers, and bodies. Save any request with{' '}
                <Code>save</Code> and replay it instantly with <Code>run</Code>. Every response
                includes its timing.
              </>
            }
          >
            <TerminalCard>
              <Line prompt>
                <Cmd>var set</Cmd> <Muted>token eyJhbGciOiJIUzI1NiJ9</Muted>
              </Line>
              <div className="mt-4" />
              <Line prompt>
                <Method>GET</Method>{' '}
                <Fg>
                  /users/
                  <span className="text-terminal-accent">
                    {'{{'}token{'}}'}
                  </span>
                </Fg>
              </Line>
              <Line cont>
                <Fg>###</Fg>
              </Line>
              <div className="mt-3 text-xs text-terminal-success">HTTP/1.1 200 OK 142ms</div>
              <div className="mt-5" />
              <Line prompt>
                <Cmd>req save</Cmd> <Muted>get-users</Muted>
              </Line>
              <Line prompt>
                <Cmd>req run</Cmd> <Muted>get-users</Muted>
              </Line>
            </TerminalCard>
          </FeatureRow>
        </section>

        <section
          className="relative w-full overflow-hidden rounded-2xl sm:rounded-3xl border border-border bg-card px-4 py-10 sm:px-8 sm:py-16 md:py-24"
          aria-labelledby="cta-heading"
        >
          <div className="section-glow pointer-events-none absolute inset-0" />
          <Reveal className="relative z-10 mx-auto flex w-full max-w-2xl flex-col items-center text-center">
            <h2
              id="cta-heading"
              className="w-full text-2xl font-bold tracking-tight text-balance text-foreground sm:text-3xl md:text-5xl"
            >
              Ready to leave curl flags behind?
            </h2>
            <p className="mx-auto mt-4 max-w-lg text-xs leading-relaxed text-muted-foreground sm:text-base md:text-lg">
              One command. No config files, no accounts, no runtime. Just a fast shell for your API
              work.
            </p>

            <div className="mx-auto mt-6 flex w-full max-w-md items-center justify-between gap-2 rounded-xl border border-terminal-border bg-terminal px-3 py-2 font-mono text-[11px] sm:text-xs md:text-sm shadow-sm sm:px-4 sm:py-2.5">
              <div className="flex min-w-0 flex-1 items-center gap-2 overflow-x-auto scrollbar-none py-0.5">
                <span className="text-terminal-accent shrink-0 select-none text-xs sm:text-sm">
                  $
                </span>
                <span className="whitespace-nowrap text-terminal-foreground">
                  curl -fsSL https://reqsh.dev/install.sh | sh
                </span>
              </div>
              <button
                onClick={() => handleCopyCurl('cta')}
                className="shrink-0 rounded-md p-1 text-terminal-muted hover:bg-white/10 hover:text-terminal-foreground transition-colors"
                aria-label="Copy curl command"
              >
                {curlCopied ? (
                  <Check size={14} className="text-terminal-success" />
                ) : (
                  <Copy size={14} />
                )}
              </button>
            </div>

            <div className="mt-7 flex flex-col sm:flex-row items-center justify-center gap-3 w-full sm:w-auto px-2 sm:px-0">
              <Link
                href="/docs/install"
                className="group flex w-full sm:w-auto min-w-36 items-center justify-center gap-2 rounded-full bg-accent px-6 py-3 text-xs sm:text-sm font-semibold text-accent-foreground transition-snappy hover:brightness-110"
              >
                Get started
                <ArrowRight
                  size={15}
                  className="stroke-current transition-snappy group-hover:translate-x-1"
                />
              </Link>
              <a
                href="https://github.com/hars-21/reqsh"
                target="_blank"
                rel="noopener noreferrer"
                onClick={() => posthog.capture('github_link_clicked', { location: 'home_cta' })}
                className="flex w-full sm:w-auto min-w-36 items-center justify-center rounded-full border border-border bg-background px-6 py-3 text-xs sm:text-sm font-semibold text-foreground transition-snappy hover:bg-muted"
              >
                Star on GitHub
              </a>
            </div>
          </Reveal>
        </section>
      </div>
    </div>
  );
}
