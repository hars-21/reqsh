'use client';

import { useState } from 'react';
import { Check, Copy } from 'lucide-react';

const installCommand =
  'curl -fsSL https://github.com/hars-21/reqsh/releases/latest/download/reqsh-installer.sh | sh';

export default function InstallCommand() {
  const [copied, setCopied] = useState(false);

  async function copyCommand() {
    await navigator.clipboard.writeText(installCommand);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 2000);
  }

  return (
    <div className="flex min-w-0 items-center gap-3 rounded-lg border border-terminal-border bg-terminal px-4 py-3 font-mono text-xs text-terminal-foreground sm:text-sm">
      <span className="shrink-0 text-terminal-accent">$</span>
      <code className="min-w-0 flex-1 overflow-x-auto whitespace-nowrap py-1">
        {installCommand}
      </code>
      <button
        type="button"
        onClick={copyCommand}
        className="inline-flex size-8 shrink-0 items-center justify-center rounded-md text-terminal-muted transition-colors hover:bg-white/8 hover:text-terminal-foreground"
        aria-label={copied ? 'Install command copied' : 'Copy install command'}
      >
        {copied ? <Check size={15} className="text-terminal-success" /> : <Copy size={15} />}
      </button>
      <span className="sr-only" aria-live="polite">
        {copied ? 'Install command copied' : ''}
      </span>
    </div>
  );
}
