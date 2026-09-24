use sqlx::{Sqlite, SqlitePool};

use crate::data::entities::Record as RecordEntity;
use crate::models::{FastaSeqResult, FastqSeqResult};

pub async fn create_record_from_fastq_result(
    pool: SqlitePool,
    record: &FastqSeqResult,
    run_id: u32,
) -> Result<FastqSeqResult, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let inserted_record = sqlx::query_as::<Sqlite, RecordEntity>(
        r#"
        INSERT INTO records (run_id, seq_id, description, gc_content, n_orfs, is_valid, seq_len, phred_score)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        RETURNING id, run_id, seq_id, description, gc_content, n_orfs, is_valid, seq_len, phred_score
        "#,
    )
    .bind(run_id)
    .bind(&record.id)
    .bind(&record.desc)
    .bind(record.gc)
    .bind(record.n_orfs)
    .bind(record.is_valid)
    .bind(record.seq_len)
    .bind(record.phred_score)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    let record = FastqSeqResult {
        id: inserted_record.seq_id,
        desc: inserted_record.description,
        gc: inserted_record.gc_content,
        n_orfs: inserted_record.n_orfs,
        is_valid: inserted_record.is_valid,
        seq_len: inserted_record.seq_len,
        phred_score: inserted_record.phred_score.unwrap(),
    };
    Ok(record)
}

pub async fn create_record_from_fasta_result(
    pool: SqlitePool,
    record: &FastaSeqResult,
    run_id: u32,
) -> Result<FastaSeqResult, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let inserted_record = sqlx::query_as::<Sqlite, RecordEntity>(
        r#"
        INSERT INTO records (run_id, seq_id, description, gc_content, n_orfs, is_valid, seq_len, phred_score)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        RETURNING id, run_id, seq_id, description, gc_content, n_orfs, is_valid, seq_len, phred_score
        "#,
    )
    .bind(run_id)
    .bind(&record.id)
    .bind(&record.desc)
    .bind(record.gc)
    .bind(record.n_orfs)
    .bind(record.is_valid)
    .bind(record.seq_len)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    let record = FastaSeqResult {
        id: inserted_record.seq_id,
        desc: inserted_record.description,
        gc: inserted_record.gc_content,
        n_orfs: inserted_record.n_orfs,
        is_valid: inserted_record.is_valid,
        seq_len: inserted_record.seq_len,
    };
    Ok(record)
}
