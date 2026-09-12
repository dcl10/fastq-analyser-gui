"use client";

import type { LucideIcon } from "lucide-react";
import { cn } from "cn";

export interface RailNavItem {
  id?: string;
  label?: string;
  Icon?: LucideIcon;
  /** Right-aligned count. Counts only — never status colour. */
  badge?: React.ReactNode;
  /** Renders an uppercase section heading instead of a link. */
  section?: string;
  /** Renders the row inert with a muted "Soon" badge, for screens not built yet. */
  disabled?: boolean;
}

/**
 * Left navigation rail: 216px (w-54), one 32px row per view, green tint plus a
 * green label on the active row. Icons are required on rows.
 */
export function RailNav({
  items,
  value,
  onChange,
  footer,
  className,
}: {
  items: RailNavItem[];
  value?: string;
  onChange?: (id: string) => void;
  footer?: React.ReactNode;
  className?: string;
}) {
  return (
    <nav
      className={cn(
        "flex w-54 shrink-0 flex-col border-r border-sidebar-border bg-sidebar p-2",
        className
      )}
    >
      <div className="flex flex-1 flex-col gap-px">
        {items.map((it, i) =>
          it.section ? (
            <span
              key={`section-${i}`}
              className="px-2 pt-3 pb-1 text-[0.6875rem] font-medium tracking-[0.04em] text-muted-foreground uppercase"
            >
              {it.section}
            </span>
          ) : (
            <button
              key={it.id}
              type="button"
              disabled={it.disabled}
              onClick={() => it.id && onChange?.(it.id)}
              className={cn(
                "flex h-8 items-center gap-2 rounded-md px-2 text-left transition-colors",
                it.disabled
                  ? "cursor-not-allowed text-muted-foreground/60"
                  : value === it.id
                    ? "bg-primary/12 font-medium text-primary"
                    : "text-sidebar-foreground hover:bg-sidebar-accent"
              )}
            >
              {it.Icon ? <it.Icon className="size-4" /> : null}
              <span className="flex-1 overflow-hidden text-ellipsis whitespace-nowrap">{it.label}</span>
              {it.disabled ? (
                <span className="text-[0.6875rem] tracking-[0.02em] text-muted-foreground/60 uppercase">
                  Soon
                </span>
              ) : it.badge ? (
                <span className="font-mono text-[0.6875rem] tabular-nums text-muted-foreground">
                  {it.badge}
                </span>
              ) : null}
            </button>
          )
        )}
      </div>
      {footer ? <div className="border-t border-sidebar-border pt-2">{footer}</div> : null}
    </nav>
  );
}
