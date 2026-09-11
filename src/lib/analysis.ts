import { invoke } from "@tauri-apps/api/core";
import type { SeqFormat, SeqResult } from "@/types/results";

// Send the sequence text to the backend and return the analytics
export async function analyseTextSequences(
  sequences: string,
  format: SeqFormat,
): Promise<SeqResult[]> {
  const command =
    format === "fastq" ? "analyse_fastq_sequences" : "analyse_fasta_sequences";
  return invoke<SeqResult[]>(command, { sequences });
}

// Send the sequence file to the backend and return the analytics
export async function analyseFileSequences(
  path: string,
  format: SeqFormat,
): Promise<SeqResult[]> {
  const command = format === "fastq" ? "analyse_fastq_file" : "analyse_fasta_file";
  return invoke<SeqResult[]>(command, { path });
}
