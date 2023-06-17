#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod analysis;
mod models;
mod services;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            analysis::commands::analyse_fastq_sequences,
            analysis::commands::analyse_fastq_file,
            analysis::commands::analyse_fasta_sequences,
            analysis::commands::analyse_fasta_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
