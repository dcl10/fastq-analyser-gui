use sqlx::SqlitePool;
use tauri::Manager;

mod analysis;
mod commands;
mod data;
mod models;
mod services;

pub struct AppState {
    pub db: SqlitePool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let pool = data::db::init_db(&handle)
                    .await
                    .expect("Failed to initialise database");
                handle.manage(AppState { db: pool });
                Ok(())
            })
        })
        .invoke_handler(tauri::generate_handler![
            commands::analysis::analyse_fastq_sequences,
            commands::analysis::analyse_fastq_file,
            commands::analysis::analyse_fasta_sequences,
            commands::analysis::analyse_fasta_file,
            commands::run::delete_run,
            commands::run::list_runs,
            commands::run::load_run,
            commands::run::save_run,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
