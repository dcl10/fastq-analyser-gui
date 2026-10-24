import { invoke } from "@tauri-apps/api/core";
import type { ThreadSettings } from "@/types/settings";

// Load the persisted analysis thread count and the core count that bounds it
export async function getThreadSettings(): Promise<ThreadSettings> {
  return invoke<ThreadSettings>("get_thread_settings");
}

// Save the analysis thread count (null for "Auto"); the app restarts to apply it
export async function saveThreadSettings(threadCount: number | null): Promise<void> {
  return invoke<void>("save_thread_settings", { threadCount });
}
