use std::path::Path;

use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

use crate::options::threads::ThreadOptions;

/// Settings store file, relative to the app data directory
pub const SETTINGS_STORE: &str = "settings.json";
const THREAD_COUNT_KEY: &str = "thread_count";

/// ## Description
/// Load the persisted thread options. A missing, malformed or out-of-range value falls back
/// to the default ("Auto") so a bad settings file can't stop the app starting.
/// ## Arguments
/// - `app`: `&AppHandle<R>` - The app handle; generic so tests can pass a mock app.
/// - `store_path`: `impl AsRef<Path>` - The store file, relative to the app data directory.
/// ## Returns
/// `Result<ThreadOptions, tauri_plugin_store::Error>` - The options, or an error if the store
/// can't be opened.
pub fn load_thread_options<R: Runtime>(
    app: &AppHandle<R>,
    store_path: impl AsRef<Path>,
) -> Result<ThreadOptions, tauri_plugin_store::Error> {
    let store = app.store(store_path)?;
    let Some(value) = store.get(THREAD_COUNT_KEY) else {
        return Ok(ThreadOptions::default());
    };
    let options = serde_json::from_value::<Option<u32>>(value)
        .map_err(|e| e.to_string())
        .map(|thread_count| ThreadOptions { thread_count })
        .and_then(|options| options.validate().map(|_| options));
    Ok(options.unwrap_or_else(|e| {
        log::warn!("Ignoring stored thread count: {e}");
        ThreadOptions::default()
    }))
}

/// ## Description
/// Persist the thread options, writing the store to disk before returning so a restart
/// straight afterwards picks them up.
/// ## Arguments
/// - `app`: `&AppHandle<R>` - The app handle; generic so tests can pass a mock app.
/// - `store_path`: `impl AsRef<Path>` - The store file, relative to the app data directory.
/// - `options`: `ThreadOptions` - The options to save.
/// ## Returns
/// `Result<(), tauri_plugin_store::Error>` - An error if the store can't be opened or written.
pub fn save_thread_options<R: Runtime>(
    app: &AppHandle<R>,
    store_path: impl AsRef<Path>,
    options: ThreadOptions,
) -> Result<(), tauri_plugin_store::Error> {
    let store = app.store(store_path)?;
    store.set(THREAD_COUNT_KEY, serde_json::json!(options.thread_count));
    store.save()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use tauri::test::{mock_builder, mock_context, noop_assets, MockRuntime};
    use uuid::Uuid;

    use super::*;

    fn mock_app() -> tauri::App<MockRuntime> {
        mock_builder()
            .plugin(tauri_plugin_store::Builder::default().build())
            .build(mock_context(noop_assets()))
            .expect("Failed to build mock app")
    }

    // An absolute path, so the store doesn't land in the (unset) mock app data directory
    fn temp_store_path() -> PathBuf {
        std::env::temp_dir().join(format!("fastq-analyser-test-{}.json", Uuid::new_v4()))
    }

    #[test]
    fn test_load_thread_options_defaults_when_unset() {
        let app = mock_app();
        let options = load_thread_options(app.handle(), temp_store_path()).unwrap();
        assert_eq!(options, ThreadOptions::default());
    }

    #[test]
    fn test_save_then_load_thread_options() {
        let app = mock_app();
        let path = temp_store_path();
        let saved = ThreadOptions { thread_count: Some(1) };

        save_thread_options(app.handle(), &path, saved).unwrap();
        assert!(path.exists());
        assert_eq!(load_thread_options(app.handle(), &path).unwrap(), saved);

        save_thread_options(app.handle(), &path, ThreadOptions::default()).unwrap();
        assert_eq!(load_thread_options(app.handle(), &path).unwrap(), ThreadOptions::default());

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_load_thread_options_ignores_invalid_value() {
        let path = temp_store_path();
        std::fs::write(&path, r#"{"thread_count": 0}"#).unwrap();

        let app = mock_app();
        assert_eq!(load_thread_options(app.handle(), &path).unwrap(), ThreadOptions::default());

        std::fs::remove_file(path).unwrap();
    }
}
