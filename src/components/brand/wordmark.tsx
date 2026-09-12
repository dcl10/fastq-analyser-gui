import { cn } from "cn";
import { DnaMotif } from "@/components/brand/dna-motif";

/**
 * Product signature. Type-only by design: the repo has no logo asset
 * (src-tauri/icons still holds the default Tauri mark). Use this wherever a logo
 * would go. Spelling is "FastQ Analyser" exactly.
 */
export function Wordmark({
  size = 15,
  showMotif = true,
  subdued = false,
  className,
}: {
  size?: number;
  showMotif?: boolean;
  subdued?: boolean;
  className?: string;
}) {
  return (
    <span
      className={cn(
        "inline-flex items-center leading-none tracking-tight",
        subdued && "opacity-70",
        className
      )}
      style={{ fontSize: size, gap: Math.round(size * 0.5) }}
    >
      {showMotif ? <DnaMotif variant="mark" size={Math.round(size * 1.25)} columns={4} /> : null}
      <span>
        <span className="font-semibold">FastQ</span>
        <span className="font-normal text-muted-foreground"> Analyser</span>
      </span>
    </span>
  );
}
