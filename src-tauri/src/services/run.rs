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

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    use flate2::write::GzEncoder;
    use flate2::Compression;
    use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};
    use tauri::async_runtime::block_on;
    use uuid::Uuid;

    use super::{count_runs, create_run_from_file, list_runs};
    use crate::data::entities::ResultType;
    use crate::models::{FastqSeqResult, ImportProgress};
    use crate::services::record::{count_records_for_run, list_records_for_run};

    async fn test_pool() -> SqlitePool {
        let options = SqliteConnectOptions::from_str("sqlite::memory:").unwrap().foreign_keys(true);
        // One connection, since each in-memory connection is a separate database
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    fn fastq_input(n: usize) -> String {
        (1..=n).map(|i| format!("@id{i}\nATGC\n+\n!!!!\n")).collect()
    }

    fn fasta_input(n: usize) -> String {
        (1..=n).map(|i| format!(">id{i}\nATGC\n")).collect()
    }

    fn write_file(extension: &str, contents: &str) -> PathBuf {
        let path = PathBuf::from(format!("test_run_{}.{extension}", Uuid::new_v4()));
        let file = std::fs::File::create(&path).unwrap();
        if extension.ends_with(".gz") {
            let mut encoder = GzEncoder::new(file, Compression::default());
            encoder.write_all(contents.as_bytes()).unwrap();
            encoder.finish().unwrap();
        } else {
            let mut file = file;
            file.write_all(contents.as_bytes()).unwrap();
        }
        path
    }

    fn remove_file(path: &Path) {
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_create_run_from_file_saves_every_batch() {
        block_on(async {
            let pool = test_pool().await;
            let path = write_file("fq", &fastq_input(25));
            let file_len = std::fs::metadata(&path).unwrap().len();
            let progress = RefCell::new(Vec::<ImportProgress>::new());

            let run_id = create_run_from_file(pool.clone(), path.clone(), ResultType::Fastq, 10, |p| {
                progress.borrow_mut().push(p)
            })
            .await;
            remove_file(&path);
            let run_id = run_id.unwrap();

            assert_eq!(count_records_for_run(pool.clone(), run_id).await.unwrap(), 25);
            let records: Vec<FastqSeqResult> = list_records_for_run(pool.clone(), run_id, 100, 0).await.unwrap();
            let ids: Vec<String> = records.into_iter().map(|r| r.id).collect();
            let expected: Vec<String> = (1..=25).map(|i| format!("id{i}")).collect();
            assert_eq!(ids, expected);

            let progress = progress.into_inner();
            let counts: Vec<u64> = progress.iter().map(|p| p.records).collect();
            assert_eq!(counts, vec![10, 20, 25]);
            let last = progress.last().unwrap();
            assert_eq!((last.bytes_read, last.total_bytes), (file_len, file_len));
        });
    }

    #[test]
    fn test_create_run_from_file_gzipped_fasta() {
        block_on(async {
            let pool = test_pool().await;
            let path = write_file("fa.gz", &fasta_input(5));

            let run_id = create_run_from_file(pool.clone(), path.clone(), ResultType::Fasta, 10, |_| {}).await;
            remove_file(&path);
            let run_id = run_id.unwrap();

            assert_eq!(count_records_for_run(pool.clone(), run_id).await.unwrap(), 5);
            let runs = list_runs(pool.clone(), 10, 0).await.unwrap();
            assert_eq!(runs.len(), 1);
            assert_eq!(runs[0].result_type, ResultType::Fasta);
        });
    }

    #[test]
    fn test_create_run_from_file_error_saves_nothing() {
        block_on(async {
            let pool = test_pool().await;
            // Record 15's quality is shorter than its sequence, so it fails in the second batch of 10, after the
            // first batch has been inserted
            let mut input = fastq_input(14);
            input.push_str("@bad\nATGC\n+\n!!\n");
            input.push_str(&fastq_input(5));
            let path = write_file("fq", &input);

            let result = create_run_from_file(pool.clone(), path.clone(), ResultType::Fastq, 10, |_| {}).await;
            remove_file(&path);

            let err = result.unwrap_err();
            assert!(err.starts_with("Record 15 (bad)"), "unexpected error: {err}");
            assert_eq!(count_runs(pool.clone()).await.unwrap(), 0);
            let records: u32 = sqlx::query_scalar("SELECT COUNT(*) FROM records").fetch_one(&pool).await.unwrap();
            assert_eq!(records, 0);
        });
    }

    #[test]
    fn test_create_run_from_file_wrong_format_saves_nothing() {
        block_on(async {
            let pool = test_pool().await;
            let path = write_file("fq", &fasta_input(3));

            let result = create_run_from_file(pool.clone(), path.clone(), ResultType::Fastq, 10, |_| {}).await;
            remove_file(&path);

            let err = result.unwrap_err();
            assert!(err.contains("expected '@'"), "unexpected error: {err}");
            assert_eq!(count_runs(pool.clone()).await.unwrap(), 0);
        });
    }

    #[test]
    fn test_create_run_from_file_missing() {
        block_on(async {
            let pool = test_pool().await;
            let path = PathBuf::from(format!("test_run_{}.fq", Uuid::new_v4()));

            let result = create_run_from_file(pool.clone(), path, ResultType::Fastq, 10, |_| {}).await;

            assert!(result.is_err());
            assert_eq!(count_runs(pool.clone()).await.unwrap(), 0);
        });
    }

    // Peak-memory and timing benchmark for analysing and saving a large FASTQ file into an on-disk WAL database, as
    // the app does. Run with:
    // BENCH_FILE=<path> cargo test --release services::run::tests::bench_create_run_from_file -- --ignored --exact
    #[test]
    #[ignore]
    fn bench_create_run_from_file() {
        let path = std::env::var("BENCH_FILE").expect("set BENCH_FILE to a FASTQ file");
        let db_path = std::env::temp_dir().join(format!("bench_run_{}.sqlite", Uuid::new_v4()));
        let result = block_on(async {
            let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path.display()))
                .unwrap()
                .create_if_missing(true)
                .foreign_keys(true)
                .journal_mode(SqliteJournalMode::Wal);
            let pool = SqlitePoolOptions::new().connect_with(options).await.unwrap();
            sqlx::migrate!("./migrations").run(&pool).await.unwrap();

            let start = std::time::Instant::now();
            let last = RefCell::new(None);
            let run_id = create_run_from_file(pool.clone(), PathBuf::from(&path), ResultType::Fastq, crate::analysis::analysers::BATCH_SIZE, |p| {
                *last.borrow_mut() = Some(p)
            })
            .await
            .unwrap();
            println!("saved run {run_id} in {:.1?}: {:?}", start.elapsed(), last.into_inner());
            pool.close().await;
        });
        for suffix in ["", "-wal", "-shm"] {
            let _ = std::fs::remove_file(format!("{}{suffix}", db_path.display()));
        }
        result
    }
}
