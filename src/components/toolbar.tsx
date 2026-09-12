import { cn } from "cn";

/**
 * Screen header band: the view's name, the file in context, and that view's
 * controls. One primary button at most; keep the subtitle to file plus one figure.
 */
export function Toolbar({
  title,
  subtitle,
  children,
  className,
}: {
  title: React.ReactNode;
  /** Mono context line, e.g. "run_014.fastq.gz · 1,248,331 reads". */
  subtitle?: React.ReactNode;
  children?: React.ReactNode;
  className?: string;
}) {
  return (
    <div
      className={cn(
        "flex min-h-12 shrink-0 items-center justify-between gap-4 border-b border-border bg-background px-4 py-2",
        className
      )}
    >
      <div className="flex min-w-0 flex-col gap-0.5">
        <span className="overflow-hidden text-base font-medium tracking-tight text-ellipsis whitespace-nowrap">
          {title}
        </span>
        {subtitle ? (
          <span className="overflow-hidden font-mono text-xs text-ellipsis whitespace-nowrap text-muted-foreground">
            {subtitle}
          </span>
        ) : null}
      </div>
      <div className="flex shrink-0 items-center gap-2">{children}</div>
    </div>
  );
}
