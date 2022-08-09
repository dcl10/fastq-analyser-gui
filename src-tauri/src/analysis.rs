use tauri::Result;
use serde::{Deserialize, Serialize};

#[derive(Default)]
#[derive(Deserialize, Serialize)]
pub struct SeqResult {
  gc: f32,
  n_orfs: i32,
}


#[tauri::command]
pub fn analyse_sequences(sequences: String) -> Result<SeqResult> {
  todo!()
}