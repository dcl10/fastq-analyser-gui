use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FastqSeqResult {
    pub id: String,
    pub desc: String,
    pub gc: f32,
    pub n_orfs: usize,
    pub is_valid: bool,
    pub phred_score: u32,
    pub seq_len: usize,
    pub result_type: String,
}

impl Default for FastqSeqResult {
    fn default() -> Self {
        FastqSeqResult {
            id: String::from("id"),
            desc: String::from("..."),
            gc: 0.0,
            n_orfs: 0,
            is_valid: false,
            phred_score: 0,
            seq_len: 0,
            result_type: String::from("fastq"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct FastaSeqResult {
    pub id: String,
    pub desc: String,
    pub gc: f32,
    pub n_orfs: usize,
    pub is_valid: bool,
    pub seq_len: usize,
    pub result_type: String,
}

impl Default for FastaSeqResult {
    fn default() -> Self {
        FastaSeqResult {
            id: String::from("id"),
            desc: String::from("..."),
            gc: 0.0,
            n_orfs: 0,
            is_valid: false,
            seq_len: 0,
            result_type: String::from("fasta"),
        }
    }
}