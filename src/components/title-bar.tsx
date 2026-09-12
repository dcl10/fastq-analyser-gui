"use client";

import * as React from "react";
import { MinusIcon, SquareIcon, XIcon } from "lucide-react";
import { cn } from "cn";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { platform } from "@tauri-apps/plugin-os";

/**
 * Tauri window title bar: 36px, drag region (data-tauri-drag-region).
 *
 * macOS uses titleBarStyle: "Overlay" (see src-tauri/tauri.macos.conf.json), so the
 * real native traffic lights float over the window content — this component just
 * reserves space for them and draws nothing there. Windows and Linux run with
 * decorations: false (no native chrome at all), so this component draws real,
 * functional minimize/maximize/close buttons on the right using the window API.
 */
export function TitleBar({
  title,
  actions,
  className,
}: {
  /** Document title, e.g. "run_014.fastq.gz — FastQ Analyser". */
  title?: React.ReactNode;
  actions?: React.ReactNode;
  className?: string;
}) {
  // Static export: platform() reads a value injected into the page at runtime, so
  // it isn't available during the server-rendered pass — read it after mount, once,
  // same hydration-safe pattern as ThemeProvider's stored-theme restore.
  const [os, setOs] = React.useState<"macos" | "other" | null>(null);

  React.useEffect(() => {
    // platform() reads a global the Tauri webview injects; outside Tauri (e.g. `npm
    // run dev` in a plain browser) it isn't there, so fall back rather than throw.
    let detected: "macos" | "other" = "other";
    try {
      detected = platform() === "macos" ? "macos" : "other";
    } catch {
      // not running inside a Tauri webview
    }
    // eslint-disable-next-line react-hooks/set-state-in-effect -- one-time read of a runtime-injected global, not a reactive sync
    setOs(detected);
  }, []);

  const win = React.useMemo(() => {
    try {
      return getCurrentWindow();
    } catch {
      return null;
    }
  }, []);

  return (
    <div
      data-tauri-drag-region
      className={cn(
        "flex h-9 shrink-0 items-center gap-3 border-b border-titlebar-border bg-titlebar px-2.5 text-xs text-titlebar-foreground select-none",
        className
      )}
    >
      {os === "macos" ? <span className="block w-[70px] shrink-0" aria-hidden /> : null}

      <div className="flex min-w-0 flex-1 items-center justify-center gap-2">
        <span className="overflow-hidden text-ellipsis whitespace-nowrap">{title}</span>
      </div>

      <div className="flex items-center gap-1">{actions}</div>

      {os === "other" && win ? (
        <div className="-mr-2.5 flex h-full items-stretch">
          <button
            type="button"
            aria-label="Minimize"
            onClick={() => win.minimize()}
            className="inline-flex w-10 items-center justify-center text-titlebar-foreground transition-colors hover:bg-muted"
          >
            <MinusIcon className="size-3.5" />
          </button>
          <button
            type="button"
            aria-label="Maximize"
            onClick={() => win.toggleMaximize()}
            className="inline-flex w-10 items-center justify-center text-titlebar-foreground transition-colors hover:bg-muted"
          >
            <SquareIcon className="size-3" />
          </button>
          <button
            type="button"
            aria-label="Close"
            onClick={() => win.close()}
            className="inline-flex w-10 items-center justify-center text-titlebar-foreground transition-colors hover:bg-destructive hover:text-white"
          >
            <XIcon className="size-3.5" />
          </button>
        </div>
      ) : null}
    </div>
  );
}
