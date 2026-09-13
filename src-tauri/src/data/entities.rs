use sqlx::types::chrono::{self, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Run {
    id: usize,
    created_at: chrono::DateTime<Utc>,
    records: Option<Vec<Record>>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Record {
    id: u32,
    seq_id: String,
    description: Option<String>,
    gc_content: f32,
    n_orfs: u32,
    is_valid: bool,
    seq_len: u32,
    phred_score: Option<u32>,
    result_type: ResultType,
}

#[derive(Debug, Clone, sqlx::Type)]
pub enum ResultType {
    Fasta,
    Fastq,
}
