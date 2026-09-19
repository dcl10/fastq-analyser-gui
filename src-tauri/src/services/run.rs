use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::data::entities::{Record as RecordEntity, Run as RunEntity};
use crate::models::Run as RunModel;

pub async fn create_run(pool: SqlitePool, run: RunModel) -> Result<u32, sqlx::Error> {
    let RunEntity { id: _, created_at, records, result_type } = RunEntity::from(run);
    let records = records.unwrap_or_default();
    
    let mut tx = pool.begin().await?;

    let inserted_run = sqlx::query_as::<Sqlite, RunEntity>(
        r#"
        INSERT INTO runs (created_at, result_type)
        VALUES (?1, ?2)
        RETURNING id, created_at, result_type
        "#
    )
    .bind(&created_at)
    .bind(&result_type)
    .fetch_one(&mut *tx)
    .await?;

    let run_id = inserted_run.id;
    let mut saved_records: Vec<RecordEntity> = Vec::with_capacity(records.len());

    for chunk in records.chunks(1000) {
        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO records \
            (run_id, seq_id, description, gc_content, n_orfs, is_valid, seq_len, phred_score) "
        );

        builder.push_values(chunk, |mut row, record| {
            row.push_bind(run_id)
                .push_bind(&record.seq_id)
                .push_bind(&record.description)
                .push_bind(record.gc_content)
                .push_bind(record.n_orfs)
                .push_bind(record.is_valid)
                .push_bind(record.seq_len)
                .push_bind(record.phred_score);
        });

        let mut saved = builder
        .build_query_as::<RecordEntity>()
        .fetch_all(&mut *tx)
        .await?;

        saved_records.append(&mut saved);
    }

    tx.commit().await?;
    Ok(run_id)
}