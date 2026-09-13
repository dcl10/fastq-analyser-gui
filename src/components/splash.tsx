"use client";

import { useRouter } from "next/navigation";
import { PlayIcon } from "lucide-react";
import { TitleBar } from "@/components/title-bar";
import { ThemeSwitch } from "@/components/theme-switch";
import { Button } from "@/components/ui/button";
import { DnaMotif } from "@/components/brand/dna-motif";
import pkg from "../../package.json";

/**
 * The window before a run exists: one statement of what the app does and one
 * action. No rail nav — that only appears once the user is inside a screen.
 */
export function Splash() {
  const router = useRouter();

  return (
    <div className="flex min-h-0 flex-1 flex-col overflow-hidden">
      <TitleBar title="FastQ Analyser" actions={<ThemeSwitch />} />

      <div className="relative flex min-h-0 flex-1 items-center justify-center">
        <DnaMotif
          variant="texture"
          size={90}
          columns={34}
          className="pointer-events-none absolute inset-x-0 bottom-[7%] w-full"
        />

        <div className="relative flex flex-col items-center gap-6 px-8 text-center">
          <DnaMotif variant="mark" size={64} />

          <div className="flex flex-col gap-2">
            <h1 className="m-0 text-4xl font-semibold tracking-tight text-foreground">
              FastQ <span className="font-normal text-muted-foreground">Analyser</span>
            </h1>
            <p className="mx-auto max-w-[46ch] text-base text-pretty text-muted-foreground">
              Per-record statistics for FASTA and FASTQ sequence files. GC content, ORF counts,
              read length and Phred quality, computed locally.
            </p>
          </div>

          <div className="flex items-center gap-2">
            <Button size="lg" onClick={() => router.push("/import")}>
              <PlayIcon /> Get started
            </Button>
            <Button size="lg" variant="ghost" disabled title="Coming soon">
              Open recent
            </Button>
          </div>
        </div>
      </div>

      <div className="flex items-center justify-between border-t border-border px-4 py-2 font-mono text-[0.6875rem] text-muted-foreground">
        <span>v{pkg.version}</span>
        <span>Nothing leaves this machine.</span>
      </div>
    </div>
  );
}
