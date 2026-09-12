"use client";

import { MonitorIcon, MoonIcon, SunIcon } from "lucide-react";
import { cn } from "cn";
import { useTheme } from "@/components/theme-provider";

const MODES = [
  { id: "light", Icon: SunIcon, label: "Light" },
  { id: "dark", Icon: MoonIcon, label: "Dark" },
  { id: "system", Icon: MonitorIcon, label: "System" },
] as const;

/** Light / dark / system. Icon-only at 28px; lives in the title bar or in Settings, never both. */
export function ThemeSwitch({ className }: { className?: string }) {
  const { mode, setMode } = useTheme();
  return (
    <div
      role="radiogroup"
      aria-label="Colour mode"
      className={cn(
        "inline-flex h-7 items-center gap-0.5 rounded-md border border-border bg-card p-0.5",
        className
      )}
    >
      {MODES.map(({ id, Icon, label }) => (
        <button
          key={id}
          type="button"
          role="radio"
          aria-checked={mode === id}
          title={label}
          onClick={() => setMode(id)}
          className={cn(
            "inline-flex h-full w-[22px] items-center justify-center rounded-sm transition-colors",
            mode === id
              ? "bg-primary/15 text-primary"
              : "text-muted-foreground hover:bg-muted hover:text-foreground"
          )}
        >
          <Icon className="size-3.5" />
        </button>
      ))}
    </div>
  );
}
