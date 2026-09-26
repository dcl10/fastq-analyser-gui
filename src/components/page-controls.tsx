"use client";

import { ChevronLeftIcon, ChevronRightIcon } from "lucide-react";
import { Button } from "@/components/ui/button";

interface PageControlsProps {
  /** Zero-based page being shown. */
  page: number;
  hasNextPage: boolean;
  /** Disables both buttons, e.g. while a page is loading. */
  disabled?: boolean;
  onPageChange: (page: number) => void;
}

// Previous/Next buttons for a zero-based page; hidden when there's only one page to show
export function PageControls({ page, hasNextPage, disabled = false, onPageChange }: PageControlsProps) {
  if (page === 0 && !hasNextPage) return null;

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
      <span className="text-sm text-muted-foreground">Page {page + 1}</span>
      <Button
        variant="outline"
        size="sm"
        disabled={disabled || !hasNextPage}
        onClick={() => onPageChange(page + 1)}
      >
        Next <ChevronRightIcon />
      </Button>
    </div>
  );
}
