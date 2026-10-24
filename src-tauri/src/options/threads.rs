use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

/// ## Description
/// Get the most threads analysis may use: the number of cores the OS reports.
/// ## Returns
/// `u32` - `available_parallelism()`, or 1 if it can't be determined.
pub fn max_threads() -> u32 {
    std::thread::available_parallelism()
        .map(NonZeroUsize::get)
        .unwrap_or(1)
        .try_into()
        .unwrap_or(u32::MAX)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct ThreadOptions {
    /// Threads in rayon's global pool; `None` is "Auto" (`max_threads()`)
    pub thread_count: Option<u32>
}

impl ThreadOptions {
    /// ## Description
    /// Check the thread count is within `1..=max_threads()`.
    /// ## Returns
    /// `Result<(), String>` - An error describing the allowed range if it isn't.
    pub fn validate(&self) -> Result<(), String> {
        let max = max_threads();
        match self.thread_count {
            Some(n) if n == 0 || n > max => {
                Err(format!("Thread count must be between 1 and {max}, got {n}"))
            }
            _ => Ok(()),
        }
    }

    /// ## Description
    /// Get the number of threads to build rayon's global pool with.
    /// ## Returns
    /// `usize` - The thread count, or `max_threads()` for "Auto".
    pub fn resolve(&self) -> usize {
        self.thread_count.unwrap_or_else(max_threads) as usize
    }
}
