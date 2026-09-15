use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct FastqSeqResult {
    pub id: String,
    pub desc: Option<String>,
    pub gc: f32,
    pub n_orfs: u32,
    pub is_valid: bool,
    pub phred_score: u32,
    pub seq_len: u32,
    pub result_type: ResultType,
}

impl Default for FastqSeqResult {
    fn default() -> Self {
        FastqSeqResult {
            id: String::from("id"),
            desc: None,
            gc: 0.0,
            n_orfs: 0,
            is_valid: false,
            phred_score: 0,
            seq_len: 0,
            result_type: ResultType::Fastq,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct FastaSeqResult {
    pub id: String,
    pub desc: Option<String>,
    pub gc: f32,
    pub n_orfs: u32,
    pub is_valid: bool,
    pub seq_len: u32,
    pub result_type: ResultType,
}

impl Default for FastaSeqResult {
    fn default() -> Self {
        FastaSeqResult {
            id: String::from("id"),
            desc: None,
            gc: 0.0,
            n_orfs: 0,
            is_valid: false,
            seq_len: 0,
            result_type: ResultType::Fasta,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResultType {
    Fasta,
    Fastq,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Run {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub records: Option<RunRecords>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RunRecords {
    FastaRecords(Vec<FastaSeqResult>),
    FastqRecords(Vec<FastqSeqResult>),
}
