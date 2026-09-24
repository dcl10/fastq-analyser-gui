"use client";

import { useEffect, useState } from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { LoadingIndicator } from "@/components/loading-indicator";
import { Toolbar } from "@/components/toolbar";
import { listRuns } from "@/lib/runs";
import type { Run } from "@/types/runs";

export function RunsList() {
  const [runs, setRuns] = useState<Run[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState("");

  // Load every saved run once, newest first (list_runs sorts them)
  useEffect(() => {
    listRuns()
      .then(setRuns)
      .catch((e) => setError(`Couldn't load runs: ${e}`))
      .finally(() => setIsLoading(false));
  }, []);

  return (
    <>
      <Toolbar title="Runs" subtitle={isLoading ? undefined : `${runs.length} saved`} />

      <div className="min-h-0 flex-1 overflow-y-auto">
        <div className="mx-auto flex w-full max-w-2xl flex-col gap-6 px-4 py-10">
          {isLoading ? (
            <LoadingIndicator message="Loading runs..." />
          ) : error ? (
            <p role="alert" className="text-sm text-destructive">
              {error}
            </p>
          ) : runs.length === 0 ? (
            <p className="text-sm text-muted-foreground">
              No runs yet. Import some sequences to create one.
            </p>
          ) : (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Run</TableHead>
                  <TableHead>Created</TableHead>
                  <TableHead>Type</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {runs.map((run) => (
                  <TableRow key={run.id}>
                    <TableCell className="font-mono tabular-nums">#{run.id}</TableCell>
                    <TableCell>{new Date(run.created_at).toLocaleString()}</TableCell>
                    <TableCell>{run.result_type.toUpperCase()}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}
        </div>
      </div>
    </>
  );
}
