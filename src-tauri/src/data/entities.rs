use sqlx::types::chrono::{self, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Run {
    pub id: usize,
    pub created_at: chrono::DateTime<Utc>,
    pub records: Option<Vec<Record>>,
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
    pub result_type: ResultType,
}

#[derive(Debug, Clone, sqlx::Type)]
pub enum ResultType {
    Fasta,
    Fastq,
}
