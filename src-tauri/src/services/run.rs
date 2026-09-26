use sqlx::{Sqlite, SqlitePool};

use crate::data::entities::{ResultType, Run as RunEntity};
use crate::models::RunRecords::{FastaRecords, FastqRecords};
use crate::models::{FastaSeqResult, FastqSeqResult, Run as RunModel};
use crate::services::record::{
    create_records_from_fasta_results, create_records_from_fastq_results, list_records_for_run,
};

pub async fn create_run(pool: SqlitePool, run: RunModel) -> Result<u32, sqlx::Error> {
    let RunEntity {
        id: _,
        created_at,
        records,
        result_type,
    } = RunEntity::from(run);
    let records = records.unwrap_or_default();

    let mut tx = pool.begin().await?;

    let inserted_run = sqlx::query_as::<Sqlite, RunEntity>(
        r#"
        INSERT INTO runs (created_at, result_type)
        VALUES (?1, ?2)
        RETURNING id, created_at, result_type
        "#,
    )
    .bind(&created_at)
    .bind(&result_type)
    .fetch_one(&mut *tx)
    .await?;

    let run_id = inserted_run.id;
    if records.len() > 0 {
        match result_type {
            ResultType::Fasta => {
                let records: Vec<FastaSeqResult> = records.into_iter().map(|r| r.into()).collect();
                let _ = create_records_from_fasta_results(&mut tx, &records, run_id).await?;
            }
            ResultType::Fastq => {
                let records: Vec<FastqSeqResult> = records.into_iter().map(|r| r.into()).collect();
                let _ = create_records_from_fastq_results(&mut tx, &records, run_id).await?;
            }
        }
    }

    tx.commit().await?;
    Ok(run_id)
}

pub async fn list_runs(pool: SqlitePool, limit: u32, offset: u32) -> Result<Vec<RunModel>, sqlx::Error> {
    let runs: Vec<RunModel> = sqlx::query_as::<Sqlite, RunEntity>(
        r#"
        SELECT id, created_at, result_type
        FROM runs
        ORDER BY created_at DESC
        LIMIT ?1 OFFSET ?2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await?
    .into_iter()
    .map(|r| RunModel::from(r))
    .collect();

    Ok(runs)
}

pub async fn get_run_with_records(pool: SqlitePool, run_id: u32, limit: u32, offset: u32) -> Result<RunModel, sqlx::Error> {
    let mut run: RunModel = sqlx::query_as::<Sqlite, RunEntity>(
        r#"
        SELECT id, created_at, result_type
        FROM runs
        WHERE id = ?1
        "#,
    )
    .bind(&run_id)
    .fetch_one(&pool)
    .await?
    .into();

    match run.result_type {
        ResultType::Fasta => {
            let records: Vec<FastaSeqResult> = list_records_for_run(pool, run_id, limit, offset).await?;
            run.records = FastaRecords(records);
            Ok(run)
        }
        ResultType::Fastq => {
            let records: Vec<FastqSeqResult> = list_records_for_run(pool, run_id, limit, offset).await?;
            run.records = FastqRecords(records);
            Ok(run)
        }
    }
}

pub async fn delete_run(pool: SqlitePool, run_id: u32) -> Result<(), sqlx::Error> {
    sqlx::query::<Sqlite>(r#"DELETE FROM runs WHERE id = ?1"#)
        .bind(&run_id)
        .execute(&pool)
        .await?;

    Ok(())
}
