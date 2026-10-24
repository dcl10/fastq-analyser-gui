// Mirrors the Rust ThreadSettings type in src-tauri/src/models.rs, as serde serializes it

export interface ThreadSettings {
  /** Persisted thread count; null is "Auto" (every core). */
  thread_count: number | null;
  /** Upper bound for thread_count: the number of cores. */
  max_threads: number;
}
