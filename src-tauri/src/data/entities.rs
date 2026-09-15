use sqlx::types::chrono::{self, Utc};

use crate::models::{FastaSeqResult, FastqSeqResult, Run as RunModel, RunRecords};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Run {
    pub id: u32,
    pub created_at: chrono::DateTime<Utc>,
    pub records: Option<Vec<Record>>,
    pub result_type: ResultType,
}

impl From<RunModel> for Run {
    fn from(value: RunModel) -> Self {
        let records: Vec<Record> = match value.records {
            RunRecords::FastaRecords(fasta_seq_results) => {
                fasta_seq_results.into_iter().map(Into::into).collect()
            }
            RunRecords::FastqRecords(fastq_seq_results) => {
                fastq_seq_results.into_iter().map(Into::into).collect()
            }
        };
        Run {
            id: value.id,
            created_at: value.created_at,
            records: if !records.is_empty() {
                Some(records)
            } else {
                None
            },
            result_type: value.result_type,
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Record {
    pub id: u32,
    pub seq_id: String,
    pub description: Option<String>,
    pub gc_content: f32,
    pub n_orfs: u32,
    pub is_valid: bool,
    pub seq_len: u32,
    pub phred_score: Option<u32>,
}

impl Into<FastaSeqResult> for Record {
    fn into(self) -> FastaSeqResult {
        FastaSeqResult {
            id: self.seq_id,
            desc: self.description,
            gc: self.gc_content,
            n_orfs: self.n_orfs,
            is_valid: self.is_valid,
            seq_len: self.seq_len,
        }
    }
}

impl Into<FastqSeqResult> for Record {
    fn into(self) -> FastqSeqResult {
        FastqSeqResult {
            id: self.seq_id,
            desc: self.description,
            gc: self.gc_content,
            n_orfs: self.n_orfs,
            is_valid: self.is_valid,
            seq_len: self.seq_len,
            phred_score: self.phred_score.unwrap(),
        }
    }
}

impl From<FastaSeqResult> for Record {
    fn from(value: FastaSeqResult) -> Self {
        Record {
            id: 0,
            seq_id: value.id,
            description: value.desc,
            gc_content: value.gc,
            n_orfs: value.n_orfs,
            is_valid: value.is_valid,
            seq_len: value.seq_len,
            phred_score: None,
        }
    }
}

impl From<FastqSeqResult> for Record {
    fn from(value: FastqSeqResult) -> Self {
        Record {
            id: 0,
            seq_id: value.id,
            description: value.desc,
            gc_content: value.gc,
            n_orfs: value.n_orfs,
            is_valid: value.is_valid,
            seq_len: value.seq_len,
            phred_score: Some(value.phred_score),
        }
    }
}

#[derive(
    Debug, Clone, sqlx::Type, serde::Deserialize, serde::Serialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum ResultType {
    Fasta,
    Fastq,
}
