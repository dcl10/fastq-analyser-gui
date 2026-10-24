use std::path::Path;

use sqlx::SqlitePool;
use tauri::{AppHandle, Manager, Runtime};

use crate::options::pagination::Pagination;
use crate::services::settings::{load_thread_options, SETTINGS_STORE};

mod analysis;
mod commands;
mod data;
mod models;
mod options;
mod services;

pub struct AppState {
    pub db: SqlitePool,
    pub pagination_options: Pagination
}

/// ## Description
/// Build rayon's global pool from the persisted thread count. The global pool can only be
/// built once per process and before the first `par_iter()`, so this runs in `setup()` and a
/// changed setting takes effect on the next start.
/// ## Arguments
/// - `app`: `&AppHandle<R>` - The app handle; generic so tests can pass a mock app.
/// - `store_path`: `impl AsRef<Path>` - The settings store file, relative to the app data directory.
pub fn configure_thread_pool<R: Runtime>(app: &AppHandle<R>, store_path: impl AsRef<Path>) {
    let options = load_thread_options(app, store_path).unwrap_or_else(|e| {
        log::error!("Couldn't open the settings store, using the default thread count: {e}");
        Default::default()
    });
    let threads = options.resolve();
    match rayon::ThreadPoolBuilder::new().num_threads(threads).build_global() {
        Ok(()) => log::info!("rayon global pool: {threads} threads"),
        Err(e) => log::error!("Couldn't build rayon's global pool with {threads} threads: {e}"),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        // A single setup(): each call replaces the previous closure rather than adding to it
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            configure_thread_pool(app.handle(), SETTINGS_STORE);

            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let pool = data::db::init_db(&handle)
                    .await
                    .expect("Failed to initialise database");
                handle.manage(AppState { db: pool, pagination_options: Pagination::default() });
                Ok(())
            })
        })
        .invoke_handler(tauri::generate_handler![
            commands::analysis::analyse_fastq_sequences,
            commands::analysis::analyse_fastq_file,
            commands::analysis::analyse_fasta_sequences,
            commands::analysis::analyse_fasta_file,
            commands::record::count_record_pages,
            commands::run::analyse_and_save_file,
            commands::record::list_records_for_run,
            commands::run::count_run_pages,
            commands::run::delete_run,
            commands::run::list_runs,
            commands::run::load_run,
            commands::run::save_run,
            commands::settings::get_thread_settings,
            commands::settings::save_thread_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
