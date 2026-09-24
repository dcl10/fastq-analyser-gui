import { invoke } from "@tauri-apps/api/core";
import type { FastaSeqResult, FastqSeqResult, SeqFormat } from "@/types/results";
import type { RunRecords } from "@/types/runs";

// Send the sequence text to the backend and return the analytics
export async function analyseTextSequences(
  sequences: string,
  format: SeqFormat,
): Promise<RunRecords> {
  if (format === "fastq") {
    return { FastqRecords: await invoke<FastqSeqResult[]>("analyse_fastq_sequences", { sequences }) };
  }
  return { FastaRecords: await invoke<FastaSeqResult[]>("analyse_fasta_sequences", { sequences }) };
}

// Send the sequence file to the backend and return the analytics
export async function analyseFileSequences(
  path: string,
  format: SeqFormat,
): Promise<RunRecords> {
  if (format === "fastq") {
    return { FastqRecords: await invoke<FastqSeqResult[]>("analyse_fastq_file", { path }) };
  }
  return { FastaRecords: await invoke<FastaSeqResult[]>("analyse_fasta_file", { path }) };
}
