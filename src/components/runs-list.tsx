"use client";

import { useEffect, useState } from "react";
import { Trash2Icon } from "lucide-react";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { Button } from "@/components/ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { LoadingIndicator } from "@/components/loading-indicator";
import { Toolbar } from "@/components/toolbar";
import { deleteRun, listRuns } from "@/lib/runs";
import type { Run } from "@/types/runs";

export function RunsList() {
  const [runs, setRuns] = useState<Run[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState("");
  const [runToDelete, setRunToDelete] = useState<Run | null>(null);
  const [isDeleting, setIsDeleting] = useState(false);
  const [deleteError, setDeleteError] = useState("");

  // Load every saved run once, newest first (list_runs sorts them)
  useEffect(() => {
    listRuns()
      .then(setRuns)
      .catch((e) => setError(`Couldn't load runs: ${e}`))
      .finally(() => setIsLoading(false));
  }, []);

  // Delete the run the user confirmed and drop it from the table
  const confirmDelete = async () => {
    if (!runToDelete) return;
    setDeleteError("");
    setIsDeleting(true);
    try {
      await deleteRun(runToDelete.id);
      setRuns((current) => current.filter((run) => run.id !== runToDelete.id));
    } catch (e) {
      setDeleteError(`Couldn't delete run #${runToDelete.id}: ${e}`);
    } finally {
      setIsDeleting(false);
      setRunToDelete(null);
    }
  };

  return (
    <>
      <Toolbar title="Runs" subtitle={isLoading ? undefined : `${runs.length} saved`} />

      <AlertDialog
        open={runToDelete !== null}
        onOpenChange={(open) => !open && !isDeleting && setRunToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete run #{runToDelete?.id}?</AlertDialogTitle>
            <AlertDialogDescription>
              This permanently deletes the run and all of its records. It can&apos;t be undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel disabled={isDeleting}>Cancel</AlertDialogCancel>
            <AlertDialogAction variant="destructive" onClick={confirmDelete} disabled={isDeleting}>
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <div className="min-h-0 flex-1 overflow-y-auto">
        <div className="mx-auto flex w-full max-w-2xl flex-col gap-6 px-4 py-10">
          {deleteError ? (
            <p role="alert" className="text-sm text-destructive">
              {deleteError}
            </p>
          ) : null}

          {isLoading ? (
            <LoadingIndicator message="Loading runs..." />
          ) : error ? (
            <p role="alert" className="text-sm text-destructive">
              {error}
            </p>
          ) : runs.length === 0 ? (
            <p className="text-sm text-muted-foreground">
              No runs yet. Import some sequences to create one.
            </p>
          ) : (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Run</TableHead>
                  <TableHead>Created</TableHead>
                  <TableHead>Type</TableHead>
                  <TableHead>
                    <span className="sr-only">Actions</span>
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {runs.map((run) => (
                  <TableRow key={run.id}>
                    <TableCell className="font-mono tabular-nums">#{run.id}</TableCell>
                    <TableCell>{new Date(run.created_at).toLocaleString()}</TableCell>
                    <TableCell>{run.result_type.toUpperCase()}</TableCell>
                    <TableCell className="text-right">
                      <Button
                        variant="ghost"
                        size="icon-sm"
                        aria-label={`Delete run #${run.id}`}
                        className="text-destructive hover:bg-destructive/10 hover:text-destructive"
                        onClick={() => setRunToDelete(run)}
                      >
                        <Trash2Icon />
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}
        </div>
      </div>
    </>
  );
}
