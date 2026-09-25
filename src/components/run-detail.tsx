"use client";

import { useEffect, useState } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import { ArrowLeftIcon } from "lucide-react";
import { Button } from "@/components/ui/button";
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
import { loadRun } from "@/lib/runs";
import type { FastaSeqResult, FastqSeqResult } from "@/types/results";
import type { Run } from "@/types/runs";

// Per-base Phred score; invalid records have no length, so there's nothing to average
function perBasePhred(record: FastqSeqResult): string {
  return record.seq_len > 0 ? (record.phred_score / record.seq_len).toFixed(1) : "–";
}

export function RunDetail() {
  const router = useRouter();
  const idParam = useSearchParams().get("id");
  const runId = Number(idParam);
  const isValidId = idParam !== null && Number.isInteger(runId) && runId > 0;

  const [loaded, setLoaded] = useState<{ runId: number; run?: Run; error?: string } | null>(null);

  // Load the run and its records whenever the id in the URL changes
  useEffect(() => {
    if (!isValidId) return;
    loadRun(runId)
      .then((run) => setLoaded({ runId, run }))
      .catch((e) => setLoaded({ runId, error: `Couldn't load run #${runId}: ${e}` }));
  }, [runId, isValidId]);

  // Ignore a result left over from a previously viewed run until this one loads
  const current = loaded?.runId === runId ? loaded : null;
  const run = current?.run ?? null;
  const error = current?.error ?? "";

  const fastqRecords = run && "FastqRecords" in run.records ? run.records.FastqRecords : null;
  const records: (FastaSeqResult | FastqSeqResult)[] = run
    ? "FastqRecords" in run.records
      ? run.records.FastqRecords
      : run.records.FastaRecords
    : [];

  const subtitle = run
    ? `${run.result_type.toUpperCase()} · ${records.length.toLocaleString()} records · ${new Date(run.created_at).toLocaleString()}`
    : undefined;

  return (
    <>
      <Toolbar title={isValidId ? `Run #${runId}` : "Run"} subtitle={subtitle}>
        <Button variant="outline" size="sm" onClick={() => router.push("/runs")}>
          <ArrowLeftIcon /> Runs
        </Button>
      </Toolbar>

      <div className="min-h-0 flex-1 overflow-y-auto">
        <div className="mx-auto flex w-full max-w-5xl flex-col gap-6 px-4 py-10">
          {!isValidId ? (
            <p role="alert" className="text-sm text-destructive">
              No valid run id was given.
            </p>
          ) : error ? (
            <p role="alert" className="text-sm text-destructive">
              {error}
            </p>
          ) : !run ? (
            <LoadingIndicator message="Loading run..." />
          ) : records.length === 0 ? (
            <p className="text-sm text-muted-foreground">This run has no records.</p>
          ) : (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Record</TableHead>
                  <TableHead>Description</TableHead>
                  <TableHead className="text-right">Length</TableHead>
                  <TableHead className="text-right">GC %</TableHead>
                  <TableHead className="text-right">ORFs</TableHead>
                  {fastqRecords ? <TableHead className="text-right">Phred / base</TableHead> : null}
                </TableRow>
              </TableHeader>
              <TableBody>
                {records.map((record, index) => (
                  // Record ids aren't guaranteed unique within a file, so key on position
                  <TableRow key={index}>
                    <TableCell className="font-mono">{record.id}</TableCell>
                    <TableCell
                      className="max-w-64 overflow-hidden text-ellipsis text-muted-foreground"
                      title={record.desc ?? undefined}
                    >
                      {record.desc ?? "–"}
                    </TableCell>
                    <TableCell className="text-right font-mono tabular-nums">
                      {record.seq_len.toLocaleString()}
                    </TableCell>
                    <TableCell className="text-right font-mono tabular-nums">
                      {(record.gc * 100).toFixed(1)}
                    </TableCell>
                    <TableCell className="text-right font-mono tabular-nums">
                      {record.n_orfs}
                    </TableCell>
                    {fastqRecords ? (
                      <TableCell className="text-right font-mono tabular-nums">
                        {perBasePhred(fastqRecords[index])}
                      </TableCell>
                    ) : null}
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
