use std::path::PathBuf;

use tauri::ipc::Channel;
use tauri::State;

use crate::{
    analysis::analysers::BATCH_SIZE,
    data::entities::ResultType,
    models::{ImportProgress, Run as RunModel},
    services::run::{create_run, create_run_from_file, get_run_with_records},
    AppState,
};

/// Analyses a file and saves it as a new run in one step, returning the run id. The results never cross to the
/// frontend, which instead gets an `ImportProgress` on `on_progress` after each batch is saved.
#[tauri::command]
pub async fn analyse_and_save_file(
    state: State<'_, AppState>,
    path: PathBuf,
    result_type: ResultType,
    on_progress: Channel<ImportProgress>,
) -> Result<u32, String> {
    create_run_from_file(state.db.clone(), path, result_type, BATCH_SIZE, |progress| {
        if let Err(e) = on_progress.send(progress) {
            log::warn!("Couldn't send import progress: {e}");
        }
    })
    .await
}

#[tauri::command]
pub async fn save_run(state: State<'_, AppState>, run: RunModel) -> Result<u32, String> {
    create_run(state.db.clone(), run)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_run(state: State<'_, AppState>, run_id: u32) -> Result<RunModel, String> {
    let (limit, offset) = state.pagination_options.limit_offset(0);
    get_run_with_records(state.db.clone(), run_id, limit, offset)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_run(state: State<'_, AppState>, run_id: u32) -> Result<(), String> {
    crate::services::run::delete_run(state.db.clone(), run_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_runs(state: State<'_, AppState>, page: u32) -> Result<Vec<RunModel>, String> {
    let (limit, offset) = state.pagination_options.limit_offset(page);
    crate::services::run::list_runs(state.db.clone(), limit, offset)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn count_run_pages(state: State<'_, AppState>) -> Result<u32, String> {
    let total = crate::services::run::count_runs(state.db.clone())
        .await
        .map_err(|e| e.to_string())?;
    Ok(state.pagination_options.total_pages(total))
}
