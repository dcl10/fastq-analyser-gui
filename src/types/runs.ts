// Mirrors the Rust Run / RunRecords types in src-tauri/src/models.rs, as serde serializes them

import type { FastaSeqResult, FastqSeqResult } from "@/types/results";

export type ResultType = "Fasta" | "Fastq";

export type RunRecords = { FastaRecords: FastaSeqResult[] } | { FastqRecords: FastqSeqResult[] };

export interface Run {
  id: number;
  /** RFC 3339 timestamp, e.g. "2026-09-24T10:15:00Z". */
  created_at: string;
  result_type: ResultType;
  records: RunRecords;
}
