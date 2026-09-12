"use client";

import * as React from "react";
import { cn } from "cn";

type Variant = "mark" | "strip" | "loader" | "texture";

const PRESETS: Record<Variant, { columns: number; animated: boolean; opacity: number; wide: boolean }> = {
  mark: { columns: 5, animated: false, opacity: 1, wide: false },
  loader: { columns: 7, animated: true, opacity: 1, wide: false },
  strip: { columns: 14, animated: false, opacity: 1, wide: true },
  texture: { columns: 26, animated: false, opacity: 0.14, wide: true },
};

/**
 * Brand motif: two strands of dots in counter-phase with a rung between them,
 * built from divs — no illustration, no SVG. Allowed in four places only:
 * the app mark, the loading state, a section divider, a faint window texture.
 */
export function DnaMotif({
  variant = "mark",
  size = 32,
  columns,
  animated,
  opacity,
  className,
}: {
  variant?: Variant;
  size?: number;
  columns?: number;
  animated?: boolean;
  opacity?: number;
  className?: string;
}) {
  const preset = PRESETS[variant];
  const cols = columns ?? preset.columns;
  const isAnimated = animated ?? preset.animated;
  const [phase, setPhase] = React.useState(0);

  React.useEffect(() => {
    if (!isAnimated) return;
    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) return;
    let raf = 0;
    const start = performance.now();
    const tick = (t: number) => {
      setPhase(((t - start) / 1600) * Math.PI * 2);
      raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(raf);
  }, [isAnimated]);

  const dot = Math.max(2, Math.round(size * 0.16));
  const amp = (size - dot) / 2;
  const step = (Math.PI / Math.max(3, cols - 1)) * 1.15;

  // Round pixel/opacity values: Math.sin can return a value that differs in its
  // last few decimal digits between the server and client JS engines, and an
  // unrounded float fails React's hydration equality check even though the
  // rendered result is visually identical.
  const round = (n: number) => Math.round(n * 1000) / 1000;

  return (
    <span
      role="img"
      aria-label="FastQ Analyser"
      className={cn("pointer-events-none inline-flex items-center justify-between", className)}
      style={{
        width: preset.wide ? "100%" : variant === "loader" ? size * 1.9 : size,
        height: size,
        opacity: round(opacity ?? preset.opacity),
      }}
    >
      {Array.from({ length: cols }, (_, i) => {
        const offset = Math.sin(phase + i * step) * amp;
        const top = round(size / 2 - offset - dot / 2);
        const bottom = round(size / 2 + offset - dot / 2);
        const fade = round(0.35 + 0.65 * Math.abs(Math.sin(phase + i * step)));
        return (
          <span key={i} className="relative h-full flex-1" style={{ minWidth: dot }}>
            <span
              className="absolute left-1/2 w-px bg-border"
              style={{
                top: round(Math.min(top, bottom) + dot / 2),
                height: round(Math.abs(top - bottom)),
                marginLeft: -0.5,
              }}
            />
            <span
              className="absolute left-1/2 rounded-full bg-primary"
              style={{ top, width: dot, height: dot, marginLeft: -dot / 2, opacity: fade }}
            />
            <span
              className="absolute left-1/2 rounded-full bg-accent-foreground"
              style={{ top: bottom, width: dot, height: dot, marginLeft: -dot / 2, opacity: round(1.35 - fade) }}
            />
          </span>
        );
      })}
    </span>
  );
}
