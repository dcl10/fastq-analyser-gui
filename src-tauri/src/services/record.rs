use sqlx::{QueryBuilder, Sqlite, SqliteConnection, SqlitePool};

use crate::data::entities::Record as RecordEntity;
use crate::models::{FastaSeqResult, FastqSeqResult, RunRecords};

/// Inserts `records` for `run_id`, whichever format they are.
pub async fn create_records(conn: &mut SqliteConnection, records: &RunRecords, run_id: u32) -> Result<(), sqlx::Error> {
    match records {
        RunRecords::FastaRecords(records) => create_records_from_fasta_results(conn, records, run_id).await,
        RunRecords::FastqRecords(records) => create_records_from_fastq_results(conn, records, run_id).await,
    }
}

pub async fn create_records_from_fastq_results(
    conn: &mut SqliteConnection,
    records: &[FastqSeqResult],
    run_id: u32,
) -> Result<(), sqlx::Error> {
    for chunk in records.chunks(1000) {
        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO records \
            (run_id, seq_id, description, gc_content, n_orfs, seq_len, phred_score) ",
        );

        builder.push_values(chunk, |mut row, record| {
            row.push_bind(run_id)
                .push_bind(&record.id)
                .push_bind(&record.desc)
                .push_bind(record.gc)
                .push_bind(record.n_orfs)
                .push_bind(record.seq_len)
                .push_bind(record.phred_score);
        });

        builder.build().execute(&mut *conn).await?;
    }

    Ok(())
}

pub async fn create_records_from_fasta_results(
    conn: &mut SqliteConnection,
    records: &[FastaSeqResult],
    run_id: u32,
) -> Result<(), sqlx::Error> {
    for chunk in records.chunks(1000) {
        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO records \
            (run_id, seq_id, description, gc_content, n_orfs, seq_len) ",
        );

        builder.push_values(chunk, |mut row, record| {
            row.push_bind(run_id)
                .push_bind(&record.id)
                .push_bind(&record.desc)
                .push_bind(record.gc)
                .push_bind(record.n_orfs)
                .push_bind(record.seq_len);
        });

        builder.build().execute(&mut *conn).await?;
    }

    Ok(())
}

pub async fn list_records_for_run<T>(pool: SqlitePool, run_id: u32, limit: u32, offset: u32) -> Result<Vec<T>, sqlx::Error>
where
    T: From<RecordEntity>,
{
    let records: Vec<T> = sqlx::query_as::<Sqlite, RecordEntity>(
        r#"
        SELECT id, run_id, seq_id, description, gc_content, n_orfs, seq_len, phred_score
        FROM records
        WHERE run_id = ?1
        ORDER BY id ASC
        LIMIT ?2 OFFSET ?3
        "#,
    )
    .bind(run_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await?
    .into_iter()
    .map(|r| T::from(r))
    .collect();

    Ok(records)
}

pub async fn count_records_for_run(pool: SqlitePool, run_id: u32) -> Result<u32, sqlx::Error> {
    sqlx::query_scalar::<Sqlite, u32>(r#"SELECT COUNT(*) FROM records WHERE run_id = ?1"#)
        .bind(run_id)
        .fetch_one(&pool)
        .await
}
