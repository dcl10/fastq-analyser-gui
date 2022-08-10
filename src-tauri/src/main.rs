#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod analysis;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![analysis::analyse_sequences])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
