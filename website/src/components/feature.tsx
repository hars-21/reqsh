import Reveal from './reveal';

export function FeatureRow({
  title,
  description,
  children,
  reverse = false,
}: {
  title: string;
  description: React.ReactNode;
  children: React.ReactNode;
  reverse?: boolean;
}) {
  return (
    <section
      className="py-16 md:py-24"
      aria-labelledby={`feature-${title.replace(/\s+/g, '-').toLowerCase()}`}
    >
      <div className="grid grid-cols-1 items-center gap-10 md:grid-cols-2 md:gap-20">
        <Reveal className={reverse ? 'md:order-2' : ''}>
          <h3
            id={`feature-${title.replace(/\s+/g, '-').toLowerCase()}`}
            className="mb-5 text-2xl font-bold tracking-tight text-foreground md:text-4xl"
          >
            {title}
          </h3>
          <p className="text-base leading-relaxed text-muted-foreground md:text-lg">
            {description}
          </p>
        </Reveal>
        <Reveal className={reverse ? 'md:order-1' : ''} delay={0.1}>
          {children}
        </Reveal>
      </div>
    </section>
  );
}

export function TerminalCard({ children }: { children: React.ReactNode }) {
  return (
    <div className="overflow-x-auto rounded-2xl border border-terminal-border bg-terminal p-4 sm:p-6 font-mono text-xs sm:text-sm leading-7 sm:leading-8 md:p-8">
      {children}
    </div>
  );
}

export function Line({
  prompt = false,
  cont = false,
  children,
}: {
  prompt?: boolean;
  cont?: boolean;
  children: React.ReactNode;
}) {
  return (
    <div className="whitespace-pre">
      <span className="font-bold text-terminal-accent">
        {prompt ? 'reqsh>' : cont ? '.....>' : ''}
      </span>{' '}
      {children}
    </div>
  );
}

export function Cmd({ children }: { children: React.ReactNode }) {
  return <span className="font-semibold text-terminal-foreground">{children}</span>;
}

export function Fg({ children }: { children: React.ReactNode }) {
  return <span className="text-terminal-foreground">{children}</span>;
}

export function Muted({ children }: { children: React.ReactNode }) {
  return <span className="text-terminal-muted">{children}</span>;
}

export function Method({ children }: { children: React.ReactNode }) {
  return <span className="font-semibold text-terminal-accent">{children}</span>;
}

export function Code({ children }: { children: React.ReactNode }) {
  return (
    <code className="rounded bg-muted px-1.5 py-0.5 font-mono text-sm text-foreground">
      {children}
    </code>
  );
}
