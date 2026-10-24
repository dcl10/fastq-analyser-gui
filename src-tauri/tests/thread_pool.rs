// Its own test binary: rayon's global pool can only be built once per process, and the unit
// tests in the library start the default pool through par_iter.

use tauri::test::{mock_builder, mock_context, noop_assets};
use uuid::Uuid;

#[test]
fn test_configure_thread_pool_applies_persisted_thread_count() {
    // What a previous run's save_thread_settings would have left on disk
    let path = std::env::temp_dir().join(format!("fastq-analyser-test-{}.json", Uuid::new_v4()));
    std::fs::write(&path, r#"{"thread_count": 1}"#).unwrap();

    let app = mock_builder()
        .plugin(tauri_plugin_store::Builder::default().build())
        .build(mock_context(noop_assets()))
        .expect("Failed to build mock app");
    app_lib::configure_thread_pool(app.handle(), &path);

    assert_eq!(rayon::current_num_threads(), 1);
    std::fs::remove_file(path).unwrap();
}
