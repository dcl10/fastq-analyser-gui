use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::data::entities::Record as RecordEntity;
use crate::models::{FastaSeqResult, FastqSeqResult};

pub async fn create_records_from_fastq_results(
    pool: SqlitePool,
    records: &Vec<FastqSeqResult>,
    run_id: u32,
) -> Result<Vec<u32>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let mut record_ids: Vec<u32> = Vec::new();
    for chunk in records.chunks(1000) {
        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO records \
            (run_id, seq_id, description, gc_content, n_orfs, is_valid, seq_len, phred_score) ",
        );

        builder.push_values(chunk, |mut row, record| {
            row.push_bind(run_id)
                .push_bind(&record.id)
                .push_bind(&record.desc)
                .push_bind(record.gc)
                .push_bind(record.n_orfs)
                .push_bind(record.is_valid)
                .push_bind(record.seq_len)
                .push_bind(record.phred_score);
        });

        builder.push(r#"RETURNING id"#);
        let query = builder.build_query_scalar::<u32>();

        let mut ids = query.fetch_all(&mut *tx).await?;
        record_ids.append(&mut ids);
    }

    tx.commit().await?;
    Ok(record_ids)
}

pub async fn create_records_from_fasta_results(
    pool: SqlitePool,
    records: &Vec<FastaSeqResult>,
    run_id: u32,
) -> Result<Vec<u32>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let mut record_ids: Vec<u32> = Vec::new();
    for chunk in records.chunks(1000) {
        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO records \
            (run_id, seq_id, description, gc_content, n_orfs, is_valid, seq_len) ",
        );

        builder.push_values(chunk, |mut row, record| {
            row.push_bind(run_id)
                .push_bind(&record.id)
                .push_bind(&record.desc)
                .push_bind(record.gc)
                .push_bind(record.n_orfs)
                .push_bind(record.is_valid)
                .push_bind(record.seq_len);
        });

        builder.push(r#"RETURNING id"#);
        let query = builder.build_query_scalar::<u32>();

        let mut ids = query.fetch_all(&mut *tx).await?;
        record_ids.append(&mut ids);
    }

    tx.commit().await?;
    Ok(record_ids)
}

pub async fn list_records_for_run<T>(pool: SqlitePool, run_id: u32) -> Result<Vec<T>, sqlx::Error>
where
    T: From<RecordEntity>,
{
    let records: Vec<T> = sqlx::query_as::<Sqlite, RecordEntity>(
        r#"
        SELECT id, run_id, seq_id, description, gc_content, n_orfs, is_valid, seq_len, phred_score
        FROM records
        WHERE run_id = ?1
        ORDER BY id DESC
        "#,
    )
    .bind(run_id)
    .fetch_all(&pool)
    .await?
    .into_iter()
    .map(|r| T::from(r))
    .collect();

    Ok(records)
}
