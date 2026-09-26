'use client';
import React, { useState, useMemo } from 'react';
import posthog from 'posthog-js';
import { Copy, Check } from 'lucide-react';

type Token = { text: string; className: string };

function tokenizeReplContent(content: string): Token[] {
  const tokens: Token[] = [];
  let i = 0;

  while (i < content.length) {
    // {{variable}}
    if (content[i] === '{' && content[i + 1] === '{') {
      const end = content.indexOf('}}', i + 2);
      if (end !== -1) {
        tokens.push({ text: content.slice(i, end + 2), className: 'syn-variable' });
        i = end + 2;
        continue;
      }
    }

    // URL
    if (content.slice(i).startsWith('https://') || content.slice(i).startsWith('http://')) {
      const match = content.slice(i).match(/^https?:\/\/\S+/);
      if (match) {
        tokens.push({ text: match[0], className: 'syn-url' });
        i += match[0].length;
        continue;
      }
    }

    // ::send
    if (content.slice(i).startsWith('::send')) {
      tokens.push({ text: '::send', className: 'syn-special' });
      i += 6;
      continue;
    }

    // param:
    if (content.slice(i).startsWith('param:')) {
      tokens.push({ text: 'param:', className: 'syn-keyword' });
      i += 6;
      continue;
    }

    // HTTP method
    const rest = content.slice(i);
    const methodMatch = rest.match(/^(GET|POST|PUT|PATCH|DELETE|HEAD|OPTIONS)\b/);
    if (methodMatch && (i === 0 || content[i - 1] === ' ')) {
      tokens.push({ text: methodMatch[0], className: 'syn-method' });
      i += methodMatch[0].length;
      continue;
    }

    // REPL command
    const cmdMatch = rest.match(
      /^(base|set|unset|header|headers|save|run|vars|requests|history|rerun|timeout|clear|help|exit)\b/
    );
    if (cmdMatch && (i === 0 || content[i - 1] === ' ')) {
      tokens.push({ text: cmdMatch[0], className: 'syn-command' });
      i += cmdMatch[0].length;
      continue;
    }

    // Quoted string
    if (content[i] === '"') {
      const end = content.indexOf('"', i + 1);
      if (end !== -1) {
        tokens.push({ text: content.slice(i, end + 1), className: 'syn-string' });
        i = end + 1;
        continue;
      }
    }

    // Default: consume until next special char
    let j = i + 1;
    while (j < content.length) {
      if (content[j] === '{' && content[j + 1] === '{') break;
      if (content[j] === '"' || content[j] === ':') break;
      if (content.slice(j).startsWith('https://') || content.slice(j).startsWith('http://')) break;
      if (content.slice(j).startsWith('::send') || content.slice(j).startsWith('param:')) break;
      const m = content
        .slice(j)
        .match(
          /^(GET|POST|PUT|PATCH|DELETE|HEAD|OPTIONS|base|set|unset|header|headers|save|run|vars|requests|history|rerun|timeout|clear|help|exit)\b/
        );
      if (m && (j === 0 || content[j - 1] === ' ')) break;
      j++;
    }
    tokens.push({ text: content.slice(i, j), className: 'text' });
    i = j;
  }

  return tokens;
}

function tokenizeShellLine(line: string): Token[] {
  const tokens: Token[] = [];
  let i = 0;

  while (i < line.length) {
    // Comment
    if (line[i] === '#' && (i === 0 || line[i - 1] === ' ')) {
      tokens.push({ text: line.slice(i), className: 'syn-comment' });
      return tokens;
    }

    // URL
    if (line.slice(i).startsWith('https://') || line.slice(i).startsWith('http://')) {
      const match = line.slice(i).match(/^https?:\/\/\S+/);
      if (match) {
        tokens.push({ text: match[0], className: 'syn-url' });
        i += match[0].length;
        continue;
      }
    }

    // Flag
    if (line[i] === '-' && (i === 0 || line[i - 1] === ' ')) {
      const match = line.slice(i).match(/^-{1,2}\S+/);
      if (match) {
        tokens.push({ text: match[0], className: 'syn-flag' });
        i += match[0].length;
        continue;
      }
    }

    // Pipe
    if (line[i] === '|') {
      tokens.push({ text: '|', className: 'syn-pipe' });
      i++;
      continue;
    }

    // Path
    if (line[i] === '~' || (line[i] === '.' && (line[i + 1] === '/' || line[i + 1] === '.'))) {
      const match = line.slice(i).match(/^(\.\.?\/|~\/)\S*/);
      if (match) {
        tokens.push({ text: match[0], className: 'syn-path' });
        i += match[0].length;
        continue;
      }
    }

    // Shell command
    const rest = line.slice(i);
    const cmdMatch = rest.match(/^(curl|git|cargo|mv|chmod|cd|reqsh|sh|mkdir|echo)\b/);
    if (cmdMatch && (i === 0 || line[i - 1] === ' ')) {
      tokens.push({ text: cmdMatch[0], className: 'syn-command' });
      i += cmdMatch[0].length;
      continue;
    }

    // Quoted string
    if (line[i] === '"') {
      const end = line.indexOf('"', i + 1);
      if (end !== -1) {
        tokens.push({ text: line.slice(i, end + 1), className: 'syn-string' });
        i = end + 1;
        continue;
      }
    }

    // Default
    let j = i + 1;
    while (j < line.length) {
      if (line[j] === '#' || line[j] === '|' || line[j] === '"') break;
      if (line.slice(j).startsWith('https://') || line.slice(j).startsWith('http://')) break;
      if (line[j] === '-' && line[j - 1] === ' ') break;
      if (line[j] === '~' || (line[j] === '.' && line[j + 1] === '/')) break;
      const m = line.slice(j).match(/^(curl|git|cargo|mv|chmod|cd|reqsh|sh|mkdir|echo)\b/);
      if (m && line[j - 1] === ' ') break;
      j++;
    }
    tokens.push({ text: line.slice(i, j), className: 'text' });
    i = j;
  }

  return tokens;
}

function tokenizeResponseLine(line: string): Token[] {
  const tokens: Token[] = [];

  // HTTP version + status
  const httpMatch = line.match(/^HTTP\/[\d.]+/);
  if (httpMatch) {
    tokens.push({ text: httpMatch[0], className: 'syn-http' });
    let i = httpMatch[0].length;

    // Status code
    const statusMatch = line.slice(i).match(/^\s*(\d{3})\s+(.*)/);
    if (statusMatch) {
      tokens.push({ text: statusMatch[1], className: getStatusCodeClass(statusMatch[1]) });
      i += statusMatch[0].indexOf(statusMatch[2]);
      const statusText = statusMatch[2];

      // Check for response time at end
      const timeMatch = statusText.match(/(\d+ms)$/);
      if (timeMatch) {
        const beforeTime = statusText.slice(0, statusText.length - timeMatch[0].length);
        if (beforeTime) tokens.push({ text: ' ' + beforeTime, className: 'text' });
        tokens.push({ text: timeMatch[1], className: 'syn-time' });
      } else {
        tokens.push({ text: ' ' + statusText, className: 'text' });
      }
    }
    return tokens;
  }

  // Response header
  const headerMatch = line.match(/^([\w-]+):\s*(.*)/);
  if (headerMatch) {
    tokens.push({ text: headerMatch[1] + ':', className: 'syn-header' });
    if (headerMatch[2]) {
      tokens.push({ text: ' ' + headerMatch[2], className: 'text' });
    }
    return tokens;
  }

  // JSON
  if (
    line.trim().startsWith('{') ||
    line.trim().startsWith('}') ||
    line.trim().startsWith('[') ||
    line.trim().startsWith(']') ||
    line.trim().startsWith('"')
  ) {
    tokens.push({ text: line, className: 'syn-string' });
    return tokens;
  }

  tokens.push({ text: line, className: 'text' });
  return tokens;
}

function getStatusCodeClass(code: string): string {
  const num = parseInt(code, 10);
  if (num >= 200 && num < 300) return 'syn-status-ok';
  if (num >= 300 && num < 400) return 'syn-url';
  if (num >= 400 && num < 500) return 'syn-status-warn';
  return 'syn-status-err';
}

type LineMode = 'repl' | 'response' | 'shell';

function detectLineMode(line: string): LineMode {
  const trimmed = line.trimStart();
  if (trimmed.startsWith('reqsh>') || trimmed.startsWith('.....>')) return 'repl';
  if (trimmed.startsWith('HTTP/')) return 'response';
  return 'shell';
}

function highlightLine(line: string, mode: LineMode): Token[] {
  if (mode === 'response') return tokenizeResponseLine(line);

  // REPL mode: extract prompt then tokenize content
  const replMatch = line.match(/^(reqsh>|\.{5}>)(.*)/);
  if (replMatch) {
    const tokens: Token[] = [];
    tokens.push({ text: replMatch[1], className: 'syn-prompt' });
    tokens.push(...tokenizeReplContent(replMatch[2]));
    return tokens;
  }

  // Shell mode
  return tokenizeShellLine(line);
}

const SYNTAX_COLORS: Record<string, string> = {
  'syn-prompt': 'var(--syn-prompt)',
  'syn-method': 'var(--syn-method)',
  'syn-command': 'var(--syn-command)',
  'syn-keyword': 'var(--syn-keyword)',
  'syn-variable': 'var(--syn-variable)',
  'syn-url': 'var(--syn-url)',
  'syn-string': 'var(--syn-string)',
  'syn-comment': 'var(--syn-comment)',
  'syn-flag': 'var(--syn-flag)',
  'syn-path': 'var(--syn-path)',
  'syn-status-ok': 'var(--syn-status-ok)',
  'syn-status-warn': 'var(--syn-status-warn)',
  'syn-status-err': 'var(--syn-status-err)',
  'syn-time': 'var(--syn-time)',
  'syn-http': 'var(--syn-http)',
  'syn-header': 'var(--syn-header)',
  'syn-special': 'var(--syn-method)',
  'syn-pipe': 'var(--syn-comment)',
  text: 'var(--foreground)',
};

export default function DocsCodeBlock({ code }: { code: string; language?: string }) {
  const [copied, setCopied] = useState(false);
  const lines = code.split('\n');

  const handleCopy = () => {
    navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
    posthog.capture('docs_code_copied');
  };

  const highlightedLines = useMemo(() => {
    return lines.map((line) => {
      if (line.trim() === '') return [];
      const mode = detectLineMode(line);
      return highlightLine(line, mode);
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [code]);

  return (
    <div className="group relative my-4 overflow-hidden rounded-xl border border-border bg-muted/30 font-mono text-[13px] leading-relaxed">
      <div className="flex items-center justify-between border-b border-border/60 px-4 py-2">
        <span className="text-xs font-medium text-muted-foreground">reqsh</span>
        <button
          onClick={handleCopy}
          className="flex items-center gap-1.5 rounded-md px-2 py-1 text-xs text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
          aria-label="Copy code"
        >
          {copied ? (
            <>
              <Check size={12} className="text-success" />
              <span>Copied</span>
            </>
          ) : (
            <>
              <Copy size={12} />
              <span>Copy</span>
            </>
          )}
        </button>
      </div>
      <div className="overflow-x-auto p-4">
        <div className="grid" style={{ gridTemplateColumns: 'minmax(2rem, auto) 1fr' }}>
          {lines.map((line, i) => (
            <React.Fragment key={i}>
              <div className="select-none pr-3 text-right text-muted-foreground/50 leading-relaxed">
                {i + 1}
              </div>
              <div className="leading-relaxed" style={{ whiteSpace: 'pre' }}>
                {highlightedLines[i].length === 0
                  ? '\u00A0'
                  : highlightedLines[i].map((token, j) => (
                      <span
                        key={j}
                        style={{
                          color: SYNTAX_COLORS[token.className] || 'var(--foreground)',
                          fontWeight:
                            token.className === 'syn-method' || token.className === 'syn-prompt'
                              ? 600
                              : undefined,
                          whiteSpace: 'pre',
                        }}
                      >
                        {token.text}
                      </span>
                    ))}
              </div>
            </React.Fragment>
          ))}
        </div>
      </div>
    </div>
  );
}
