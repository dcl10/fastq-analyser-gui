use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::data::entities::{Record as RecordEntity, ResultType, Run as RunEntity};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct FastqSeqResult {
    pub id: String,
    pub desc: Option<String>,
    pub gc: f32,
    pub n_orfs: u32,
    pub is_valid: bool,
    pub phred_score: u32,
    pub seq_len: u32,
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
        }
    }
}

impl From<RecordEntity> for FastqSeqResult {
    fn from(value: RecordEntity) -> Self {
        FastqSeqResult {
            id: value.seq_id,
            desc: value.description,
            gc: value.gc_content,
            n_orfs: value.n_orfs,
            is_valid: value.is_valid,
            seq_len: value.seq_len,
            phred_score: value.phred_score.unwrap(),
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
        }
    }
}

impl From<RecordEntity> for FastaSeqResult {
    fn from(value: RecordEntity) -> Self {
        FastaSeqResult {
            id: value.seq_id,
            desc: value.description,
            gc: value.gc_content,
            n_orfs: value.n_orfs,
            is_valid: value.is_valid,
            seq_len: value.seq_len,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Run {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub records: RunRecords,
    pub result_type: ResultType,
}

impl From<RunEntity> for Run {
    fn from(value: RunEntity) -> Self {
        match value.result_type {
            ResultType::Fasta => {
                let records: Vec<FastaSeqResult> = value
                    .records
                    .unwrap_or_default()
                    .into_iter()
                    .map(Into::into)
                    .collect();
                Run {
                    id: value.id,
                    created_at: value.created_at,
                    records: RunRecords::FastaRecords(records),
                    result_type: value.result_type,
                }
            }
            ResultType::Fastq => {
                let records: Vec<FastqSeqResult> = value
                    .records
                    .unwrap_or_default()
                    .into_iter()
                    .map(Into::into)
                    .collect();
                Run {
                    id: value.id,
                    created_at: value.created_at,
                    records: RunRecords::FastqRecords(records),
                    result_type: value.result_type,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RunRecords {
    FastaRecords(Vec<FastaSeqResult>),
    FastqRecords(Vec<FastqSeqResult>),
}
