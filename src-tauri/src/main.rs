#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![increase, decrease])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

/// A tauri command to return the supplied number `num` + 1.
/// 
/// # Arguments:
/// * num: the number to be increased.
/// 
/// # Example
/// `example.js`
/// ```javascript
/// invoke('increase', {num: 1})
/// // 2
/// ```
#[tauri::command]
fn increase(num: i32) -> i32 {
  num + 1
} 

/// A tauri command to return the supplied number `num` - 1.
/// 
/// # Arguments:
/// * num: the number to be decreased.
/// 
/// # Example
/// `example.js`
/// ```javascript
/// invoke('decrease', {num: 1})
/// // 0
/// ```
#[tauri::command]
fn decrease(num: i32) -> i32 {
  num - 1
} 
