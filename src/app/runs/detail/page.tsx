import { Suspense } from "react";
import { AppShell } from "@/components/app-shell";
import { RunDetail } from "@/components/run-detail";

// Static export can't prerender /runs/[id] for ids that only exist in the database,
// so the run id comes from the query string (/runs/detail?id=3) instead
export default function RunDetailPage() {
  return (
    <AppShell>
      <Suspense>
        <RunDetail />
      </Suspense>
    </AppShell>
  );
}
