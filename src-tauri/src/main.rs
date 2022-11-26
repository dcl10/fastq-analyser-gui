#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod analysis;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            analysis::analyse_fastq_sequences,
            analysis::analyse_fastq_file,
            analysis::analyse_fasta_sequences,
            analysis::analyse_fasta_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
