// Mirrors the Rust Run / RunRecords / ImportProgress types in src-tauri/src/models.rs, as serde serializes them

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

/** Sent after each batch while a file is analysed and saved. */
export interface ImportProgress {
  /** Records analysed and saved so far. */
  records: number;
  /** Bytes read from the file so far (compressed bytes for .gz); runs slightly ahead of records. */
  bytes_read: number;
  /** Size of the file in bytes. */
  total_bytes: number;
}
