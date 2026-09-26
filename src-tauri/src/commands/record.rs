use tauri::State;

use crate::{AppState, data::entities::ResultType, models::{FastaSeqResult, FastqSeqResult, RunRecords}, services::record};

#[tauri::command]
pub async fn list_records_for_run(state: State<'_, AppState>, run_id: u32, result_type: ResultType, page: u32) -> Result<RunRecords, String> {
    let (limit, offset) = state.pagination_options.limit_offset(page);
    match result_type {
        ResultType::Fasta => {
            let records = record::list_records_for_run::<FastaSeqResult>(state.db.clone(), run_id, limit, offset)
                .await
                .map_err(|e| e.to_string())?;
            Ok(RunRecords::FastaRecords(records))
        },
        ResultType::Fastq => {
            let records = record::list_records_for_run::<FastqSeqResult>(state.db.clone(), run_id, limit, offset)
                .await
                .map_err(|e| e.to_string())?;
            Ok(RunRecords::FastqRecords(records))
        },
    }
}