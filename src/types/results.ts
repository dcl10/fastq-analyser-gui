// Mirrors the Rust result structs in src-tauri/src/models.rs

interface BaseSeqResult {
  id: string;
  desc: string | null;
  gc: number;
  n_orfs: number;
  seq_len: number;
}

export type FastaSeqResult = BaseSeqResult;

export interface FastqSeqResult extends BaseSeqResult {
  phred_score: number;
}

export type SeqFormat = "fasta" | "fastq";
