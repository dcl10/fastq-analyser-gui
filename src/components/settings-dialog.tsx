"use client";

import { useState } from "react";
import { SettingsIcon } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { LoadingIndicator } from "@/components/loading-indicator";
import { getThreadSettings, saveThreadSettings } from "@/lib/settings";
import { pluralise } from "@/lib/utils";
import type { ThreadSettings } from "@/types/settings";

/** Title-bar gear that opens the settings dialog; currently just the analysis thread count. */
export function SettingsDialog() {
  const [open, setOpen] = useState(false);
  const [saved, setSaved] = useState<ThreadSettings | null>(null);
  const [isAuto, setIsAuto] = useState(true);
  // Kept as text so the field can be cleared while typing
  const [threadCount, setThreadCount] = useState("");
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState("");

  // Reload the persisted setting each time the dialog opens, discarding any unsaved edits
  const handleOpenChange = (nextOpen: boolean) => {
    if (isSaving) return;
    setOpen(nextOpen);
    if (!nextOpen) return;
    setSaved(null);
    setError("");
    getThreadSettings()
      .then((settings) => {
        setSaved(settings);
        setIsAuto(settings.thread_count === null);
        setThreadCount(String(settings.thread_count ?? settings.max_threads));
      })
      .catch((e) => setError(`Couldn't load settings: ${e}`));
  };

  const parsed = Number(threadCount);
  const isValid =
    isAuto || (Number.isInteger(parsed) && parsed >= 1 && parsed <= (saved?.max_threads ?? 0));
  const draft = isAuto ? null : parsed;
  const isChanged = saved !== null && draft !== saved.thread_count;

  // Persisting triggers a restart from the Rust side, so on success this window goes away
  const save = async () => {
    setError("");
    setIsSaving(true);
    try {
      await saveThreadSettings(draft);
    } catch (e) {
      setError(`Couldn't save settings: ${e}`);
      setIsSaving(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <button
        type="button"
        aria-label="Settings"
        title="Settings"
        onClick={() => handleOpenChange(true)}
        className="inline-flex size-7 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
      >
        <SettingsIcon className="size-3.5" />
      </button>

      <DialogContent showCloseButton={!isSaving}>
        <DialogHeader>
          <DialogTitle>Settings</DialogTitle>
          <DialogDescription>
            Saving restarts FastQ Analyser to apply the new thread count.
          </DialogDescription>
        </DialogHeader>

        {saved === null ? (
          error ? (
            <p role="alert" className="text-sm text-destructive">
              {error}
            </p>
          ) : (
            <LoadingIndicator message="Loading settings..." />
          )
        ) : (
          <div className="flex flex-col gap-4">
            <Label>
              <Switch checked={isAuto} onCheckedChange={setIsAuto} disabled={isSaving} />
              Auto ({pluralise(saved.max_threads, "thread")})
            </Label>
            {isAuto ? null : (
              <div className="flex flex-col gap-2">
                <Label htmlFor="thread-count">Analysis threads</Label>
                <Input
                  id="thread-count"
                  type="number"
                  min={1}
                  max={saved.max_threads}
                  step={1}
                  value={threadCount}
                  onChange={(e) => setThreadCount(e.target.value)}
                  disabled={isSaving}
                  aria-invalid={!isValid}
                />
                <p className="text-xs text-muted-foreground">
                  Between 1 and {saved.max_threads}, the number of cores on this machine.
                </p>
              </div>
            )}
            {error ? (
              <p role="alert" className="text-sm text-destructive">
                {error}
              </p>
            ) : null}
          </div>
        )}

        <DialogFooter>
          <Button variant="outline" onClick={() => handleOpenChange(false)} disabled={isSaving}>
            Cancel
          </Button>
          <Button onClick={save} disabled={!isChanged || !isValid || isSaving}>
            {isSaving ? "Restarting..." : "Save and restart"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
