import { invoke } from "@tauri-apps/api/core";
import type { SeqFormat } from "@/types/results";
import type { ResultType, Run, RunRecords } from "@/types/runs";

// Save analysed records as a new run and return the id the database assigned it
export async function saveRun(format: SeqFormat, records: RunRecords): Promise<number> {
  const run: Run = {
    // Placeholder: the database assigns the real id, and create_run ignores this one
    id: 0,
    created_at: new Date().toISOString(),
    result_type: format === "fastq" ? "Fastq" : "Fasta",
    records,
  };
  return invoke<number>("save_run", { run });
}

// List one zero-based page of saved runs, newest first, without their records
export async function listRuns(page: number): Promise<Run[]> {
  return invoke<Run[]>("list_runs", { page });
}

// Number of pages listRuns can return; 0 when there are no runs
export async function countRunPages(): Promise<number> {
  return invoke<number>("count_run_pages");
}

// Delete a run; its records are removed with it
export async function deleteRun(runId: number): Promise<void> {
  return invoke<void>("delete_run", { runId });
}

// Load a run together with the first page of its records
export async function loadRun(runId: number): Promise<Run> {
  return invoke<Run>("load_run", { runId });
}

// List one zero-based page of a run's records, in the order they were saved
export async function listRecordsForRun(
  runId: number,
  resultType: ResultType,
  page: number,
): Promise<RunRecords> {
  return invoke<RunRecords>("list_records_for_run", { runId, resultType, page });
}

// Number of pages listRecordsForRun can return for a run; 0 when it has no records
export async function countRecordPages(runId: number): Promise<number> {
  return invoke<number>("count_record_pages", { runId });
}
