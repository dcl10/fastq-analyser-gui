"use client";

import { ChevronLeftIcon, ChevronRightIcon } from "lucide-react";
import { Button } from "@/components/ui/button";

interface PageControlsProps {
  /** Zero-based page being shown. */
  page: number;
  totalPages: number;
  /** Disables both buttons, e.g. while a page is loading. */
  disabled?: boolean;
  onPageChange: (page: number) => void;
}

// Previous/Next buttons for a zero-based page; hidden when there's only one page to show
export function PageControls({ page, totalPages, disabled = false, onPageChange }: PageControlsProps) {
  if (totalPages <= 1) return null;

  return (
    <div className="flex items-center justify-between">
      <Button
        variant="outline"
        size="sm"
        disabled={disabled || page === 0}
        onClick={() => onPageChange(page - 1)}
      >
        <ChevronLeftIcon /> Previous
      </Button>
      <span className="text-sm text-muted-foreground tabular-nums">
        Page {(page + 1).toLocaleString()} of {totalPages.toLocaleString()}
      </span>
      <Button
        variant="outline"
        size="sm"
        disabled={disabled || page >= totalPages - 1}
        onClick={() => onPageChange(page + 1)}
      >
        Next <ChevronRightIcon />
      </Button>
    </div>
  );
}
