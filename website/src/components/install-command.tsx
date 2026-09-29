'use client';

import { useState } from 'react';
import { Check, Copy } from 'lucide-react';
import posthog from 'posthog-js';

const installCommand = 'curl -fsSL https://reqsh.dev/install.sh | sh';

export default function InstallCommand({ location = 'unknown' }: { location?: string }) {
  const [copied, setCopied] = useState(false);

  async function copyCommand() {
    await navigator.clipboard.writeText(installCommand);
    posthog.capture('install_command_copied', { location });
    setCopied(true);
    window.setTimeout(() => setCopied(false), 2000);
  }

  return (
    <div className="flex min-w-0 items-center gap-3 bg-fd-card px-5 py-3.5 font-mono text-xs sm:px-6 sm:text-sm lg:px-8">
      <span className="shrink-0 text-brand">$</span>
      <code className="min-w-0 flex-1 overflow-x-auto whitespace-nowrap py-1">
        {installCommand}
      </code>
      <button
        type="button"
        onClick={copyCommand}
        className="inline-flex h-8 shrink-0 items-center justify-center gap-2 rounded-md border border-fd-border px-2.5 font-sans text-xs font-medium text-fd-muted-foreground transition-colors hover:border-brand/40 hover:bg-fd-accent hover:text-fd-foreground sm:px-3"
        aria-label={copied ? 'Install command copied' : 'Copy install command'}
      >
        {copied ? <Check size={14} className="text-brand" /> : <Copy size={14} />}
        <span className="hidden sm:inline">{copied ? 'Copied' : 'Copy'}</span>
      </button>
      <span className="sr-only" aria-live="polite">
        {copied ? 'Install command copied' : ''}
      </span>
    </div>
  );
}
