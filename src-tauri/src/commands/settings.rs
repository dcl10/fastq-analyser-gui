use tauri::AppHandle;

use crate::{
    models::ThreadSettings,
    options::threads::{max_threads, ThreadOptions},
    services::settings::{load_thread_options, save_thread_options, SETTINGS_STORE},
};

#[tauri::command]
pub fn get_thread_settings(app: AppHandle) -> Result<ThreadSettings, String> {
    let options = load_thread_options(&app, SETTINGS_STORE).map_err(|e| e.to_string())?;
    Ok(ThreadSettings { thread_count: options.thread_count, max_threads: max_threads() })
}

/// Rayon's global pool can only be built once per process, so a new thread count needs a
/// restart: persist first, then restart, or the new process starts with the old value.
#[tauri::command]
pub fn save_thread_settings(app: AppHandle, thread_count: Option<u32>) -> Result<(), String> {
    let options = ThreadOptions { thread_count };
    options.validate()?;
    save_thread_options(&app, SETTINGS_STORE, options).map_err(|e| e.to_string())?;
    app.request_restart();
    Ok(())
}
