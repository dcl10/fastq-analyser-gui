use crate::models::Run as RunModel;

#[tauri::command]
pub fn save_run(run: RunModel) -> Result<(), sqlx::Error> {
    todo!()
}

#[tauri::command]
pub fn load_run(run_id: u32) -> Result<RunModel, sqlx::Error> {
    todo!()
}
