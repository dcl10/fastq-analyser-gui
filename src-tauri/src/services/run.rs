use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use chrono::{DateTime, Utc};
use sqlx::{Sqlite, SqliteConnection, SqlitePool};
use tokio::sync::mpsc;

use crate::analysis::analysers::{analyse_fasta_reader_batches, analyse_fastq_reader_batches};
use crate::data::entities::{ResultType, Run as RunEntity};
use crate::models::RunRecords::{FastaRecords, FastqRecords};
use crate::models::{FastaSeqResult, FastqSeqResult, ImportProgress, Run as RunModel, RunRecords};
use crate::services::io::{read_fasta_streaming_counted, read_fastq_streaming_counted, DEFAULT_READ_CAPACITY};
use crate::services::record::{
    create_records, create_records_from_fasta_results, create_records_from_fastq_results, list_records_for_run,
};

/// Number of analysed batches that can wait to be inserted, so analysis runs a little ahead of the inserts without
/// piling results up in memory.
const IMPORT_QUEUE_CAPACITY: usize = 2;

pub async fn create_run(pool: SqlitePool, run: RunModel) -> Result<u32, sqlx::Error> {
    let RunEntity {
        id: _,
        created_at,
        records,
        result_type,
    } = RunEntity::from(run);
    let records = records.unwrap_or_default();

    let mut tx = pool.begin().await?;

    let run_id = insert_run(&mut tx, &created_at, &result_type).await?.id;
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

/// Analyses the file at `path` and saves its results as a new run, returning the run's id. Each batch of
/// `batch_size` results is inserted as soon as it's analysed instead of collecting them all first, so memory stays
/// at about a batch however big the file is. Everything happens in one transaction, so a failure part-way through
/// saves nothing. `on_progress` is called after each batch is inserted.
pub async fn create_run_from_file<F>(
    pool: SqlitePool,
    path: PathBuf,
    result_type: ResultType,
    batch_size: usize,
    on_progress: F,
) -> Result<u32, String>
where
    F: Fn(ImportProgress),
{
    let total_bytes = std::fs::metadata(&path).map_err(|e| e.to_string())?.len();
    let bytes_read = Arc::new(AtomicU64::new(0));
    let (sender, mut receiver) = mpsc::channel(IMPORT_QUEUE_CAPACITY);

    // Parsing and analysis block (rayon), so they run on a blocking thread and stream batches back here
    let analysis = {
        let result_type = result_type.clone();
        let bytes_read = bytes_read.clone();
        tauri::async_runtime::spawn_blocking(move || {
            analyse_file_batches(&path, result_type, batch_size, bytes_read, sender)
        })
    };

    // Returning early drops `receiver`, which stops the analysis at its next batch
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let run_id = insert_run(&mut tx, &Utc::now(), &result_type)
        .await
        .map_err(|e| e.to_string())?
        .id;

    let mut records = 0;
    while let Some(batch) = receiver.recv().await {
        create_records(&mut tx, &batch, run_id).await.map_err(|e| e.to_string())?;
        records += batch.len() as u64;
        on_progress(ImportProgress {
            records,
            bytes_read: bytes_read.load(Ordering::Relaxed),
            total_bytes,
        });
    }

    // The queue only closes once the analysis has finished, so this reports any analysis error without waiting
    analysis.await.map_err(|e| e.to_string())??;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(run_id)
}

/// Analyses the file at `path` in batches, sending each batch's results to `sender`.
fn analyse_file_batches(
    path: &Path,
    result_type: ResultType,
    batch_size: usize,
    bytes_read: Arc<AtomicU64>,
    sender: mpsc::Sender<RunRecords>,
) -> Result<(), String> {
    // Only fails when the inserting side has stopped, and its own error is the one reported
    let send = |batch| sender.blocking_send(batch).map_err(|_| "Saving the run stopped".to_owned());
    match result_type {
        ResultType::Fasta => {
            let reader = read_fasta_streaming_counted(path, DEFAULT_READ_CAPACITY, bytes_read)
                .map_err(|e| e.to_string())?;
            analyse_fasta_reader_batches(reader, batch_size, |batch| send(FastaRecords(batch)))
        }
        ResultType::Fastq => {
            let reader = read_fastq_streaming_counted(path, DEFAULT_READ_CAPACITY, bytes_read)
                .map_err(|e| e.to_string())?;
            analyse_fastq_reader_batches(reader, batch_size, |batch| send(FastqRecords(batch)))
        }
    }
}

async fn insert_run(
    conn: &mut SqliteConnection,
    created_at: &DateTime<Utc>,
    result_type: &ResultType,
) -> Result<RunEntity, sqlx::Error> {
    sqlx::query_as::<Sqlite, RunEntity>(
        r#"
        INSERT INTO runs (created_at, result_type)
        VALUES (?1, ?2)
        RETURNING id, created_at, result_type
        "#,
    )
    .bind(created_at)
    .bind(result_type)
    .fetch_one(conn)
    .await
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

pub async fn count_runs(pool: SqlitePool) -> Result<u32, sqlx::Error> {
    sqlx::query_scalar::<Sqlite, u32>(r#"SELECT COUNT(*) FROM runs"#)
        .fetch_one(&pool)
        .await
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
