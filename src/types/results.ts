// Mirrors the Rust result structs in src-tauri/src/models.rs

interface BaseSeqResult {
  id: string;
  desc: string;
  gc: number;
  n_orfs: number;
  is_valid: boolean;
  seq_len: number;
}

export interface FastaSeqResult extends BaseSeqResult {
  result_type: "fasta";
}

export interface FastqSeqResult extends BaseSeqResult {
  result_type: "fastq";
  phred_score: number;
}

export type SeqResult = FastaSeqResult | FastqSeqResult;

export type SeqFormat = "fasta" | "fastq";
