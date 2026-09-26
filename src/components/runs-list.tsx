"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";
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
import { PageControls } from "@/components/page-controls";
import { Toolbar } from "@/components/toolbar";
import { countRunPages, deleteRun, listRuns } from "@/lib/runs";
import type { Run } from "@/types/runs";

const runDetailHref = (run: Run) => `/runs/detail?id=${run.id}`;

export function RunsList() {
  const router = useRouter();
  // Zero-based page of runs being shown
  const [page, setPage] = useState(0);
  const [loaded, setLoaded] = useState<{
    page: number;
    runs?: Run[];
    totalPages?: number;
    error?: string;
  } | null>(null);
  const [runToDelete, setRunToDelete] = useState<Run | null>(null);
  const [isDeleting, setIsDeleting] = useState(false);
  const [deleteError, setDeleteError] = useState("");

  // Load the current page of runs, newest first (list_runs sorts them), and the page count
  // whenever the page changes
  useEffect(() => {
    let cancelled = false;
    Promise.all([listRuns(page), countRunPages()])
      .then(([runs, totalPages]) => !cancelled && setLoaded({ page, runs, totalPages }))
      .catch((e) => !cancelled && setLoaded({ page, error: `Couldn't load runs: ${e}` }));
    // Drop a slow response for a page the user has already moved away from
    return () => {
      cancelled = true;
    };
  }, [page]);

  // Ignore a result left over from the previous page until this one loads
  const current = loaded?.page === page ? loaded : null;
  const isLoading = current === null;
  const runs = current?.runs ?? [];
  const totalPages = current?.totalPages ?? 0;
  const error = current?.error ?? "";

  // Delete the run the user confirmed, then refetch the page and page count so later runs move
  // up to fill the gap
  const confirmDelete = async () => {
    if (!runToDelete) return;
    setDeleteError("");
    setIsDeleting(true);
    try {
      await deleteRun(runToDelete.id);
      const [refreshed, refreshedTotalPages] = await Promise.all([listRuns(page), countRunPages()]);
      // Deleting the only run on the last page leaves it empty, so step back to the new last page
      if (page > 0 && page >= refreshedTotalPages) setPage(refreshedTotalPages - 1);
      else setLoaded({ page, runs: refreshed, totalPages: refreshedTotalPages });
    } catch (e) {
      setDeleteError(`Couldn't delete run #${runToDelete.id}: ${e}`);
    } finally {
      setIsDeleting(false);
      setRunToDelete(null);
    }
  };

  return (
    <>
      <Toolbar title="Runs" />

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
                  <TableRow
                    key={run.id}
                    className="cursor-pointer"
                    onClick={() => router.push(runDetailHref(run))}
                  >
                    <TableCell className="font-mono tabular-nums">
                      {/* Keyboard route to the same page the row click opens */}
                      <Link href={runDetailHref(run)} className="hover:underline">
                        #{run.id}
                      </Link>
                    </TableCell>
                    <TableCell>{new Date(run.created_at).toLocaleString()}</TableCell>
                    <TableCell>{run.result_type.toUpperCase()}</TableCell>
                    <TableCell className="text-right">
                      <Button
                        variant="ghost"
                        size="icon-sm"
                        aria-label={`Delete run #${run.id}`}
                        className="text-destructive hover:bg-destructive/10 hover:text-destructive"
                        onClick={(e) => {
                          // Don't also open the run via the row click
                          e.stopPropagation();
                          setRunToDelete(run);
                        }}
                      >
                        <Trash2Icon />
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}

          <PageControls
            page={page}
            totalPages={totalPages}
            disabled={isLoading}
            onPageChange={setPage}
          />
        </div>
      </div>
    </>
  );
}
