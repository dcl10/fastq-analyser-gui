"use client";

import { ChartColumnIcon, DnaIcon, DownloadIcon, FileUpIcon } from "lucide-react";
import { RailNav, type RailNavItem } from "@/components/rail-nav";
import { TitleBar } from "@/components/title-bar";
import { ThemeSwitch } from "@/components/theme-switch";
import { Wordmark } from "@/components/brand/wordmark";
import pkg from "../../package.json";

const NAV_ITEMS: RailNavItem[] = [
  { section: "Run" },
  { id: "import", label: "Import", Icon: FileUpIcon },
  { id: "quality", label: "Quality", Icon: ChartColumnIcon, disabled: true },
  { section: "Explore" },
  { id: "motifs", label: "Motif browser", Icon: DnaIcon, disabled: true },
  { id: "export", label: "Export report", Icon: DownloadIcon, disabled: true },
];

/**
 * Window shell: title bar, navigation rail, one screen at a time. Only "import"
 * is wired up today — the other rail items are disabled until their screens
 * exist (see the design system's ui_kits/desktop-app/README.md).
 */
export function AppShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex min-h-0 flex-1 flex-col overflow-hidden">
      <TitleBar title="FastQ Analyser" actions={<ThemeSwitch />} />
      <div className="flex min-h-0 flex-1">
        <RailNav
          value="import"
          items={NAV_ITEMS}
          footer={
            <div className="flex items-center justify-between px-2 py-1">
              <Wordmark size={12} subdued />
              <span className="font-mono text-[0.6875rem] text-muted-foreground">
                v{pkg.version}
              </span>
            </div>
          }
        />
        <main className="flex min-h-0 min-w-0 flex-1 flex-col">{children}</main>
      </div>
    </div>
  );
}
