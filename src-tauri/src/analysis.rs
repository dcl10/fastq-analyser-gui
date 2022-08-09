#[derive(Default)]
struct Result {
  gc: f32,
  n_orfs: i32,
}


#[tauri::command]
pub fn analyse_sequences(sequences: String) -> Result {
  Result::default()
}