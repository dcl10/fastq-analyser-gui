use tauri::State;

use crate::{
    models::Run as RunModel,
    services::run::{create_run, get_run_with_records},
    AppState,
};

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
