'use client';

import React from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import DocsCodeBlock from '@/components/docs-code-block';
import Link from 'next/link';

interface MarkdownRendererProps {
  content: string;
}

const CodeComponent = ({
  className,
  children,
  ...props
}: {
  className?: string;
  children?: React.ReactNode;
}) => {
  const isInline = !className;
  if (isInline) {
    return (
      <code className="relative rounded bg-muted px-1.5 py-0.5 font-mono text-sm text-foreground">
        {children}
      </code>
    );
  }
  return (
    <code className={className} {...props}>
      {children}
    </code>
  );
};

export default function MarkdownRenderer({ content }: MarkdownRendererProps) {
  return (
    <ReactMarkdown
      remarkPlugins={[remarkGfm]}
      components={{
        h1: ({ children }) => (
          <h1 className="scroll-m-24 text-3xl font-semibold tracking-tight text-foreground sm:text-3xl">
            {children}
          </h1>
        ),
        h2: ({ children }) => (
          <h2 className="scroll-m-24 text-xl font-semibold tracking-tight text-foreground mt-10 mb-4">
            {children}
          </h2>
        ),
        h3: ({ children }) => (
          <h3 className="scroll-m-24 text-base font-semibold text-foreground mt-8 mb-3">
            {children}
          </h3>
        ),
        p: ({ children }) => <p className="text-[15px] leading-7 text-foreground">{children}</p>,
        ul: ({ children }) => (
          <ul className="my-4 ml-6 list-disc text-[15px] leading-7 text-foreground [&>li]:mt-2">
            {children}
          </ul>
        ),
        ol: ({ children }) => (
          <ol className="my-4 ml-6 list-decimal text-[15px] leading-7 text-foreground [&>li]:mt-2">
            {children}
          </ol>
        ),
        li: ({ children }) => <li className="pl-1">{children}</li>,
        code: CodeComponent,
        pre: ({ children }) => {
          const codeChild = React.Children.toArray(children).find(
            (child) => React.isValidElement(child) && child.type === CodeComponent
          ) as React.ReactElement<{ className?: string; children?: React.ReactNode }> | undefined;

          const codeContent = codeChild?.props?.children;
          const code = typeof codeContent === 'string' ? codeContent.trim() : '';

          if (code) {
            return <DocsCodeBlock code={code} />;
          }

          return (
            <pre className="my-4 overflow-x-auto rounded-lg border border-border bg-muted/50 p-4 font-mono text-sm">
              {children}
            </pre>
          );
        },
        table: ({ children }) => (
          <div className="my-6 overflow-x-auto rounded-lg border border-border">
            <table className="w-full text-sm">{children}</table>
          </div>
        ),
        thead: ({ children }) => (
          <thead className="border-b border-border bg-muted/50">{children}</thead>
        ),
        tbody: ({ children }) => <tbody className="divide-y divide-border">{children}</tbody>,
        tr: ({ children }) => <tr className="transition-colors hover:bg-muted/30">{children}</tr>,
        th: ({ children }) => (
          <th className="px-4 py-2.5 text-left font-medium text-foreground">{children}</th>
        ),
        td: ({ children }) => <td className="px-4 py-2.5 text-foreground">{children}</td>,
        a: ({ href, children }) => {
          if (href?.startsWith('/')) {
            return (
              <Link
                href={href}
                className="font-medium text-accent underline underline-offset-4 transition-colors hover:text-accent/80"
              >
                {children}
              </Link>
            );
          }
          return (
            <a
              href={href}
              target="_blank"
              rel="noopener noreferrer"
              className="font-medium text-accent underline underline-offset-4 transition-colors hover:text-accent/80"
            >
              {children}
            </a>
          );
        },
        strong: ({ children }) => (
          <strong className="font-semibold text-foreground">{children}</strong>
        ),
        blockquote: ({ children }) => (
          <blockquote className="my-6 border-l-2 border-accent pl-4 text-foreground italic">
            {children}
          </blockquote>
        ),
        hr: () => <hr className="my-8 border-border" />,
      }}
    >
      {content}
    </ReactMarkdown>
  );
}
